use flexion::Path;

/// Words ending -ja.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2684);
					match s {
						[] => true,
						['a', 'c', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2686);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				['ł', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n513(s, out),
		['e', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2696);
					match s {
						[] => true,
						['z', 'd', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'z', s @ ..] => {
									super::push(out, 10816);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10817);
									match s {
										[] => true,
										['n', 'z', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 13293);
											s.is_empty()
										}
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
					super::push(out, 2697);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6851);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6853);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				['z', 'r', 'e', 'i', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6855);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 6856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 513);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['f', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2699);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2700);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2701);
					match s {
						[] => true,
						['e', 'z', 'y', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6858);
							s.is_empty()
						}
						['i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2702);
					match s {
						[] => true,
						['e', 'd', 'a', 'k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'm', 'r', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2703);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2704);
					match s {
						[] => true,
						['a', 'd', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'u', s @ ..] => {
			super::push(out, 516);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 2706);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2707);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6863);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2709);
					match s {
						[] => true,
						['c', 'e', 's', s @ ..] => {
							super::push(out, 6865);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'o', s @ ..] => {
									super::push(out, 10818);
									s.is_empty()
								}
								['p', 'o', 'p', s @ ..] => {
									super::push(out, 10819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2710);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2711);
					match s {
						[] => true,
						['a', 'r', 'p', 'o', 't', s @ ..] => {
							super::push(out, 6866);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'f', s @ ..] => {
									super::push(out, 10820);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10821);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6867);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6868);
							s.is_empty()
						}
						['o', 'd', 'o', 't', s @ ..] => {
							super::push(out, 6869);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2712);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2713);
					match s {
						[] => true,
						['e', 'l', 'a', 't', 'e', s @ ..] => {
							super::push(out, 6871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6872);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', 's', 'a', s @ ..] => {
									super::push(out, 10823);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 10824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2715);
					s.is_empty()
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['c', s @ ..] => super::t2::n2781(s, out),
				['m', 'o', 'ł', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2718);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2719);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6879);
							match s {
								[] => true,
								['m', s @ ..] => super::t2::n11474(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t2::n7190(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t2::n2784(s, out),
				['t', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2721);
					s.is_empty()
				}
				['z', s @ ..] => super::t2::n2786(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t2::n527(s, out),
		['ó', s @ ..] => {
			super::push(out, 526);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ka.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2726);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6895);
							s.is_empty()
						}
						['e', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2727);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2728);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2729);
					match s {
						[] => true,
						['b', 'a', 'r', 'z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6898);
							s.is_empty()
						}
						['d', 'a', 'w', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2730);
					match s {
						[] => true,
						['i', 'b', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2731);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 6901);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 10838);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13308);
											s.is_empty()
										}
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
					super::push(out, 2732);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['o', 'k', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					super::push(out, 2735);
					match s {
						[] => true,
						['w', 'o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 6902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2736);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2737);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2738);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2739);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6906);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2740);
					match s {
						[] => true,
						['r', '-', s @ ..] => {
							match s {
								[] => true,
								['t', 'a', 'm', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 10840);
									s.is_empty()
								}
								['ć', 'ś', 'e', 'i', 'w', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10841);
									s.is_empty()
								}
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
			super::push(out, 532);
			match s {
				[] => true,
				['a', 'z', 's', 'r', s @ ..] => {
					super::push(out, 2741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2742);
					match s {
						[] => true,
						['o', 'b', s @ ..] => {
							super::push(out, 6907);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6909);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2743);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 10842);
									s.is_empty()
								}
								['t', 'o', 'b', 'o', s @ ..] => {
									super::push(out, 10843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10844);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6911);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 10846);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 13309);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 13310);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6912);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2744);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 2745);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2746);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6915);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									super::push(out, 10848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'c', 'n', s @ ..] => {
					super::push(out, 2747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2748);
					s.is_empty()
				}
				['u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6916);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6917);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 6918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2751);
					match s {
						[] => true,
						['i', 'c', 'a', s @ ..] => {
							super::push(out, 6919);
							match s {
								[] => true,
								['d', 'e', 'i', 'w', 'o', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10849);
									s.is_empty()
								}
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
			super::push(out, 537);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2752);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n2820(s, out),
				['i', 'n', s @ ..] => {
					super::push(out, 2754);
					s.is_empty()
				}
				['o', s @ ..] => super::t2::n2822(s, out),
				['y', s @ ..] => super::t2::n2823(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n541(s, out),
		['p', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['r', s @ ..] => super::t2::n543(s, out),
		['s', s @ ..] => super::t2::n544(s, out),
		['t', s @ ..] => super::t2::n545(s, out),
		['u', s @ ..] => super::t2::n546(s, out),
		['w', s @ ..] => super::t2::n547(s, out),
		['y', s @ ..] => super::t2::n548(s, out),
		['z', s @ ..] => super::t2::n549(s, out),
		['ó', s @ ..] => super::t2::n550(s, out),
		['ą', s @ ..] => super::t2::n551(s, out),
		['ć', s @ ..] => super::t2::n552(s, out),
		['ę', s @ ..] => super::t2::n553(s, out),
		['ł', s @ ..] => super::t2::n554(s, out),
		['ń', s @ ..] => super::t2::n555(s, out),
		['ś', s @ ..] => super::t2::n556(s, out),
		['ź', s @ ..] => super::t2::n557(s, out),
		['ż', s @ ..] => super::t2::n558(s, out),
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2802);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2804);
					match s {
						[] => true,
						['-', 'a', 's', 'k', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2805);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2806);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2807);
					match s {
						[] => true,
						['d', 'ą', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['a', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 559);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6983);
							s.is_empty()
						}
						['a', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2809);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6985);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'z', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2810);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2811);
					match s {
						[] => true,
						['a', 'k', s @ ..] => {
							super::push(out, 6987);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'a', s @ ..] => {
					super::push(out, 2812);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'o', 'l', 'o', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6989);
							s.is_empty()
						}
						['u', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 561);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 562);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 563);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6991);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2814);
					match s {
						[] => true,
						['q', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 6993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 565);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				['y', 'z', s @ ..] => {
					super::push(out, 2817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 567);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6994);
							match s {
								[] => true,
								['r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6996);
							s.is_empty()
						}
						['-', 'a', 'c', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6998);
							s.is_empty()
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2818);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2819);
					match s {
						[] => true,
						['d', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 2820);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7001);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2821);
					match s {
						[] => true,
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 7003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7004);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7005);
							s.is_empty()
						}
						['e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7006);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7008);
							s.is_empty()
						}
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7009);
							s.is_empty()
						}
						['e', 'r', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7010);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2824);
					s.is_empty()
				}
				['u', 'k', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2825);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 569);
			s.is_empty()
		}
		['r', 'o', 'd', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 570);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 571);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2828);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					super::push(out, 2829);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2830);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', 't', 'j', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 574);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['r', 'd', 'a', 'p', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2832);
					s.is_empty()
				}
				['z', 's', 'n', 'y', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 575);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2834);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2835);
					match s {
						[] => true,
						['r', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 2836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2838);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2839);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 7017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2840);
					match s {
						[] => true,
						['g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['ó', s @ ..] => {
					super::push(out, 2842);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2843);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2844);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 580);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2846);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 7021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 582);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2847);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2848);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2849);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 2850);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'o', s @ ..] => {
							super::push(out, 7022);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2852);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 2853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', s @ ..] => {
			super::push(out, 586);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2854);
					s.is_empty()
				}
				['g', 'o', s @ ..] => {
					super::push(out, 2855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'h', s @ ..] => {
			super::push(out, 588);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['y', 'r', 'a', s @ ..] => {
					super::push(out, 2856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ć', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2857);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2858);
					s.is_empty()
				}
				['u', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ś', s @ ..] => {
			super::push(out, 591);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 592);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2860);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2861);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7025);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 10901);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 10902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7026);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2862);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2863);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2864);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7029);
							match s {
								[] => true,
								['a', 'z', 'l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7030);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7031);
							match s {
								[] => true,
								['a', 's', 'r', 'a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 10904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7032);
							match s {
								[] => true,
								['c', 'i', 'w', 'e', 'i', 'k', 'n', 'e', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 10905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2865);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7033);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2866);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					super::push(out, 2867);
					match s {
						[] => true,
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 7036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7037);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2868);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2869);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 7039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2870);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7040);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7041);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2871);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7043);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 10906);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 10907);
									s.is_empty()
								}
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
			super::push(out, 594);
			match s {
				[] => true,
				['ę', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['u', 'ł', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2874);
					s.is_empty()
				}
				['z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['c', 'o', 'o', s @ ..] => {
					super::push(out, 2876);
					s.is_empty()
				}
				['i', 'g', s @ ..] => {
					super::push(out, 2877);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2878);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2879);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 7044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2880);
					s.is_empty()
				}
				['w', 'd', s @ ..] => {
					super::push(out, 2881);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 7046);
							s.is_empty()
						}
						['o', 'g', s @ ..] => {
							super::push(out, 7047);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7048);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 10908);
									s.is_empty()
								}
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
			super::push(out, 599);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2882);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2883);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 7049);
							s.is_empty()
						}
						['u', 'r', s @ ..] => {
							super::push(out, 7050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2884);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', 'o', 'w', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10909);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 10910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'd', 'u', 't', 's', '\u{1}', s @ ..] => super::t2::n7374(s, out),
						['u', s @ ..] => super::t2::n7375(s, out),
						['ę', s @ ..] => super::t2::n7376(s, out),
						['ś', 'o', 'g', s @ ..] => super::t2::n7377(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t2::n2966(s, out),
				['f', s @ ..] => super::t2::n2967(s, out),
				['g', 'a', 'r', s @ ..] => super::t2::n2968(s, out),
				['l', s @ ..] => super::t2::n2969(s, out),
				['m', s @ ..] => super::t2::n2970(s, out),
				['n', s @ ..] => super::t2::n2971(s, out),
				['o', s @ ..] => super::t2::n2972(s, out),
				['p', s @ ..] => super::t2::n2973(s, out),
				['s', s @ ..] => super::t2::n2974(s, out),
				['w', s @ ..] => super::t2::n2975(s, out),
				['z', s @ ..] => super::t2::n2976(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t2::n603(s, out),
		['l', s @ ..] => super::t2::n604(s, out),
		['m', s @ ..] => super::t2::n605(s, out),
		['n', s @ ..] => super::t2::n606(s, out),
		['o', s @ ..] => super::t2::n607(s, out),
		['r', s @ ..] => super::t2::n608(s, out),
		['s', s @ ..] => super::t2::n609(s, out),
		['t', s @ ..] => super::t2::n610(s, out),
		['u', s @ ..] => super::t2::n611(s, out),
		['w', s @ ..] => super::t2::n612(s, out),
		['y', s @ ..] => super::t3::n613(s, out),
		['z', s @ ..] => super::t3::n614(s, out),
		['ł', s @ ..] => super::t3::n615(s, out),
		['ź', s @ ..] => super::t3::n616(s, out),
		['ż', s @ ..] => super::t3::n617(s, out),
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2932);
					s.is_empty()
				}
				['r', 't', s @ ..] => {
					super::push(out, 2933);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 7117);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', 't', 'j', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['l', 'ś', s @ ..] => {
					super::push(out, 2936);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2938);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2939);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['d', 'i', 's', 'ę', 'z', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 2940);
					s.is_empty()
				}
				['r', 'k', 'y', 'z', 'c', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2942);
					match s {
						[] => true,
						['z', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2943);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7121);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 10964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 7123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10966);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2944);
					match s {
						[] => true,
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2945);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2946);
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							super::push(out, 7126);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 10968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', s @ ..] => {
					super::push(out, 2947);
					match s {
						[] => true,
						['m', 'a', 'l', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 7128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 2948);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2949);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 2951);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2952);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7129);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7130);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2953);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2954);
					match s {
						[] => true,
						['h', 'c', 'a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 7132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2955);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 2956);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['f', 's', 'o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7133);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7134);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2958);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7135);
							match s {
								[] => true,
								['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2959);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2960);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2961);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'k', s @ ..] => {
					super::push(out, 2962);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 625);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2966);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2967);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2968);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2971);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2972);
					match s {
						[] => true,
						['a', 'j', 'y', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2973);
					match s {
						[] => true,
						['c', 'o', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2974);
					match s {
						[] => true,
						['n', 'a', 'm', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7142);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7143);
							s.is_empty()
						}
						['z', 'd', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2975);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7145);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							super::push(out, 7146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					super::push(out, 2977);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7148);
							s.is_empty()
						}
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7149);
							s.is_empty()
						}
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7150);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10973);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n3069(s, out),
				['e', s @ ..] => super::t3::n3070(s, out),
				['n', s @ ..] => {
					super::push(out, 2983);
					s.is_empty()
				}
				['s', s @ ..] => super::t3::n3072(s, out),
				['u', s @ ..] => {
					super::push(out, 2985);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => super::t3::n7496(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n635(s, out),
		['ó', s @ ..] => super::t3::n636(s, out),
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 2996);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					super::push(out, 2997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2998);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2999);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 7170);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['i', 'b', s @ ..] => {
					super::push(out, 3001);
					match s {
						[] => true,
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3002);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3003);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 7174);
							match s {
								[] => true,
								['o', 't', 'o', 'ł', 'z', '\u{1}', s @ ..] => {
									super::push(out, 10988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'e', 's', s @ ..] => {
			super::push(out, 640);
			s.is_empty()
		}
		['r', 'o', s @ ..] => {
			super::push(out, 641);
			s.is_empty()
		}
		['s', 'e', 'b', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['ł', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3007);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 7175);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7176);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 13361);
											s.is_empty()
										}
										['p', 'a', 'z', s @ ..] => {
											super::push(out, 13362);
											s.is_empty()
										}
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
					super::push(out, 3008);
					match s {
						[] => true,
						['z', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3009);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 7178);
							match s {
								[] => true,
								['r', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 10989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'ś', s @ ..] => {
							super::push(out, 7179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7180);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7181);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 10990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3010);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 7182);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10991);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 10992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7184);
							match s {
								[] => true,
								['l', 'p', 'y', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 's', s @ ..] => {
							super::push(out, 7185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3012);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3013);
					match s {
						[] => true,
						['o', 'ł', s @ ..] => {
							super::push(out, 7186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3014);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7187);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 7188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10994);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7189);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							match s {
								[] => true,
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10996);
									s.is_empty()
								}
								['r', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 3017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['b', 'a', s @ ..] => {
					super::push(out, 3018);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 's', s @ ..] => {
					super::push(out, 3019);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'o', s @ ..] => {
					super::push(out, 3020);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3022);
					s.is_empty()
				}
				['h', 'c', 'e', s @ ..] => {
					super::push(out, 3023);
					match s {
						[] => true,
						['t', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3024);
					match s {
						[] => true,
						['k', 'y', s @ ..] => {
							super::push(out, 7194);
							match s {
								[] => true,
								['t', 'e', 'n', s @ ..] => {
									super::push(out, 10998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3025);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7195);
							s.is_empty()
						}
						['k', 'o', 'z', 's', 'r', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7196);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7197);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7198);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 10999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3026);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3027);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 7199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3028);
					s.is_empty()
				}
				['r', 'o', 'h', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3030);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11000);
									s.is_empty()
								}
								['i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 11001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3031);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 3033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3034);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 7200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7201);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7202);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 11002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 7203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t3::n3126(s, out),
				['l', s @ ..] => super::t3::n3127(s, out),
				['m', s @ ..] => super::t3::n3128(s, out),
				['n', s @ ..] => super::t3::n3129(s, out),
				['p', s @ ..] => super::t3::n3130(s, out),
				['r', s @ ..] => super::t3::n3131(s, out),
				['s', s @ ..] => super::t3::n3132(s, out),
				['u', 's', s @ ..] => super::t3::n3133(s, out),
				['w', 'ś', s @ ..] => super::t3::n3134(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t3::n653(s, out),
		['k', s @ ..] => super::t3::n654(s, out),
		['l', s @ ..] => super::t3::n655(s, out),
		['n', s @ ..] => super::t3::n656(s, out),
		['o', s @ ..] => super::t3::n657(s, out),
		['p', s @ ..] => super::t3::n658(s, out),
		['r', s @ ..] => super::t3::n659(s, out),
		['s', s @ ..] => super::t3::n660(s, out),
		['t', s @ ..] => super::t3::n661(s, out),
		['u', s @ ..] => super::t3::n662(s, out),
		['y', s @ ..] => super::t3::n663(s, out),
		['z', s @ ..] => super::t3::n664(s, out),
		['ą', s @ ..] => super::t3::n665(s, out),
		['ę', s @ ..] => super::t3::n666(s, out),
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['a', 'l', 'k', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 664);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 665);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wa.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7285);
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									super::push(out, 11043);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 11044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '-', s @ ..] => {
							super::push(out, 7286);
							match s {
								[] => true,
								['a', 'w', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 11045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3102);
					s.is_empty()
				}
				['ł', 's', s @ ..] => {
					super::push(out, 3103);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7290);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7291);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 11046);
									s.is_empty()
								}
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
			super::push(out, 671);
			match s {
				[] => true,
				['l', 'ś', s @ ..] => {
					super::push(out, 3108);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 7292);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 11047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['ż', s @ ..] => {
							super::push(out, 7293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 7294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 't', s @ ..] => {
					super::push(out, 3111);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['b', 'y', 'z', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3113);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 7296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3114);
					match s {
						[] => true,
						['u', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11048);
									s.is_empty()
								}
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 11049);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 11050);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 11051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 7297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				['h', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3118);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7298);
							s.is_empty()
						}
						['z', 'a', 'i', 'n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'o', 'k', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3120);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', 'y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 11052);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7300);
							s.is_empty()
						}
						['s', 'i', 'n', 'g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3121);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 7302);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 7303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7304);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7305);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11054);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 13378);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7306);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3123);
					match s {
						[] => true,
						['a', 'i', 'c', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7308);
							s.is_empty()
						}
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7309);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 11055);
									s.is_empty()
								}
								['r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 11056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7310);
							s.is_empty()
						}
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7311);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7312);
							match s {
								[] => true,
								['z', 'r', 'a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 11057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'e', 'l', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7313);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7314);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7315);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 7316);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7317);
							match s {
								[] => true,
								['d', 'i', 'm', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['t', s @ ..] => super::t3::n3221(s, out),
				['z', s @ ..] => {
					super::push(out, 3128);
					match s {
						[] => true,
						['c', 'i', 'w', 'o', 'c', 'i', 'z', 'd', 'o', 'w', 'e', 'j', 'o', 'w', '\u{1}', s @ ..] => super::t3::n7666(s, out),
						['r', 'a', 'n', 'd', 'e', 'b', '\u{1}', s @ ..] => super::t3::n7667(s, out),
						_ => false,
					}
				}
				['ł', s @ ..] => super::t3::n3223(s, out),
				['ż', s @ ..] => super::t3::n3224(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n679(s, out),
		['t', s @ ..] => super::t3::n680(s, out),
		['u', s @ ..] => super::t3::n681(s, out),
		['y', s @ ..] => super::t3::n682(s, out),
		['z', s @ ..] => super::t3::n683(s, out),
		['ó', s @ ..] => super::t3::n684(s, out),
		['ł', s @ ..] => super::t3::n685(s, out),
		['ż', s @ ..] => super::t3::n686(s, out),
		_ => false,
	}
}

/// Words ending -ya.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7331);
							match s {
								[] => true,
								['y', 'd', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 13380);
											s.is_empty()
										}
										['u', 'i', s @ ..] => {
											super::push(out, 13381);
											match s {
												[] => true,
												['d', 'a', '\u{1}', s @ ..] => {
													super::push(out, 14674);
													s.is_empty()
												}
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
							super::push(out, 7332);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 11062);
									s.is_empty()
								}
								['s', 'u', s @ ..] => {
									super::push(out, 11063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'n', s @ ..] => {
							super::push(out, 7333);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 7334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
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
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7337);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 7338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'r', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 686);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7339);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', 'd', 'y', 'h', 'o', 'l', 'y', 'c', s @ ..] => {
							super::push(out, 7341);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7343);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7344);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', 's', 'n', 'a', 'r', 't', 'o', s @ ..] => {
									match s {
										[] => true,
										['l', 'y', 't', s @ ..] => {
											super::push(out, 13382);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 13383);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'e', 'o', s @ ..] => {
									super::push(out, 11064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 689);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 3156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'ł', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				['ń', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7347);
							s.is_empty()
						}
						['ł', 'w', 'u', s @ ..] => {
							super::push(out, 7348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3159);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 11065);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 11066);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3164);
					match s {
						[] => true,
						['e', 'g', 'o', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 11067);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11068);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11069);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									super::push(out, 11070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3165);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3167);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3168);
					match s {
						[] => true,
						['a', 't', 'a', 'k', 'o', s @ ..] => {
							super::push(out, 7351);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7352);
							match s {
								[] => true,
								['m', 'e', s @ ..] => {
									super::push(out, 11071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3170);
					match s {
						[] => true,
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 695);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['d', 'l', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				['e', 't', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['f', 'r', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				['g', 'r', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3176);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7354);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 11072);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['c', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7356);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7358);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7359);
							s.is_empty()
						}
						['y', 'm', s @ ..] => {
							super::push(out, 7360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7361);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3180);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3181);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7363);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7364);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['k', 'a', 'l', 'a', s @ ..] => {
							super::push(out, 7366);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7367);
							s.is_empty()
						}
						['y', 'c', s @ ..] => {
							super::push(out, 7368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['e', 'z', 'c', 'e', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3185);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
						['z', s @ ..] => super::t3::n7721(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t3::n3290(s, out),
				['u', s @ ..] => super::t3::n3291(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t3::n702(s, out),
		['u', s @ ..] => super::t3::n703(s, out),
		['y', s @ ..] => super::t3::n704(s, out),
		['z', s @ ..] => super::t3::n705(s, out),
		['ę', s @ ..] => super::t3::n706(s, out),
		['ł', s @ ..] => super::t3::n707(s, out),
		_ => false,
	}
}

/// Words ending -ła.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['b', 'e', 'z', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3203);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3204);
					match s {
						[] => true,
						['ę', s @ ..] => {
							super::push(out, 7381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3205);
					s.is_empty()
				}
				['j', 'i', 'p', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3206);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3207);
					match s {
						[] => true,
						['y', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7382);
							s.is_empty()
						}
						['ą', s @ ..] => {
							super::push(out, 7383);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 11079);
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
						['y', 'z', 'c', 'z', 's', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 7384);
							s.is_empty()
						}
						['ę', 'z', 'r', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3208);
					match s {
						[] => true,
						['d', 'z', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7386);
							s.is_empty()
						}
						['g', 'z', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3209);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3210);
					match s {
						[] => true,
						['h', 'c', 'o', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 7389);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7390);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3213);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3214);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3215);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3217);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				['s', 'z', s @ ..] => {
					super::push(out, 3220);
					s.is_empty()
				}
				['w', 'a', 'ł', 'a', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'ę', 'z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3226);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3228);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 714);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3231);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7395);
							match s {
								[] => true,
								['z', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 11080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'z', 'c', 'ę', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3232);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7397);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7398);
							match s {
								[] => true,
								['m', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 11081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7399);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', 'ł', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7401);
							s.is_empty()
						}
						['z', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				['m', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 716);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				['r', 'a', 'b', 'm', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 717);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -źa.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['d', 'ń', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ża.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3239);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 7403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 722);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['g', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3242);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 724);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ec.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 777);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['ł', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 7426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 779);
			s.is_empty()
		}
		['i', s @ ..] => super::t3::n784(s, out),
		['j', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3312);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 7469);
							match s {
								[] => true,
								['i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 11172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['z', 'r', 'd', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3314);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7472);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 7473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				['ó', 'r', 'g', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3317);
					match s {
						[] => true,
						['b', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7474);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 7475);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7476);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7477);
							match s {
								[] => true,
								['m', 'u', 'z', 'o', 'r', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 11174);
									s.is_empty()
								}
								['n', 'd', 'o', 'r', 'y', 'w', 'z', '\u{1}', s @ ..] => {
									super::push(out, 11175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 7478);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7479);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11176);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7480);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 7481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', 'd', 'ą', 'm', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7482);
							s.is_empty()
						}
						['ż', 'u', 'ł', 's', '\u{1}', s @ ..] => {
							super::push(out, 7483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3319);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7485);
							match s {
								[] => true,
								['i', 'w', 'o', s @ ..] => {
									super::push(out, 11179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7486);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11180);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 11181);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 11182);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13564);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 11183);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11184);
									match s {
										[] => true,
										['i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 13565);
											s.is_empty()
										}
										['y', 'ł', '\u{1}', s @ ..] => {
											super::push(out, 13566);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 11185);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 14812);
													match s {
														[] => true,
														['a', 'w', 't', '\u{1}', s @ ..] => super::t3::n16408(s, out),
														_ => false,
													}
												}
												['u', s @ ..] => {
													match s {
														[] => true,
														['h', 'c', '\u{1}', s @ ..] => super::t3::n16409(s, out),
														['r', '\u{1}', s @ ..] => super::t3::n16410(s, out),
														_ => false,
													}
												}
												['ź', 'i', 'p', '\u{1}', s @ ..] => {
													super::push(out, 14813);
													s.is_empty()
												}
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
							super::push(out, 7487);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7488);
							match s {
								[] => true,
								['r', 'k', s @ ..] => {
									super::push(out, 11186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3320);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7489);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 11187);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 13567);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 11188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 7490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t3::n3429(s, out),
				['u', s @ ..] => {
					super::push(out, 3322);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7498);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 11192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3323);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3324);
					s.is_empty()
				}
				['r', s @ ..] => super::t3::n3433(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n788(s, out),
		['z', s @ ..] => super::t3::n789(s, out),
		['ż', s @ ..] => super::t3::n790(s, out),
		_ => false,
	}
}

/// Words ending -oid.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3373);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 7525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', s @ ..] => {
			super::push(out, 3374);
			match s {
				[] => true,
				['m', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3375);
			match s {
				[] => true,
				['o', 'r', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3376);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7528);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7529);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ld.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 3379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nd.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3382);
					match s {
						[] => true,
						['r', 'b', 'u', 'a', 'e', 't', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3383);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3384);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3385);
					match s {
						[] => true,
						['s', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 851);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 3387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7533);
							s.is_empty()
						}
						['m', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 7534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3390);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3391);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3392);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', 'b', s @ ..] => {
									super::push(out, 11212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13578);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 11213);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 13579);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 11214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 3393);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3394);
					match s {
						[] => true,
						['a', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 7536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3395);
					match s {
						[] => true,
						['u', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3396);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3397);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3398);
					match s {
						[] => true,
						['i', 'm', 'a', 'n', 'y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7538);
							s.is_empty()
						}
						['s', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3399);
					match s {
						[] => true,
						['e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 7540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3400);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7541);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3401);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3402);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3404);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3406);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 858);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3408);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3409);
					match s {
						[] => true,
						['m', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7546);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yd.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 862);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 863);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 3412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zd.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 869);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 870);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 871);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3414);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ód.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['c', 'o', 'r', 'o', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 3415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 875);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['m', 's', 'o', 'h', 'c', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7548);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 11215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 879);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['g', 'y', 'z', 'r', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąd.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['i', 'w', 'u', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11216);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 11217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 7550);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7551);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['p', 'o', s @ ..] => {
					super::push(out, 3418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 884);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 886);
			match s {
				[] => true,
				['r', s @ ..] => {
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
						['d', s @ ..] => {
							super::push(out, 7554);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7555);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7556);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11219);
									s.is_empty()
								}
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
			super::push(out, 887);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7557);
							s.is_empty()
						}
						['l', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęd.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 3421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'r', 'o', s @ ..] => {
			super::push(out, 890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żołd.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	s.is_empty()
}

/// Words ending -ie.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3431);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3432);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7561);
							match s {
								[] => true,
								['m', 'ś', '\u{1}', s @ ..] => {
									super::push(out, 11220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7562);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11221);
									s.is_empty()
								}
								['o', 'i', 'c', 'ę', 'i', 's', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
									super::push(out, 11222);
									s.is_empty()
								}
								['u', 't', s @ ..] => {
									super::push(out, 11223);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 13580);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'b', 'o', s @ ..] => {
					super::push(out, 3433);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3434);
					match s {
						[] => true,
						['e', 'l', 'a', 't', 'z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3435);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7565);
							match s {
								[] => true,
								['z', 'o', s @ ..] => {
									super::push(out, 11224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11225);
									match s {
										[] => true,
										['z', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 13581);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 11226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'p', s @ ..] => {
							super::push(out, 7567);
							s.is_empty()
						}
						['z', 'c', 'o', 'p', 'o', s @ ..] => {
							super::push(out, 7568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 7569);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 3436);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 3437);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7572);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11227);
									s.is_empty()
								}
								['n', s @ ..] => super::t3::n11903(s, out),
								['p', s @ ..] => {
									super::push(out, 11229);
									s.is_empty()
								}
								['z', 'w', 'o', s @ ..] => {
									super::push(out, 11230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 7573);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 3438);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7575);
							s.is_empty()
						}
						['j', 'ó', s @ ..] => {
							super::push(out, 7576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n3555(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n917(s, out),
		['n', s @ ..] => super::t3::n918(s, out),
		['p', s @ ..] => super::t3::n919(s, out),
		['r', s @ ..] => super::t3::n920(s, out),
		['s', s @ ..] => super::t3::n921(s, out),
		['w', s @ ..] => super::t3::n922(s, out),
		['z', s @ ..] => super::t3::n923(s, out),
		_ => false,
	}
}

/// Words ending -line.
pub(crate) fn n3601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3485);
	s.is_empty()
}

/// Words ending -jne.
pub(crate) fn n949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 943);
	s.is_empty()
}

/// Words ending -mne.
pub(crate) fn n950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 944);
	match s {
		[] => true,
		['y', s @ ..] => {
			super::push(out, 3486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nne.
pub(crate) fn n951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 945);
	s.is_empty()
}

/// Words ending -one.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 946);
	s.is_empty()
}

/// Words ending -rne.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	s.is_empty()
}

/// Words ending -sne.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	s.is_empty()
}

/// Words ending -wne.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	s.is_empty()
}

/// Words ending -źne.
pub(crate) fn n956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	s.is_empty()
}

/// Words ending -pe.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['y', s @ ..] => {
			super::push(out, 951);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		['b', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 953);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3487);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3488);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3489);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', 'z', s @ ..] => {
									super::push(out, 11307);
									match s {
										[] => true,
										['s', 'i', s @ ..] => {
											super::push(out, 13679);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', s @ ..] => {
									super::push(out, 11308);
									s.is_empty()
								}
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
			super::push(out, 958);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 3490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -we.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['k', 'ż', s @ ..] => {
					super::push(out, 3491);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ze.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3493);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7624);
							match s {
								[] => true,
								['w', 'o', s @ ..] => {
									super::push(out, 11309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3494);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3495);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3496);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11310);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 13680);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 11311);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 13681);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 11312);
									match s {
										[] => true,
										['l', 'g', 'z', '\u{1}', s @ ..] => {
											super::push(out, 13682);
											s.is_empty()
										}
										['w', 'r', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 13683);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 11313);
									match s {
										[] => true,
										['t', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 13684);
											s.is_empty()
										}
										['z', 'r', 'o', 'w', 'd', '\u{1}', s @ ..] => {
											super::push(out, 13685);
											s.is_empty()
										}
										['ż', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 13686);
											s.is_empty()
										}
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
			super::push(out, 962);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3497);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 11314);
									s.is_empty()
								}
								['w', 'ź', 'd', '\u{1}', s @ ..] => {
									super::push(out, 11315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3498);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 7626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3499);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7627);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 7628);
							match s {
								[] => true,
								['n', 'w', '\u{1}', s @ ..] => {
									super::push(out, 11316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 3500);
					match s {
						[] => true,
						['g', 'z', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['e', 'l', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3501);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3502);
					s.is_empty()
				}
				['u', 'z', 'r', 'b', 'o', s @ ..] => {
					super::push(out, 3503);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łe.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 964);
			s.is_empty()
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -że.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['a', 'n', 'a', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['ż', s @ ..] => {
					super::push(out, 3504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3505);
					match s {
						[] => true,
						['c', 'ś', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['b', 'z', s @ ..] => {
					super::push(out, 3506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3507);
					s.is_empty()
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7632);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 7633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word demograf.
pub(crate) fn n15965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14918);
	s.is_empty()
}

/// Words ending -yling.
pub(crate) fn n8034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7656);
	match s {
		[] => true,
		['z', 's', '\u{1}', s @ ..] => {
			super::push(out, 11328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ming.
pub(crate) fn n3660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3539);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			super::push(out, 7657);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ning.
pub(crate) fn n3661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3540);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 7659);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ring.
pub(crate) fn n3663(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 7661);
			s.is_empty()
		}
		['o', 't', 'i', s @ ..] => {
			super::push(out, 7662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sing.
pub(crate) fn n3664(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7663);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7664);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ting.
pub(crate) fn n3665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3542);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7666);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ong.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1005);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 3543);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3544);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 7668);
					s.is_empty()
				}
				['a', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3545);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 3546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lung.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	s.is_empty()
}

/// Words ending -og.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1007);
			s.is_empty()
		}
		['g', 'a', 'm', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1008);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3547);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3548);
					match s {
						[] => true,
						['e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7670);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7671);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 7672);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7673);
							s.is_empty()
						}
						['r', 'e', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7674);
							s.is_empty()
						}
						['t', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3550);
					match s {
						[] => true,
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3551);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 7678);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 11329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7679);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7680);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 11331);
									match s {
										[] => true,
										['e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 13697);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 11332);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13698);
											s.is_empty()
										}
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
					super::push(out, 3552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3554);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1012);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3556);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rg.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3557);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 7682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['o', 'b', 'u', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
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
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1016);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 1017);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ug.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		['ł', 'p', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yg.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zg.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['ó', 'm', s @ ..] => {
			match s {
				[] => true,
				['z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3565);
					s.is_empty()
				}
				['ł', 'ó', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3566);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -óg.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3567);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 3568);
					match s {
						[] => true,
						['ó', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['i', 's', 'a', 't', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3569);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3570);
					match s {
						[] => true,
						['ó', s @ ..] => {
							super::push(out, 7687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3571);
					match s {
						[] => true,
						['i', 'c', 'ś', s @ ..] => {
							super::push(out, 7688);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 7689);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'z', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7690);
							s.is_empty()
						}
						['w', 't', '\u{1}', s @ ..] => {
							super::push(out, 7691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3576);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				['j', 'ó', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3578);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3579);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3580);
					s.is_empty()
				}
				['t', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				['u', 'w', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 1028);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąg.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['o', 'h', s @ ..] => {
							super::push(out, 7693);
							match s {
								[] => true,
								['c', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3585);
					match s {
						[] => true,
						['k', 'o', 'i', s @ ..] => {
							super::push(out, 7694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3586);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3587);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3588);
					match s {
						[] => true,
						['w', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3591);
					match s {
						[] => true,
						['o', 'i', 'm', 'e', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3592);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3593);
					match s {
						[] => true,
						['s', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3594);
					s.is_empty()
				}
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'r', 'p', 'a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęg.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	s.is_empty()
}

/// Words ending -ach.
pub(crate) fn n1046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1036);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3596);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3598);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3599);
			match s {
				[] => true,
				['l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3600);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7702);
					match s {
						[] => true,
						['e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 11335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3601);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3602);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7704);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 7705);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7706);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 11338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 3603);
			s.is_empty()
		}
		['w', 's', '\u{1}', s @ ..] => {
			super::push(out, 3604);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3605);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 3606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7707);
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11339);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11340);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 14919);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 14920);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 11341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iech.
pub(crate) fn n3733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3609);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11342);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 11343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mech.
pub(crate) fn n3735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3611);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pech.
pub(crate) fn n3736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3612);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zech.
pub(crate) fn n3737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3613);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7715);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 11345);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13699);
							s.is_empty()
						}
						['y', 'b', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 13700);
							s.is_empty()
						}
						['ł', 'd', 'y', 'z', 'r', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 13701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ich.
pub(crate) fn n1048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1038);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3614);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11346);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lch.
pub(crate) fn n1049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1039);
	s.is_empty()
}

/// Words ending -nch.
pub(crate) fn n1050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1040);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 3616);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -och.
pub(crate) fn n1051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1041);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 3617);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3618);
			match s {
				[] => true,
				['c', 'ś', s @ ..] => {
					match s {
						[] => true,
						['u', 'ł', 't', '\u{1}', s @ ..] => {
							super::push(out, 11348);
							s.is_empty()
						}
						['y', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7716);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 7717);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7718);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13702);
									s.is_empty()
								}
								['ń', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13703);
									s.is_empty()
								}
								['ź', s @ ..] => {
									super::push(out, 13704);
									s.is_empty()
								}
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
			super::push(out, 3619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7719);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7720);
					s.is_empty()
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 7721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3620);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3621);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7722);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7723);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7724);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3622);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3623);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11350);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 3624);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7726);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7727);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 7728);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rch.
pub(crate) fn n1052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1042);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sch.
pub(crate) fn n1053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	match s {
		[] => true,
		['n', 'e', 'm', 'r', 'e', 'z', 'd', 'o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3626);
			s.is_empty()
		}
		['t', 'u', 'e', 'd', 's', 'k', 'l', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 3627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tch.
pub(crate) fn n1054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1044);
	match s {
		[] => true,
		['a', 'r', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 3628);
			s.is_empty()
		}
		['e', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 3629);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uch.
pub(crate) fn n1055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1045);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3630);
			match s {
				[] => true,
				['m', 'a', 't', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 7729);
					s.is_empty()
				}
				['y', 'w', s @ ..] => {
					super::push(out, 7730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3631);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7731);
					s.is_empty()
				}
				['e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7732);
					s.is_empty()
				}
				['l', 'e', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3632);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7734);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', 'k', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11352);
							s.is_empty()
						}
						['n', 'k', 'ę', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3633);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11354);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11355);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 14921);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 14922);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 14923);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 14924);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 15366);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11356);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'ł', '\u{1}', s @ ..] => {
									super::push(out, 13706);
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 13707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 13708);
									s.is_empty()
								}
								['ż', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 13709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ę', 'r', 't', 's', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11357);
							s.is_empty()
						}
						['ł', 'a', 't', 'z', 's', 'k', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11358);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 11359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11360);
							s.is_empty()
						}
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11361);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'd', 'r', 'e', 'i', 'm', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 7735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3634);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3635);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7736);
					match s {
						[] => true,
						['b', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11363);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11364);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13710);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7737);
					match s {
						[] => true,
						['p', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 7738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 7739);
					s.is_empty()
				}
				['z', 'r', 'a', 'm', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3636);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7741);
					s.is_empty()
				}
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3637);
			match s {
				[] => true,
				['r', 'a', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7743);
					s.is_empty()
				}
				['u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3638);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3639);
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3640);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7746);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 11369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'i', 'c', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7747);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3641);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7749);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3642);
			match s {
				[] => true,
				['j', 'e', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7751);
					s.is_empty()
				}
				['n', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 7752);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11371);
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 11373);
							s.is_empty()
						}
						['ż', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 11374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 3643);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7754);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11375);
							s.is_empty()
						}
						['m', 'a', 'ł', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11376);
							s.is_empty()
						}
						['z', 's', 'e', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11378);
							s.is_empty()
						}
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7756);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13711);
									s.is_empty()
								}
								['i', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13712);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13713);
									match s {
										[] => true,
										['s', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 14925);
											s.is_empty()
										}
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
		['ł', s @ ..] => {
			super::push(out, 3645);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7757);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 3646);
			match s {
				[] => true,
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ych.
pub(crate) fn n1056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 3647);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3648);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 7760);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 11380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 11381);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zch.
pub(crate) fn n1057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1047);
	s.is_empty()
}

/// Words ending -ęch.
pub(crate) fn n1058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1048);
	match s {
		[] => true,
		['z', 'r', '\u{1}', s @ ..] => {
			super::push(out, 3650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kh.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oh.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	s.is_empty()
}

/// Words ending -sh.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uh.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ski.
pub(crate) fn n1081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1071);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3675);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7772);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3676);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 7774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3677);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3678);
			match s {
				[] => true,
				['y', 'r', 'u', s @ ..] => {
					super::push(out, 7776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3679);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3680);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3681);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7778);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3682);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 7781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'a', 'k', 'e', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3683);
			s.is_empty()
		}
		['ń', s @ ..] => {
			super::push(out, 3684);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7782);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11385);
							s.is_empty()
						}
						['s', 'u', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7783);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7784);
					match s {
						[] => true,
						['z', 'c', 'ł', 'e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yki.
pub(crate) fn n1085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3689);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3690);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zki.
pub(crate) fn n1086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1076);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3692);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3693);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7787);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 11388);
							match s {
								[] => true,
								['o', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 13714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7788);
					s.is_empty()
				}
				['u', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7789);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7790);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 7791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3694);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7793);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7794);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3695);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dzięki.
pub(crate) fn n3822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3696);
	s.is_empty()
}

/// Words ending -ręki.
pub(crate) fn n3823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3697);
	s.is_empty()
}

/// Words ending -łki.
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1078);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3698);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7796);
					s.is_empty()
				}
				['p', 'o', '-', 'i', 'k', 'ł', 'a', 'z', 's', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7797);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńki.
pub(crate) fn n1089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1079);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3700);
			match s {
				[] => true,
				['i', 'n', 'a', 'h', 'c', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7799);
					s.is_empty()
				}
				['z', 's', 'r', 'e', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 7800);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żki.
pub(crate) fn n1090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1080);
	match s {
		[] => true,
		['e', 'i', 'n', 'ś', '\u{1}', s @ ..] => {
			super::push(out, 3701);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 3702);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 3703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ni.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1081);
			match s {
				[] => true,
				['p', 'ć', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3704);
					s.is_empty()
				}
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1085);
			s.is_empty()
		}
		['ź', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['i', 'l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7801);
							s.is_empty()
						}
						['ł', 'ó', 'p', 's', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1087);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wi.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 1088);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word paparazzi.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	s.is_empty()
}

/// The word oklej.
pub(crate) fn n8206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7814);
	s.is_empty()
}

/// The word uklej.
pub(crate) fn n8207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7815);
	s.is_empty()
}

/// Words ending -olej.
pub(crate) fn n3871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3742);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7816);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -slej.
pub(crate) fn n3872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3743);
	s.is_empty()
}

/// Words ending -mej.
pub(crate) fn n1119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['g', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pej.
pub(crate) fn n1120(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3745);
			s.is_empty()
		}
		['z', 's', '\u{1}', s @ ..] => {
			super::push(out, 3746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rej.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3747);
			s.is_empty()
		}
		['p', 's', '\u{1}', s @ ..] => {
			super::push(out, 3748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sej.
pub(crate) fn n1122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1111);
	s.is_empty()
}

/// Words ending -zej.
pub(crate) fn n1123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1112);
	match s {
		[] => true,
		['r', 'a', 't', 'n', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 3749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żej.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1113);
	match s {
		[] => true,
		['d', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 3750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ij.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['y', 'z', 'c', 'ó', 'ł', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oj.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3752);
					s.is_empty()
				}
				['d', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uj.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3754);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yj.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['i', 'd', 'o', 'r', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 3758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ój.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1123);
			match s {
				[] => true,
				['o', 'm', s @ ..] => {
					super::push(out, 3759);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 7818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3761);
					match s {
						[] => true,
						['z', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['o', 'p', 's', s @ ..] => {
					super::push(out, 3762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', s @ ..] => {
			super::push(out, 1126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['k', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3764);
					s.is_empty()
				}
				['t', 's', 'y', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', 'o', 'm', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['r', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3767);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 7821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3769);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ak.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3770);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3771);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 7822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 7823);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7824);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3773);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7827);
							match s {
								[] => true,
								['k', 's', '\u{1}', s @ ..] => {
									super::push(out, 11394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3777);
					match s {
						[] => true,
						['t', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 7829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3778);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 7830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 3779);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 7831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1132);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3780);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3781);
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3782);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3783);
					match s {
						[] => true,
						['a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 7832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'm', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 3784);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 7833);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3785);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7835);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11395);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11396);
									match s {
										[] => true,
										['p', 'a', 'ł', '\u{1}', s @ ..] => {
											super::push(out, 13717);
											s.is_empty()
										}
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
					super::push(out, 3786);
					s.is_empty()
				}
				['ł', 'o', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 3787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1134);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n1147(s, out),
		['j', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3799);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7881);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 11478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3802);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11479);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11480);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11481);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 11482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13776);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'm', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7883);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 7884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3803);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7885);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 7886);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 7887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11483);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 11484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'e', 'n', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1138);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3805);
					s.is_empty()
				}
				['a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7888);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3806);
					match s {
						[] => true,
						['m', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 7890);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => super::t3::n8294(s, out),
						['ó', 'r', 'w', '\u{1}', s @ ..] => super::t3::n8295(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t3::n3940(s, out),
				['e', s @ ..] => super::t3::n3941(s, out),
				['f', s @ ..] => super::t3::n3942(s, out),
				['g', s @ ..] => super::t3::n3943(s, out),
				['h', s @ ..] => super::t3::n3944(s, out),
				['k', s @ ..] => super::t3::n3945(s, out),
				['o', s @ ..] => super::t3::n3946(s, out),
				['p', 'e', 'i', 'c', '\u{1}', s @ ..] => super::t3::n3947(s, out),
				['r', s @ ..] => super::t3::n3948(s, out),
				['u', 'b', 'e', 'c', '\u{1}', s @ ..] => super::t3::n3949(s, out),
				['w', 'o', 'm', 'e', 'i', 'n', '\u{1}', s @ ..] => super::t3::n3950(s, out),
				['y', s @ ..] => super::t3::n3951(s, out),
				['z', s @ ..] => super::t3::n3952(s, out),
				['ó', s @ ..] => super::t3::n3953(s, out),
				['ś', s @ ..] => super::t3::n3954(s, out),
				['ź', 'o', 'k', '\u{1}', s @ ..] => super::t3::n3955(s, out),
				['ż', s @ ..] => super::t3::n3956(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n1151(s, out),
		['n', s @ ..] => super::t3::n1152(s, out),
		['p', s @ ..] => super::t3::n1153(s, out),
		['r', s @ ..] => super::t3::n1154(s, out),
		['s', s @ ..] => super::t3::n1155(s, out),
		['t', s @ ..] => super::t3::n1156(s, out),
		['w', s @ ..] => super::t3::n1157(s, out),
		['z', s @ ..] => super::t3::n1158(s, out),
		['ł', s @ ..] => super::t3::n1159(s, out),
		['ż', s @ ..] => super::t3::n1160(s, out),
		_ => false,
	}
}

/// Words ending -ek.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3912);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3913);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 8028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3914);
					s.is_empty()
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3915);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3916);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8029);
							match s {
								[] => true,
								['z', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13786);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 13787);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8030);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11545);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 13788);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14935);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 13789);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 11546);
									s.is_empty()
								}
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 11547);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 11548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'z', 'c', 'z', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3917);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8032);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8033);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 8034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3919);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8035);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 11549);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', 'g', 'd', s @ ..] => {
							super::push(out, 8036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3920);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8037);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 11550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 3921);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 3922);
					match s {
						[] => true,
						['z', 'r', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 8039);
							s.is_empty()
						}
						['ł', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 8040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 3923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8041);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8042);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 8043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3924);
					s.is_empty()
				}
				['o', 'n', s @ ..] => {
					super::push(out, 3925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3926);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3927);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8045);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11551);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8046);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 11552);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 13790);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 11553);
									match s {
										[] => true,
										['ź', s @ ..] => {
											super::push(out, 13791);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 11554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 8047);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8048);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8049);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8050);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8051);
							s.is_empty()
						}
						['ł', s @ ..] => super::t3::n8462(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n4069(s, out),
				['l', s @ ..] => super::t3::n4070(s, out),
				['n', 'e', '\u{1}', s @ ..] => super::t3::n4071(s, out),
				['o', s @ ..] => super::t3::n4072(s, out),
				['r', s @ ..] => super::t3::n4073(s, out),
				['s', 'e', '\u{1}', s @ ..] => super::t3::n4074(s, out),
				['u', s @ ..] => super::t3::n4075(s, out),
				['y', s @ ..] => super::t3::n4076(s, out),
				['ó', s @ ..] => super::t3::n4077(s, out),
				['ą', s @ ..] => super::t3::n4078(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n1167(s, out),
		['i', s @ ..] => super::t3::n1168(s, out),
		['j', s @ ..] => super::t3::n1169(s, out),
		['l', s @ ..] => super::t3::n1170(s, out),
		['m', s @ ..] => super::t3::n1171(s, out),
		['n', s @ ..] => super::t3::n1172(s, out),
		['p', s @ ..] => super::t3::n1173(s, out),
		['r', s @ ..] => super::t3::n1174(s, out),
		['s', s @ ..] => super::t3::n1175(s, out),
		['t', s @ ..] => super::t3::n1176(s, out),
		['w', s @ ..] => super::t3::n1177(s, out),
		['z', s @ ..] => super::t3::n1178(s, out),
		['ł', s @ ..] => super::t3::n1179(s, out),
		['ż', s @ ..] => super::t3::n1180(s, out),
		_ => false,
	}
}

/// Words ending -ik.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4033);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4034);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4035);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8349);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 8350);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 8351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1171);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4038);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8352);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8353);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 11863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'e', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4041);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 8355);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8356);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4042);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11864);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 11865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 's', 'a', 'd', 'e', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 4043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4044);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 8359);
							match s {
								[] => true,
								['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 11866);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11867);
									match s {
										[] => true,
										['w', 'd', 'a', '\u{1}', s @ ..] => {
											super::push(out, 13966);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8360);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8361);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 11868);
									match s {
										[] => true,
										['c', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 13967);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'a', s @ ..] => {
							super::push(out, 8362);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8363);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 8364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4045);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8365);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 8366);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8367);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['a', 'ż', '\u{1}', s @ ..] => {
											super::push(out, 13968);
											s.is_empty()
										}
										['n', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 13969);
											s.is_empty()
										}
										['r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 13970);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8368);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 11870);
									s.is_empty()
								}
								['e', 'i', 'k', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 11871);
									s.is_empty()
								}
								['p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 11872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8369);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 11873);
									match s {
										[] => true,
										['r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 13971);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 8370);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8371);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11874);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 11875);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13972);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8372);
							s.is_empty()
						}
						['t', 'i', s @ ..] => {
							super::push(out, 8373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4046);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4047);
					match s {
						[] => true,
						['ó', 'w', '\u{1}', s @ ..] => {
							super::push(out, 8374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'a', s @ ..] => super::t3::n4192(s, out),
				['n', s @ ..] => super::t3::n4193(s, out),
				['o', s @ ..] => super::t3::n4194(s, out),
				['r', s @ ..] => super::t3::n4195(s, out),
				['u', s @ ..] => super::t3::n4196(s, out),
				['y', 't', s @ ..] => super::t3::n4197(s, out),
				['ą', s @ ..] => super::t3::n4198(s, out),
				['ę', 'z', 'r', 'p', 's', '\u{1}', s @ ..] => super::t3::n4199(s, out),
				['ś', s @ ..] => super::t3::n4200(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n1184(s, out),
		['f', s @ ..] => super::t3::n1185(s, out),
		['g', s @ ..] => super::t3::n1186(s, out),
		['l', s @ ..] => super::t3::n1187(s, out),
		['m', s @ ..] => super::t3::n1188(s, out),
		['n', s @ ..] => super::t3::n1189(s, out),
		['o', s @ ..] => super::t3::n1190(s, out),
		['p', s @ ..] => super::t3::n1191(s, out),
		['r', s @ ..] => super::t3::n1192(s, out),
		['s', s @ ..] => super::t3::n1193(s, out),
		['t', s @ ..] => super::t3::n1194(s, out),
		['w', s @ ..] => super::t3::n1195(s, out),
		['z', s @ ..] => super::t3::n1196(s, out),
		_ => false,
	}
}

/// Words ending -mok.
pub(crate) fn n1215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1203);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4177);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4178);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4179);
			s.is_empty()
		}
		['ć', s @ ..] => {
			super::push(out, 4180);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8591);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8592);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ook.
pub(crate) fn n1216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1204);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4181);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pok.
pub(crate) fn n1217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1205);
	s.is_empty()
}

/// Words ending -rok.
pub(crate) fn n1218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1206);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4183);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4184);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4185);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4186);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4187);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 8594);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 8595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4188);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tok.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1207);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 4190);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12140);
							s.is_empty()
						}
						['y', 'r', '\u{1}', s @ ..] => {
							super::push(out, 12141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wok.
pub(crate) fn n1220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1208);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4191);
			s.is_empty()
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4192);
			s.is_empty()
		}
		['ć', '\u{1}', s @ ..] => {
			super::push(out, 4193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zok.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1209);
	s.is_empty()
}

/// Words ending -łok.
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1210);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4194);
			match s {
				[] => true,
				['a', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 8597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8599);
					s.is_empty()
				}
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 8600);
					s.is_empty()
				}
				['e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rk.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1211);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 4197);
					match s {
						[] => true,
						['z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4198);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4199);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 8603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1212);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4200);
					match s {
						[] => true,
						['w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 8604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4201);
					match s {
						[] => true,
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4202);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'w', 'ś', '\u{1}', s @ ..] => {
			super::push(out, 1213);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sk.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 1215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1216);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1217);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1218);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4206);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 8608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uk.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4207);
					s.is_empty()
				}
				['m', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4209);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1220);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4211);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4212);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4214);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4215);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4216);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1223);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1224);
			match s {
				[] => true,
				['ł', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 4219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['m', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1227);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4221);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				['t', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1229);
			match s {
				[] => true,
				['z', 's', s @ ..] => {
					match s {
						[] => true,
						['m', 'l', 'e', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 8614);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1230);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4226);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 8616);
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 8617);
							s.is_empty()
						}
						['ń', s @ ..] => {
							super::push(out, 8618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4227);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1231);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 1232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wk.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	s.is_empty()
}

/// Words ending -yk.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1233);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4230);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4232);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 8620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8621);
							s.is_empty()
						}
						['ł', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4233);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4234);
					s.is_empty()
				}
				['r', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4235);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1235);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4237);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8623);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12145);
									s.is_empty()
								}
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 12146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4238);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8625);
							s.is_empty()
						}
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 8626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4239);
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8627);
							s.is_empty()
						}
						['t', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4240);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 8629);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 8630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4241);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4242);
					s.is_empty()
				}
				['o', 'ł', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4243);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8631);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8632);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							super::push(out, 8633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4245);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4247);
					s.is_empty()
				}
				['a', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4248);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4249);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'o', 'h', 'c', 'o', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4250);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4251);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8635);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8636);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 12147);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 14154);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 14155);
											s.is_empty()
										}
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
					super::push(out, 4252);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12148);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4253);
					match s {
						[] => true,
						['t', 's', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8637);
							s.is_empty()
						}
						['z', 'y', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4254);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4255);
					match s {
						[] => true,
						['a', 'i', 'h', 'c', 'y', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8639);
							s.is_empty()
						}
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 8640);
							s.is_empty()
						}
						['k', 'e', 'l', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12150);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8641);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12152);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 12153);
									s.is_empty()
								}
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
				['a', s @ ..] => {
					super::push(out, 4256);
					match s {
						[] => true,
						['l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t3::n1252(s, out),
		['w', s @ ..] => {
			super::push(out, 1240);
			s.is_empty()
		}
		['z', s @ ..] => super::t3::n1254(s, out),
		['ł', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4276);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4277);
					match s {
						[] => true,
						['t', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4278);
					s.is_empty()
				}
				['o', 'k', 'a', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 4279);
					s.is_empty()
				}
				['r', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 4280);
					s.is_empty()
				}
				['z', 's', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 8703);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4281);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4282);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 8706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4283);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4284);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4285);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4286);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 8707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4287);
					match s {
						[] => true,
						['z', 'r', 'k', s @ ..] => {
							super::push(out, 8708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 4288);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => super::t3::n9175(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ók.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['ł', s @ ..] => {
			super::push(out, 1244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ük.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąk.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1246);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1247);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 1248);
			match s {
				[] => true,
				['b', 'a', s @ ..] => {
					super::push(out, 4289);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęk.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1249);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4290);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 8710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'ź', 'd', 'y', s @ ..] => {
					super::push(out, 4291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1252);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łk.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1253);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word skandal.
pub(crate) fn n16137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15087);
	s.is_empty()
}

/// Words ending -sskandal.
pub(crate) fn n16138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15088);
	s.is_empty()
}

/// Words ending -nal.
pub(crate) fn n1278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1265);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 4322);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4323);
			match s {
				[] => true,
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4324);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4325);
			match s {
				[] => true,
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4326);
			s.is_empty()
		}
		['y', 's', '\u{1}', s @ ..] => {
			super::push(out, 4327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oal.
pub(crate) fn n1279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1266);
	s.is_empty()
}

/// Words ending -pal.
pub(crate) fn n1280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1267);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4328);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ral.
pub(crate) fn n1281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1268);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4330);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4331);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4332);
			match s {
				[] => true,
				['d', 'e', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8724);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 8725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 4333);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 8726);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sal.
pub(crate) fn n1282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1269);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4334);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4335);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4336);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4337);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 8727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4338);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4339);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4340);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 4341);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 8728);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tal.
pub(crate) fn n1283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1270);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4342);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4343);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8729);
					s.is_empty()
				}
				['p', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 8730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4344);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4345);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4346);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4347);
			match s {
				[] => true,
				['s', 'p', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 4348);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -val.
pub(crate) fn n1284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1271);
	s.is_empty()
}

/// Words ending -wal.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1272);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4349);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4351);
			match s {
				[] => true,
				['n', 'i', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4352);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 4353);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8737);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 8738);
					match s {
						[] => true,
						['o', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 12211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4354);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 8740);
					s.is_empty()
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4355);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 8742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zal.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1273);
	match s {
		[] => true,
		['a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4356);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żal.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4358);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bl.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1275);
			match s {
				[] => true,
				['t', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -el.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4361);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4362);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 8744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4363);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4364);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4365);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4366);
					match s {
						[] => true,
						['c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 4367);
					match s {
						[] => true,
						['r', 'w', '\u{1}', s @ ..] => {
							super::push(out, 8747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 4368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4369);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4370);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4371);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 4372);
					s.is_empty()
				}
				['n', 'a', 'b', 'a', 'r', 't', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 4373);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8749);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 8750);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4375);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 8753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4377);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4378);
					match s {
						[] => true,
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8755);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4379);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4380);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8758);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 12215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'w', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 8759);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8760);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 12216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4381);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8761);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4382);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8763);
							match s {
								[] => true,
								['i', 'k', s @ ..] => {
									super::push(out, 12218);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4383);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8765);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8766);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4385);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8768);
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 12219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4386);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4387);
					match s {
						[] => true,
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 4388);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 8770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1281);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4389);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1282);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n1296(s, out),
		['j', 'c', 'i', 'f', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1284);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4402);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1286);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4404);
					match s {
						[] => true,
						['g', 'r', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 8801);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 8802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4405);
					match s {
						[] => true,
						['e', 'm', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 8803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 4406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4407);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4408);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4409);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['a', 'r', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 8804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4411);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1289);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4414);
					match s {
						[] => true,
						['a', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 8806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4415);
					match s {
						[] => true,
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8808);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 8809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', 'r', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n1303(s, out),
		['s', s @ ..] => super::t3::n1304(s, out),
		['t', s @ ..] => super::t3::n1305(s, out),
		['u', s @ ..] => super::t3::n1306(s, out),
		['w', s @ ..] => super::t3::n1307(s, out),
		['x', 'a', '\u{1}', s @ ..] => super::t3::n1308(s, out),
		['z', s @ ..] => super::t3::n1309(s, out),
		['ż', s @ ..] => super::t3::n1310(s, out),
		_ => false,
	}
}

/// Words ending -fl.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	s.is_empty()
}

/// Words ending -il.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1298);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1299);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4446);
					match s {
						[] => true,
						['l', 'a', 'k', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4447);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8835);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12247);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 12248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8836);
							s.is_empty()
						}
						['l', 'g', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8837);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8838);
							match s {
								[] => true,
								['o', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8839);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12250);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1301);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1303);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1304);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4449);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4450);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1306);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 4451);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 8842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jl.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kl.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1308);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1309);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8843);
							s.is_empty()
						}
						['o', 't', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ll.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8845);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8846);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8847);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 12252);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 12253);
									s.is_empty()
								}
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
			super::push(out, 1312);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 4455);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ol.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4456);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 8848);
							s.is_empty()
						}
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 8849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4457);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4458);
					match s {
						[] => true,
						['f', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8850);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 8851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4459);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4460);
					s.is_empty()
				}
				['s', 'j', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4461);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4462);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8852);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8854);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4464);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4465);
					match s {
						[] => true,
						['e', 'i', 'p', 'z', s @ ..] => {
							super::push(out, 8857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4467);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1319);
			match s {
				[] => true,
				['c', 'y', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1320);
			match s {
				[] => true,
				['z', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1321);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 8858);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 4471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4472);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4474);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4475);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4476);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8860);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 12254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 8861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['u', 'ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4479);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4480);
					s.is_empty()
				}
				['y', 'w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4482);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4483);
					s.is_empty()
				}
				['y', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 4484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4485);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 8863);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8864);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8865);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4486);
					s.is_empty()
				}
				['y', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4488);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 8866);
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8867);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8868);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 4489);
					match s {
						[] => true,
						['e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 8870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 4490);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1328);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 4492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rl.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tl.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	s.is_empty()
}

/// Words ending -ul.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1330);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['t', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1332);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1334);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1335);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['r', 'a', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 4497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1337);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1341);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yl.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4502);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4503);
					s.is_empty()
				}
				['e', 'b', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 4504);
					s.is_empty()
				}
				['k', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4506);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4507);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4509);
					s.is_empty()
				}
				['z', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4511);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1347);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4513);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8874);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 8875);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 12256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 8876);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 12257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4515);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8877);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 8878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8879);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4517);
					match s {
						[] => true,
						['i', 'n', 'o', s @ ..] => {
							super::push(out, 8881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1349);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4518);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4519);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12258);
									s.is_empty()
								}
								['o', 'r', 'e', 't', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12259);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 12260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4520);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 8883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4521);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8884);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8886);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', 'b', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4524);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4525);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 8888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4526);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 8889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ól.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4528);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4529);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8890);
							s.is_empty()
						}
						['w', 'r', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 8891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4530);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -śl.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1356);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 4531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nom.
pub(crate) fn n1401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1385);
	match s {
		[] => true,
		['e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 4554);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4555);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4556);
			match s {
				[] => true,
				['k', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8906);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12266);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -brom.
pub(crate) fn n4722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4558);
	s.is_empty()
}

/// Words ending -drom.
pub(crate) fn n4723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4559);
	s.is_empty()
}

/// Words ending -hrom.
pub(crate) fn n4724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4560);
	s.is_empty()
}

/// Words ending -som.
pub(crate) fn n1403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	s.is_empty()
}

/// Words ending -tom.
pub(crate) fn n1404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	match s {
		[] => true,
		['a', 'n', s @ ..] => {
			super::push(out, 4561);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 8908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łom.
pub(crate) fn n1405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	match s {
		[] => true,
		['o', 'z', 's', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rm.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1390);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4563);
					match s {
						[] => true,
						['n', 'a', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 8909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1391);
			match s {
				[] => true,
				['f', 'o', s @ ..] => {
					super::push(out, 4564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tm.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	s.is_empty()
}

/// Words ending -um.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1392);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1393);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4565);
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', 'o', 'r', s @ ..] => {
			super::push(out, 1394);
			s.is_empty()
		}
		['f', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1395);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4567);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'l', s @ ..] => {
							super::push(out, 8910);
							s.is_empty()
						}
						['o', 't', 'a', 'g', s @ ..] => {
							super::push(out, 8911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4568);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8912);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12268);
									s.is_empty()
								}
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
			super::push(out, 1397);
			match s {
				[] => true,
				['y', 'r', 'o', 'b', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['o', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1400);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4572);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 4573);
					match s {
						[] => true,
						['e', 't', 'i', 'f', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'i', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['o', 'i', 'r', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ym.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		['t', 'a', 'r', 'b', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1407);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4579);
					match s {
						[] => true,
						['j', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4580);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4581);
					match s {
						[] => true,
						['b', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8915);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zm.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8917);
							s.is_empty()
						}
						['r', 'b', s @ ..] => {
							super::push(out, 8918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', 'r', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				['f', 'r', s @ ..] => {
					super::push(out, 4586);
					match s {
						[] => true,
						['o', 'm', 'o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12269);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 12270);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 12271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4587);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							match s {
								[] => true,
								['o', 'e', s @ ..] => {
									super::push(out, 12272);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 12273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'c', 'e', s @ ..] => {
					super::push(out, 4588);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', s @ ..] => {
									super::push(out, 12274);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12275);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 12276);
									s.is_empty()
								}
								['i', 'w', s @ ..] => {
									super::push(out, 12277);
									s.is_empty()
								}
								['k', 'o', s @ ..] => {
									super::push(out, 12278);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 12279);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 15093);
													s.is_empty()
												}
												['j', s @ ..] => {
													match s {
														[] => true,
														['c', s @ ..] => super::t3::n16466(s, out),
														['z', s @ ..] => super::t3::n16467(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 15094);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'u', 'l', s @ ..] => {
									super::push(out, 12280);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 12281);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 14228);
											s.is_empty()
										}
										['t', 'k', 'a', s @ ..] => {
											super::push(out, 14229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 8919);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 8920);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8921);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 12282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8922);
							s.is_empty()
						}
						['u', 't', s @ ..] => {
							super::push(out, 8923);
							s.is_empty()
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
								['f', s @ ..] => {
									super::push(out, 12283);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 12284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'k', s @ ..] => {
							super::push(out, 8924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => super::t3::n9400(s, out),
						['e', 't', s @ ..] => {
							super::push(out, 8925);
							s.is_empty()
						}
						['i', 'a', s @ ..] => {
							super::push(out, 8926);
							s.is_empty()
						}
						['o', s @ ..] => super::t3::n9403(s, out),
						['y', 't', 'a', s @ ..] => super::t3::n9404(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t3::n4758(s, out),
				['p', 'o', 'r', 'p', s @ ..] => super::t3::n4759(s, out),
				['s', 'k', 'r', 'a', 'm', 'o', s @ ..] => super::t3::n4760(s, out),
				['w', s @ ..] => super::t3::n4761(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t3::n1427(s, out),
		_ => false,
	}
}

/// Words ending -łm.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	s.is_empty()
}

/// Words ending -an.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4597);
					s.is_empty()
				}
				['a', 'k', s @ ..] => {
					super::push(out, 4598);
					s.is_empty()
				}
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4599);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4600);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4601);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1413);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8941);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4603);
					s.is_empty()
				}
				['p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1414);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4605);
					s.is_empty()
				}
				['g', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4606);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4607);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4608);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 8944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['c', 'o', 'h', s @ ..] => {
					super::push(out, 4609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4610);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'y', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8945);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 8946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1417);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4611);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8947);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8948);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 12313);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 14248);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8949);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 8950);
							s.is_empty()
						}
						['ż', s @ ..] => {
							super::push(out, 8951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4612);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4613);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4614);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4615);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1418);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8954);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 8955);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8956);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 8957);
							match s {
								[] => true,
								['d', 'g', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 12314);
									s.is_empty()
								}
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
			super::push(out, 1419);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4616);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8958);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 12315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ś', s @ ..] => {
							super::push(out, 8959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4617);
					match s {
						[] => true,
						['a', 'r', 'e', s @ ..] => {
							super::push(out, 8960);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 12316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4618);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8961);
							match s {
								[] => true,
								['t', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4619);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4620);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4621);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 8963);
							match s {
								[] => true,
								['y', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 12318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4622);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4623);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8965);
							match s {
								[] => true,
								['y', 'r', 'k', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 12319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4624);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 8966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'd', 'o', 'ł', s @ ..] => {
					super::push(out, 4625);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 8967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1420);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4626);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 4627);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4628);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8969);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4630);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8971);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8972);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 14249);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 14250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n4805(s, out),
				['i', s @ ..] => {
					super::push(out, 4632);
					match s {
						[] => true,
						['l', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4633);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4634);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => super::t3::n9460(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n4809(s, out),
				['u', s @ ..] => super::t3::n4810(s, out),
				['y', s @ ..] => super::t3::n4811(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n1438(s, out),
		['m', s @ ..] => super::t3::n1439(s, out),
		['n', s @ ..] => super::t3::n1440(s, out),
		['p', s @ ..] => super::t3::n1441(s, out),
		['r', s @ ..] => super::t3::n1442(s, out),
		['s', s @ ..] => super::t3::n1443(s, out),
		['t', s @ ..] => super::t3::n1444(s, out),
		['u', 'ż', 'n', 'o', 'd', '\u{1}', s @ ..] => super::t3::n1445(s, out),
		['v', s @ ..] => super::t3::n1446(s, out),
		['w', s @ ..] => super::t3::n1447(s, out),
		['z', s @ ..] => super::t3::n1448(s, out),
		['ł', s @ ..] => super::t3::n1449(s, out),
		['ż', s @ ..] => super::t3::n1450(s, out),
		_ => false,
	}
}

/// Words ending -in.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1452);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 4731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9045);
							s.is_empty()
						}
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'e', 'w', 'n', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4732);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4733);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4734);
					match s {
						[] => true,
						['k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9047);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12348);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 9048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 's', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1454);
			match s {
				[] => true,
				['e', 'h', 'a', 'ż', 'd', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['t', 's', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4738);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4739);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4740);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4742);
					s.is_empty()
				}
				['d', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4743);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 9049);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4745);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4746);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9051);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9052);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14255);
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											super::push(out, 14256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9053);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9054);
							match s {
								[] => true,
								['b', 'm', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 12350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4747);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4748);
					s.is_empty()
				}
				['s', 'e', 'l', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4750);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4751);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4752);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4753);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4755);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 4757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4758);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4759);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4760);
					s.is_empty()
				}
				['ź', s @ ..] => {
					super::push(out, 4761);
					match s {
						[] => true,
						['d', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4762);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 9059);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 12351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 9060);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12352);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 12353);
									match s {
										[] => true,
										['r', 'f', s @ ..] => {
											super::push(out, 14257);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 9061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4763);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4764);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4765);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9064);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 12354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9065);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12355);
									s.is_empty()
								}
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
			super::push(out, 1462);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4768);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4770);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4771);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4772);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 4773);
					s.is_empty()
				}
				['ź', 'd', 'ę', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4776);
					s.is_empty()
				}
				['e', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4777);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4778);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4779);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9067);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'e', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['a', 't', '\u{1}', s @ ..] => {
					super::push(out, 4781);
					s.is_empty()
				}
				['d', s @ ..] => super::t3::n4960(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1473);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['k', 'i', s @ ..] => {
					super::push(out, 4783);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4784);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4785);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1476);
			match s {
				[] => true,
				['o', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9073);
							s.is_empty()
						}
						['o', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9075);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9077);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9078);
							match s {
								[] => true,
								['d', 'e', '\u{1}', s @ ..] => {
									super::push(out, 12356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4787);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 4788);
					match s {
						[] => true,
						['i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 9079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4789);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9080);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9081);
							s.is_empty()
						}
						['k', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9082);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4791);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							super::push(out, 9085);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 12357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4792);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 9086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4793);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4794);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9087);
							match s {
								[] => true,
								['a', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 12358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4796);
					match s {
						[] => true,
						['l', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 9088);
							s.is_empty()
						}
						['r', 'd', s @ ..] => {
							super::push(out, 9089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9090);
							s.is_empty()
						}
						['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9091);
							s.is_empty()
						}
						['i', 'c', 'e', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 9092);
							s.is_empty()
						}
						['k', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9093);
							s.is_empty()
						}
						['l', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9094);
							s.is_empty()
						}
						['m', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 9095);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9096);
							match s {
								[] => true,
								['t', 'a', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 12360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 9097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4799);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1482);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4801);
					match s {
						[] => true,
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4802);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9099);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 9100);
							match s {
								[] => true,
								['c', 'e', 'd', 'n', s @ ..] => {
									super::push(out, 12361);
									s.is_empty()
								}
								['d', 'r', 'e', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12362);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12363);
									match s {
										[] => true,
										['a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 14258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 12364);
									match s {
										[] => true,
										['z', 'c', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 14259);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 12365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4803);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4804);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4805);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9102);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9103);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 14260);
											s.is_empty()
										}
										['z', 's', '\u{1}', s @ ..] => {
											super::push(out, 14261);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 12366);
									match s {
										[] => true,
										['z', 'c', '\u{1}', s @ ..] => {
											super::push(out, 14262);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9104);
							match s {
								[] => true,
								['o', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 12367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 9105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4806);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9106);
							s.is_empty()
						}
						['t', 'i', 'f', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9107);
							s.is_empty()
						}
						['u', s @ ..] => super::t3::n9596(s, out),
						_ => false,
					}
				}
				['s', 'u', 'b', '\u{1}', s @ ..] => super::t3::n4989(s, out),
				['t', 'a', 'm', 'i', 'k', '\u{1}', s @ ..] => super::t3::n4990(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t3::n1499(s, out),
		['k', s @ ..] => super::t3::n1500(s, out),
		['l', s @ ..] => super::t3::n1501(s, out),
		['m', s @ ..] => super::t3::n1502(s, out),
		['n', s @ ..] => super::t3::n1503(s, out),
		['p', s @ ..] => super::t3::n1504(s, out),
		['r', s @ ..] => super::t3::n1505(s, out),
		['s', s @ ..] => super::t3::n1506(s, out),
		['t', s @ ..] => super::t3::n1507(s, out),
		['w', s @ ..] => super::t3::n1508(s, out),
		['z', s @ ..] => super::t3::n1509(s, out),
		['ł', s @ ..] => super::t3::n1510(s, out),
		_ => false,
	}
}

/// Words ending -un.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4868);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4869);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1501);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1502);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 4871);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 4872);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 4873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1503);
			match s {
				[] => true,
				['j', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 4874);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4875);
					match s {
						[] => true,
						['l', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1504);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4876);
					match s {
						[] => true,
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 9163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4877);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4878);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1505);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1506);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4880);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4881);
					match s {
						[] => true,
						['i', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'z', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1507);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1508);
			match s {
				[] => true,
				['u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['ć', '\u{1}', s @ ..] => {
					super::push(out, 4885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1511);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4886);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4887);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12388);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 12389);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 14270);
											s.is_empty()
										}
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
					super::push(out, 4888);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9168);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 14271);
											s.is_empty()
										}
										['w', 'z', '\u{1}', s @ ..] => {
											super::push(out, 14272);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 4889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1513);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4890);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4891);
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -own.
pub(crate) fn n1532(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9170);
					s.is_empty()
				}
				['k', 'c', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 9172);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 9173);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yn.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4893);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4895);
					s.is_empty()
				}
				['e', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 4896);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4897);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9174);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4898);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4899);
					match s {
						[] => true,
						['a', 'n', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['a', 'd', s @ ..] => {
					super::push(out, 4900);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4901);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4902);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9178);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 9179);
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4903);
					s.is_empty()
				}
				['i', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4904);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4905);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['k', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 12391);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 12392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4907);
					s.is_empty()
				}
				['e', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4908);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4909);
					match s {
						[] => true,
						['y', 'd', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'b', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 4910);
					s.is_empty()
				}
				['r', 'e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4911);
					s.is_empty()
				}
				['s', 'i', 'l', 'i', 'f', '\u{1}', s @ ..] => {
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
		['z', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4914);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4915);
					match s {
						[] => true,
						['e', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4916);
					match s {
						[] => true,
						['a', 't', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 9184);
							s.is_empty()
						}
						['g', 'ę', 'w', '\u{1}', s @ ..] => {
							super::push(out, 9185);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9186);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4917);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 9188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1521);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4918);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -manko.
pub(crate) fn n5132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4945);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onko.
pub(crate) fn n5133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4946);
	match s {
		[] => true,
		['z', 'd', 'e', 'j', s @ ..] => {
			super::push(out, 9195);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oko.
pub(crate) fn n1580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1560);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4947);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4948);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rko.
pub(crate) fn n1581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1561);
	s.is_empty()
}

/// Words ending -sko.
pub(crate) fn n1582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1562);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4949);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9196);
					match s {
						[] => true,
						['u', 'g', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', s @ ..] => {
					super::push(out, 9197);
					match s {
						[] => true,
						['r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9198);
					match s {
						[] => true,
						['o', 'z', s @ ..] => {
							super::push(out, 12395);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 14273);
									match s {
										[] => true,
										['r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 15097);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 12396);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 14274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 9199);
					match s {
						[] => true,
						['d', 'y', s @ ..] => {
							super::push(out, 12397);
							match s {
								[] => true,
								['ż', '\u{1}', s @ ..] => {
									super::push(out, 14275);
									s.is_empty()
								}
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
			super::push(out, 4950);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9200);
					match s {
						[] => true,
						['z', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 12398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4951);
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12399);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14276);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['e', 'i', 'b', '\u{1}', s @ ..] => {
													super::push(out, 15398);
													s.is_empty()
												}
												['i', 'b', '\u{1}', s @ ..] => {
													super::push(out, 15399);
													s.is_empty()
												}
												_ => false,
											}
										}
										['j', 'i', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['j', s @ ..] => {
									super::push(out, 14277);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 15099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 12400);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 14278);
									match s {
										[] => true,
										['k', 'a', '\u{1}', s @ ..] => {
											super::push(out, 15100);
											s.is_empty()
										}
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
		['ę', s @ ..] => {
			super::push(out, 4952);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ń', s @ ..] => {
			super::push(out, 4953);
			match s {
				[] => true,
				['a', 'l', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tko.
pub(crate) fn n1583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1563);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 4954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 4955);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4956);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 9205);
					match s {
						[] => true,
						['z', 's', 'w', '\u{1}', s @ ..] => {
							super::push(out, 12401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 4957);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wko.
pub(crate) fn n1584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1564);
	s.is_empty()
}

/// Words ending -yko.
pub(crate) fn n1585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1565);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zko.
pub(crate) fn n1586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	s.is_empty()
}

/// Words ending -łko.
pub(crate) fn n1587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1567);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'y', 'b', s @ ..] => {
					super::push(out, 9206);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9207);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1569);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4959);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4961);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4962);
					match s {
						[] => true,
						['n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4963);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4964);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9210);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 9211);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 'r', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 1573);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4965);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

