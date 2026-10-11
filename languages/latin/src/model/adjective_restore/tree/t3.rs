use flexion::Path;

/// Words ending -atus.
pub(crate) fn n483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3704);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 7025);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3705);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7026);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10842);
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
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 10843);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3707);
					match s {
						[] => true,
						['u', 'l', 'p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 3708);
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
								['g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'p', s @ ..] => {
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
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3709);
					match s {
						[] => true,
						['e', 'c', 'a', s @ ..] => {
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
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10844);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10845);
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
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
										['a', 'm', '\u{1}', s @ ..] => super::t5::n24101(s, out),
										['i', 'd', '\u{1}', s @ ..] => super::t5::n24102(s, out),
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => super::t5::n16572(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t5::n4457(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t5::n1639(s, out),
		['d', s @ ..] => super::t5::n1640(s, out),
		['e', s @ ..] => super::t5::n1641(s, out),
		['f', s @ ..] => super::t5::n1642(s, out),
		['g', s @ ..] => super::t5::n1643(s, out),
		['h', s @ ..] => super::t5::n1644(s, out),
		['i', s @ ..] => super::t5::n1645(s, out),
		['l', s @ ..] => super::t6::n1646(s, out),
		['m', s @ ..] => super::t6::n1647(s, out),
		['n', s @ ..] => super::t6::n1648(s, out),
		['o', s @ ..] => super::t6::n1649(s, out),
		['p', s @ ..] => super::t6::n1650(s, out),
		['r', s @ ..] => super::t6::n1651(s, out),
		['s', s @ ..] => super::t6::n1652(s, out),
		['t', s @ ..] => super::t6::n1653(s, out),
		['u', s @ ..] => super::t6::n1654(s, out),
		['v', s @ ..] => super::t6::n1655(s, out),
		['x', s @ ..] => super::t6::n1656(s, out),
		['z', s @ ..] => super::t6::n1657(s, out),
		_ => false,
	}
}

/// Words ending -ctus.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3856);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3857);
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
					super::push(out, 3858);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7472);
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
				['e', s @ ..] => {
					super::push(out, 3859);
					match s {
						[] => true,
						['t', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => super::t6::n4620(s, out),
				['i', s @ ..] => {
					super::push(out, 3861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7477);
							match s {
								[] => true,
								['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3864);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7478);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11383);
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
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7479);
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
				['r', s @ ..] => super::t6::n4626(s, out),
				['s', s @ ..] => {
					super::push(out, 3867);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t6::n4628(s, out),
				['x', s @ ..] => {
					super::push(out, 3869);
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
		['e', s @ ..] => super::t6::n1659(s, out),
		['i', s @ ..] => super::t6::n1660(s, out),
		['l', s @ ..] => {
			super::push(out, 1512);
			s.is_empty()
		}
		['n', s @ ..] => super::t6::n1662(s, out),
		['o', s @ ..] => super::t6::n1663(s, out),
		['u', s @ ..] => super::t6::n1664(s, out),
		_ => false,
	}
}

/// Words ending -ppletus.
pub(crate) fn n10366(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -netus.
pub(crate) fn n1671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1522);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3902);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
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

/// Words ending -oetus.
pub(crate) fn n1672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1523);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 3903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -petus.
pub(crate) fn n1673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1524);
	s.is_empty()
}

/// Words ending -retus.
pub(crate) fn n1674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1525);
	match s {
		[] => true,
		['c', s @ ..] => {
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
					super::push(out, 7570);
					match s {
						[] => true,
						['d', s @ ..] => {
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
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7571);
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
				['s', s @ ..] => {
					super::push(out, 7572);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -setus.
pub(crate) fn n1675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1526);
	match s {
		[] => true,
		['i', 'n', s @ ..] => {
			super::push(out, 3904);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 7573);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uetus.
pub(crate) fn n1676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1527);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7574);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7575);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11465);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 11466);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 11467);
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

/// Words ending -vetus.
pub(crate) fn n1677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1528);
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

/// Words ending -itus.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 7576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3906);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7578);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7579);
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
				['i', s @ ..] => {
					super::push(out, 3907);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11468);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['h', s @ ..] => {
							super::push(out, 7581);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11469);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 14704);
											s.is_empty()
										}
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
					super::push(out, 3908);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11470);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 11471);
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
					super::push(out, 3909);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7582);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11472);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11473);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3910);
					match s {
						[] => true,
						['c', s @ ..] => {
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
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11474);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1530);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3911);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11475);
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
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3912);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => super::t6::n10393(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t6::n4685(s, out),
				['n', 'o', 'c', '\u{1}', s @ ..] => super::t6::n4686(s, out),
				['o', s @ ..] => super::t6::n4687(s, out),
				['r', s @ ..] => super::t6::n4688(s, out),
				['s', s @ ..] => super::t6::n4689(s, out),
				['x', s @ ..] => super::t6::n4690(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t6::n1680(s, out),
		['e', s @ ..] => super::t6::n1681(s, out),
		['f', s @ ..] => super::t6::n1682(s, out),
		['g', s @ ..] => super::t6::n1683(s, out),
		['k', s @ ..] => super::t6::n1684(s, out),
		['l', s @ ..] => super::t6::n1685(s, out),
		['m', s @ ..] => super::t6::n1686(s, out),
		['n', s @ ..] => super::t6::n1687(s, out),
		['o', s @ ..] => super::t6::n1688(s, out),
		['p', s @ ..] => super::t6::n1689(s, out),
		['r', s @ ..] => super::t6::n1690(s, out),
		['s', s @ ..] => super::t6::n1691(s, out),
		['t', s @ ..] => super::t6::n1692(s, out),
		['u', s @ ..] => super::t6::n1693(s, out),
		['v', s @ ..] => super::t6::n1694(s, out),
		['x', s @ ..] => super::t6::n1695(s, out),
		_ => false,
	}
}

/// Words ending -ltus.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3980);
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
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
				['d', s @ ..] => {
					super::push(out, 3981);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
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
				['m', s @ ..] => {
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3983);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11537);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 11538);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 14743);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 14744);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3984);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7691);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11539);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11540);
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
							super::push(out, 7692);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
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
							super::push(out, 7693);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7694);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mtus.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1548);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntus.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1550);
			match s {
				[] => true,
				['c', s @ ..] => {
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
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1551);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3986);
					match s {
						[] => true,
						['s', 'e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7696);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7697);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'v', s @ ..] => {
									super::push(out, 11541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7698);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 11542);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 11543);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14745);
											match s {
												[] => true,
												['m', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['c', s @ ..] => {
											super::push(out, 14746);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14747);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 14748);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 14749);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 14750);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 16750);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 16751);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 14751);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 16752);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 11544);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 11545);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11546);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 14752);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													super::push(out, 16753);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 14753);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 14754);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 11547);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14755);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11548);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14756);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14757);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 14758);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 14759);
											s.is_empty()
										}
										['u', s @ ..] => super::t6::n25600(s, out),
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 11549);
									s.is_empty()
								}
								['t', s @ ..] => super::t6::n18102(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t6::n4774(s, out),
				['t', s @ ..] => super::t6::n4775(s, out),
				['u', s @ ..] => super::t6::n4776(s, out),
				['v', s @ ..] => super::t6::n4777(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t6::n1702(s, out),
		_ => false,
	}
}

/// Words ending -otus.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3993);
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
		['l', s @ ..] => {
			super::push(out, 1554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 1555);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3995);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3996);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3999);
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
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 4002);
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
			super::push(out, 1556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 11554);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14764);
											s.is_empty()
										}
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
					super::push(out, 4004);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['h', 't', 'n', 'a', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7705);
							s.is_empty()
						}
						['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7706);
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
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4006);
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
				['s', s @ ..] => {
					super::push(out, 4007);
					match s {
						[] => true,
						['e', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1558);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['q', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4009);
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

/// Words ending -ptus.
pub(crate) fn n491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1561);
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
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7708);
							match s {
								[] => true,
								['s', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1562);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11555);
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
				['c', s @ ..] => {
					super::push(out, 4011);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7709);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11556);
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
						['n', 'o', 'c', s @ ..] => {
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
						['r', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
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
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4012);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4013);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7710);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 11557);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 11559);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 11560);
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
							super::push(out, 7712);
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
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7713);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7714);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7715);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11561);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7716);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11563);
									s.is_empty()
								}
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
			super::push(out, 1563);
			match s {
				[] => true,
				['r', 'c', s @ ..] => super::t6::n4822(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['u', 'c', 's', s @ ..] => super::t6::n4823(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t6::n1715(s, out),
		['o', s @ ..] => {
			super::push(out, 1566);
			s.is_empty()
		}
		['r', s @ ..] => super::t6::n1717(s, out),
		['u', s @ ..] => super::t6::n1718(s, out),
		['y', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtus.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1570);
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
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7729);
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
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
				['s', s @ ..] => {
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
			super::push(out, 1571);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4023);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4024);
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
				['f', s @ ..] => {
					super::push(out, 4025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7730);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7731);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11585);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 11586);
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
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4026);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7732);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 14773);
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
						['m', 'o', 'c', s @ ..] => {
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
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', s @ ..] => {
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
						['x', 'e', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 4027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7733);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7734);
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
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7735);
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
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7736);
							match s {
								[] => true,
								['i', s @ ..] => super::t6::n18222(s, out),
								['o', s @ ..] => super::t6::n18223(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => super::t6::n10696(s, out),
						['x', s @ ..] => super::t6::n10697(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t6::n1722(s, out),
		['o', s @ ..] => super::t6::n1723(s, out),
		_ => false,
	}
}

/// Words ending -stus.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 464);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1574);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4028);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11591);
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
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4031);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4032);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4033);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
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
				['f', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7740);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7741);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7742);
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
				['g', s @ ..] => {
					super::push(out, 4035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11592);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11593);
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
						['g', s @ ..] => {
							super::push(out, 7744);
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
							super::push(out, 7745);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 7746);
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
					super::push(out, 4036);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'm', s @ ..] => {
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
				['m', s @ ..] => {
					super::push(out, 4037);
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
					super::push(out, 4038);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7747);
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
						['u', s @ ..] => {
							super::push(out, 7748);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4039);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11594);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 11595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['q', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 1576);
			match s {
				[] => true,
				['m', s @ ..] => super::t6::n4863(s, out),
				['p', s @ ..] => {
					super::push(out, 4042);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 4043);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t6::n1727(s, out),
		['p', s @ ..] => {
			super::push(out, 1578);
			match s {
				[] => true,
				['e', 'd', s @ ..] => super::t6::n4870(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t6::n1729(s, out),
		_ => false,
	}
}

/// Words ending -ttus.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 465);
	s.is_empty()
}

/// Words ending -utus.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 466);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7757);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7758);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11600);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7759);
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
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1582);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4057);
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
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4058);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 11602);
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
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11603);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 14776);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 14777);
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
									super::push(out, 11604);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 11605);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11606);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14778);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 14779);
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
								['l', s @ ..] => {
									super::push(out, 11607);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
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
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11608);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => super::t6::n18299(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t6::n1733(s, out),
		['e', s @ ..] => super::t6::n1734(s, out),
		['g', s @ ..] => super::t6::n1735(s, out),
		['i', s @ ..] => super::t6::n1736(s, out),
		['l', s @ ..] => super::t6::n1737(s, out),
		['n', s @ ..] => super::t6::n1738(s, out),
		['p', s @ ..] => super::t6::n1739(s, out),
		['r', s @ ..] => super::t6::n1740(s, out),
		['s', s @ ..] => super::t6::n1741(s, out),
		['t', s @ ..] => super::t6::n1742(s, out),
		['u', 'q', 'e', 's', s @ ..] => super::t6::n1743(s, out),
		_ => false,
	}
}

/// Words ending -xtus.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 467);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4087);
					match s {
						[] => true,
						['i', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 7777);
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
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11622);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11623);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ytus.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 468);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1595);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4088);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nuus.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4099);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4100);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11624);
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
		_ => false,
	}
}

/// Words ending -quus.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1606);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4101);
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
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							super::push(out, 7787);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 11625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 4102);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4103);
					match s {
						[] => true,
						['i', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 4104);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
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
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['i', 'p', 'o', 'r', 'p', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 1609);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7789);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 7790);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7791);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7792);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11628);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 11629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7793);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7794);
							match s {
								[] => true,
								['n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 11630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7795);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11631);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 11632);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 14785);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 14786);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 11633);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11634);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 7796);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11636);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11637);
									s.is_empty()
								}
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

/// Words ending -ruus.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4105);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4106);
					match s {
						[] => true,
						['n', 'e', 'm', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tuus.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1612);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1613);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['o', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7797);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 11638);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 11639);
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
							super::push(out, 7798);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
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
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7799);
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

/// Words ending -ivus.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 479);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1620);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'a', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1623);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1624);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7800);
							s.is_empty()
						}
						['s', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1625);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1626);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1627);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4108);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 7802);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 11640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4109);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4110);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7803);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7804);
							match s {
								[] => true,
								['f', 'e', s @ ..] => {
									super::push(out, 11641);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 14787);
											s.is_empty()
										}
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
					super::push(out, 4111);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4112);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7805);
							match s {
								[] => true,
								['r', 'g', 'o', s @ ..] => {
									super::push(out, 11642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t6::n1782(s, out),
		['v', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7842);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -infixus.
pub(crate) fn n10975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7848);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onfixus.
pub(crate) fn n10976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7849);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word perfixus.
pub(crate) fn n5025(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -sfixus.
pub(crate) fn n5026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4134);
	match s {
		[] => true,
		['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lixus.
pub(crate) fn n1794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1636);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4135);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nixus.
pub(crate) fn n1795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1637);
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
			super::push(out, 4137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', '\u{1}', s @ ..] => {
			s.is_empty()
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
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rixus.
pub(crate) fn n1796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1638);
	s.is_empty()
}

/// Words ending -oxus.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	s.is_empty()
}

/// Words ending -uxus.
pub(crate) fn n516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	match s {
		[] => true,
		['l', s @ ..] => {
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7850);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 11694);
									s.is_empty()
								}
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

/// The word tolerabilior.
pub(crate) fn n31643(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word intolerabilior.
pub(crate) fn n31644(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -aestabilior.
pub(crate) fn n31645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16774);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -testabilior.
pub(crate) fn n31646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16775);
	match s {
		[] => true,
		['e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -plior.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	s.is_empty()
}

/// Words ending -ulior.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	s.is_empty()
}

/// Words ending -fusior.
pub(crate) fn n2015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4246);
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

/// The word consideratior.
pub(crate) fn n31647(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inconsideratior.
pub(crate) fn n31648(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word moderatior.
pub(crate) fn n31649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16776);
	s.is_empty()
}

/// The word immoderatior.
pub(crate) fn n31650(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inmoderatior.
pub(crate) fn n31651(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inusitatior.
pub(crate) fn n25796(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -vatior.
pub(crate) fn n2025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1826);
	s.is_empty()
}

/// Words ending -ctior.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1827);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4267);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 4268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4269);
					match s {
						[] => true,
						['r', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4270);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4271);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1829);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4274);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4275);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11733);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
		['o', s @ ..] => {
			super::push(out, 1830);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4277);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4278);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etior.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1833);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1834);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['a', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7914);
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
		['i', s @ ..] => {
			super::push(out, 1836);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1837);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4280);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itior.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'l', 'l', 'o', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1840);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7915);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7916);
							match s {
								[] => true,
								['m', 'i', '\u{1}', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 4283);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4284);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4285);
					match s {
						[] => true,
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1841);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 1842);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4286);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1844);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1845);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltior.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 607);
	match s {
		[] => true,
		['u', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'o', '\u{1}', s @ ..] => {
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

/// Words ending -ntior.
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1847);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4288);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7917);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4289);
					match s {
						[] => true,
						['e', 'p', 'm', 'e', 't', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 4290);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7919);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11734);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7920);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11736);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11737);
									s.is_empty()
								}
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
			super::push(out, 1848);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4291);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
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
						['s', s @ ..] => {
							super::push(out, 7921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4292);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 11738);
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
								['r', s @ ..] => {
									super::push(out, 11739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4293);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7922);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'g', 'e', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7923);
							match s {
								[] => true,
								['l', 'i', 'd', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 4294);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7924);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11740);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
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
				['m', s @ ..] => {
					super::push(out, 4296);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7925);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7926);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 11741);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 11742);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4297);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otior.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1849);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1850);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptior.
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1852);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtior.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4299);
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
				['f', s @ ..] => {
					super::push(out, 4300);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stior.
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4302);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4304);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7927);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4305);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4306);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7929);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1856);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4307);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4308);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4309);
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
					super::push(out, 4310);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4311);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utior.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1858);
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
		['c', s @ ..] => {
			super::push(out, 1859);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1860);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4312);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1862);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1864);
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
		_ => false,
	}
}

/// Words ending -galis.
pub(crate) fn n722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1910);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7957);
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
		['i', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4356);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1913);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1914);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4358);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ialis.
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4361);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4362);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4363);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11750);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11751);
									s.is_empty()
								}
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
			super::push(out, 1917);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4364);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7959);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11752);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7960);
							match s {
								[] => true,
								['f', 'o', s @ ..] => {
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
								['i', 't', 'r', 'a', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 4365);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7961);
							match s {
								[] => true,
								['v', 'o', 'r', 'p', s @ ..] => {
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
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4366);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4367);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7963);
							match s {
								[] => true,
								['i', 'm', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 7964);
							match s {
								[] => true,
								['y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4368);
					match s {
						[] => true,
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7965);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4369);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11756);
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
								['m', s @ ..] => {
									super::push(out, 11757);
									s.is_empty()
								}
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
			super::push(out, 1919);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1920);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 4370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1921);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1922);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7966);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1923);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4371);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4372);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4373);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11758);
									s.is_empty()
								}
								['i', 'r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 14821);
											s.is_empty()
										}
										['t', 'a', s @ ..] => {
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 16777);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 16778);
													s.is_empty()
												}
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
		['p', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4374);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11759);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4376);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4377);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							super::push(out, 7968);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 11761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4378);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7969);
							s.is_empty()
						}
						['t', s @ ..] => super::t6::n11213(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4379);
					match s {
						[] => true,
						['c', s @ ..] => super::t6::n11214(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t6::n2144(s, out),
		['t', s @ ..] => super::t6::n2145(s, out),
		['v', s @ ..] => super::t6::n2146(s, out),
		_ => false,
	}
}

/// Words ending -malis.
pub(crate) fn n725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1929);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4390);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1930);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4392);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 4393);
					match s {
						[] => true,
						['a', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 4394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1931);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1932);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4395);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
					super::push(out, 4396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1933);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1934);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nalis.
pub(crate) fn n726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1937);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4398);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1938);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1939);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4400);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4401);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7978);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 11771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7979);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4402);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11772);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 11774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4404);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7981);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 11775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7982);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4405);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11776);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 11777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4406);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7985);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 14826);
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
						['g', s @ ..] => {
							super::push(out, 7986);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7987);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7988);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7989);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4407);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4408);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 7991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4409);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7992);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4411);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1940);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1941);
			match s {
				[] => true,
				['a', s @ ..] => {
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
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7994);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11779);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11780);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 14827);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 14828);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7995);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1942);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4413);
					s.is_empty()
				}
				['i', s @ ..] => super::t6::n5412(s, out),
				['r', s @ ..] => {
					super::push(out, 4415);
					match s {
						[] => true,
						['t', 'a', 'm', s @ ..] => {
							super::push(out, 8003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4416);
					match s {
						[] => true,
						['r', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t6::n2161(s, out),
		['u', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4421);
					match s {
						[] => true,
						['e', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -palis.
pub(crate) fn n727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1945);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1946);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4423);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8009);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['c', 's', 'i', 'p', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ralis.
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1948);
			match s {
				[] => true,
				['e', 'r', 'e', 'c', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 1949);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1950);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4425);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4426);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['c', s @ ..] => {
					super::push(out, 4427);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4428);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8011);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'p', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4430);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4431);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 8014);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 11788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4432);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
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
										['n', s @ ..] => {
											super::push(out, 14837);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													super::push(out, 16779);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 1951);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1952);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4433);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4434);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8015);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8016);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8017);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11789);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11790);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11791);
									s.is_empty()
								}
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
			super::push(out, 1953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4435);
					s.is_empty()
				}
				['c', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8018);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8019);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4437);
					match s {
						[] => true,
						['m', 'e', 't', s @ ..] => {
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
						['r', 'o', 'c', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 4438);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8020);
							match s {
								[] => true,
								['e', 'p', 'i', s @ ..] => {
									super::push(out, 11792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8021);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1954);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4439);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4440);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4441);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11793);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 11794);
									s.is_empty()
								}
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
			super::push(out, 1955);
			match s {
				[] => true,
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4442);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4443);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4444);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4445);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4446);
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
				['t', s @ ..] => super::t6::n5448(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -salis.
pub(crate) fn n729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1956);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1957);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4449);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1958);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1959);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4451);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11795);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11796);
									s.is_empty()
								}
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
			super::push(out, 1960);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -talis.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1961);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8029);
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
				['n', s @ ..] => {
					super::push(out, 4452);
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
			super::push(out, 1962);
			match s {
				[] => true,
				['i', 'd', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1963);
			match s {
				[] => true,
				['g', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 4454);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4455);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4456);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4457);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4458);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4459);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1964);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4461);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4462);
					match s {
						[] => true,
						['r', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 11797);
									match s {
										[] => true,
										['r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 14838);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4463);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4464);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8030);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8031);
							match s {
								[] => true,
								['g', 'o', s @ ..] => {
									super::push(out, 11798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4465);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'c', 'c', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'o', 'h', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 4466);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8032);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4467);
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
			super::push(out, 1965);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8034);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'c', 'c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14839);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 14840);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 8035);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 11799);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11800);
									match s {
										[] => true,
										['r', 't', 'u', 'n', '\u{1}', s @ ..] => {
											super::push(out, 14841);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11801);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 11802);
									match s {
										[] => true,
										['r', 't', 's', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 14842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8036);
							match s {
								[] => true,
								['i', 't', 'n', 'o', 'c', 'a', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 11803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4468);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1966);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4469);
					match s {
						[] => true,
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 8037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
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
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1968);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1969);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4470);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4471);
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

/// Words ending -ualis.
pub(crate) fn n731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1970);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4472);
					match s {
						[] => true,
						['v', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1971);
			match s {
				[] => true,
				['n', 'i', 'l', s @ ..] => {
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
			super::push(out, 1972);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1973);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['e', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 1975);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8039);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4473);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4474);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4475);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4476);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1977);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4478);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8041);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8042);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4479);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8044);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 11804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -valis.
pub(crate) fn n732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1979);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1981);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4482);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4483);
					match s {
						[] => true,
						['s', 'e', 'f', s @ ..] => {
							super::push(out, 8045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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

/// Words ending -bilis.
pub(crate) fn n736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 663);
	match s {
		[] => true,
		['a', s @ ..] => super::t6::n2205(s, out),
		['e', s @ ..] => {
			super::push(out, 1987);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4503);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8102);
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
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 8103);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1988);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4504);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4505);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8105);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14870);
											match s {
												[] => true,
												['r', 't', 'n', 'o', 'c', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t6::n35158(s, out),
														['n', 'i', '\u{1}', s @ ..] => super::t6::n35159(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 14871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8106);
							match s {
								[] => true,
								['i', 'v', s @ ..] => {
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
							super::push(out, 8107);
							s.is_empty()
						}
						['s', s @ ..] => super::t6::n11414(s, out),
						['u', s @ ..] => {
							super::push(out, 8109);
							match s {
								[] => true,
								['d', 'o', s @ ..] => {
									super::push(out, 11874);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 14874);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => super::t6::n18775(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t6::n5518(s, out),
				['g', s @ ..] => super::t6::n5519(s, out),
				['l', s @ ..] => super::t6::n5520(s, out),
				['n', s @ ..] => super::t6::n5521(s, out),
				['p', s @ ..] => super::t6::n5522(s, out),
				['r', s @ ..] => super::t6::n5523(s, out),
				['s', s @ ..] => super::t6::n5524(s, out),
				['t', s @ ..] => super::t6::n5525(s, out),
				['u', s @ ..] => super::t6::n5526(s, out),
				['x', s @ ..] => super::t6::n5527(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t6::n2208(s, out),
		['r', s @ ..] => super::t6::n2209(s, out),
		['u', s @ ..] => super::t6::n2210(s, out),
		_ => false,
	}
}

/// Words ending -tilis.
pub(crate) fn n745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 672);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2004);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4525);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4526);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4527);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4528);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8136);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2005);
			match s {
				[] => true,
				['u', 's', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4530);
					match s {
						[] => true,
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
				['e', s @ ..] => {
					super::push(out, 4531);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4532);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4533);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4534);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4535);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11890);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 11891);
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
						['r', s @ ..] => {
							super::push(out, 8139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2007);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4536);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2009);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4538);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4539);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8140);
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
		['s', s @ ..] => {
			super::push(out, 2011);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8141);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 8142);
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
				['u', '\u{1}', s @ ..] => {
					super::push(out, 4541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8143);
							match s {
								[] => true,
								['o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2013);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11892);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11893);
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
						['t', s @ ..] => {
							super::push(out, 8144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lexilis.
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4544);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11894);
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
		_ => false,
	}
}

/// Words ending -ibularis.
pub(crate) fn n11554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8190);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 11937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cularis.
pub(crate) fn n5646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4584);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8191);
			match s {
				[] => true,
				['t', 'n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 11938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8192);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11939);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 11940);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14897);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11941);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 16800);
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
					super::push(out, 11942);
					match s {
						[] => true,
						['t', 'n', 'e', 'v', 'o', s @ ..] => {
							super::push(out, 14899);
							match s {
								[] => true,
								['i', 'r', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 16801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11943);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
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
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 11944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8193);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8194);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8195);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11945);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gularis.
pub(crate) fn n5647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4585);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8196);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8197);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'b', 'l', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lularis.
pub(crate) fn n5648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4586);
	match s {
		[] => true,
		['l', 'e', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11947);
					match s {
						[] => true,
						['n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mularis.
pub(crate) fn n5649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4587);
	s.is_empty()
}

/// Words ending -nularis.
pub(crate) fn n5650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4588);
	s.is_empty()
}

/// Words ending -pularis.
pub(crate) fn n5651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4589);
	s.is_empty()
}

/// Words ending -sularis.
pub(crate) fn n5652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4590);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11949);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 16802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17705);
											s.is_empty()
										}
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
					super::push(out, 11950);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16803);
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
		_ => false,
	}
}

/// Words ending -tularis.
pub(crate) fn n5653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4591);
	s.is_empty()
}

/// Words ending -vularis.
pub(crate) fn n5654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4592);
	match s {
		[] => true,
		['i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 8198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ensis.
pub(crate) fn n801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2072);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2073);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4609);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8207);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8208);
							match s {
								[] => true,
								['o', 'b', 'm', 'i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4610);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4611);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8209);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 11958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 8210);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 8211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4613);
					match s {
						[] => true,
						['n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8212);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8213);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4614);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8215);
							match s {
								[] => true,
								['o', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 11959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4615);
					match s {
						[] => true,
						['e', 'f', 'e', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11960);
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
		['e', s @ ..] => {
			super::push(out, 2075);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4616);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2076);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4618);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4619);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 11961);
									match s {
										[] => true,
										['e', 't', 'a', s @ ..] => {
											super::push(out, 14904);
											match s {
												[] => true,
												['r', '\u{1}', s @ ..] => {
													super::push(out, 16804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 11962);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 16805);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 16806);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 11963);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11964);
									s.is_empty()
								}
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
			super::push(out, 2077);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4620);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2078);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4622);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 8216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 4623);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4624);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 11965);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4625);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4626);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4627);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4628);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4629);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8218);
							match s {
								[] => true,
								['m', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 11967);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11968);
									s.is_empty()
								}
								['p', 's', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11969);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11970);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 8219);
							match s {
								[] => true,
								['r', 'a', 'c', '\u{1}', s @ ..] => super::t6::n18926(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t6::n11596(s, out),
						['o', s @ ..] => super::t6::n11597(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t6::n5694(s, out),
				['n', s @ ..] => super::t6::n5695(s, out),
				['r', s @ ..] => super::t6::n5696(s, out),
				['s', s @ ..] => super::t6::n5697(s, out),
				['t', s @ ..] => super::t6::n5698(s, out),
				['v', s @ ..] => super::t6::n5699(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t6::n2328(s, out),
		['l', s @ ..] => super::t6::n2329(s, out),
		['m', s @ ..] => super::t6::n2330(s, out),
		['n', s @ ..] => super::t6::n2331(s, out),
		['o', s @ ..] => super::t6::n2332(s, out),
		['p', s @ ..] => super::t6::n2333(s, out),
		['r', s @ ..] => super::t6::n2334(s, out),
		['t', s @ ..] => super::t6::n2335(s, out),
		['u', s @ ..] => super::t6::n2336(s, out),
		['v', s @ ..] => super::t6::n2337(s, out),
		['y', s @ ..] => super::t6::n2338(s, out),
		_ => false,
	}
}

/// Words ending -icans.
pub(crate) fn n824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 732);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2112);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4683);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12032);
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
				['b', s @ ..] => {
					super::push(out, 4684);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4685);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8296);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 12033);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 12034);
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
						['d', s @ ..] => {
							super::push(out, 8297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 8298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 8299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12035);
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4686);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8300);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14922);
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
						['i', s @ ..] => {
							super::push(out, 8301);
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
				['o', s @ ..] => {
					super::push(out, 4687);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8302);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8303);
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
				['u', s @ ..] => {
					super::push(out, 4688);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8304);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 12036);
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
									super::push(out, 12037);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14923);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 14924);
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
									super::push(out, 12038);
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
						['p', s @ ..] => {
							super::push(out, 8306);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t7::n2383(s, out),
		['h', s @ ..] => {
			super::push(out, 2115);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2116);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4690);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4691);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4692);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4693);
					s.is_empty()
				}
				['p', s @ ..] => super::t7::n5802(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2117);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4695);
					s.is_empty()
				}
				['a', s @ ..] => super::t7::n5804(s, out),
				['e', s @ ..] => super::t7::n5805(s, out),
				['i', s @ ..] => super::t7::n5806(s, out),
				['r', s @ ..] => super::t7::n5807(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t7::n2387(s, out),
		['p', s @ ..] => super::t7::n2388(s, out),
		['r', s @ ..] => super::t7::n2389(s, out),
		['s', s @ ..] => super::t7::n2390(s, out),
		['t', s @ ..] => super::t7::n2391(s, out),
		['v', s @ ..] => super::t7::n2392(s, out),
		_ => false,
	}
}

/// Words ending -ncans.
pub(crate) fn n826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 734);
	match s {
		[] => true,
		['u', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4719);
					match s {
						[] => true,
						['e', 'v', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocans.
pub(crate) fn n827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 735);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2124);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2125);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4722);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', s @ ..] => {
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
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2127);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2128);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4724);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 4726);
					match s {
						[] => true,
						['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4727);
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
					super::push(out, 4728);
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

/// Words ending -rcans.
pub(crate) fn n828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 736);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2129);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8338);
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
		['u', s @ ..] => {
			super::push(out, 2130);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8339);
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

/// Words ending -scans.
pub(crate) fn n829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 737);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2132);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4729);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4730);
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
		['u', s @ ..] => {
			super::push(out, 2133);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8340);
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

/// Words ending -ucans.
pub(crate) fn n830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 738);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4731);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4732);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -andans.
pub(crate) fn n2424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2149);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8346);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12076);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 12077);
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
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -endans.
pub(crate) fn n2425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2150);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8348);
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

/// Words ending -undans.
pub(crate) fn n2426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2151);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4738);
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
		['c', s @ ..] => {
			super::push(out, 4739);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8349);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8350);
					match s {
						[] => true,
						['i', s @ ..] => {
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
		['d', 'e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8351);
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
		['n', s @ ..] => {
			super::push(out, 4741);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8352);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8353);
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
		['t', 'o', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odans.
pub(crate) fn n837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2152);
			match s {
				[] => true,
				['m', 'o', 'c', s @ ..] => {
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2153);
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
					super::push(out, 4742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdans.
pub(crate) fn n838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 745);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
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
		['o', 'c', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4743);
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

/// Words ending -udans.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 746);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4744);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 4745);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12078);
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
		['e', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2156);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4746);
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
		['s', s @ ..] => {
			super::push(out, 2157);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4747);
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

/// Words ending -ydans.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 747);
	s.is_empty()
}

/// Words ending -ligans.
pub(crate) fn n2457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2170);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4761);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4762);
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
			super::push(out, 4763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8362);
					s.is_empty()
				}
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8363);
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
		['f', s @ ..] => {
			super::push(out, 4764);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4765);
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

/// Words ending -remigans.
pub(crate) fn n11877(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rigans.
pub(crate) fn n2459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2172);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4768);
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

/// Words ending -tigans.
pub(crate) fn n2460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2173);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4769);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12081);
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
			super::push(out, 4770);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4771);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4772);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8365);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8366);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
				['n', s @ ..] => {
					super::push(out, 8367);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vigans.
pub(crate) fn n2461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2174);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4773);
			match s {
				[] => true,
				['n', s @ ..] => {
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
							super::push(out, 12082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 12083);
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
		['e', s @ ..] => {
			super::push(out, 4774);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgans.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 758);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8369);
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
		['u', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4775);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4776);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8370);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8371);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8372);
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
		_ => false,
	}
}

/// Words ending -ngans.
pub(crate) fn n858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 759);
	match s {
		[] => true,
		['o', 'l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4777);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ogans.
pub(crate) fn n859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 760);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4779);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 8373);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4781);
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
					super::push(out, 4782);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4783);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
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
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -rgans.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 761);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4784);
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
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8374);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ugans.
pub(crate) fn n861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 762);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4785);
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
					super::push(out, 4786);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4787);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4788);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 2179);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ipians.
pub(crate) fn n2498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2201);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -propians.
pub(crate) fn n2499(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word expians.
pub(crate) fn n2500(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rians.
pub(crate) fn n870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 771);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2202);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2203);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2204);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4798);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8379);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2205);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2206);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2207);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4800);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4801);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4802);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 4803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sians.
pub(crate) fn n871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2208);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tians.
pub(crate) fn n872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2210);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2211);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 2212);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4804);
					s.is_empty()
				}
				['p', 'o', 'r', 'p', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 2213);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4805);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4806);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4807);
					match s {
						[] => true,
						['n', s @ ..] => {
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
									super::push(out, 12086);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14933);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 14934);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 14935);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 14936);
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
										_ => false,
									}
								}
								['n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12087);
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
			super::push(out, 2214);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2215);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vians.
pub(crate) fn n873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2217);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4808);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4809);
					match s {
						[] => true,
						['b', s @ ..] => {
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

/// The word pervigilans.
pub(crate) fn n11964(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -hilans.
pub(crate) fn n2538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2232);
	match s {
		[] => true,
		['i', 'n', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -milans.
pub(crate) fn n2539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2233);
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
				['n', s @ ..] => {
					super::push(out, 8384);
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

/// Words ending -pilans.
pub(crate) fn n2540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2234);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4815);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4816);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
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
		['p', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tilans.
pub(crate) fn n2541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2235);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 4817);
			match s {
				[] => true,
				['u', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -llans.
pub(crate) fn n879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 780);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4818);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4819);
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
		['e', s @ ..] => {
			super::push(out, 2237);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4820);
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
									super::push(out, 12089);
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
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4821);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4822);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4823);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2238);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4824);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8387);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4825);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4826);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 8389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4827);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8390);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8391);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12090);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 12091);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 12092);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 12093);
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
				['u', s @ ..] => {
					super::push(out, 4828);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2239);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4830);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4831);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olans.
pub(crate) fn n880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 781);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4833);
					match s {
						[] => true,
						['r', 'g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4834);
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
		['d', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4835);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2243);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 2244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4836);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4837);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4838);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4839);
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
			super::push(out, 2245);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
					super::push(out, 4841);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 8393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8394);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 't', 'l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4842);
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
					super::push(out, 4843);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4844);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
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
					super::push(out, 4845);
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

/// Words ending -plans.
pub(crate) fn n881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 782);
	s.is_empty()
}

/// Words ending -ulans.
pub(crate) fn n882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 783);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2247);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4846);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8395);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 12094);
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
						['p', s @ ..] => {
							super::push(out, 8396);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4847);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
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
								['t', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4848);
					match s {
						[] => true,
						['a', s @ ..] => {
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
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
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
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2248);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4849);
					match s {
						[] => true,
						['m', s @ ..] => {
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4850);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12095);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12096);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14937);
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
				['i', s @ ..] => {
					super::push(out, 4851);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4852);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4853);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2249);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4854);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4855);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4856);
					match s {
						[] => true,
						['m', s @ ..] => {
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
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2250);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4857);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4858);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4859);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['u', s @ ..] => {
					super::push(out, 4860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4861);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4862);
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
		['l', s @ ..] => {
			super::push(out, 2252);
			match s {
				[] => true,
				['u', s @ ..] => {
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
		['m', s @ ..] => super::t7::n2559(s, out),
		['p', s @ ..] => {
			super::push(out, 2254);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4864);
					s.is_empty()
				}
				['i', s @ ..] => super::t7::n6114(s, out),
				['o', s @ ..] => super::t7::n6115(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t7::n2561(s, out),
		['t', s @ ..] => super::t7::n2562(s, out),
		_ => false,
	}
}

/// The word adproximans.
pub(crate) fn n12057(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word approximans.
pub(crate) fn n12058(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lmans.
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 787);
	s.is_empty()
}

/// Words ending -mmans.
pub(crate) fn n887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 788);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2273);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8406);
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
			super::push(out, 2274);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omans.
pub(crate) fn n888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 789);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'n', 'g', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
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

/// Words ending -rmans.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 790);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2276);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2277);
			match s {
				[] => true,
				['f', s @ ..] => {
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
						['f', s @ ..] => {
							super::push(out, 8407);
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
						['n', s @ ..] => {
							super::push(out, 8408);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12101);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 12102);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2278);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8409);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 12104);
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
							super::push(out, 8410);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8411);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12105);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 12106);
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
							super::push(out, 8412);
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
					super::push(out, 4881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -smans.
pub(crate) fn n890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 791);
	match s {
		[] => true,
		['a', 'l', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -umans.
pub(crate) fn n891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 792);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2279);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2280);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2281);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2282);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8413);
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
		['r', s @ ..] => {
			super::push(out, 2284);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -esignans.
pub(crate) fn n12093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8418);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 12107);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12108);
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

/// Words ending -nsignans.
pub(crate) fn n12094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8419);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 12109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12110);
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

/// Words ending -ugnans.
pub(crate) fn n2610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2294);
	match s {
		[] => true,
		['p', s @ ..] => {
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
					super::push(out, 8420);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12112);
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
					super::push(out, 8421);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -inans.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2295);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2296);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4889);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4890);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4891);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8422);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8423);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4892);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4893);
					match s {
						[] => true,
						['i', 't', 'a', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4894);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8425);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4895);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4896);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8427);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8428);
							match s {
								[] => true,
								['a', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2297);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4897);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8429);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4898);
					match s {
						[] => true,
						['o', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 2298);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4899);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8431);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 8433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4900);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 12113);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 12114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4901);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8434);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4902);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12115);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 12116);
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
		['h', s @ ..] => {
			super::push(out, 2299);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2300);
			match s {
				[] => true,
				['c', s @ ..] => {
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
							super::push(out, 8436);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12117);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 12118);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8437);
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
		['m', s @ ..] => super::t7::n2617(s, out),
		['p', s @ ..] => {
			super::push(out, 2302);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4911);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4912);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4913);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'e', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8446);
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
				['u', s @ ..] => {
					super::push(out, 4915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2303);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2304);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4918);
					s.is_empty()
				}
				['c', s @ ..] => super::t7::n6204(s, out),
				['n', s @ ..] => super::t7::n6205(s, out),
				['s', s @ ..] => super::t7::n6206(s, out),
				['u', s @ ..] => super::t7::n6207(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t7::n2621(s, out),
		['v', s @ ..] => super::t7::n2622(s, out),
		_ => false,
	}
}

/// Words ending -mnans.
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 798);
	match s {
		[] => true,
		['a', 'd', s @ ..] => {
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

/// Words ending -nnans.
pub(crate) fn n899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onans.
pub(crate) fn n900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 800);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2307);
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
		['d', s @ ..] => {
			super::push(out, 2308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2309);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4923);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4924);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8451);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8452);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2310);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2311);
			match s {
				[] => true,
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2312);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
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
		['s', s @ ..] => {
			super::push(out, 2313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4925);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 8454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4926);
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
				['i', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4927);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8456);
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
				['r', s @ ..] => {
					super::push(out, 4928);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2314);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4929);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8458);
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
				['i', 't', 'l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -rnans.
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 801);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2315);
			match s {
				[] => true,
				['c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2316);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4930);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8459);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8460);
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
										['g', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4931);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4932);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8461);
							match s {
								[] => true,
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
		['o', s @ ..] => {
			super::push(out, 2317);
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
				['t', s @ ..] => {
					super::push(out, 4933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -unans.
pub(crate) fn n902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 802);
	match s {
		[] => true,
		['d', 'a', s @ ..] => {
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
		['l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word emancipans.
pub(crate) fn n31820(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

