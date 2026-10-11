use flexion::Path;

/// Words ending -knąć.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 569);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1815);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3708);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3709);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 5866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3711);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3712);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1816);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1817);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3714);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3715);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7470);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7471);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 8242);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7472);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7473);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 8243);
											s.is_empty()
										}
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
					super::push(out, 3716);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5868);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5869);
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
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3717);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3718);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5870);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5871);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5872);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5873);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5874);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5875);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 7474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3720);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3721);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1819);
			match s {
				[] => true,
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1820);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3724);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3725);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5877);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5879);
							s.is_empty()
						}
						['p', 'a', s @ ..] => {
							super::push(out, 5880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3726);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 5881);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1821);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5883);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5884);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5885);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5886);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['a', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 3729);
					s.is_empty()
				}
				['e', 'z', s @ ..] => {
					super::push(out, 3730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1823);
			match s {
				[] => true,
				['m', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1824);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3732);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5887);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3733);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3734);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5889);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1825);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3735);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5891);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3736);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3737);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3738);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3739);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1826);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3741);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3742);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3743);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3744);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5898);
							s.is_empty()
						}
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5899);
							s.is_empty()
						}
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5900);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => super::t2::n4133(s, out),
				['ł', s @ ..] => super::t2::n4134(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t2::n1958(s, out),
		['ą', s @ ..] => super::t2::n1959(s, out),
		['ę', s @ ..] => super::t2::n1960(s, out),
		['ł', s @ ..] => super::t2::n1961(s, out),
		_ => false,
	}
}

/// Words ending -lnąć.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 570);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3764);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1833);
			match s {
				[] => true,
				['s', 'a', 'k', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mnąć.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['y', 'ż', 'z', '\u{1}', s @ ..] => {
			super::push(out, 1834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onąć.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['z', 'y', s @ ..] => {
					super::push(out, 3767);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 5910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3769);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7485);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8248);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 8249);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 7486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5911);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pnąć.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3771);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3774);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5914);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3775);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 5916);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 5917);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7487);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 7489);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8250);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8251);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8252);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5918);
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
				['k', '\u{1}', s @ ..] => {
					super::push(out, 3777);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1840);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1841);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3779);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3780);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5920);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8253);
											s.is_empty()
										}
										['ś', s @ ..] => {
											super::push(out, 8254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1842);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3781);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3782);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3783);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3784);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3785);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5922);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5923);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 1844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnąć.
pub(crate) fn n594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1845);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5924);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5925);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5926);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7492);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1846);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3791);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3792);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3793);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3794);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1847);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -snąć.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 575);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1849);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1850);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5928);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3797);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5930);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 5931);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3798);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5933);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7494);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 7495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['z', 'r', 'k', 's', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1852);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3802);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3803);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3804);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3805);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5934);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5935);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5936);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 5937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1853);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3806);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5938);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5939);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7496);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7497);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 5940);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5941);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8255);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 8256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5942);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5943);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5944);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5945);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5946);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5947);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 3809);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 5948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5949);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3810);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3812);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7499);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7500);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5952);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7502);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3813);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3814);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 1858);
			match s {
				[] => true,
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5954);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7504);
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
										['\u{1}', s @ ..] => {
											super::push(out, 8257);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 8258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'w', s @ ..] => {
									super::push(out, 7505);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7506);
									s.is_empty()
								}
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
			super::push(out, 1859);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3815);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3816);
					match s {
						[] => true,
						['r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tnąć.
pub(crate) fn n596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 576);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3817);
					s.is_empty()
				}
				['ł', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['w', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5956);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5957);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5958);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5959);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', 'a', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3819);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3821);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1862);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 1864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unąć.
pub(crate) fn n597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 577);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5961);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5962);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5964);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3823);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3824);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5966);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wnąć.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1868);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ynąć.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1870);
			s.is_empty()
		}
		['ł', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5968);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5969);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5970);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5971);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7507);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5973);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -znąć.
pub(crate) fn n600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 580);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1871);
			s.is_empty()
		}
		['c', 'u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1872);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1873);
			match s {
				[] => true,
				['z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5975);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1874);
			match s {
				[] => true,
				['l', 'ś', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1875);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7509);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7510);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7511);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7512);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7513);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 8259);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8260);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 7514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8261);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8262);
											s.is_empty()
										}
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
					super::push(out, 3829);
					match s {
						[] => true,
						['z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3830);
					match s {
						[] => true,
						['e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1876);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1877);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 1878);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3831);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7515);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3832);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7517);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7518);
									s.is_empty()
								}
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
			super::push(out, 1879);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żnąć.
pub(crate) fn n601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 581);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3834);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rykiwać.
pub(crate) fn n4717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4291);
	match s {
		[] => true,
		['t', 's', 'p', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 6182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zykiwać.
pub(crate) fn n4719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4293);
	s.is_empty()
}

/// Words ending -omieszkiwać.
pub(crate) fn n2616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2421);
	s.is_empty()
}

/// Words ending -ąkiwać.
pub(crate) fn n2617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2422);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 4294);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4295);
			s.is_empty()
		}
		['ł', 'b', s @ ..] => {
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 6183);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ękiwać.
pub(crate) fn n2618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2423);
	s.is_empty()
}

/// Words ending -liwać.
pub(crate) fn n1120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1062);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2424);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2425);
			match s {
				[] => true,
				['s', 'a', 'k', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ś', s @ ..] => {
			super::push(out, 2426);
			match s {
				[] => true,
				['y', 'm', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -piwać.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1063);
	s.is_empty()
}

/// Words ending -wiwać.
pub(crate) fn n1122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1064);
	match s {
		[] => true,
		['r', 'd', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -acować.
pub(crate) fn n2634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2439);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4311);
			s.is_empty()
		}
		['j', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4312);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4313);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7579);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7580);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8268);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 7581);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8270);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7582);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7583);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 8272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7584);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7585);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7586);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 7587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4314);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6194);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wecować.
pub(crate) fn n4747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4316);
	s.is_empty()
}

/// Words ending -icować.
pub(crate) fn n2637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2442);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4317);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4318);
			match s {
				[] => true,
				['z', 's', 'a', s @ ..] => {
					super::push(out, 6197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4319);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4320);
			match s {
				[] => true,
				['ż', 'ó', 'r', 'z', s @ ..] => {
					super::push(out, 6198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jcować.
pub(crate) fn n2638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2443);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4322);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6199);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 6200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4323);
			match s {
				[] => true,
				['b', 'a', s @ ..] => {
					super::push(out, 6201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4324);
			s.is_empty()
		}
		['ó', 'b', 'z', '\u{1}', s @ ..] => {
			super::push(out, 4325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lcować.
pub(crate) fn n2639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2444);
	match s {
		[] => true,
		['a', 'w', 'z', s @ ..] => {
			super::push(out, 4326);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ncować.
pub(crate) fn n2640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2445);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6202);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocować.
pub(crate) fn n2641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2446);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 4328);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 4329);
			s.is_empty()
		}
		['w', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pcować.
pub(crate) fn n2642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2447);
	match s {
		[] => true,
		['o', 'k', 'a', s @ ..] => {
			super::push(out, 4331);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcować.
pub(crate) fn n2643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2448);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4332);
			match s {
				[] => true,
				['h', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4333);
			match s {
				[] => true,
				['t', 'z', 's', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucować.
pub(crate) fn n2644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2449);
	match s {
		[] => true,
		['p', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ycować.
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2450);
	match s {
		[] => true,
		['r', 'p', 'z', 's', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4335);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word obtańcować.
pub(crate) fn n2646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2451);
	s.is_empty()
}

/// Words ending -dować.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1067);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2452);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4336);
					s.is_empty()
				}
				['k', 'y', 'r', 'a', 'b', 'a', s @ ..] => {
					super::push(out, 4337);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4338);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6206);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7588);
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
							super::push(out, 6207);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6208);
							s.is_empty()
						}
						['g', 'e', 'd', 'z', s @ ..] => {
							super::push(out, 6209);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6210);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 6211);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 's', 'r', 'e', 'p', 'y', s @ ..] => {
					super::push(out, 4339);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 4340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6213);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6214);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 6215);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2453);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4341);
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 4342);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4343);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 6217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'r', 'o', 't', 's', s @ ..] => {
					super::push(out, 4344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 6218);
							s.is_empty()
						}
						['s', 'n', 'o', 'k', 's', s @ ..] => {
							super::push(out, 6219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4345);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7589);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'i', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7591);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2454);
			match s {
				[] => true,
				['a', 'n', 'z', 'o', s @ ..] => {
					super::push(out, 4346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'm', 'a', s @ ..] => {
			super::push(out, 2455);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2456);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4347);
					match s {
						[] => true,
						['p', 's', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4348);
					match s {
						[] => true,
						['m', 'o', 'k', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7593);
									s.is_empty()
								}
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4349);
					match s {
						[] => true,
						['b', 'z', s @ ..] => {
							super::push(out, 6222);
							s.is_empty()
						}
						['w', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6224);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6225);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 7595);
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
						['f', s @ ..] => {
							super::push(out, 6226);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7596);
									s.is_empty()
								}
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8273);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8274);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7597);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 6227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2457);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4350);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6228);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6230);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6231);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4352);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4353);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4354);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4355);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7599);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6233);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7601);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4357);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 4358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2458);
			match s {
				[] => true,
				['a', 'b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4359);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2459);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n4798(s, out),
				['b', s @ ..] => {
					super::push(out, 4362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t2::n6913(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t2::n4800(s, out),
				['l', 'k', 'n', 'o', 'k', '\u{1}', s @ ..] => super::t2::n4801(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t2::n2656(s, out),
		['ą', s @ ..] => super::t2::n2657(s, out),
		['ę', s @ ..] => super::t2::n2658(s, out),
		['ł', s @ ..] => super::t2::n2659(s, out),
		_ => false,
	}
}

/// Words ending -ykreować.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1068);
	s.is_empty()
}

/// Words ending -fować.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1069);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6237);
							match s {
								[] => true,
								['l', 'e', 't', 'a', s @ ..] => {
									super::push(out, 7605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6238);
							match s {
								[] => true,
								['t', 'o', 'f', 's', s @ ..] => {
									super::push(out, 7606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 4369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2463);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4370);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 6239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'u', 'l', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4372);
					s.is_empty()
				}
				['a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2464);
			match s {
				[] => true,
				['l', 'z', 's', 'o', s @ ..] => {
					super::push(out, 4374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 'i', 'r', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4375);
					s.is_empty()
				}
				['a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2465);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4377);
					match s {
						[] => true,
						['t', 's', 'e', s @ ..] => {
							super::push(out, 6240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2466);
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					super::push(out, 4379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gować.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1070);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2468);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4380);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6241);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4381);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7607);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7608);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 7609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4382);
					match s {
						[] => true,
						['b', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4383);
					match s {
						[] => true,
						['o', 'r', 'p', 'z', s @ ..] => {
							super::push(out, 6244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4384);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2469);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4386);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4387);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6245);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8275);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 8276);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7610);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6246);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8277);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8278);
											s.is_empty()
										}
										['y', 'w', '\u{1}', s @ ..] => {
											super::push(out, 8279);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', s @ ..] => {
					super::push(out, 4388);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4389);
					match s {
						[] => true,
						['e', 'z', 's', 'u', s @ ..] => {
							super::push(out, 6247);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7611);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8280);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8281);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 's', 'o', s @ ..] => {
									super::push(out, 7612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4390);
					match s {
						[] => true,
						['o', 'r', 'p', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['b', 'z', s @ ..] => {
					super::push(out, 4391);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6249);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6250);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2471);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4393);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7614);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2472);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4395);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4396);
					match s {
						[] => true,
						['b', 'b', 'u', 'd', 'z', s @ ..] => {
							super::push(out, 6251);
							s.is_empty()
						}
						['f', 's', s @ ..] => {
							super::push(out, 6252);
							s.is_empty()
						}
						['p', 'o', 'd', 'z', s @ ..] => {
							super::push(out, 6253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2473);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6254);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6255);
							match s {
								[] => true,
								['t', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 6256);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4399);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6258);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2474);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2475);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4400);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4401);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4402);
					s.is_empty()
				}
				['r', 'y', s @ ..] => {
					super::push(out, 4403);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 6261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 4404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2476);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6262);
							s.is_empty()
						}
						['o', 'k', 's', s @ ..] => {
							super::push(out, 6263);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6264);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7617);
									match s {
										[] => true,
										['i', s @ ..] => super::t2::n9181(s, out),
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => super::t2::n6946(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t2::n4845(s, out),
				_ => false,
			}
		}
		['ę', s @ ..] => super::t2::n2677(s, out),
		_ => false,
	}
}

/// Words ending -hować.
pub(crate) fn n1129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1071);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2478);
			match s {
				[] => true,
				['r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6274);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8282);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 8283);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 6275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2479);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4407);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4408);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 6276);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6277);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7621);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6279);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7623);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7624);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8284);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 6280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4409);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7625);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4411);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4412);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4413);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6283);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4415);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6285);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iować.
pub(crate) fn n1130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1072);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['ś', 'o', s @ ..] => {
					match s {
						[] => true,
						['t', 'r', 'a', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7627);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7628);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2481);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4416);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'a', 'p', 's', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				['u', 't', 's', 'e', s @ ..] => {
					super::push(out, 4418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'a', 'l', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 2482);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2483);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2484);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4420);
					match s {
						[] => true,
						['p', 'm', 'o', 'k', 'a', 'a', s @ ..] => {
							super::push(out, 6289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4421);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6290);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7630);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', 'k', 'u', '\u{1}', s @ ..] => {
											super::push(out, 8286);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8287);
											match s {
												[] => true,
												['r', 'p', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9592(s, out),
														['a', s @ ..] => super::t2::n9593(s, out),
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
							super::push(out, 6291);
							match s {
								[] => true,
								['r', 'e', 't', 'n', 'i', 'a', s @ ..] => {
									super::push(out, 7631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4422);
					match s {
						[] => true,
						['f', 'e', 'd', 'z', s @ ..] => {
							super::push(out, 6292);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6293);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6294);
							s.is_empty()
						}
						['p', 'o', 'a', s @ ..] => {
							super::push(out, 6295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4423);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2486);
			match s {
				[] => true,
				['o', 'k', 's', s @ ..] => {
					super::push(out, 4425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2487);
			match s {
				[] => true,
				['a', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6296);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2488);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jować.
pub(crate) fn n1131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1073);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2490);
			match s {
				[] => true,
				['i', 'n', 'i', 'a', s @ ..] => {
					super::push(out, 4426);
					s.is_empty()
				}
				['n', 'u', 'n', 'e', 'd', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4427);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', 'e', 'n', 'y', s @ ..] => {
							super::push(out, 6298);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2491);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2492);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4428);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6300);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6301);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2494);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4429);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 2495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kować.
pub(crate) fn n1132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1074);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2496);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4431);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4432);
					match s {
						[] => true,
						['n', 'a', 'w', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4433);
					match s {
						[] => true,
						['z', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4434);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7632);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7633);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4435);
					match s {
						[] => true,
						['z', 'o', s @ ..] => {
							super::push(out, 6306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6307);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6308);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6309);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 6310);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6311);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7636);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4437);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 6313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4438);
					match s {
						[] => true,
						['a', 'a', s @ ..] => {
							super::push(out, 6314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4439);
					match s {
						[] => true,
						['i', 'b', 'a', s @ ..] => {
							super::push(out, 6315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 4440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2497);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4441);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4442);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 4443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2498);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4444);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6316);
							match s {
								[] => true,
								['f', 'u', 'z', 's', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 6317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4445);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4446);
					match s {
						[] => true,
						['r', 'ś', 'o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7640);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 7641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 4447);
					match s {
						[] => true,
						['z', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 4448);
					match s {
						[] => true,
						['w', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2499);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4449);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6320);
							match s {
								[] => true,
								['d', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6321);
							match s {
								[] => true,
								['i', 'a', s @ ..] => {
									super::push(out, 7643);
									s.is_empty()
								}
								['y', 'z', 'e', 'd', 'z', s @ ..] => {
									super::push(out, 7644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4450);
					match s {
						[] => true,
						['p', 'o', 'a', s @ ..] => {
							super::push(out, 6322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4451);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2500);
			match s {
				[] => true,
				['f', s @ ..] => super::t2::n4900(s, out),
				['l', s @ ..] => super::t2::n4901(s, out),
				['n', s @ ..] => super::t2::n4902(s, out),
				['p', s @ ..] => {
					super::push(out, 4455);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6333);
							match s {
								[] => true,
								['s', 'a', s @ ..] => {
									super::push(out, 7658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t2::n4904(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t2::n2701(s, out),
		['l', s @ ..] => super::t2::n2702(s, out),
		['m', 'u', 'g', 'y', 'w', '\u{1}', s @ ..] => super::t2::n2703(s, out),
		['n', s @ ..] => super::t2::n2704(s, out),
		['o', s @ ..] => super::t2::n2705(s, out),
		['p', s @ ..] => super::t2::n2706(s, out),
		['r', s @ ..] => super::t2::n2707(s, out),
		['s', s @ ..] => super::t2::n2708(s, out),
		['t', s @ ..] => super::t2::n2709(s, out),
		['u', s @ ..] => super::t2::n2710(s, out),
		['w', s @ ..] => super::t2::n2711(s, out),
		['y', s @ ..] => super::t2::n2712(s, out),
		['z', s @ ..] => super::t2::n2713(s, out),
		['ę', s @ ..] => super::t2::n2714(s, out),
		['ł', s @ ..] => super::t2::n2715(s, out),
		['ż', s @ ..] => super::t2::n2716(s, out),
		_ => false,
	}
}

/// Words ending -lować.
pub(crate) fn n1133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2516);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6402);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6403);
							s.is_empty()
						}
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6404);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6405);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4495);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4496);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 6407);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 7700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4498);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4499);
					match s {
						[] => true,
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2517);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4500);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7701);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7702);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4501);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6412);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6413);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7704);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7705);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6414);
							match s {
								[] => true,
								['k', 'a', s @ ..] => {
									super::push(out, 7707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4502);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4503);
					match s {
						[] => true,
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2518);
			match s {
				[] => true,
				['i', 'p', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 4505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2519);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 4506);
					match s {
						[] => true,
						['h', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2520);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4507);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4508);
					match s {
						[] => true,
						['e', 'b', 'y', s @ ..] => {
							super::push(out, 6418);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'o', 'm', 'y', s @ ..] => {
					super::push(out, 4509);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4510);
					match s {
						[] => true,
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4511);
					match s {
						[] => true,
						['a', 'a', s @ ..] => {
							super::push(out, 6421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4512);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4513);
					s.is_empty()
				}
				['w', 'i', 'n', 'z', s @ ..] => {
					super::push(out, 4514);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4515);
					match s {
						[] => true,
						['e', 'd', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6423);
							s.is_empty()
						}
						['y', 'c', 'y', s @ ..] => {
							super::push(out, 6424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2521);
			match s {
				[] => true,
				['u', 'm', 'a', 'k', 'a', s @ ..] => {
					super::push(out, 4516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2522);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4517);
					match s {
						[] => true,
						['m', 'z', s @ ..] => {
							super::push(out, 6425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4518);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4519);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4520);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6426);
							s.is_empty()
						}
						['m', 'z', 's', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2523);
			match s {
				[] => true,
				['c', 'a', 'w', 'o', s @ ..] => {
					super::push(out, 4522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2524);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4523);
					s.is_empty()
				}
				['m', 'y', 's', 'a', 'z', s @ ..] => {
					super::push(out, 4524);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4525);
					match s {
						[] => true,
						['e', 'd', 'y', s @ ..] => {
							super::push(out, 6428);
							s.is_empty()
						}
						['m', 'o', 'k', 's', s @ ..] => {
							super::push(out, 6429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2525);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2526);
			match s {
				[] => true,
				['e', 'p', 'a', s @ ..] => {
					super::push(out, 4526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2527);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6430);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 't', 's', s @ ..] => {
					super::push(out, 4527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2528);
			match s {
				[] => true,
				['h', 'e', s @ ..] => {
					super::push(out, 4528);
					s.is_empty()
				}
				['m', 'e', 'd', 'z', s @ ..] => {
					super::push(out, 4529);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6433);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6434);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6435);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4531);
					match s {
						[] => true,
						['t', 'n', 'o', 'k', 's', s @ ..] => {
							super::push(out, 6437);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6438);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => super::t2::n4990(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t2::n2730(s, out),
		['s', s @ ..] => {
			super::push(out, 2530);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4533);
					s.is_empty()
				}
				['p', 'a', 'k', s @ ..] => super::t2::n4993(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t2::n2732(s, out),
		['u', s @ ..] => super::t2::n2733(s, out),
		['y', s @ ..] => super::t2::n2734(s, out),
		['z', s @ ..] => super::t2::n2735(s, out),
		['ó', s @ ..] => super::t2::n2736(s, out),
		_ => false,
	}
}

/// Words ending -mować.
pub(crate) fn n1134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1076);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2536);
			match s {
				[] => true,
				['f', 'y', 'd', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4550);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4552);
					match s {
						[] => true,
						['b', 'z', s @ ..] => {
							super::push(out, 6465);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4553);
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 6466);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7721);
									s.is_empty()
								}
								['o', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8329);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8330);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'l', 'a', 'b', 'a', s @ ..] => {
					super::push(out, 4554);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 4555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2537);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', 's', s @ ..] => {
							super::push(out, 6467);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 6468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2538);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4556);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4557);
					match s {
						[] => true,
						['a', 'e', s @ ..] => {
							super::push(out, 6469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6470);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2539);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4559);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6472);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'i', 'f', 's', s @ ..] => {
			super::push(out, 2540);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['r', 'p', 'y', s @ ..] => {
					super::push(out, 4560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2542);
			match s {
				[] => true,
				['a', 'l', 'a', 'a', s @ ..] => {
					super::push(out, 4561);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4562);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4563);
					match s {
						[] => true,
						['f', 'a', s @ ..] => {
							super::push(out, 6474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4564);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7722);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7723);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'z', s @ ..] => {
											super::push(out, 8331);
											s.is_empty()
										}
										['r', 'z', s @ ..] => {
											super::push(out, 8332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', 'o', s @ ..] => {
									super::push(out, 7724);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7725);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', s @ ..] => {
							super::push(out, 6475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2543);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4566);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4567);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4568);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6476);
							s.is_empty()
						}
						['a', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7727);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7728);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6477);
							s.is_empty()
						}
						['n', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7729);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4570);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4571);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6479);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6480);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 7731);
									s.is_empty()
								}
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
			super::push(out, 2544);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4572);
					match s {
						[] => true,
						['p', 'e', 'd', 'z', s @ ..] => {
							super::push(out, 6481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'k', 'o', 'r', 'p', 'a', 'z', s @ ..] => {
					super::push(out, 4573);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'g', 'e', 'l', 'y', s @ ..] => {
							super::push(out, 6482);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6483);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6484);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2545);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4575);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nować.
pub(crate) fn n1135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1077);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2546);
			match s {
				[] => true,
				['b', 'z', s @ ..] => {
					super::push(out, 4577);
					s.is_empty()
				}
				['f', 'o', 'r', 'p', 's', s @ ..] => {
					super::push(out, 4578);
					s.is_empty()
				}
				['i', 'm', 'e', s @ ..] => {
					super::push(out, 4579);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4580);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 6485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4581);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 6486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4582);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6487);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6489);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6490);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6491);
							s.is_empty()
						}
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 6492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 't', 's', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				['z', 's', 'u', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2547);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4586);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6493);
							match s {
								[] => true,
								['a', 'y', s @ ..] => {
									super::push(out, 7733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6494);
							match s {
								[] => true,
								['a', 'k', 'u', s @ ..] => {
									super::push(out, 7734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4587);
					match s {
						[] => true,
						['d', 'y', s @ ..] => {
							super::push(out, 6495);
							s.is_empty()
						}
						['t', 'y', s @ ..] => {
							super::push(out, 6496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'a', s @ ..] => {
					super::push(out, 4588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2548);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4589);
					match s {
						[] => true,
						['r', 'p', 'm', 'i', 'a', s @ ..] => {
							super::push(out, 6497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 6498);
							match s {
								[] => true,
								['a', 'y', s @ ..] => {
									super::push(out, 7735);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6499);
							match s {
								[] => true,
								['e', 'r', 'z', s @ ..] => {
									super::push(out, 7737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 4590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2549);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4591);
					match s {
						[] => true,
						['m', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4592);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4593);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4594);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6502);
							match s {
								[] => true,
								['t', 'n', 'o', 'k', 's', s @ ..] => {
									super::push(out, 7738);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 's', 'n', 'i', 'a', s @ ..] => {
							super::push(out, 6503);
							s.is_empty()
						}
						['i', 'l', 'e', 'y', s @ ..] => {
							super::push(out, 6504);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6505);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6506);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7740);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8333);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8334);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'z', s @ ..] => {
											super::push(out, 8335);
											s.is_empty()
										}
										['z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8336);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6507);
							s.is_empty()
						}
						['y', 'r', 'k', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7743);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', 'u', 'r', 'z', s @ ..] => {
			super::push(out, 2550);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2552);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6508);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n2754(s, out),
		['r', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4606);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 6530);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7763);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4607);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6532);
							match s {
								[] => true,
								['i', 'h', 'a', s @ ..] => {
									super::push(out, 7765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7766);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7767);
									s.is_empty()
								}
								['s', 'n', 'o', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 7768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'ę', 'i', 'p', 'a', s @ ..] => {
			super::push(out, 2555);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2556);
			s.is_empty()
		}
		['y', s @ ..] => super::t2::n2758(s, out),
		['z', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['a', 'ł', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 2559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pować.
pub(crate) fn n1136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1078);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4611);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4612);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4613);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4614);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6544);
							s.is_empty()
						}
						['d', 'u', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						['f', 'a', s @ ..] => {
							super::push(out, 6546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2561);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4615);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4616);
					s.is_empty()
				}
				['w', 'k', 'e', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 4617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2562);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4618);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6547);
							s.is_empty()
						}
						['r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4619);
					match s {
						[] => true,
						['a', 'g', 'a', s @ ..] => {
							super::push(out, 6549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4620);
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 6550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2564);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4621);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2565);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4623);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4624);
					match s {
						[] => true,
						['g', 'e', s @ ..] => {
							super::push(out, 6551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2566);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6552);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'e', 'y', s @ ..] => {
									super::push(out, 7770);
									s.is_empty()
								}
								['o', 'k', 'y', s @ ..] => {
									super::push(out, 7771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6553);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6554);
							match s {
								[] => true,
								['t', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'y', s @ ..] => {
					super::push(out, 4625);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 2567);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4626);
					match s {
						[] => true,
						['k', 's', s @ ..] => {
							super::push(out, 6555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4627);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8349);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8350);
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', s @ ..] => {
									super::push(out, 7773);
									s.is_empty()
								}
								['d', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7774);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7775);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7776);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7777);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 7778);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7779);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7780);
									s.is_empty()
								}
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

/// Words ending -rować.
pub(crate) fn n1137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1079);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2568);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4628);
					match s {
						[] => true,
						['o', 'p', 's', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4629);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4631);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4632);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7781);
									s.is_empty()
								}
								['e', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8351);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8352);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7782);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4633);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4634);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6559);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7784);
									match s {
										[] => true,
										['p', 's', s @ ..] => {
											super::push(out, 8353);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7785);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 8354);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 6560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4635);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6561);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4636);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2569);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4637);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4638);
					match s {
						[] => true,
						['l', 'a', 'k', 's', s @ ..] => {
							super::push(out, 6564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'z', 's', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4639);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4641);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4642);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4643);
					match s {
						[] => true,
						['w', 'ś', 'e', s @ ..] => {
							super::push(out, 6565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4644);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4645);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4646);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 4647);
					match s {
						[] => true,
						['l', 'p', 's', s @ ..] => {
							super::push(out, 6566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 4648);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7786);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7787);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7788);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7789);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7790);
									s.is_empty()
								}
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
			super::push(out, 2571);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4649);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4650);
					match s {
						[] => true,
						['a', s @ ..] => super::t2::n7278(s, out),
						['i', s @ ..] => {
							super::push(out, 6568);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6569);
							match s {
								[] => true,
								['a', 'k', 'n', 'i', '\u{1}', s @ ..] => super::t2::n8632(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t2::n5123(s, out),
				['f', s @ ..] => super::t2::n5124(s, out),
				['g', s @ ..] => super::t2::n5125(s, out),
				['i', s @ ..] => super::t2::n5126(s, out),
				['j', s @ ..] => super::t2::n5127(s, out),
				['l', 'o', s @ ..] => super::t2::n5128(s, out),
				['m', s @ ..] => super::t2::n5129(s, out),
				['n', s @ ..] => super::t2::n5130(s, out),
				['p', s @ ..] => super::t2::n5131(s, out),
				['s', s @ ..] => super::t2::n5132(s, out),
				['t', s @ ..] => super::t2::n5133(s, out),
				['w', 'a', 'r', 'g', 'y', s @ ..] => super::t2::n5134(s, out),
				['z', 's', s @ ..] => super::t2::n5135(s, out),
				['ż', s @ ..] => super::t2::n5136(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t2::n2774(s, out),
		['g', s @ ..] => super::t2::n2775(s, out),
		['h', 'c', 'a', 'z', 's', 'o', '\u{1}', s @ ..] => super::t2::n2776(s, out),
		['i', s @ ..] => super::t2::n2777(s, out),
		['k', s @ ..] => super::t2::n2778(s, out),
		['m', 'a', 'l', 'k', 'a', s @ ..] => super::t2::n2779(s, out),
		['o', s @ ..] => super::t2::n2780(s, out),
		['t', s @ ..] => super::t2::n2781(s, out),
		['u', s @ ..] => super::t2::n2782(s, out),
		['w', s @ ..] => super::t2::n2783(s, out),
		['y', s @ ..] => super::t2::n2784(s, out),
		['ó', s @ ..] => super::t2::n2785(s, out),
		_ => false,
	}
}

/// Words ending -sować.
pub(crate) fn n1138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1080);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2584);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4697);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4698);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'a', s @ ..] => {
							super::push(out, 6641);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4699);
					match s {
						[] => true,
						['k', 'e', 'd', 'z', s @ ..] => {
							super::push(out, 6643);
							s.is_empty()
						}
						['p', 'u', s @ ..] => {
							super::push(out, 6644);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6646);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6648);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7851);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6649);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'm', 'a', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7853);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									super::push(out, 7854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6651);
							s.is_empty()
						}
						['f', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6652);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6653);
							s.is_empty()
						}
						['p', 'y', s @ ..] => {
							super::push(out, 6654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6655);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'c', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6657);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 4702);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2585);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4703);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4704);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4705);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6660);
							s.is_empty()
						}
						['d', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7855);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7856);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7857);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7858);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 't', 'n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7860);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'm', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7862);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7864);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8374);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 8375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 7865);
									s.is_empty()
								}
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
			super::push(out, 2586);
			match s {
				[] => true,
				['m', 'e', 'r', 'z', s @ ..] => {
					super::push(out, 4706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2587);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4707);
					match s {
						[] => true,
						['l', 'e', 'r', 'z', s @ ..] => {
							super::push(out, 6661);
							s.is_empty()
						}
						['w', 'u', 'z', 's', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4708);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7866);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7867);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8376);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6663);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7868);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4709);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2588);
			s.is_empty()
		}
		['n', s @ ..] => super::t2::n2791(s, out),
		['o', s @ ..] => {
			super::push(out, 2590);
			match s {
				[] => true,
				['g', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4712);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4713);
					match s {
						[] => true,
						['i', 's', 'a', s @ ..] => {
							super::push(out, 6669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4714);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 4715);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7885);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t2::n2793(s, out),
		['r', s @ ..] => super::t2::n2794(s, out),
		['s', s @ ..] => super::t2::n2795(s, out),
		['u', s @ ..] => super::t2::n2796(s, out),
		['y', s @ ..] => super::t2::n2797(s, out),
		_ => false,
	}
}

/// Words ending -tować.
pub(crate) fn n1139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1081);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2596);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4722);
					match s {
						[] => true,
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6684);
							s.is_empty()
						}
						['y', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4723);
					s.is_empty()
				}
				['i', 'g', 'a', 'l', 'p', 's', s @ ..] => {
					super::push(out, 4724);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6686);
							match s {
								[] => true,
								['w', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4725);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6688);
							s.is_empty()
						}
						['r', 'o', 'f', 's', s @ ..] => {
							super::push(out, 6689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', 'p', 's', 'k', 'e', 'y', s @ ..] => {
					super::push(out, 4726);
					s.is_empty()
				}
				['p', 'a', s @ ..] => {
					super::push(out, 4727);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6690);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6691);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4729);
					match s {
						[] => true,
						['s', 'n', 'o', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 6693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4730);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2597);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4732);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4733);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4734);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4735);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 8381);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8382);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7892);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 7893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4736);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6694);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', s @ ..] => {
									super::push(out, 7894);
									s.is_empty()
								}
								['f', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 8383);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 8384);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6695);
							s.is_empty()
						}
						['p', 'm', 'o', 'k', 's', s @ ..] => {
							super::push(out, 6696);
							s.is_empty()
						}
						['y', 't', 'z', 's', 'a', s @ ..] => {
							super::push(out, 6697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4737);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'y', s @ ..] => {
							super::push(out, 6698);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4738);
					match s {
						[] => true,
						['p', 'r', 'e', 't', 'n', 'i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7895);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6700);
							match s {
								[] => true,
								['l', 'a', 'm', 'z', s @ ..] => {
									super::push(out, 7897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'e', 'r', 'z', s @ ..] => {
					super::push(out, 4739);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6701);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'a', 'h', 'y', s @ ..] => {
			super::push(out, 2598);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'z', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7898);
									s.is_empty()
								}
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6703);
							s.is_empty()
						}
						['y', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2600);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4741);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4743);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7900);
									s.is_empty()
								}
								['y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7901);
									s.is_empty()
								}
								['z', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4744);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6706);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 7903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n5246(s, out),
				['m', s @ ..] => super::t2::n5247(s, out),
				['n', s @ ..] => {
					super::push(out, 4747);
					s.is_empty()
				}
				['w', s @ ..] => super::t2::n5249(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t2::n2803(s, out),
		['k', s @ ..] => super::t2::n2804(s, out),
		['l', s @ ..] => super::t2::n2805(s, out),
		['n', s @ ..] => super::t2::n2806(s, out),
		['o', s @ ..] => super::t2::n2807(s, out),
		['p', s @ ..] => super::t2::n2808(s, out),
		['r', s @ ..] => super::t3::n2809(s, out),
		['s', s @ ..] => super::t3::n2810(s, out),
		['u', s @ ..] => super::t3::n2811(s, out),
		['y', s @ ..] => super::t3::n2812(s, out),
		['z', s @ ..] => super::t3::n2813(s, out),
		['ę', s @ ..] => super::t3::n2814(s, out),
		['ł', s @ ..] => super::t3::n2815(s, out),
		_ => false,
	}
}

/// Words ending -uować.
pub(crate) fn n1140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1082);
	match s {
		[] => true,
		['b', 'y', 'r', 't', 's', 'y', 'd', 'z', s @ ..] => {
			super::push(out, 2614);
			s.is_empty()
		}
		['k', 'a', 'w', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2615);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2616);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4799);
					match s {
						[] => true,
						['w', 'e', 'd', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4800);
					match s {
						[] => true,
						['w', 'e', 'y', s @ ..] => {
							super::push(out, 6813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'y', s @ ..] => {
			match s {
				[] => true,
				['s', 'n', 'i', 'a', s @ ..] => {
					super::push(out, 4801);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2617);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6814);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 7986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7987);
									s.is_empty()
								}
								['i', 'o', s @ ..] => {
									super::push(out, 7988);
									s.is_empty()
								}
								['o', 'k', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'z', s @ ..] => {
											super::push(out, 8425);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8426);
											s.is_empty()
										}
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
			super::push(out, 2618);
			match s {
				[] => true,
				['a', 't', 'y', s @ ..] => {
					super::push(out, 4803);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4804);
					match s {
						[] => true,
						['u', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['s', 'u', s @ ..] => {
							super::push(out, 6816);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7989);
									s.is_empty()
								}
								['n', 'o', 'k', 'u', s @ ..] => {
									super::push(out, 7990);
									s.is_empty()
								}
								['o', 'r', 'p', 's', s @ ..] => {
									super::push(out, 7991);
									s.is_empty()
								}
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

/// Words ending -wować.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1083);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['r', 'p', 'e', s @ ..] => {
					super::push(out, 4805);
					match s {
						[] => true,
						['d', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2620);
			match s {
				[] => true,
				['z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'e', 'z', 'g', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4807);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2621);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4809);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2622);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7992);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7993);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7995);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7996);
									s.is_empty()
								}
								['b', 'o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8427);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8428);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8429);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7997);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2623);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 6818);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8431);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8432);
											match s {
												[] => true,
												['e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 8580);
													s.is_empty()
												}
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
							super::push(out, 6819);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6820);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8433);
											s.is_empty()
										}
										['e', 'd', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8581);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 8582);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 8434);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8435);
											s.is_empty()
										}
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

/// Words ending -zować.
pub(crate) fn n1142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1084);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2624);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4812);
					s.is_empty()
				}
				['g', 'a', s @ ..] => {
					super::push(out, 4813);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6821);
							s.is_empty()
						}
						['f', 'a', 'r', 'a', 'p', 's', s @ ..] => {
							super::push(out, 6822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2625);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4814);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6823);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4815);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 6825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', 'm', 'z', s @ ..] => {
					super::push(out, 4816);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4817);
					match s {
						[] => true,
						['i', 'l', 'z', s @ ..] => {
							super::push(out, 6826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4818);
					s.is_empty()
				}
				['r', 'a', 'k', 'y', s @ ..] => {
					super::push(out, 4819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2626);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6827);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n2831(s, out),
		['n', 'e', 'c', 'e', 'r', 'z', s @ ..] => {
			super::push(out, 2628);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2629);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4830);
					s.is_empty()
				}
				['d', 'e', s @ ..] => {
					super::push(out, 4831);
					s.is_empty()
				}
				['n', 'g', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'd', 'z', s @ ..] => {
							super::push(out, 6849);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2630);
			match s {
				[] => true,
				['a', 'k', 'e', 't', 'o', 'i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2631);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4834);
					match s {
						[] => true,
						['ł', 'a', 'p', 's', s @ ..] => {
							super::push(out, 6851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6852);
							match s {
								[] => true,
								['e', 'd', 'a', s @ ..] => {
									super::push(out, 8042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6853);
							match s {
								[] => true,
								['e', 'i', 'k', 'o', s @ ..] => {
									super::push(out, 8043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4835);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6854);
							match s {
								[] => true,
								['a', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6856);
							match s {
								[] => true,
								['l', 'b', 'z', s @ ..] => {
									super::push(out, 8045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6857);
							match s {
								[] => true,
								['w', 'o', s @ ..] => {
									super::push(out, 8046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4836);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 6858);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6859);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 8047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4837);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6860);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6861);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 4838);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4839);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6863);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6864);
							s.is_empty()
						}
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6865);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4841);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t3::n2837(s, out),
		['z', s @ ..] => {
			super::push(out, 2634);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 2635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łować.
pub(crate) fn n1143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1085);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2636);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4848);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4849);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4850);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4851);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6882);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4852);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 4853);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2637);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4854);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4855);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							super::push(out, 6885);
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
				['m', s @ ..] => {
					super::push(out, 4856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6886);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4858);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 6889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2638);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6890);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6892);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6893);
							match s {
								[] => true,
								['t', 'o', 'r', 'p', 'a', s @ ..] => {
									super::push(out, 8067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6894);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4860);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4861);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 6896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2639);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2640);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2641);
			match s {
				[] => true,
				['k', 'y', 't', 'r', 'a', 'y', s @ ..] => {
					super::push(out, 4862);
					s.is_empty()
				}
				['m', 'r', 'o', 'f', 's', s @ ..] => {
					super::push(out, 4863);
					s.is_empty()
				}
				['t', 'y', 't', 'a', s @ ..] => {
					super::push(out, 4864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2642);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żować.
pub(crate) fn n1144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1086);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2644);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4865);
					match s {
						[] => true,
						['n', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8068);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'a', 'a', s @ ..] => {
					super::push(out, 4866);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4867);
					match s {
						[] => true,
						['o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4868);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4869);
					s.is_empty()
				}
				['t', 'n', 'a', 'z', 's', 'a', s @ ..] => {
					super::push(out, 4870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'i', 'p', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2645);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['l', 'a', 'r', 'a', 'p', 's', s @ ..] => {
					super::push(out, 4871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2647);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6899);
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6900);
							match s {
								[] => true,
								['a', 'a', s @ ..] => {
									super::push(out, 8071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 6901);
							match s {
								[] => true,
								['e', 'r', 'z', s @ ..] => {
									super::push(out, 8072);
									s.is_empty()
								}
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
			super::push(out, 2648);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2649);
			match s {
				[] => true,
				['z', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'r', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4873);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ekupywać.
pub(crate) fn n7693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6916);
	s.is_empty()
}

/// Words ending -okupywać.
pub(crate) fn n7694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6917);
	s.is_empty()
}

/// Words ending -wkupywać.
pub(crate) fn n7695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6918);
	s.is_empty()
}

/// Words ending -ykupywać.
pub(crate) fn n7696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6919);
	s.is_empty()
}

/// Words ending -tupywać.
pub(crate) fn n5431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4903);
	s.is_empty()
}

/// Words ending -łupywać.
pub(crate) fn n5432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4904);
	s.is_empty()
}

/// Words ending -ypywać.
pub(crate) fn n2895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2687);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 4905);
			match s {
				[] => true,
				['c', 'z', 's', 'd', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6920);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rywać.
pub(crate) fn n1163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2688);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4906);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2689);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4908);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2690);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2691);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2692);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2693);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 4910);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4911);
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4912);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2694);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4914);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8078);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4915);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 6922);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2695);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6924);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6925);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6926);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6927);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6928);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6929);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 8080);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sywać.
pub(crate) fn n1164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	match s {
		[] => true,
		['i', 'p', 'o', s @ ..] => {
			super::push(out, 2697);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tywać.
pub(crate) fn n1165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2698);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 6931);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6932);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6933);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6934);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2699);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2700);
			s.is_empty()
		}
		['s', 'y', 'ł', s @ ..] => {
			super::push(out, 2701);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2702);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4918);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4919);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4920);
					match s {
						[] => true,
						['c', 'y', s @ ..] => {
							super::push(out, 6936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 2703);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 2704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wywać.
pub(crate) fn n1166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1107);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4921);
					match s {
						[] => true,
						['a', 'r', 'p', 'd', s @ ..] => {
							super::push(out, 6937);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8083);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4922);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4923);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6938);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4924);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6940);
							match s {
								[] => true,
								['p', 'y', s @ ..] => {
									super::push(out, 8085);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 8503);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8504);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6941);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6942);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 8505);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8506);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6943);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 8507);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'y', s @ ..] => {
							super::push(out, 6944);
							match s {
								[] => true,
								['p', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 8086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6945);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8087);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 8509);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 8510);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4925);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4926);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6946);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6947);
							match s {
								[] => true,
								['i', 'p', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8511);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 8512);
											s.is_empty()
										}
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
					super::push(out, 4927);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6948);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4928);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6950);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 8089);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8513);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 8514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6951);
							match s {
								[] => true,
								['ę', 'w', 'y', s @ ..] => {
									super::push(out, 8090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6952);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6953);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 8091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4929);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4930);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6954);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6955);
							match s {
								[] => true,
								['o', 'r', 'p', 'z', s @ ..] => {
									super::push(out, 8092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4931);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zywać.
pub(crate) fn n1167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2705);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4932);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2706);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2707);
			s.is_empty()
		}
		['e', 'z', 'r', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4934);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2708);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6958);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'z', s @ ..] => {
							super::push(out, 6959);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8094);
									s.is_empty()
								}
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
			super::push(out, 2709);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2710);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 2711);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2712);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 2713);
			match s {
				[] => true,
				['i', 'w', s @ ..] => {
					match s {
						[] => true,
						['o', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6960);
							s.is_empty()
						}
						['y', 'w', s @ ..] => {
							super::push(out, 6961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ływać.
pub(crate) fn n1168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2714);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2715);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2716);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2717);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2718);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4936);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4937);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6963);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6964);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 8095);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żywać.
pub(crate) fn n1169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2720);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2721);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2722);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4942);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2723);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2724);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pomnieć.
pub(crate) fn n5613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5063);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7053);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7054);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8113);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7055);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7056);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7057);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 7058);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 8115);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8116);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tomnieć.
pub(crate) fn n5614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5064);
	match s {
		[] => true,
		['y', 'z', 'r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8117);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nnieć.
pub(crate) fn n1276(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'i', 's', 'e', 'j', '\u{1}', s @ ..] => {
			super::push(out, 2854);
			s.is_empty()
		}
		['i', 'c', 'e', 'i', 'z', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5065);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', 's', 'u', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5067);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5068);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pnieć.
pub(crate) fn n1278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1206);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2855);
			s.is_empty()
		}
		['o', 't', 's', s @ ..] => {
			super::push(out, 2856);
			s.is_empty()
		}
		['ę', 's', 'o', 'p', 's', s @ ..] => {
			super::push(out, 2857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnieć.
pub(crate) fn n1279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1207);
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5069);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'ę', 'j', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5071);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7059);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7060);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7062);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7063);
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
				['k', 'o', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7064);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2858);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 5074);
					s.is_empty()
				}
				['m', 'h', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7066);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7067);
							match s {
								[] => true,
								['p', 's', s @ ..] => {
									super::push(out, 8119);
									s.is_empty()
								}
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

/// Words ending -tnieć.
pub(crate) fn n1280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1208);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2859);
			match s {
				[] => true,
				['k', 'i', 'l', 'e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7068);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7069);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7071);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'p', 'z', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5075);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'k', 'ę', 'ł', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5077);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2860);
			match s {
				[] => true,
				['g', 'l', 'i', 'w', 'z', s @ ..] => {
					super::push(out, 5079);
					s.is_empty()
				}
				['k', 'r', 'a', 'm', 'z', s @ ..] => {
					super::push(out, 5080);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5081);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2861);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5083);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5084);
					match s {
						[] => true,
						['k', 's', s @ ..] => {
							super::push(out, 7074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 5085);
					match s {
						[] => true,
						['g', 'z', s @ ..] => {
							super::push(out, 7075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5086);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			match s {
				[] => true,
				['j', 'o', 'b', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7076);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7078);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7079);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wnieć.
pub(crate) fn n1281(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'r', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5088);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', 'z', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5090);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word kretynieć.
pub(crate) fn n1282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1209);
	s.is_empty()
}

/// Words ending -znieć.
pub(crate) fn n1283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1210);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 5092);
					s.is_empty()
				}
				['ę', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7081);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7082);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -śnieć.
pub(crate) fn n1284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1211);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 5093);
					s.is_empty()
				}
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5094);
					s.is_empty()
				}
				['w', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2863);
			s.is_empty()
		}
		['u', 'n', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żnieć.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1212);
	match s {
		[] => true,
		['a', 'w', 'o', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5096);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7084);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7086);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 'o', 'd', 'e', 'i', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7088);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ietrzeć.
pub(crate) fn n5767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5195);
	match s {
		[] => true,
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8145);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 8146);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zetrzeć.
pub(crate) fn n5769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5197);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7135);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7136);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utrzeć.
pub(crate) fn n3253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2972);
	match s {
		[] => true,
		['l', 'z', '\u{1}', s @ ..] => {
			super::push(out, 5198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytrzeć.
pub(crate) fn n3254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2973);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wrzeć.
pub(crate) fn n1355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1268);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2974);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2975);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 5200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5201);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 7138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'o', s @ ..] => {
					super::push(out, 5202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bodzić.
pub(crate) fn n3777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3433);
	match s {
		[] => true,
		['o', 'w', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7235);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7236);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7237);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 8163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -godzić.
pub(crate) fn n3778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3434);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5508);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5509);
			match s {
				[] => true,
				['ł', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8164);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 8165);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5510);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5511);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 7238);
					s.is_empty()
				}
				['z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hodzić.
pub(crate) fn n3779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3435);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7240);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8167);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 7241);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 8169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7242);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7243);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8527);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 7245);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kodzić.
pub(crate) fn n3780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3436);
	match s {
		[] => true,
		['z', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7247);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7248);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7249);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rodzić.
pub(crate) fn n3782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5513);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5514);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7251);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5516);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7253);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7254);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 8173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7255);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7256);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7257);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 8174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5518);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 7258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5519);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5520);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 5521);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5522);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 7259);
					s.is_empty()
				}
				['z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wodzić.
pub(crate) fn n3783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3439);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5524);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 7261);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5525);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5526);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5527);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5528);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7263);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5529);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5530);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 7265);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5531);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7267);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łodzić.
pub(crate) fn n3784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3440);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5532);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7270);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7271);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7272);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5533);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5534);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7274);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7276);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdzić.
pub(crate) fn n1739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1612);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3441);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5536);
					s.is_empty()
				}
				['w', 't', 'a', s @ ..] => {
					super::push(out, 5537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3442);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7278);
							s.is_empty()
						}
						['w', 't', '\u{1}', s @ ..] => {
							super::push(out, 7279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udzić.
pub(crate) fn n1740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1613);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3443);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3444);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 3445);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5539);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3446);
			match s {
				[] => true,
				['s', 'a', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7280);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3447);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 3448);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3449);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5541);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5543);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 3450);
			s.is_empty()
		}
		['ł', '\u{1}', s @ ..] => {
			super::push(out, 3451);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word krzywdzić.
pub(crate) fn n3796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3452);
	s.is_empty()
}

/// Words ending -ydzić.
pub(crate) fn n1742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1615);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5545);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3453);
			match s {
				[] => true,
				['s', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7284);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7285);
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
				['r', s @ ..] => {
					super::push(out, 5547);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8176);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7286);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5549);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5550);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ądzić.
pub(crate) fn n1743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1616);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3455);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7288);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7289);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 8178);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8179);
									s.is_empty()
								}
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
			super::push(out, 3456);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7290);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7291);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ędzić.
pub(crate) fn n1744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1617);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3457);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 3458);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3459);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5553);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5554);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7293);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 3460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5556);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5557);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8180);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5558);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5559);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3461);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5561);
					match s {
						[] => true,
						['z', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8182);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ździć.
pub(crate) fn n1745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1618);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5563);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5564);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3463);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5566);
					match s {
						[] => true,
						['n', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7296);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7297);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3464);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3465);
			match s {
				[] => true,
				['r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7299);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -liczyć.
pub(crate) fn n3839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3489);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5599);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5600);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7316);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7318);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5601);
			s.is_empty()
		}
		['z', 'o', s @ ..] => {
			super::push(out, 5602);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -niczyć.
pub(crate) fn n3840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3490);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5603);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5604);
			match s {
				[] => true,
				['e', 'r', 'ś', 'o', 'p', 'a', s @ ..] => {
					super::push(out, 7321);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 8189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'w', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5605);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5606);
			match s {
				[] => true,
				['e', 'i', 'p', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8190);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 8191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5607);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 5608);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ćwiczyć.
pub(crate) fn n6223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5610);
	s.is_empty()
}

/// Words ending -ziczyć.
pub(crate) fn n3842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3492);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7323);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7324);
					match s {
						[] => true,
						['i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jczyć.
pub(crate) fn n1783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1655);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 5611);
					s.is_empty()
				}
				['f', 's', '\u{1}', s @ ..] => {
					super::push(out, 5612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'j', '\u{1}', s @ ..] => {
			super::push(out, 3493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kczyć.
pub(crate) fn n1784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1656);
	match s {
		[] => true,
		['ę', 'i', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5613);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lczyć.
pub(crate) fn n1785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1657);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5615);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5616);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5617);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 3494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mczyć.
pub(crate) fn n1786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1658);
	match s {
		[] => true,
		['e', 'i', 'n', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5619);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5620);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oczyć.
pub(crate) fn n1787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1659);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5621);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3496);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3497);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3498);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7325);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8193);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7326);
							match s {
								[] => true,
								['z', 'r', s @ ..] => {
									super::push(out, 8195);
									s.is_empty()
								}
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
			super::push(out, 3499);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5623);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5624);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5625);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 5626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3500);
			match s {
				[] => true,
				['d', 'e', 'j', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7327);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3501);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7329);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7331);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7332);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8196);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8197);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7333);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7334);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 7335);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7337);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', '\u{1}', s @ ..] => {
					super::push(out, 5631);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7339);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3502);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3503);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7341);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7342);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 8198);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8199);
									s.is_empty()
								}
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

/// Words ending -pczyć.
pub(crate) fn n1788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1660);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5634);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 7343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rczyć.
pub(crate) fn n1789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1661);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3505);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5636);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5637);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5638);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 7344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3506);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3507);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 5639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tczyć.
pub(crate) fn n1790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1662);
	match s {
		[] => true,
		['a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uczyć.
pub(crate) fn n1791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1663);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3509);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3510);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 3511);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3512);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3513);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7345);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7346);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7347);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3514);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5640);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yczyć.
pub(crate) fn n1792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1664);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 3517);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5642);
					s.is_empty()
				}
				['z', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3518);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5644);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3519);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5646);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3520);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 3521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5648);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5649);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5650);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zczyć.
pub(crate) fn n1793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1665);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5651);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 7349);
							s.is_empty()
						}
						['ł', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8200);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8201);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 8529);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8202);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 8203);
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
						['i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7350);
							s.is_empty()
						}
						['l', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8204);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7351);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7352);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5652);
					match s {
						[] => true,
						['o', 'p', 's', s @ ..] => {
							super::push(out, 7354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5653);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8206);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7355);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7356);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5654);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 7358);
							s.is_empty()
						}
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 7359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5655);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 7360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -óczyć.
pub(crate) fn n1794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['ł', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5656);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ączyć.
pub(crate) fn n1795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1667);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 3523);
			s.is_empty()
		}
		['ł', '\u{1}', s @ ..] => {
			super::push(out, 3524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęczyć.
pub(crate) fn n1796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1668);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3525);
			match s {
				[] => true,
				['z', 'd', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7361);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5659);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5660);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 7363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5661);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 5662);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5663);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 5664);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łczyć.
pub(crate) fn n1797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1669);
	match s {
		[] => true,
		['a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 3528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńczyć.
pub(crate) fn n1798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1670);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3529);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5665);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8208);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 8209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7364);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7365);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7366);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3530);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 5667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3531);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 5668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ysuszyć.
pub(crate) fn n6393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5758);
	s.is_empty()
}

/// Words ending -tuszyć.
pub(crate) fn n3967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3602);
	match s {
		[] => true,
		['o', '\u{1}', s @ ..] => {
			super::push(out, 5759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zuszyć.
pub(crate) fn n3968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3603);
	s.is_empty()
}

/// Words ending -łuszyć.
pub(crate) fn n3969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3604);
	match s {
		[] => true,
		['g', '\u{1}', s @ ..] => {
			super::push(out, 5760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wszyć.
pub(crate) fn n1824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1696);
	s.is_empty()
}

/// Words ending -yszyć.
pub(crate) fn n1825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1697);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 3605);
			s.is_empty()
		}
		['z', 'r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5761);
					match s {
						[] => true,
						['w', 'o', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8229);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8230);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8231);
									s.is_empty()
								}
								['ł', 'ó', 'p', 's', 'w', '\u{1}', s @ ..] => {
									super::push(out, 8232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5762);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zszyć.
pub(crate) fn n1826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1698);
	s.is_empty()
}

/// Words ending -ószyć.
pub(crate) fn n1827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1699);
	match s {
		[] => true,
		['r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5763);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5764);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęszyć.
pub(crate) fn n1828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1700);
	match s {
		[] => true,
		['w', '\u{1}', s @ ..] => {
			super::push(out, 3606);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ższyć.
pub(crate) fn n1829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1701);
	s.is_empty()
}

/// Words ending -zyknąć.
pub(crate) fn n4133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3746);
	match s {
		[] => true,
		['r', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7477);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łyknąć.
pub(crate) fn n4134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3747);
	s.is_empty()
}

/// Words ending -zknąć.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1827);
	match s {
		[] => true,
		['c', 'z', s @ ..] => {
			super::push(out, 3748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąknąć.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1828);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3749);
			match s {
				[] => true,
				['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3750);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3751);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 5903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3752);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3753);
			match s {
				[] => true,
				['r', 'h', 'c', 'd', s @ ..] => {
					super::push(out, 5904);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęknąć.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1829);
	match s {
		[] => true,
		['d', 'z', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3754);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3755);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7479);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7480);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8244);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8245);
									s.is_empty()
								}
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
			super::push(out, 3756);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3757);
			match s {
				[] => true,
				['k', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5905);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 5907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3758);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3759);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3760);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5908);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5909);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8246);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8247);
									s.is_empty()
								}
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

/// Words ending -łknąć.
pub(crate) fn n1961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1830);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3761);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3762);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 3763);
			match s {
				[] => true,
				['ż', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7483);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -audować.
pub(crate) fn n4798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4361);
	match s {
		[] => true,
		['r', 'f', 'e', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7603);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word budować.
pub(crate) fn n6913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6235);
	s.is_empty()
}

/// Words ending -cudować.
pub(crate) fn n4800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4363);
	s.is_empty()
}

/// The word konkludować.
pub(crate) fn n4801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4364);
	s.is_empty()
}

/// Words ending -ydować.
pub(crate) fn n2656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2460);
	match s {
		[] => true,
		['c', 'e', 'd', 'z', s @ ..] => {
			super::push(out, 4365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ądować.
pub(crate) fn n2657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2461);
	match s {
		[] => true,
		['l', 'y', s @ ..] => {
			super::push(out, 4366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ędować.
pub(crate) fn n2658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2462);
	s.is_empty()
}

/// Words ending -łdować.
pub(crate) fn n2659(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4367);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 6236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'h', 'z', s @ ..] => {
			super::push(out, 4368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -intrygować.
pub(crate) fn n9181(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8532);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrygować.
pub(crate) fn n6946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6265);
	s.is_empty()
}

/// Words ending -tygować.
pub(crate) fn n4845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4405);
	match s {
		[] => true,
		['a', 'f', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7619);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', 'z', '\u{1}', s @ ..] => {
			super::push(out, 6266);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęgować.
pub(crate) fn n2677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2477);
	match s {
		[] => true,
		['i', 's', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6268);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6269);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'o', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6271);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4406);
			match s {
				[] => true,
				['a', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 6273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word promieniować.
pub(crate) fn n9592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8644);
	s.is_empty()
}

/// Words ending -apromieniować.
pub(crate) fn n9593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8645);
	s.is_empty()
}

/// Words ending -fikować.
pub(crate) fn n4900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4452);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6324);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', 'k', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8534);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 8288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'm', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7645);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['o', 's', 'r', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 8289);
							s.is_empty()
						}
						['u', 'z', s @ ..] => {
							super::push(out, 8290);
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
				['c', 'a', 'p', 's', s @ ..] => {
					super::push(out, 7646);
					s.is_empty()
				}
				['d', 'o', s @ ..] => {
					match s {
						[] => true,
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 8291);
							s.is_empty()
						}
						['m', 'z', s @ ..] => {
							super::push(out, 8292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7647);
					match s {
						[] => true,
						['e', 'w', 'z', s @ ..] => {
							super::push(out, 8293);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', 's', s @ ..] => {
									super::push(out, 8536);
									s.is_empty()
								}
								['k', 'e', 'l', 'e', 'z', s @ ..] => {
									super::push(out, 8537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7648);
					match s {
						[] => true,
						['a', 'l', 'k', 's', s @ ..] => {
							super::push(out, 8294);
							s.is_empty()
						}
						['l', 'a', 'f', 's', s @ ..] => {
							super::push(out, 8295);
							s.is_empty()
						}
						['n', 'e', 't', 'n', 'i', 'z', s @ ..] => {
							super::push(out, 8296);
							s.is_empty()
						}
						['r', 'e', 'w', 'y', 'd', 'z', s @ ..] => {
							super::push(out, 8297);
							s.is_empty()
						}
						['u', 'r', 'z', s @ ..] => {
							super::push(out, 8298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7649);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8299);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8538);
									s.is_empty()
								}
								['r', 't', s @ ..] => {
									super::push(out, 8539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 8300);
							match s {
								[] => true,
								['e', 'r', 'z', s @ ..] => {
									super::push(out, 8540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8301);
							match s {
								[] => true,
								['e', 'd', 'i', 'z', s @ ..] => {
									super::push(out, 8541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8302);
							match s {
								[] => true,
								['o', 'f', 'u', s @ ..] => {
									super::push(out, 8542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'm', 'z', '\u{1}', s @ ..] => {
							super::push(out, 8303);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8304);
							match s {
								[] => true,
								['s', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 7650);
					match s {
						[] => true,
						['a', 'n', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8544);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8545);
									s.is_empty()
								}
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

/// Words ending -likować.
pub(crate) fn n4901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4453);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6325);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'u', 'p', 'o', s @ ..] => {
			super::push(out, 6326);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'a', s @ ..] => {
					super::push(out, 7652);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7653);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 8305);
							s.is_empty()
						}
						['o', 'k', 's', s @ ..] => {
							super::push(out, 8306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'k', 'e', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7654);
					s.is_empty()
				}
				['u', 'd', 'z', s @ ..] => {
					super::push(out, 7655);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7656);
					match s {
						[] => true,
						['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nikować.
pub(crate) fn n4902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4454);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6327);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8308);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 6328);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6329);
			match s {
				[] => true,
				['e', 't', 's', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6330);
			match s {
				[] => true,
				['m', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8310);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 8311);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8546);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 8547);
									s.is_empty()
								}
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
			super::push(out, 6331);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 6332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tikować.
pub(crate) fn n4904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4456);
	s.is_empty()
}

/// Words ending -jkować.
pub(crate) fn n2701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2501);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6334);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7659);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6335);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lkować.
pub(crate) fn n2702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2502);
	match s {
		[] => true,
		['e', 'b', 'o', s @ ..] => {
			super::push(out, 4457);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wygumkować.
pub(crate) fn n2703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2503);
	s.is_empty()
}

/// Words ending -nkować.
pub(crate) fn n2704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2504);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4458);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6336);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7661);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 6337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'ł', 'z', 'c', 'z', s @ ..] => {
			super::push(out, 4459);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4460);
			match s {
				[] => true,
				['r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6339);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 6340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -okować.
pub(crate) fn n2705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2505);
	match s {
		[] => true,
		['d', 'a', s @ ..] => {
			super::push(out, 4461);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6341);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6342);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 7663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 6343);
					s.is_empty()
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4463);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 6345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4464);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 4465);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6346);
					s.is_empty()
				}
				['o', 'r', 'p', 's', s @ ..] => {
					super::push(out, 6347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 's', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6348);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pkować.
pub(crate) fn n2706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2506);
	s.is_empty()
}

/// Words ending -rkować.
pub(crate) fn n2707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2507);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4466);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6350);
					match s {
						[] => true,
						['m', 'a', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 7665);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6351);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', s @ ..] => {
					super::push(out, 6352);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 6353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'ę', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4467);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4468);
			match s {
				[] => true,
				['t', 's', 'j', 'a', 'm', 'o', s @ ..] => {
					super::push(out, 6354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4469);
			match s {
				[] => true,
				['k', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4470);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6356);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6357);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -skować.
pub(crate) fn n2708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2508);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4471);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7669);
							s.is_empty()
						}
						['e', 'd', 'z', s @ ..] => {
							super::push(out, 7670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4472);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', 'n', 'o', 'k', 's', s @ ..] => {
					super::push(out, 6358);
					s.is_empty()
				}
				['n', 'g', 'o', 'z', s @ ..] => {
					super::push(out, 6359);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6361);
					match s {
						[] => true,
						['n', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4473);
			match s {
				[] => true,
				['p', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6363);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tkować.
pub(crate) fn n2709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2509);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4474);
			match s {
				[] => true,
				['b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6364);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6365);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7672);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6366);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6367);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8313);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6368);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6369);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8315);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8316);
									s.is_empty()
								}
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
			super::push(out, 4475);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4476);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6370);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8317);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8318);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8319);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 8548);
											s.is_empty()
										}
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
					super::push(out, 6371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4477);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7674);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4478);
			s.is_empty()
		}
		['y', 'ż', 'u', '\u{1}', s @ ..] => {
			super::push(out, 4479);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 4480);
			match s {
				[] => true,
				['i', 's', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ukować.
pub(crate) fn n2710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2510);
	match s {
		[] => true,
		['b', 'a', s @ ..] => {
			super::push(out, 4481);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6373);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'y', s @ ..] => {
							super::push(out, 7676);
							s.is_empty()
						}
						['r', 'z', s @ ..] => {
							super::push(out, 7677);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6374);
					s.is_empty()
				}
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'z', s @ ..] => {
							super::push(out, 7679);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7680);
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
				['b', 'y', s @ ..] => {
					super::push(out, 6375);
					s.is_empty()
				}
				['d', 'y', s @ ..] => {
					super::push(out, 6376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wkować.
pub(crate) fn n2711(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4482);
			match s {
				[] => true,
				['d', 'e', s @ ..] => {
					super::push(out, 6377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 4483);
			match s {
				[] => true,
				['ł', 'g', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ykować.
pub(crate) fn n2712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2511);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4484);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6379);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6380);
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 7681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'b', 'a', 'f', 's', '\u{1}', s @ ..] => {
			super::push(out, 4485);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4486);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6382);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6383);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6384);
					match s {
						[] => true,
						['r', 'k', 's', s @ ..] => {
							super::push(out, 7683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4487);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6385);
					match s {
						[] => true,
						['l', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8320);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8321);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7684);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6387);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6388);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8322);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8323);
									s.is_empty()
								}
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
			super::push(out, 4488);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zkować.
pub(crate) fn n2713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2512);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4489);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6389);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6390);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6391);
					match s {
						[] => true,
						['l', 'o', 'p', 's', s @ ..] => {
							super::push(out, 7686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6392);
					match s {
						[] => true,
						['t', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 6393);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7688);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7689);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7690);
							match s {
								[] => true,
								['o', 'g', 'z', s @ ..] => {
									super::push(out, 8324);
									s.is_empty()
								}
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
			super::push(out, 4490);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', 'u', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7691);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7692);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6394);
					match s {
						[] => true,
						['i', 'm', 'o', s @ ..] => {
							super::push(out, 7694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6395);
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							super::push(out, 7695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ękować.
pub(crate) fn n2714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2513);
	match s {
		[] => true,
		['i', 'z', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6397);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6398);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łkować.
pub(crate) fn n2715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2514);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4491);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6399);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4492);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6400);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7697);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6401);
					match s {
						[] => true,
						['k', 'y', s @ ..] => {
							super::push(out, 7699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żkować.
pub(crate) fn n2716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2515);
	s.is_empty()
}

/// Words ending -zolować.
pub(crate) fn n4990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4532);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7708);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7709);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7710);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -plować.
pub(crate) fn n2730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2529);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6441);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8325);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 8326);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6442);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6443);
					match s {
						[] => true,
						['k', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kapslować.
pub(crate) fn n4993(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6444);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 6445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tlować.
pub(crate) fn n2732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2531);
	match s {
		[] => true,
		['j', 'a', 'b', 'z', '\u{1}', s @ ..] => {
			super::push(out, 4534);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulować.
pub(crate) fn n2733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2532);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4535);
			match s {
				[] => true,
				['f', 's', s @ ..] => {
					super::push(out, 6446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'a', 'f', 'n', 'o', 'k', 's', '\u{1}', s @ ..] => {
			super::push(out, 4536);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4537);
			match s {
				[] => true,
				['o', 'm', 'z', s @ ..] => {
					super::push(out, 6447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4538);
			match s {
				[] => true,
				['e', 'r', 'u', s @ ..] => {
					super::push(out, 6448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4539);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6449);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6450);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6451);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6452);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4540);
			match s {
				[] => true,
				['u', 'k', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', s @ ..] => {
							super::push(out, 7713);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4541);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4542);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6454);
					match s {
						[] => true,
						['n', 'a', 'm', 'z', s @ ..] => {
							super::push(out, 7715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4543);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6456);
					match s {
						[] => true,
						['r', 'g', 'o', s @ ..] => {
							super::push(out, 7716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 'a', 'k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7717);
							match s {
								[] => true,
								['r', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylować.
pub(crate) fn n2734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2533);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4545);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6458);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4546);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6460);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4547);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4548);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zlować.
pub(crate) fn n2735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2534);
	match s {
		[] => true,
		['e', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ólować.
pub(crate) fn n2736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2535);
	s.is_empty()
}

/// Words ending -onować.
pub(crate) fn n2754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2553);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4596);
			match s {
				[] => true,
				['a', 'a', s @ ..] => {
					super::push(out, 6510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4597);
			match s {
				[] => true,
				['e', 'l', 'e', 't', 'a', s @ ..] => {
					super::push(out, 6511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4598);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6512);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6513);
					match s {
						[] => true,
						['e', 'i', 'z', 'd', 'z', s @ ..] => {
							super::push(out, 7745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6514);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8337);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 8338);
									match s {
										[] => true,
										['k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8646);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 8647);
													s.is_empty()
												}
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
		['j', s @ ..] => {
			super::push(out, 4599);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 'r', 'z', s @ ..] => {
									super::push(out, 8339);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8340);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'p', 'e', 'd', 'z', s @ ..] => {
							super::push(out, 7746);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7747);
							match s {
								[] => true,
								['b', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['a', 'f', 's', 'y', 't', 'a', 's', 'u', s @ ..] => {
									super::push(out, 8343);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 8549);
											s.is_empty()
										}
										['l', 'o', 'k', 's', s @ ..] => {
											super::push(out, 8550);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8344);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8551);
											match s {
												[] => true,
												['s', 'u', s @ ..] => {
													super::push(out, 8648);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 8552);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'e', 'd', 'i', 'w', 'e', 'z', s @ ..] => {
							super::push(out, 7748);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7749);
							s.is_empty()
						}
						['u', 'l', 'o', 'w', 'e', 'r', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7750);
							s.is_empty()
						}
						['y', 'z', 'o', 'p', 'y', s @ ..] => {
							super::push(out, 7751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4600);
			match s {
				[] => true,
				['k', 's', s @ ..] => {
					super::push(out, 6516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4601);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6517);
					match s {
						[] => true,
						['d', 'z', s @ ..] => {
							super::push(out, 7752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6518);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7753);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8345);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7754);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8553);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8554);
											s.is_empty()
										}
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
					super::push(out, 6519);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7755);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7756);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7757);
							match s {
								[] => true,
								['p', 'a', s @ ..] => {
									super::push(out, 8347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6520);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7758);
							match s {
								[] => true,
								['e', 'y', s @ ..] => {
									super::push(out, 8348);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7759);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 6521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4602);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6522);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6523);
					match s {
						[] => true,
						['k', 'u', s @ ..] => {
							super::push(out, 7761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4603);
			match s {
				[] => true,
				['a', 'f', 'e', 'd', 'z', s @ ..] => {
					super::push(out, 6525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4604);
			match s {
				[] => true,
				['e', 'd', 'z', s @ ..] => {
					super::push(out, 6526);
					s.is_empty()
				}
				['n', 'i', 'a', s @ ..] => {
					super::push(out, 6527);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4605);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 6529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ynować.
pub(crate) fn n2758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2557);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['s', 'a', 'f', 'a', s @ ..] => {
					super::push(out, 6533);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6534);
					match s {
						[] => true,
						['l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'r', 'o', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6535);
					s.is_empty()
				}
				['o', 'k', 's', s @ ..] => {
					super::push(out, 6536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', 'a', s @ ..] => {
					super::push(out, 6537);
					s.is_empty()
				}
				['t', 'k', 'o', 'd', 'n', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['o', 'l', 'i', 'g', 'z', s @ ..] => {
					super::push(out, 6539);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6540);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'a', 'g', 'a', 'm', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6542);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 4609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -acerować.
pub(crate) fn n7278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6567);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7791);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7792);
			match s {
				[] => true,
				['s', 'o', s @ ..] => {
					super::push(out, 8355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 7793);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word inkarcerować.
pub(crate) fn n8632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7794);
	s.is_empty()
}

/// Words ending -derować.
pub(crate) fn n5123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4651);
	match s {
		[] => true,
		['n', 'e', 'm', 'o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7795);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7796);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 6570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ferować.
pub(crate) fn n5124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4652);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 6571);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6572);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7799);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gerować.
pub(crate) fn n5125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4653);
	match s {
		[] => true,
		['g', 'i', 'r', 't', 's', s @ ..] => {
			super::push(out, 6573);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6574);
			s.is_empty()
		}
		['u', 's', 'a', s @ ..] => {
			super::push(out, 6575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ierować.
pub(crate) fn n5126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4654);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7800);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7801);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8356);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7802);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7803);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7804);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jerować.
pub(crate) fn n5127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4655);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7806);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 8358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 7807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olerować.
pub(crate) fn n5128(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', 'y', s @ ..] => {
			super::push(out, 6577);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -merować.
pub(crate) fn n5129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4656);
	match s {
		[] => true,
		['a', 'k', 's', '\u{1}', s @ ..] => {
			super::push(out, 6579);
			s.is_empty()
		}
		['u', 'n', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'p', 'a', s @ ..] => {
					super::push(out, 7808);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nerować.
pub(crate) fn n5130(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6580);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'z', s @ ..] => {
									super::push(out, 8555);
									s.is_empty()
								}
								['r', 'z', s @ ..] => {
									super::push(out, 8556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 8359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6581);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -perować.
pub(crate) fn n5131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4657);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6583);
			match s {
				[] => true,
				['r', 'z', s @ ..] => {
					super::push(out, 7810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6584);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7811);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -serować.
pub(crate) fn n5132(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6586);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 7813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 's', s @ ..] => {
			super::push(out, 6587);
			s.is_empty()
		}
		['y', 'ż', 'e', 'r', 'y', s @ ..] => {
			super::push(out, 6588);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -terować.
pub(crate) fn n5133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4658);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6589);
			match s {
				[] => true,
				['w', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6590);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 7815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 6591);
			match s {
				[] => true,
				['e', 'r', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6592);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6593);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7817);
					match s {
						[] => true,
						['z', 'c', 'y', s @ ..] => {
							super::push(out, 8360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7818);
					match s {
						[] => true,
						['z', 'e', 'd', 'z', s @ ..] => {
							super::push(out, 8361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', s @ ..] => {
			super::push(out, 6594);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ygrawerować.
pub(crate) fn n5134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4659);
	s.is_empty()
}

/// Words ending -szerować.
pub(crate) fn n5135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4660);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7819);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'f', 's', '\u{1}', s @ ..] => {
			super::push(out, 6595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żerować.
pub(crate) fn n5136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4661);
	s.is_empty()
}

/// Words ending -frować.
pub(crate) fn n2774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2572);
	match s {
		[] => true,
		['l', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4662);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7821);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7822);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7824);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7825);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7826);
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8363);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -grować.
pub(crate) fn n2775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2573);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4663);
			match s {
				[] => true,
				['b', 'y', s @ ..] => {
					super::push(out, 6596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4664);
			match s {
				[] => true,
				['t', 'n', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4665);
			match s {
				[] => true,
				['m', 'e', 'y', s @ ..] => {
					super::push(out, 6598);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word oszachrować.
pub(crate) fn n2776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2574);
	s.is_empty()
}

/// Words ending -irować.
pub(crate) fn n2777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2575);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4666);
			s.is_empty()
		}
		['f', 'a', 't', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 4667);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4668);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7828);
							s.is_empty()
						}
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4669);
			s.is_empty()
		}
		['p', 's', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6599);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6600);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7830);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7831);
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
				['a', s @ ..] => {
					super::push(out, 6601);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6602);
					match s {
						[] => true,
						['e', 'r', 'a', s @ ..] => {
							super::push(out, 7832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 6603);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -krować.
pub(crate) fn n2778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2576);
	match s {
		[] => true,
		['a', 's', 'a', 'm', 'z', s @ ..] => {
			super::push(out, 4670);
			s.is_empty()
		}
		['e', 's', 'n', 'o', 'k', 'y', s @ ..] => {
			super::push(out, 4671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -aklamrować.
pub(crate) fn n2779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2577);
	s.is_empty()
}

/// Words ending -orować.
pub(crate) fn n2780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2578);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4672);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6604);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8557);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 8558);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 8366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'a', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6605);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 6606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4673);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4674);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7834);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7835);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4675);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6608);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6609);
					s.is_empty()
				}
				['e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7837);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4676);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6611);
					s.is_empty()
				}
				['o', 'k', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
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
		['m', 'a', 'z', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 4677);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['g', 'i', 'z', s @ ..] => {
					super::push(out, 6612);
					s.is_empty()
				}
				['o', 'h', 'u', '\u{1}', s @ ..] => {
					super::push(out, 6613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4678);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4679);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'n', 'o', 'p', 's', 'a', s @ ..] => {
			super::push(out, 4680);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4681);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4682);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4683);
			match s {
				[] => true,
				['o', 'p', 'u', s @ ..] => {
					super::push(out, 6616);
					s.is_empty()
				}
				['s', 'y', s @ ..] => {
					super::push(out, 6617);
					s.is_empty()
				}
				['w', 'd', s @ ..] => {
					super::push(out, 6618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trować.
pub(crate) fn n2781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2579);
	match s {
		[] => true,
		['e', 'n', 'e', 'p', 's', s @ ..] => {
			super::push(out, 4684);
			s.is_empty()
		}
		['l', 'i', 'f', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6619);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6620);
					s.is_empty()
				}
				['n', 'i', 'z', s @ ..] => {
					super::push(out, 6621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', 'c', 'n', 'o', 'k', 's', s @ ..] => {
					super::push(out, 6622);
					s.is_empty()
				}
				['o', 'k', 's', s @ ..] => {
					super::push(out, 6623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'k', 'y', s @ ..] => {
					super::push(out, 6624);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6625);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 7841);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8559);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8560);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 7842);
							match s {
								[] => true,
								['l', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 8367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'n', 'i', 'm', 'd', 'a', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7843);
							s.is_empty()
						}
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7844);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7845);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7846);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 7847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'a', 'm', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6626);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6627);
					match s {
						[] => true,
						['o', 'm', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8561);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 8562);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8368);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8563);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', 'z', s @ ..] => {
									super::push(out, 8369);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'f', 's', '\u{1}', s @ ..] => {
							super::push(out, 7848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4685);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4686);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urować.
pub(crate) fn n2782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2580);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4687);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7849);
							s.is_empty()
						}
						['n', 'o', 'k', 's', s @ ..] => {
							super::push(out, 7850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6628);
					s.is_empty()
				}
				['u', 'a', 'n', 'i', 'a', s @ ..] => {
					super::push(out, 6629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 4688);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['e', 's', 'a', 'a', s @ ..] => {
					super::push(out, 6630);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6631);
					s.is_empty()
				}
				['o', 'r', 'p', 's', s @ ..] => {
					super::push(out, 6632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4689);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'z', 's', 'a', s @ ..] => {
			super::push(out, 4690);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6634);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6635);
					match s {
						[] => true,
						['a', 'w', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8371);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8372);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'n', 'e', 'c', 'o', s @ ..] => {
			super::push(out, 4691);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 4692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wrować.
pub(crate) fn n2783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2581);
	match s {
		[] => true,
		['e', 'n', 'a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6637);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6638);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yrować.
pub(crate) fn n2784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2582);
	match s {
		[] => true,
		['ż', 'd', s @ ..] => {
			super::push(out, 4693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -órować.
pub(crate) fn n2785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2583);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4694);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4695);
			match s {
				[] => true,
				['s', 'o', s @ ..] => {
					super::push(out, 6640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nsować.
pub(crate) fn n2791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2589);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6664);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7870);
							s.is_empty()
						}
						['i', 'b', 'z', s @ ..] => {
							super::push(out, 7871);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6665);
					s.is_empty()
				}
				['n', 'i', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7873);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7874);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'y', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7876);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'i', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7878);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6666);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4711);
			match s {
				[] => true,
				['d', 'n', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7881);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7883);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8377);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'n', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6667);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -psować.
pub(crate) fn n2793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2591);
	match s {
		[] => true,
		['o', 'r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6671);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rsować.
pub(crate) fn n2794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2592);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4717);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8379);
									s.is_empty()
								}
								['u', 'b', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7887);
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
				['f', s @ ..] => {
					super::push(out, 6673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssować.
pub(crate) fn n2795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2593);
	match s {
		[] => true,
		['e', 'r', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4719);
			s.is_empty()
		}
		['i', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6675);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usować.
pub(crate) fn n2796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2594);
	match s {
		[] => true,
		['l', 'z', 's', 'b', s @ ..] => {
			super::push(out, 4720);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 6677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'o', s @ ..] => {
			super::push(out, 4721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ysować.
pub(crate) fn n2797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2595);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6679);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6680);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6681);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7889);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'a', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6682);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6683);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -litować.
pub(crate) fn n5246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4745);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6707);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6708);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', 'e', s @ ..] => {
							super::push(out, 8385);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8649);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8650);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6709);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8387);
							s.is_empty()
						}
						['s', 'o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8564);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8565);
									s.is_empty()
								}
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
			super::push(out, 6710);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mitować.
pub(crate) fn n5247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4746);
	match s {
		[] => true,
		['e', 'y', s @ ..] => {
			super::push(out, 6711);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7904);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'p', 'm', 'o', 'k', 's', s @ ..] => {
			super::push(out, 6712);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6713);
			match s {
				[] => true,
				['k', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7906);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -witować.
pub(crate) fn n5249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4748);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6714);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6715);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 6716);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7907);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 7908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jtować.
pub(crate) fn n2803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2601);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4749);
			match s {
				[] => true,
				['l', 'p', 's', s @ ..] => {
					super::push(out, 6717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4750);
			match s {
				[] => true,
				['h', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ktować.
pub(crate) fn n2804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2602);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6719);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6720);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8388);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6721);
					match s {
						[] => true,
						['n', 'o', 'k', 's', s @ ..] => {
							super::push(out, 7909);
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
				['f', 'a', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6722);
					s.is_empty()
				}
				['j', 'o', 'r', 'p', 'a', s @ ..] => {
					super::push(out, 6723);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6724);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7910);
							s.is_empty()
						}
						['f', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8390);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 8391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6725);
					match s {
						[] => true,
						['a', 'a', s @ ..] => {
							super::push(out, 7911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4751);
			s.is_empty()
		}
		['y', 'd', 'o', s @ ..] => {
			super::push(out, 4752);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltować.
pub(crate) fn n2805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2603);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4753);
			match s {
				[] => true,
				['f', 's', 'a', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7912);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6726);
					match s {
						[] => true,
						['g', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4754);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6727);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6728);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4755);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6729);
					match s {
						[] => true,
						['s', 'u', 'a', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6730);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6731);
					match s {
						[] => true,
						['n', 'o', 'k', 's', s @ ..] => {
							super::push(out, 7917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 6732);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntować.
pub(crate) fn n2806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2604);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4756);
			match s {
				[] => true,
				['b', 'r', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6733);
					s.is_empty()
				}
				['k', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6734);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6735);
					match s {
						[] => true,
						['a', 'w', 'g', 'a', s @ ..] => {
							super::push(out, 7919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6736);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8392);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 8393);
									s.is_empty()
								}
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
				['c', 'k', 'a', 'a', s @ ..] => {
					super::push(out, 6737);
					s.is_empty()
				}
				['i', 'r', 'o', 'z', s @ ..] => {
					super::push(out, 6738);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6739);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8394);
									s.is_empty()
								}
								['g', 'e', 'r', 'z', s @ ..] => {
									super::push(out, 8395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7921);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8396);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8566);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8397);
									s.is_empty()
								}
								['l', 'p', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 8567);
											match s {
												[] => true,
												['o', 'k', 's', s @ ..] => {
													super::push(out, 8651);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 8568);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7922);
							s.is_empty()
						}
						['o', 'k', 's', s @ ..] => {
							super::push(out, 7923);
							s.is_empty()
						}
						['r', 'e', 'f', 's', s @ ..] => {
							super::push(out, 7924);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7925);
							match s {
								[] => true,
								['k', 'o', 'd', 'u', s @ ..] => {
									super::push(out, 8398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 7926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6741);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8399);
									s.is_empty()
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8569);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8570);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8400);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'e', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7928);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7929);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4757);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7931);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'z', s @ ..] => {
									super::push(out, 8402);
									s.is_empty()
								}
								['r', 'y', s @ ..] => {
									super::push(out, 8403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', 'n', 'o', 'k', 's', s @ ..] => {
					super::push(out, 6742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', 'z', s @ ..] => {
					super::push(out, 6743);
					s.is_empty()
				}
				['r', 'g', 'u', s @ ..] => {
					super::push(out, 6744);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otować.
pub(crate) fn n2807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2605);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4758);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4759);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4760);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6746);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 7933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7934);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'j', 'o', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6747);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4761);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6749);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6750);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8404);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4762);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4763);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptować.
pub(crate) fn n2808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2606);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4765);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6752);
					match s {
						[] => true,
						['a', 'a', s @ ..] => {
							super::push(out, 7937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7938);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4766);
			match s {
				[] => true,
				['c', 'k', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7940);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6754);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6755);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7943);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

