use flexion::Path;

/// Words ending -latus.
pub(crate) fn n1646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1497);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3764);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7164);
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
				['g', s @ ..] => {
					super::push(out, 7165);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 10998);
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
				['p', s @ ..] => {
					super::push(out, 7167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10999);
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
		['b', s @ ..] => {
			super::push(out, 3765);
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
			super::push(out, 3766);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7168);
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
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7169);
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
				['d', s @ ..] => {
					super::push(out, 7170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7172);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7174);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11000);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 11001);
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
				['v', s @ ..] => {
					super::push(out, 7175);
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
					super::push(out, 7176);
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
			super::push(out, 3767);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7177);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14402);
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
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7178);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 11003);
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
					super::push(out, 7179);
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
				['u', s @ ..] => {
					super::push(out, 7180);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 14403);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 14404);
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
		['g', s @ ..] => {
			super::push(out, 3768);
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t9::n4524(s, out),
		['l', s @ ..] => super::t9::n4525(s, out),
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
		['o', s @ ..] => super::t9::n4528(s, out),
		['p', s @ ..] => {
			super::push(out, 3772);
			match s {
				[] => true,
				['m', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3773);
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
			super::push(out, 3774);
			match s {
				[] => true,
				['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t9::n4532(s, out),
		_ => false,
	}
}

/// Words ending -matus.
pub(crate) fn n1647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1498);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3776);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7215);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7216);
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
					super::push(out, 7217);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7218);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 11071);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11072);
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
					super::push(out, 7219);
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
				['l', s @ ..] => {
					super::push(out, 7220);
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
									super::push(out, 14450);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 16630);
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
									super::push(out, 14451);
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
				['u', s @ ..] => {
					super::push(out, 7221);
					match s {
						[] => true,
						['q', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 't', 'u', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 14452);
									s.is_empty()
								}
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
			super::push(out, 3777);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7222);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7223);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7224);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7225);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7226);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14453);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
			super::push(out, 3778);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7227);
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
					super::push(out, 7228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 11073);
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
							super::push(out, 11074);
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
					super::push(out, 7229);
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
					super::push(out, 7230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11075);
							s.is_empty()
						}
						['c', s @ ..] => super::t9::n17012(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t9::n9881(s, out),
				['t', s @ ..] => super::t9::n9882(s, out),
				['x', s @ ..] => super::t9::n9883(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t9::n4537(s, out),
		['m', s @ ..] => super::t9::n4538(s, out),
		['o', s @ ..] => super::t9::n4539(s, out),
		['r', s @ ..] => super::t9::n4540(s, out),
		['s', s @ ..] => super::t9::n4541(s, out),
		['u', s @ ..] => super::t9::n4542(s, out),
		_ => false,
	}
}

/// Words ending -natus.
pub(crate) fn n1648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1499);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3785);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['l', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7247);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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
					super::push(out, 7249);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14464);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14465);
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
							super::push(out, 11087);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11088);
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
				['r', s @ ..] => {
					super::push(out, 7250);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11089);
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
			super::push(out, 3786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7252);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
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
					super::push(out, 7253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11090);
							s.is_empty()
						}
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
						['c', s @ ..] => {
							super::push(out, 11091);
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
						['f', s @ ..] => {
							super::push(out, 11092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14466);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 16637);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 16638);
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
									super::push(out, 14467);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 14468);
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
		['g', s @ ..] => super::t9::n4546(s, out),
		['i', s @ ..] => super::t9::n4547(s, out),
		['m', s @ ..] => {
			super::push(out, 3789);
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
			super::push(out, 3790);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7270);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14506);
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
				['i', s @ ..] => super::t9::n9936(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t9::n4550(s, out),
		['r', s @ ..] => super::t9::n4551(s, out),
		['u', s @ ..] => super::t9::n4552(s, out),
		_ => false,
	}
}

/// Words ending -oatus.
pub(crate) fn n1649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1500);
	s.is_empty()
}

/// Words ending -patus.
pub(crate) fn n1650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1501);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3794);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7286);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11144);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 16656);
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
				['t', s @ ..] => {
					super::push(out, 7287);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14517);
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
			super::push(out, 3795);
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
				['u', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 3796);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3797);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3798);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7288);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7289);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11145);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11146);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14518);
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
			super::push(out, 3799);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3800);
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

/// Words ending -ratus.
pub(crate) fn n1651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1502);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3801);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3802);
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
					super::push(out, 7290);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11147);
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
									super::push(out, 14519);
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
							super::push(out, 11148);
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
					super::push(out, 7291);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
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
					super::push(out, 7292);
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
								['r', s @ ..] => {
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
								['s', s @ ..] => {
									super::push(out, 14520);
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
			super::push(out, 3803);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7293);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 14521);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 14522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7294);
					match s {
						[] => true,
						['l', 'e', 'c', s @ ..] => {
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
										['o', s @ ..] => {
											super::push(out, 16657);
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
								_ => false,
							}
						}
						['n', 'e', 't', 'n', s @ ..] => {
							super::push(out, 11149);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11150);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 11151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7295);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14523);
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
							super::push(out, 11153);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 11154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7296);
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
				['o', s @ ..] => super::t9::n9973(s, out),
				['u', s @ ..] => super::t9::n9974(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t9::n4563(s, out),
		['d', s @ ..] => {
			super::push(out, 3805);
			match s {
				[] => true,
				['a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t9::n4565(s, out),
		['g', s @ ..] => super::t9::n4566(s, out),
		['h', s @ ..] => super::t9::n4567(s, out),
		['i', s @ ..] => super::t9::n4568(s, out),
		['o', s @ ..] => super::t9::n4569(s, out),
		['p', s @ ..] => super::t9::n4570(s, out),
		['r', s @ ..] => super::t9::n4571(s, out),
		['t', s @ ..] => super::t9::n4572(s, out),
		['u', s @ ..] => super::t9::n4573(s, out),
		['y', s @ ..] => super::t9::n4574(s, out),
		_ => false,
	}
}

/// Words ending -satus.
pub(crate) fn n1652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1503);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3816);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3817);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3818);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3819);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11234);
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
			super::push(out, 3820);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7364);
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
							super::push(out, 11235);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14573);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 14574);
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
						['s', s @ ..] => {
							super::push(out, 11236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 14575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7365);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7366);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11238);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 17682);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n36493(s, out),
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
			super::push(out, 3821);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3822);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3823);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 14576);
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
									super::push(out, 14577);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 16686);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', '\u{1}', s @ ..] => {
											super::push(out, 16687);
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
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7367);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', '\u{1}', s @ ..] => {
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
		['s', s @ ..] => super::t9::n4583(s, out),
		['u', s @ ..] => {
			super::push(out, 3825);
			match s {
				[] => true,
				['a', s @ ..] => super::t9::n10061(s, out),
				['c', s @ ..] => super::t9::n10062(s, out),
				['t', 'b', 'o', '\u{1}', s @ ..] => super::t9::n10063(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tatus.
pub(crate) fn n1653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1504);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3826);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7373);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 11243);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11244);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11247);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11248);
							match s {
								[] => true,
								['e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11249);
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
		['c', s @ ..] => {
			super::push(out, 3827);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7375);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'b', 'a', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
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
							super::push(out, 11250);
							s.is_empty()
						}
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14580);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 16688);
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 14581);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
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
				['e', s @ ..] => super::t9::n10067(s, out),
				['i', s @ ..] => {
					super::push(out, 7377);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11258);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14591);
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
							super::push(out, 11259);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 16696);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16697);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t9::n10069(s, out),
				['o', s @ ..] => {
					super::push(out, 7379);
					s.is_empty()
				}
				['r', s @ ..] => super::t9::n10071(s, out),
				['u', s @ ..] => super::t9::n10072(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t9::n4587(s, out),
		['i', s @ ..] => super::t9::n4588(s, out),
		['l', s @ ..] => super::t9::n4589(s, out),
		['n', s @ ..] => super::t9::n4590(s, out),
		['o', s @ ..] => super::t9::n4591(s, out),
		['p', s @ ..] => super::t9::n4592(s, out),
		['r', s @ ..] => super::t9::n4593(s, out),
		['s', s @ ..] => super::t9::n4594(s, out),
		['t', s @ ..] => super::t9::n4595(s, out),
		['u', s @ ..] => super::t9::n4596(s, out),
		_ => false,
	}
}

/// Words ending -uatus.
pub(crate) fn n1654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1505);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3838);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7435);
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
					super::push(out, 7436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3839);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3840);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3841);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7437);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7438);
					match s {
						[] => true,
						['e', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 3842);
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
							super::push(out, 11342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14647);
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
			super::push(out, 3843);
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
					super::push(out, 7439);
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
					super::push(out, 7440);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 14648);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 14649);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 16717);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 16718);
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
									super::push(out, 14650);
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
							super::push(out, 11343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7441);
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
				['r', s @ ..] => {
					super::push(out, 7442);
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14651);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 16719);
											s.is_empty()
										}
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
			super::push(out, 3844);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3845);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7443);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14652);
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
					super::push(out, 7444);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11344);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11345);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7445);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7446);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7447);
					match s {
						[] => true,
						['e', 'a', s @ ..] => super::t9::n17573(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t9::n10149(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vatus.
pub(crate) fn n1655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1506);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3846);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7449);
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
				['l', s @ ..] => {
					super::push(out, 7450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11347);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 11348);
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
				['n', s @ ..] => {
					super::push(out, 7451);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7452);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 11349);
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
							super::push(out, 11350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3847);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 11351);
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
							super::push(out, 11352);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14654);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14655);
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
			super::push(out, 3848);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7453);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11353);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11354);
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
				['t', s @ ..] => {
					super::push(out, 7455);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7456);
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
		['l', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11355);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7457);
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
			super::push(out, 3851);
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
							super::push(out, 11357);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14656);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11358);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
									super::push(out, 14658);
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
					super::push(out, 7458);
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
								['n', 'i', s @ ..] => {
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
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xatus.
pub(crate) fn n1656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1507);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3853);
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
			super::push(out, 3854);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7459);
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
					super::push(out, 7460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11359);
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
			super::push(out, 3855);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7461);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7462);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zatus.
pub(crate) fn n1657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1508);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7464);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7465);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7466);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7467);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11360);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7468);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7469);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7470);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11363);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11364);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11365);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7471);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11367);
							match s {
								[] => true,
								['m', 'o', s @ ..] => {
									super::push(out, 14659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11368);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 14660);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14661);
									s.is_empty()
								}
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

/// Words ending -factus.
pub(crate) fn n4620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3860);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7473);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11369);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 16720);
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
						['u', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 11370);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
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
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 11371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14662);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14663);
							match s {
								[] => true,
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
								_ => false,
							}
						}
						['n', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14664);
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
				['g', s @ ..] => {
					super::push(out, 11372);
					match s {
						[] => true,
						['r', 'e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11373);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14665);
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
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14666);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11374);
					match s {
						[] => true,
						['e', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11375);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11376);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 't', 's', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 11377);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16721);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 16722);
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
						['b', s @ ..] => {
							super::push(out, 14668);
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14669);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14670);
							match s {
								[] => true,
								['u', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 14671);
							match s {
								[] => true,
								['d', 'b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11378);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14672);
							match s {
								[] => true,
								['a', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['q', s @ ..] => {
							super::push(out, 14673);
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14674);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16723);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 16724);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16725);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 16726);
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
				['v', s @ ..] => super::t9::n17626(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t9::n10187(s, out),
		['l', s @ ..] => super::t9::n10188(s, out),
		['n', s @ ..] => {
			super::push(out, 7476);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => super::t9::n17631(s, out),
				_ => false,
			}
		}
		['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => super::t9::n10190(s, out),
		['u', 'n', 'a', 'm', '\u{1}', s @ ..] => super::t9::n10191(s, out),
		_ => false,
	}
}

/// Words ending -ractus.
pub(crate) fn n4626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3866);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7480);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7481);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11384);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14676);
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
					super::push(out, 11385);
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
					super::push(out, 11386);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14679);
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
					super::push(out, 11387);
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
		['h', s @ ..] => {
			super::push(out, 7482);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7483);
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
					super::push(out, 11388);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14681);
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
					super::push(out, 11389);
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

/// Words ending -tactus.
pub(crate) fn n4628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3868);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
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
		['t', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ectus.
pub(crate) fn n1659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1510);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3870);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 3871);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7484);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11390);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 11391);
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
						['r', s @ ..] => {
							super::push(out, 11392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7485);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7486);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11393);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11394);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 16727);
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7487);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7488);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 14682);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 16728);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 16729);
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
			super::push(out, 3872);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7489);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7490);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14683);
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
					super::push(out, 7491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11396);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11397);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11399);
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
					super::push(out, 7492);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7493);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11400);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3873);
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
					super::push(out, 7495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11402);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11403);
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
						['r', s @ ..] => {
							super::push(out, 11404);
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
				['g', 'e', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7496);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7497);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 14684);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14685);
									match s {
										[] => true,
										['n', 'i', s @ ..] => {
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
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 11405);
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
				['n', s @ ..] => super::t9::n10230(s, out),
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t9::n4634(s, out),
		['n', s @ ..] => super::t9::n4635(s, out),
		['p', s @ ..] => super::t9::n4636(s, out),
		['r', s @ ..] => super::t9::n4637(s, out),
		['s', s @ ..] => super::t9::n4638(s, out),
		['t', s @ ..] => super::t9::n4639(s, out),
		['v', s @ ..] => super::t9::n4640(s, out),
		_ => false,
	}
}

/// Words ending -ictus.
pub(crate) fn n1660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1511);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3881);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7517);
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
					super::push(out, 7518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11430);
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
					super::push(out, 7519);
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
					super::push(out, 7520);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 7521);
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
			super::push(out, 3882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7522);
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
					super::push(out, 7523);
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
			super::push(out, 3883);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7524);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7525);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11431);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14691);
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
				['f', s @ ..] => {
					super::push(out, 7526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11433);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 11434);
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
							super::push(out, 11435);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 14692);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14693);
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
		['m', s @ ..] => {
			super::push(out, 3884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7527);
					s.is_empty()
				}
				['a', s @ ..] => {
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
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7528);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7529);
					match s {
						[] => true,
						['b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11437);
							s.is_empty()
						}
						['f', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'v', 'e', 'r', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => super::t9::n10295(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => super::t9::n10297(s, out),
				['n', s @ ..] => super::t9::n10298(s, out),
				['o', s @ ..] => super::t9::n10299(s, out),
				['r', s @ ..] => super::t9::n10300(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nctus.
pub(crate) fn n1662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1513);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3889);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7537);
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
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11441);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 14696);
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
						['o', s @ ..] => {
							super::push(out, 11442);
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
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11444);
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
						['l', s @ ..] => {
							super::push(out, 11445);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7540);
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
				['t', s @ ..] => {
					super::push(out, 7541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
								['n', s @ ..] => {
									super::push(out, 14697);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['v', s @ ..] => {
					super::push(out, 7542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14698);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14699);
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11446);
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
				['i', s @ ..] => {
					super::push(out, 7544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 11447);
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
							super::push(out, 11448);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11449);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => super::t9::n25372(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t9::n17791(s, out),
						['n', s @ ..] => super::t9::n17792(s, out),
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => super::t9::n17793(s, out),
						['s', 'i', 'd', '\u{1}', s @ ..] => super::t9::n17794(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t9::n10316(s, out),
				['n', 'i', '\u{1}', s @ ..] => super::t9::n10317(s, out),
				['p', s @ ..] => super::t9::n10318(s, out),
				['r', 'e', 'p', '\u{1}', s @ ..] => super::t9::n10319(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -octus.
pub(crate) fn n1663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1514);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7547);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11453);
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
				['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 3892);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7548);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11454);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11455);
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
					super::push(out, 7550);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7551);
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
					super::push(out, 7552);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7553);
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

/// Words ending -uctus.
pub(crate) fn n1664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1515);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3893);
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
		['d', s @ ..] => {
			super::push(out, 3894);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7554);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 7555);
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
				['d', s @ ..] => {
					super::push(out, 7556);
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
					super::push(out, 7557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11456);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11459);
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
					super::push(out, 7558);
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
				['n', s @ ..] => {
					super::push(out, 7559);
					match s {
						[] => true,
						['i', s @ ..] => {
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
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7560);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 7561);
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
								['t', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 's', '\u{1}', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 7562);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 11460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'o', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3896);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7563);
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
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 14701);
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
									super::push(out, 14702);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 16737);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 16738);
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
									super::push(out, 14703);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 16739);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 16740);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => super::t9::n35086(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => super::t9::n25395(s, out),
								['x', 'e', '\u{1}', s @ ..] => super::t9::n25396(s, out),
								_ => false,
							}
						}
						['x', s @ ..] => super::t9::n17833(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t9::n4658(s, out),
		_ => false,
	}
}

/// The word accitus.
pub(crate) fn n10393(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -icitus.
pub(crate) fn n4685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3913);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11477);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11478);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 14705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
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

/// The word concitus.
pub(crate) fn n4686(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ocitus.
pub(crate) fn n4687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3914);
	match s {
		[] => true,
		['n', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcitus.
pub(crate) fn n4688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3915);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
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

/// Words ending -scitus.
pub(crate) fn n4689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3916);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7583);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7584);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7585);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11480);
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

/// Words ending -xcitus.
pub(crate) fn n4690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3917);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ditus.
pub(crate) fn n1680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1531);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7586);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3919);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3920);
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
			super::push(out, 3921);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7587);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 7588);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7590);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7591);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7592);
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
				['r', s @ ..] => {
					super::push(out, 7593);
					match s {
						[] => true,
						['c', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3922);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7594);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3923);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7596);
					match s {
						[] => true,
						['l', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14706);
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
				['e', s @ ..] => {
					super::push(out, 7597);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14707);
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
					super::push(out, 7598);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7599);
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
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7601);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7602);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7603);
					match s {
						[] => true,
						['s', 'u', 'c', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 3925);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7604);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14708);
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
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3926);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 7607);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11481);
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

/// Words ending -eitus.
pub(crate) fn n1681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1532);
	s.is_empty()
}

/// Words ending -fitus.
pub(crate) fn n1682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1533);
	s.is_empty()
}

/// Words ending -gitus.
pub(crate) fn n1683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1534);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3927);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3928);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7609);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14709);
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
					super::push(out, 7610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3929);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 11482);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14711);
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
						['n', s @ ..] => {
							super::push(out, 11483);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11484);
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

/// Words ending -kitus.
pub(crate) fn n1684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1535);
	s.is_empty()
}

/// Words ending -litus.
pub(crate) fn n1685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1536);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3930);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3931);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7611);
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
		['b', s @ ..] => {
			super::push(out, 3932);
			match s {
				[] => true,
				['o', s @ ..] => {
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
			super::push(out, 3933);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3934);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3935);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3936);
			match s {
				[] => true,
				['b', 'a', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11485);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3937);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11486);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11487);
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
				['e', s @ ..] => {
					super::push(out, 7615);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7617);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11488);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11489);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16741);
											s.is_empty()
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
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7618);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14712);
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
		['m', s @ ..] => {
			super::push(out, 3938);
			match s {
				[] => true,
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3939);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7619);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11490);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11491);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11492);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11493);
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
					super::push(out, 7621);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7622);
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
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11494);
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
		['r', s @ ..] => {
			super::push(out, 3940);
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
		_ => false,
	}
}

/// Words ending -mitus.
pub(crate) fn n1686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1537);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3941);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3942);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7624);
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
				['v', s @ ..] => {
					super::push(out, 7625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11495);
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
			super::push(out, 3943);
			match s {
				[] => true,
				['o', 'd', 'e', s @ ..] => {
					super::push(out, 7626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nitus.
pub(crate) fn n1687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1538);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3945);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7627);
					match s {
						[] => true,
						['i', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 7628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3946);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11496);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14713);
									match s {
										[] => true,
										['o', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 14714);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11497);
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
							super::push(out, 11498);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 14715);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 16742);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 16743);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11499);
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
				['o', s @ ..] => {
					super::push(out, 7631);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7632);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3947);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11500);
							match s {
								[] => true,
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
					super::push(out, 7635);
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
			super::push(out, 3948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7636);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11501);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11502);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14717);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 14718);
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
							super::push(out, 11503);
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
				['l', s @ ..] => {
					super::push(out, 7638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7639);
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
		['n', s @ ..] => {
			super::push(out, 3949);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3950);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7640);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 11505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7641);
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
							super::push(out, 11506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14719);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
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
							super::push(out, 11507);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t9::n17985(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t9::n10474(s, out),
				['t', s @ ..] => super::t9::n10475(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t9::n4724(s, out),
		_ => false,
	}
}

/// Words ending -oitus.
pub(crate) fn n1688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1539);
	s.is_empty()
}

/// Words ending -pitus.
pub(crate) fn n1689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1540);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3952);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11512);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14723);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11513);
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
										['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16744);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3953);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11514);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 14725);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14726);
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
			super::push(out, 3954);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3955);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ritus.
pub(crate) fn n1690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1541);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7646);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7647);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7648);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11515);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3957);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7650);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11517);
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
						['m', s @ ..] => {
							super::push(out, 11518);
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
							super::push(out, 11519);
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
				['p', s @ ..] => {
					super::push(out, 7652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11520);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7653);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7654);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
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
			super::push(out, 3958);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3959);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7656);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7657);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11521);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14727);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
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
									super::push(out, 14728);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
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
								['s', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['p', 'a', '\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 7658);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7660);
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
					super::push(out, 7661);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7662);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
					super::push(out, 7663);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7664);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14729);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 14730);
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
		['u', s @ ..] => {
			super::push(out, 3961);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7665);
					s.is_empty()
				}
				['g', s @ ..] => super::t9::n10508(s, out),
				['s', s @ ..] => {
					super::push(out, 7667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -situs.
pub(crate) fn n1691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1542);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3962);
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
			super::push(out, 3963);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7668);
					match s {
						[] => true,
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
				['d', s @ ..] => {
					super::push(out, 7669);
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
			super::push(out, 3964);
			match s {
				[] => true,
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14731);
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
		['n', s @ ..] => {
			super::push(out, 3965);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7670);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
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
					super::push(out, 7671);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7672);
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
					super::push(out, 7673);
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
			super::push(out, 3966);
			match s {
				[] => true,
				['p', s @ ..] => {
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
							super::push(out, 11523);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14732);
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
									super::push(out, 14733);
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
						['m', s @ ..] => {
							super::push(out, 11524);
							match s {
								[] => true,
								['i', s @ ..] => {
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
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 16745);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 17690);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n36505(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 17691);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n36506(s, out),
														_ => false,
													}
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
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11525);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 14734);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => super::t9::n18045(s, out),
						['p', s @ ..] => super::t9::n18046(s, out),
						['r', 'e', s @ ..] => super::t9::n18047(s, out),
						['s', s @ ..] => super::t9::n18048(s, out),
						['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => super::t9::n18050(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t9::n4742(s, out),
		['s', s @ ..] => super::t9::n4743(s, out),
		_ => false,
	}
}

/// Words ending -titus.
pub(crate) fn n1692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1543);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3969);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7676);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7677);
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
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
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
								['u', 's', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 7678);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11530);
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
		['o', s @ ..] => {
			super::push(out, 3972);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7680);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3973);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7682);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14737);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 16746);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 16747);
											match s {
												[] => true,
												['q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14738);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
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
										_ => false,
									}
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14739);
									match s {
										[] => true,
										['c', 'e', 'd', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 16748);
											s.is_empty()
										}
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
					super::push(out, 7683);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14740);
									match s {
										[] => true,
										['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 16749);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
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
										_ => false,
									}
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
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
				['o', 's', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 3974);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7684);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11531);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 11532);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 11533);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 11534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14741);
									match s {
										[] => true,
										['n', 'e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => super::t9::n25576(s, out),
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => super::t9::n25577(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t9::n10536(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uitus.
pub(crate) fn n1693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1544);
	match s {
		[] => true,
		['r', 'f', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 3975);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7686);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7687);
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
				['r', s @ ..] => {
					super::push(out, 7688);
					match s {
						[] => true,
						['o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vitus.
pub(crate) fn n1694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1545);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3976);
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
		['e', s @ ..] => {
			super::push(out, 3977);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7689);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3979);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11535);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xitus.
pub(crate) fn n1695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1546);
	s.is_empty()
}

/// Words ending -urulentus.
pub(crate) fn n25600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14760);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16754);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16755);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 16756);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tulentus.
pub(crate) fn n18102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11550);
	match s {
		[] => true,
		['e', 'f', s @ ..] => {
			super::push(out, 14761);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14762);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 14763);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mentus.
pub(crate) fn n4774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3988);
	s.is_empty()
}

/// Words ending -tentus.
pub(crate) fn n4775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3989);
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
			super::push(out, 7699);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 11551);
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
		['p', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'o', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
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
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', s @ ..] => {
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
		['x', 'e', s @ ..] => {
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

/// Words ending -uentus.
pub(crate) fn n4776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3990);
	match s {
		[] => true,
		['r', 'c', s @ ..] => {
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

/// Words ending -ventus.
pub(crate) fn n4777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3991);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7700);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7701);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11552);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 11553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7702);
			match s {
				[] => true,
				['i', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 7703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ontus.
pub(crate) fn n1702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1552);
	match s {
		[] => true,
		['d', 'o', 'r', s @ ..] => {
			super::push(out, 3992);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -criptus.
pub(crate) fn n4822(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11564);
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
					super::push(out, 11565);
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
					super::push(out, 11566);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14765);
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
					super::push(out, 11567);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11568);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11569);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14766);
							match s {
								[] => true,
								['r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14767);
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
						['o', s @ ..] => {
							super::push(out, 14768);
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
					super::push(out, 11570);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11571);
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
					super::push(out, 11572);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11573);
					match s {
						[] => true,
						['s', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11574);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 11575);
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
			super::push(out, 7717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sculptus.
pub(crate) fn n4823(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7718);
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

/// Words ending -mptus.
pub(crate) fn n1715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1565);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 7719);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 4016);
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
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11576);
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
			super::push(out, 4017);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 11577);
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
							super::push(out, 11578);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14769);
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
							super::push(out, 11579);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14770);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14771);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14772);
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
						['s', s @ ..] => {
							super::push(out, 11580);
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
		_ => false,
	}
}

/// Words ending -rptus.
pub(crate) fn n1717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1567);
	match s {
		[] => true,
		['e', 'c', 'e', s @ ..] => {
			super::push(out, 4018);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7720);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4019);
			match s {
				[] => true,
				['s', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -uptus.
pub(crate) fn n1718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1568);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 4020);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11581);
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
			super::push(out, 4021);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7723);
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
		['r', s @ ..] => {
			super::push(out, 4022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7724);
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
				['e', s @ ..] => {
					super::push(out, 7725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11582);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11583);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11584);
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
					super::push(out, 7726);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7727);
					match s {
						[] => true,
						['r', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 7728);
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
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
						['u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -insertus.
pub(crate) fn n18222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11587);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onsertus.
pub(crate) fn n18223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11588);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssertus.
pub(crate) fn n10696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7737);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14774);
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

/// Words ending -xsertus.
pub(crate) fn n10697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7738);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irtus.
pub(crate) fn n1722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1572);
	s.is_empty()
}

/// Words ending -ortus.
pub(crate) fn n1723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1573);
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
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7739);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11589);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11590);
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

/// Words ending -mistus.
pub(crate) fn n4863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4041);
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

/// Words ending -ostus.
pub(crate) fn n1727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1577);
	match s {
		[] => true,
		['c', 'i', 'v', s @ ..] => {
			super::push(out, 4045);
			match s {
				[] => true,
				['a', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7749);
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
				['e', 'r', '\u{1}', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 7750);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -depstus.
pub(crate) fn n4870(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ustus.
pub(crate) fn n1729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1579);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4046);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4047);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11596);
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
					super::push(out, 7752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
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
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4048);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7753);
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
								['i', s @ ..] => {
									super::push(out, 14775);
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
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11597);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11598);
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
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4049);
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
		['e', s @ ..] => {
			super::push(out, 4050);
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
		['g', s @ ..] => {
			super::push(out, 4051);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 4052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7755);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 11599);
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
		['m', s @ ..] => {
			super::push(out, 4053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7756);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4054);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
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
				['i', '\u{1}', s @ ..] => {
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

/// The word interlocutus.
pub(crate) fn n18299(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -dutus.
pub(crate) fn n1733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1583);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4060);
			match s {
				[] => true,
				['r', 'c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', s @ ..] => {
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

/// Words ending -eutus.
pub(crate) fn n1734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1584);
	s.is_empty()
}

/// Words ending -gutus.
pub(crate) fn n1735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1585);
	match s {
		[] => true,
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'e', 'r', '\u{1}', s @ ..] => {
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

/// Words ending -iutus.
pub(crate) fn n1736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1586);
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

/// Words ending -lutus.
pub(crate) fn n1737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1587);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4061);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4062);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4063);
			s.is_empty()
		}
		['l', 'o', 'p', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 4064);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 7762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', s @ ..] => {
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
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 7763);
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
							super::push(out, 11609);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14780);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14781);
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
							super::push(out, 11610);
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
							super::push(out, 11611);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nutus.
pub(crate) fn n1738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1588);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4066);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 7764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4067);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11612);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 11613);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11614);
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
		['r', 'o', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -putus.
pub(crate) fn n1739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1589);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4068);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7765);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -rutus.
pub(crate) fn n1740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1590);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4069);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7766);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7767);
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
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7768);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7769);
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
			super::push(out, 4071);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7771);
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
		['n', s @ ..] => {
			super::push(out, 4072);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4073);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4074);
			match s {
				[] => true,
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

/// Words ending -sutus.
pub(crate) fn n1741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1591);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4075);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4076);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4077);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4078);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7772);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7774);
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
			super::push(out, 4079);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7775);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4080);
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

/// Words ending -tutus.
pub(crate) fn n1742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1592);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4081);
			match s {
				[] => true,
				['t', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4082);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4083);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11615);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11616);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14782);
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
							super::push(out, 11617);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 14783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14784);
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
							super::push(out, 11618);
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
		['n', s @ ..] => {
			super::push(out, 4084);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4085);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4086);
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
						['e', s @ ..] => {
							super::push(out, 11619);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 11620);
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
							super::push(out, 11621);
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

/// Words ending -sequutus.
pub(crate) fn n1743(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -tivus.
pub(crate) fn n1782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1628);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4115);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7807);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11643);
							match s {
								[] => true,
								['r', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 16758);
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
							super::push(out, 11644);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 14788);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 16759);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 16760);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 17692);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 17693);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 16761);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 16762);
											match s {
												[] => true,
												['i', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n36507(s, out),
														['b', 'a', '\u{1}', s @ ..] => super::t9::n36508(s, out),
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
									super::push(out, 14789);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14790);
									match s {
										[] => true,
										['u', 'm', 'm', 'o', 'c', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 11645);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 16763);
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
				['d', s @ ..] => {
					super::push(out, 7808);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11646);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11647);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11648);
							match s {
								[] => true,
								['o', 'c', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7809);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11649);
							match s {
								[] => true,
								['n', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 11650);
							match s {
								[] => true,
								['r', 'o', s @ ..] => {
									super::push(out, 14791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t9::n10913(s, out),
				['l', s @ ..] => super::t9::n10914(s, out),
				['m', s @ ..] => super::t9::n10915(s, out),
				['n', s @ ..] => super::t9::n10916(s, out),
				['p', s @ ..] => {
					super::push(out, 7814);
					s.is_empty()
				}
				['r', s @ ..] => super::t9::n10918(s, out),
				['s', s @ ..] => super::t9::n10919(s, out),
				['t', s @ ..] => super::t9::n10920(s, out),
				['v', s @ ..] => super::t9::n10921(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t9::n4987(s, out),
		['e', s @ ..] => super::t9::n4988(s, out),
		['i', s @ ..] => super::t9::n4989(s, out),
		['n', s @ ..] => super::t9::n4990(s, out),
		['o', s @ ..] => super::t9::n4991(s, out),
		['p', s @ ..] => super::t9::n4992(s, out),
		['r', s @ ..] => super::t9::n4993(s, out),
		['s', s @ ..] => super::t9::n4994(s, out),
		['u', s @ ..] => super::t9::n4995(s, out),
		_ => false,
	}
}

/// Words ending -torialis.
pub(crate) fn n11213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7970);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11762);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11763);
			match s {
				[] => true,
				['r', 'r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 14822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -curialis.
pub(crate) fn n11214(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11765);
			s.is_empty()
		}
		['r', 'e', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sialis.
pub(crate) fn n2144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1926);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tialis.
pub(crate) fn n2145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1927);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4381);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7971);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4382);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4383);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4384);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7973);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7974);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7975);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14823);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14824);
									s.is_empty()
								}
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
			super::push(out, 4385);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7976);
					match s {
						[] => true,
						['t', 's', 'b', 'u', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14825);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7977);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11766);
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
		['p', s @ ..] => {
			super::push(out, 4387);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4388);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11767);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 11768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4389);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11769);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 11770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vialis.
pub(crate) fn n2146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1928);
	s.is_empty()
}

/// Words ending -ionalis.
pub(crate) fn n5412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4414);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7997);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7998);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7999);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8000);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8001);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8002);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11781);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 14829);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14830);
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
				['c', s @ ..] => {
					super::push(out, 11782);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14831);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14832);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11783);
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 14834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11784);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11785);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11786);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnalis.
pub(crate) fn n2161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1943);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4418);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4419);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8004);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 8005);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8006);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 14835);
									s.is_empty()
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 14836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4420);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -turalis.
pub(crate) fn n5448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4447);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8024);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8025);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8026);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -abilis.
pub(crate) fn n2205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1986);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4485);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4486);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8049);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 8050);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8051);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11805);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
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
								['u', s @ ..] => {
									super::push(out, 14843);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 16780);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 11806);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 16781);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16782);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 16783);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 17694);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 17695);
													s.is_empty()
												}
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
							super::push(out, 11807);
							match s {
								[] => true,
								['p', 'x', 'e', s @ ..] => {
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
						['m', s @ ..] => {
							super::push(out, 11808);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11809);
							match s {
								[] => true,
								['u', 'm', 'm', 'o', 'c', s @ ..] => {
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
							super::push(out, 11810);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 16784);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 17696);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 17697);
													s.is_empty()
												}
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
							super::push(out, 11811);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14844);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8052);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', s @ ..] => {
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
					super::push(out, 8053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4487);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8054);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8055);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14846);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 14847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8056);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14848);
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
				['r', s @ ..] => {
					super::push(out, 8057);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8058);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'o', 'c', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 4488);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', 'r', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t9::n18681(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t9::n5501(s, out),
		['g', s @ ..] => super::t9::n5502(s, out),
		['h', s @ ..] => super::t9::n5503(s, out),
		['i', s @ ..] => super::t9::n5504(s, out),
		['l', s @ ..] => super::t9::n5505(s, out),
		['m', s @ ..] => super::t9::n5506(s, out),
		['n', s @ ..] => super::t9::n5507(s, out),
		['p', s @ ..] => super::t9::n5508(s, out),
		['r', s @ ..] => super::t9::n5509(s, out),
		['s', s @ ..] => super::t9::n5510(s, out),
		['t', s @ ..] => super::t9::n5511(s, out),
		['u', s @ ..] => super::t9::n5512(s, out),
		['v', s @ ..] => super::t9::n5513(s, out),
		['x', s @ ..] => super::t9::n5514(s, out),
		_ => false,
	}
}

/// The word contradicibilis.
pub(crate) fn n35158(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word incontradicibilis.
pub(crate) fn n35159(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -scibilis.
pub(crate) fn n11414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11870);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11871);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11872);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14872);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11873);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16794);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 16795);
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

/// Words ending -lucibilis.
pub(crate) fn n18775(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', 'o', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dibilis.
pub(crate) fn n5518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4506);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8110);
			match s {
				[] => true,
				['r', 'g', 'e', 'd', 'o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11875);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11877);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11878);
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
		['n', s @ ..] => {
			super::push(out, 8112);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8113);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11879);
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
				['p', s @ ..] => {
					super::push(out, 11880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gibilis.
pub(crate) fn n5519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4507);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8115);
			match s {
				[] => true,
				['r', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8116);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14875);
							s.is_empty()
						}
						['l', 'e', 't', 'n', 'i', s @ ..] => {
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

/// Words ending -libilis.
pub(crate) fn n5520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4508);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8117);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nibilis.
pub(crate) fn n5521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4509);
	match s {
		[] => true,
		['o', 'p', 's', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 8119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pibilis.
pub(crate) fn n5522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4510);
	s.is_empty()
}

/// Words ending -ribilis.
pub(crate) fn n5523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4511);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8120);
			match s {
				[] => true,
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 11881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8121);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sibilis.
pub(crate) fn n5524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4512);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8122);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8123);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14876);
							s.is_empty()
						}
						['r', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 11883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14877);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 8124);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14878);
							match s {
								[] => true,
								['e', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', s @ ..] => {
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16796);
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
		['s', s @ ..] => {
			super::push(out, 8125);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
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
				['e', 'c', 'c', 'a', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 11884);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 16797);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'p', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 8126);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11885);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 11886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14880);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tibilis.
pub(crate) fn n5525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4513);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8127);
			match s {
				[] => true,
				['e', 'f', 'e', s @ ..] => {
					super::push(out, 11887);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 14881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', 'e', 's', s @ ..] => {
					super::push(out, 11888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'p', 'd', s @ ..] => {
			super::push(out, 8128);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 11889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['e', 'c', 'r', 'e', 'p', s @ ..] => {
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
				['u', 'r', 'r', 'o', 'c', s @ ..] => {
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
		['r', 'e', 'v', s @ ..] => {
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

/// Words ending -uibilis.
pub(crate) fn n5526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4514);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8129);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 8130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xibilis.
pub(crate) fn n5527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4515);
	s.is_empty()
}

/// Words ending -obilis.
pub(crate) fn n2208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1989);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'c', 'u', 'a', 'p', '\u{1}', s @ ..] => {
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
		['n', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -rbilis.
pub(crate) fn n2209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1990);
	s.is_empty()
}

/// Words ending -ubilis.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1991);
	match s {
		[] => true,
		['l', 'o', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 8131);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'd', s @ ..] => {
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

/// The word careliensis.
pub(crate) fn n18926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11972);
	s.is_empty()
}

/// Words ending -iliensis.
pub(crate) fn n11596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8220);
	match s {
		[] => true,
		['s', 'a', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 11973);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oliensis.
pub(crate) fn n11597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8221);
	s.is_empty()
}

/// Words ending -miensis.
pub(crate) fn n5694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4630);
	s.is_empty()
}

/// Words ending -niensis.
pub(crate) fn n5695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4631);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11974);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 14905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8223);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8224);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11976);
					s.is_empty()
				}
				['l', 'o', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8225);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11978);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11979);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11980);
					match s {
						[] => true,
						['l', 'u', '\u{1}', s @ ..] => {
							super::push(out, 14906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 11981);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 14907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -riensis.
pub(crate) fn n5696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4632);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8227);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11982);
					match s {
						[] => true,
						['s', 'a', 'g', 'a', 'd', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8228);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 11983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8229);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11984);
					s.is_empty()
				}
				['t', 'a', 'u', s @ ..] => {
					super::push(out, 11985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8230);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siensis.
pub(crate) fn n5697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4633);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 11986);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11987);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiensis.
pub(crate) fn n5698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4634);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8233);
			match s {
				[] => true,
				['c', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 11989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			super::push(out, 8234);
			match s {
				[] => true,
				['c', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 11990);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -viensis.
pub(crate) fn n5699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4635);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8235);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11991);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14909);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 11992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kensis.
pub(crate) fn n2328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2079);
	match s {
		[] => true,
		['o', 'b', s @ ..] => {
			super::push(out, 4636);
			match s {
				[] => true,
				['m', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lensis.
pub(crate) fn n2329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2080);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4637);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4638);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mensis.
pub(crate) fn n2330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2081);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4639);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8239);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 11993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4640);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8240);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nensis.
pub(crate) fn n2331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2082);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4641);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8242);
					match s {
						[] => true,
						['o', s @ ..] => {
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
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8243);
					match s {
						[] => true,
						['i', 'x', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8244);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8245);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8246);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11996);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14911);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14912);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8247);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11997);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8248);
					match s {
						[] => true,
						['s', 'i', 'k', s @ ..] => {
							super::push(out, 11999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4642);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 8249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4643);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8250);
					match s {
						[] => true,
						['r', 'c', 'u', '\u{1}', s @ ..] => {
							super::push(out, 12000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8251);
					s.is_empty()
				}
				['d', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8252);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8253);
					match s {
						[] => true,
						['a', 'h', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14913);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 12003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8254);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8255);
					match s {
						[] => true,
						['o', 'r', 'a', s @ ..] => {
							super::push(out, 12004);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 14914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8256);
					match s {
						[] => true,
						['u', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 12005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8257);
					s.is_empty()
				}
				['p', 'p', 'i', 'l', 'i', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8258);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8259);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12006);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12007);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12009);
							match s {
								[] => true,
								['o', 'c', 's', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 14915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8261);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 12010);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12011);
							match s {
								[] => true,
								['e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14916);
									s.is_empty()
								}
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
			super::push(out, 4644);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 8262);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4645);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8264);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8265);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8266);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12013);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12014);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8267);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12015);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12016);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8268);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 12018);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12019);
							s.is_empty()
						}
						['o', 'l', s @ ..] => {
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 14917);
									s.is_empty()
								}
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 14918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8269);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12020);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12021);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8270);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12023);
							s.is_empty()
						}
						['e', s @ ..] => super::t9::n18986(s, out),
						['o', s @ ..] => super::t9::n18987(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t9::n11651(s, out),
				['z', s @ ..] => super::t9::n11652(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t9::n5710(s, out),
		_ => false,
	}
}

/// Words ending -oensis.
pub(crate) fn n2332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2083);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4647);
			match s {
				[] => true,
				['a', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pensis.
pub(crate) fn n2333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2084);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4648);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rensis.
pub(crate) fn n2334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2085);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4650);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8274);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8275);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 14920);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14921);
									s.is_empty()
								}
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
			super::push(out, 4651);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8276);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8277);
					s.is_empty()
				}
				['v', 'e', s @ ..] => {
					super::push(out, 8278);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4652);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tensis.
pub(crate) fn n2335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2086);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4654);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					super::push(out, 8280);
					match s {
						[] => true,
						['h', 'p', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 12027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8281);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12028);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4655);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12030);
							match s {
								[] => true,
								['a', 'r', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 12031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4656);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4657);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4658);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uensis.
pub(crate) fn n2336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2087);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4660);
			match s {
				[] => true,
				['r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4661);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4662);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vensis.
pub(crate) fn n2337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2088);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4664);
			match s {
				[] => true,
				['c', 'e', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8283);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8284);
					s.is_empty()
				}
				['g', 'e', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8285);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yensis.
pub(crate) fn n2338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2089);
	match s {
		[] => true,
		['a', 'l', 'a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

