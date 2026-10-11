use flexion::Path;

/// Words ending -emovens.
pub(crate) fn n7571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5520);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8973);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8975);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8976);
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

/// The word summovens.
pub(crate) fn n13810(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word conmovens.
pub(crate) fn n7574(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -omovens.
pub(crate) fn n7575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5523);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmovens.
pub(crate) fn n7576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5524);
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

/// Words ending -smovens.
pub(crate) fn n7577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5525);
	match s {
		[] => true,
		['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vovens.
pub(crate) fn n3236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2750);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5526);
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

/// Words ending -rvens.
pub(crate) fn n1083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 974);
	match s {
		[] => true,
		['e', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5527);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8977);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8978);
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
					super::push(out, 5528);
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
				['n', s @ ..] => {
					super::push(out, 5529);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8980);
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
		_ => false,
	}
}

/// Words ending -ficus.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1004);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2798);
			match s {
				[] => true,
				['n', 'e', 'v', s @ ..] => {
					super::push(out, 5581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2799);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5582);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9000);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5583);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12397);
									s.is_empty()
								}
								['o', 'd', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5584);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5585);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5586);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5587);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5588);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9002);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9003);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 12398);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 9004);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
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
							super::push(out, 9005);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5589);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9007);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9008);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9009);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 12400);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 12401);
									match s {
										[] => true,
										['o', 'h', s @ ..] => {
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
						['r', s @ ..] => {
							super::push(out, 9010);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9011);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 12402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5590);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9012);
							match s {
								[] => true,
								['v', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 12403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 9013);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5591);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9015);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9016);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9017);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9018);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9019);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9020);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12404);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 12405);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12406);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 12407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5592);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 5593);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gicus.
pub(crate) fn n1122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1005);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2800);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5594);
					match s {
						[] => true,
						['r', 'r', 'o', 'n', 'n', 'e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9022);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15038);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 15039);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9023);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9024);
							match s {
								[] => true,
								['z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15040);
											s.is_empty()
										}
										['o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9025);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 12409);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 12410);
									s.is_empty()
								}
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
			super::push(out, 2802);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5595);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5596);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5597);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9026);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9027);
							match s {
								[] => true,
								['i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 12411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -phicus.
pub(crate) fn n3307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2804);
	match s {
		[] => true,
		['a', 'r', 'g', 'o', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9029);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9030);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9031);
					match s {
						[] => true,
						['o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -thicus.
pub(crate) fn n3308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2805);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -licus.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2806);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5602);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5603);
					match s {
						[] => true,
						['n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2807);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5604);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2808);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9033);
							s.is_empty()
						}
						['a', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5606);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 12413);
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
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9034);
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
				['r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2809);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 5608);
					match s {
						[] => true,
						['p', 'y', s @ ..] => {
							super::push(out, 9035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2810);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5609);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5610);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 9036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -micus.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1008);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5611);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5612);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5613);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5614);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9037);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 9038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5615);
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
					super::push(out, 5616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2812);
			match s {
				[] => true,
				['d', 'i', s @ ..] => {
					super::push(out, 5617);
					match s {
						[] => true,
						['p', 'e', 'o', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 9039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2813);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2814);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5618);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5619);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2815);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2816);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5621);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2817);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5623);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2818);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5625);
					match s {
						[] => true,
						['c', s @ ..] => {
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nicus.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1009);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2819);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5627);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5628);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9040);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5630);
					match s {
						[] => true,
						['c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9043);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9045);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5632);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5633);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5634);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5635);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9047);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9048);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9049);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 15042);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 15043);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 16825);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 16826);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 16827);
													s.is_empty()
												}
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
							super::push(out, 9050);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12414);
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 12415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5636);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5637);
					match s {
						[] => true,
						['m', 'y', 't', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5638);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9054);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'd', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5639);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5640);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 12416);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5641);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2820);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5643);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5644);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5645);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5646);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9055);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5647);
					match s {
						[] => true,
						['u', 'c', 'e', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5648);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5649);
					match s {
						[] => true,
						['o', 'l', 's', 'o', 'm', 'e', 'h', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2821);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2822);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5650);
					s.is_empty()
				}
				['p', 'p', 'i', 'l', 'i', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5651);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9059);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2823);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2824);
			s.is_empty()
		}
		['o', s @ ..] => super::t7::n3328(s, out),
		['r', s @ ..] => {
			super::push(out, 2826);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5665);
					match s {
						[] => true,
						['f', 'i', 'l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2827);
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

/// Words ending -oicus.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1010);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2828);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2829);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2830);
			s.is_empty()
		}
		['z', 'o', 's', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -picus.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1011);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5666);
					match s {
						[] => true,
						['s', 'o', 'r', 'o', s @ ..] => {
							super::push(out, 9085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5667);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9086);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9087);
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
				['s', s @ ..] => {
					super::push(out, 5668);
					s.is_empty()
				}
				['t', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5669);
					s.is_empty()
				}
				['y', 'l', 'b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2833);
			match s {
				[] => true,
				['t', s @ ..] => {
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

/// Words ending -ricus.
pub(crate) fn n1129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1012);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2834);
			match s {
				[] => true,
				['c', s @ ..] => {
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
				['h', s @ ..] => {
					super::push(out, 5671);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 9088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9089);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9090);
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
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2835);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5673);
					match s {
						[] => true,
						['i', s @ ..] => {
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
				['u', s @ ..] => {
					super::push(out, 5674);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9091);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9092);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2836);
			match s {
				[] => true,
				['n', 'i', 'l', 'y', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9094);
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
		['e', s @ ..] => {
			super::push(out, 2837);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5675);
					match s {
						[] => true,
						['h', 'p', 's', s @ ..] => {
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
										['m', s @ ..] => {
											super::push(out, 15044);
											s.is_empty()
										}
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
					super::push(out, 5676);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 12423);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 12424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5677);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5678);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9095);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5679);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9097);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9098);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5680);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5681);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5682);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9100);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 12425);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 12426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 9101);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12427);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9102);
							match s {
								[] => true,
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 12429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2838);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2839);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2840);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5683);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5684);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5685);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5686);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5687);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9106);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2841);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2842);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5688);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9108);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5689);
					match s {
						[] => true,
						['e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5690);
					match s {
						[] => true,
						['m', s @ ..] => super::t7::n13993(s, out),
						['t', s @ ..] => {
							super::push(out, 9111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t7::n7782(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t7::n3346(s, out),
		['y', s @ ..] => super::t7::n3347(s, out),
		_ => false,
	}
}

/// Words ending -sicus.
pub(crate) fn n1130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1013);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2845);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2846);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5696);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2847);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2848);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5698);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2849);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 5700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 2850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ticus.
pub(crate) fn n1131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1014);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2851);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5701);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5702);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9115);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5703);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5704);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9117);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 15047);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 15048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9118);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 15049);
											match s {
												[] => true,
												['r', 'f', 'a', '\u{1}', s @ ..] => {
													super::push(out, 16828);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 15050);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 16829);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 16830);
													s.is_empty()
												}
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
							super::push(out, 9119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'l', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5705);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9120);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9121);
							match s {
								[] => true,
								['i', 'v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 15051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9122);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9123);
							s.is_empty()
						}
						['r', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 9124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5706);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9125);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 12434);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 12435);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 16831);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 16832);
													match s {
														[] => true,
														['m', '\u{1}', s @ ..] => super::t7::n35194(s, out),
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
									super::push(out, 12436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 9126);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 12437);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 12438);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 15052);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 15053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 12439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 9127);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9128);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9129);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9130);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12440);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15054);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 15055);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9131);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9132);
							match s {
								[] => true,
								['i', 'r', 'p', s @ ..] => {
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
							super::push(out, 9133);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 9134);
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
				['n', s @ ..] => super::t7::n7800(s, out),
				['o', s @ ..] => super::t7::n7801(s, out),
				['p', s @ ..] => super::t7::n7802(s, out),
				['r', s @ ..] => super::t7::n7803(s, out),
				['t', s @ ..] => super::t7::n7804(s, out),
				['u', s @ ..] => super::t7::n7805(s, out),
				['v', s @ ..] => super::t7::n7806(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t7::n3355(s, out),
		['e', s @ ..] => super::t7::n3356(s, out),
		['i', s @ ..] => super::t7::n3357(s, out),
		['l', s @ ..] => super::t7::n3358(s, out),
		['n', s @ ..] => super::t7::n3359(s, out),
		['o', s @ ..] => super::t7::n3360(s, out),
		['p', s @ ..] => super::t7::n3361(s, out),
		['s', s @ ..] => super::t7::n3362(s, out),
		['t', s @ ..] => super::t7::n3363(s, out),
		['u', s @ ..] => super::t7::n3364(s, out),
		['y', s @ ..] => super::t7::n3365(s, out),
		_ => false,
	}
}

/// Words ending -vicus.
pub(crate) fn n1132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1015);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2863);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2864);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2865);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					super::push(out, 5758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xicus.
pub(crate) fn n1133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1016);
	s.is_empty()
}

/// Words ending -andus.
pub(crate) fn n1162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2910);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5798);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9212);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12468);
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
				['l', 'a', s @ ..] => {
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
				['m', 'u', 'l', 'p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 5799);
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
					super::push(out, 5800);
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
									super::push(out, 12469);
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
									super::push(out, 12470);
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
				['u', s @ ..] => {
					super::push(out, 5801);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 9214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t7::n3430(s, out),
		['d', s @ ..] => super::t7::n3431(s, out),
		['e', s @ ..] => {
			super::push(out, 2913);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5821);
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
				['m', s @ ..] => {
					super::push(out, 5822);
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
							super::push(out, 9267);
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
					super::push(out, 5823);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 12532);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 15124);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', 'o', 'c', '\u{1}', s @ ..] => super::t7::n26837(s, out),
										_ => false,
									}
								}
								['t', s @ ..] => super::t7::n20734(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t7::n14232(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t7::n7959(s, out),
				['u', s @ ..] => super::t7::n7960(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t7::n3433(s, out),
		['g', s @ ..] => super::t7::n3434(s, out),
		['h', s @ ..] => super::t7::n3435(s, out),
		['i', s @ ..] => super::t7::n3436(s, out),
		['l', s @ ..] => super::t7::n3437(s, out),
		['m', s @ ..] => super::t7::n3438(s, out),
		['n', s @ ..] => super::t7::n3439(s, out),
		['o', s @ ..] => super::t7::n3440(s, out),
		['p', s @ ..] => super::t7::n3441(s, out),
		['r', s @ ..] => super::t7::n3442(s, out),
		['s', s @ ..] => super::t7::n3443(s, out),
		['t', s @ ..] => super::t7::n3444(s, out),
		['u', s @ ..] => super::t7::n3445(s, out),
		['v', s @ ..] => super::t7::n3446(s, out),
		['x', s @ ..] => super::t7::n3447(s, out),
		['z', s @ ..] => super::t7::n3448(s, out),
		_ => false,
	}
}

/// Words ending -endus.
pub(crate) fn n1163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1044);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2930);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5931);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9593);
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
						['l', s @ ..] => {
							super::push(out, 9594);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12896);
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
								['i', s @ ..] => {
									super::push(out, 12897);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 12898);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 12899);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12900);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5932);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9595);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 9596);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5933);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9598);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 12902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 12903);
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
							super::push(out, 9599);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12904);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15350);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t7::n14634(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5934);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9601);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 15352);
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
						['u', s @ ..] => super::t7::n14636(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t7::n8079(s, out),
				['u', s @ ..] => super::t7::n8080(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t7::n3450(s, out),
		['d', s @ ..] => super::t7::n3451(s, out),
		['g', s @ ..] => super::t7::n3452(s, out),
		['h', s @ ..] => super::t7::n3453(s, out),
		['i', s @ ..] => super::t7::n3454(s, out),
		['l', s @ ..] => super::t7::n3455(s, out),
		['m', s @ ..] => super::t8::n3456(s, out),
		['n', s @ ..] => super::t8::n3457(s, out),
		['p', s @ ..] => super::t8::n3458(s, out),
		['r', s @ ..] => super::t8::n3459(s, out),
		['s', s @ ..] => super::t8::n3460(s, out),
		['t', s @ ..] => super::t8::n3461(s, out),
		['u', s @ ..] => super::t8::n3462(s, out),
		['v', s @ ..] => super::t8::n3463(s, out),
		['x', s @ ..] => super::t8::n3464(s, out),
		_ => false,
	}
}

/// Words ending -undus.
pub(crate) fn n1165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2946);
			match s {
				[] => true,
				['a', s @ ..] => super::t8::n8184(s, out),
				['e', s @ ..] => {
					super::push(out, 6034);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6035);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9790);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 13235);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 13236);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 15628);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 9791);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9792);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9793);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 13237);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 17114);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t8::n35600(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 17115);
													match s {
														[] => true,
														['a', s @ ..] => super::t8::n35601(s, out),
														['i', s @ ..] => super::t8::n35602(s, out),
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
									super::push(out, 13238);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13239);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 15629);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 15630);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 13240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9794);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 15631);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 15632);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 9795);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 13241);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 13242);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t8::n3466(s, out),
		['d', s @ ..] => super::t8::n3467(s, out),
		['e', s @ ..] => super::t8::n3468(s, out),
		['f', s @ ..] => super::t8::n3469(s, out),
		['g', s @ ..] => super::t8::n3470(s, out),
		['i', s @ ..] => super::t8::n3471(s, out),
		['l', s @ ..] => super::t8::n3472(s, out),
		['m', s @ ..] => super::t8::n3473(s, out),
		['n', s @ ..] => super::t8::n3474(s, out),
		['p', s @ ..] => super::t8::n3475(s, out),
		['r', s @ ..] => super::t8::n3476(s, out),
		['s', s @ ..] => super::t8::n3477(s, out),
		['t', s @ ..] => super::t8::n3478(s, out),
		_ => false,
	}
}

/// Words ending -dineus.
pub(crate) fn n3575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3043);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 6137);
			match s {
				[] => true,
				['u', 'r', 'a', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 6138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gineus.
pub(crate) fn n3576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3044);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6139);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6140);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6141);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lineus.
pub(crate) fn n3577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3045);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6143);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6144);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9847);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9848);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mineus.
pub(crate) fn n3578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3046);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6145);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6146);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9849);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9850);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6147);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6148);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pineus.
pub(crate) fn n3579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3047);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6150);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6151);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6152);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6153);
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

/// Words ending -uineus.
pub(crate) fn n3581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3049);
	match s {
		[] => true,
		['g', 'n', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13270);
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

/// Words ending -xineus.
pub(crate) fn n3583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3051);
	s.is_empty()
}

/// Words ending -lneus.
pub(crate) fn n1232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1112);
	s.is_empty()
}

/// Words ending -nneus.
pub(crate) fn n1233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1113);
	s.is_empty()
}

/// Words ending -oneus.
pub(crate) fn n1234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1114);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3052);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3053);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6154);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3054);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3055);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6156);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6157);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6158);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6159);
					match s {
						[] => true,
						['l', 'u', s @ ..] => {
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
		['t', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6160);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rneus.
pub(crate) fn n1235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1115);
	s.is_empty()
}

/// Words ending -uneus.
pub(crate) fn n1236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1116);
	s.is_empty()
}

/// Words ending -arius.
pub(crate) fn n1338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1206);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3169);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6246);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3170);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6247);
					match s {
						[] => true,
						['b', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 9891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6248);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6249);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9892);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 9893);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9894);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9895);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9896);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9897);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 9898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6250);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6251);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3171);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6252);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 9900);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'd', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6253);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3172);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6254);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6255);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9903);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6256);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9905);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6257);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6258);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3173);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3174);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6260);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6261);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3175);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3176);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6263);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9907);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 13296);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 13297);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 13298);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 17119);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 17120);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 13299);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 13300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 9908);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9909);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 13301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6264);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6265);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6266);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6267);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9910);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9911);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 13302);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 13303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 9912);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6268);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 9914);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9915);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6269);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6270);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9917);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9918);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6271);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6272);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9921);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9922);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9923);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9924);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 13304);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6273);
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
		['l', s @ ..] => super::t8::n3756(s, out),
		['m', s @ ..] => {
			super::push(out, 3178);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6278);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6279);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9938);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6280);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t8::n3758(s, out),
		['o', s @ ..] => super::t8::n3759(s, out),
		['p', s @ ..] => super::t8::n3760(s, out),
		['r', s @ ..] => super::t8::n3761(s, out),
		['s', s @ ..] => super::t8::n3762(s, out),
		['t', s @ ..] => super::t8::n3763(s, out),
		['u', s @ ..] => super::t8::n3764(s, out),
		['v', s @ ..] => super::t8::n3765(s, out),
		_ => false,
	}
}

/// Words ending -sorius.
pub(crate) fn n3781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3202);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6320);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6321);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10015);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10016);
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
		['n', s @ ..] => {
			super::push(out, 6322);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10017);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10018);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 15667);
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
				['o', s @ ..] => {
					super::push(out, 10019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6323);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6324);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10021);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10022);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10023);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -torius.
pub(crate) fn n3782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3203);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6326);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10025);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13350);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13351);
							match s {
								[] => true,
								['r', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17123);
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
							super::push(out, 13352);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15668);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 17124);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 17790);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 15669);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 17791);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 17792);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 17793);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13353);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17125);
											match s {
												[] => true,
												['r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17126);
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
						['r', s @ ..] => {
							super::push(out, 13354);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10026);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10027);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 10028);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13356);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 15671);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15672);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13357);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 17127);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
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
										['r', s @ ..] => {
											super::push(out, 17128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13358);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10029);
					s.is_empty()
				}
				['i', s @ ..] => super::t8::n15248(s, out),
				['l', s @ ..] => super::t8::n15249(s, out),
				['m', s @ ..] => super::t8::n15250(s, out),
				['n', s @ ..] => super::t8::n15251(s, out),
				['r', s @ ..] => super::t8::n15252(s, out),
				['s', s @ ..] => super::t8::n15253(s, out),
				['t', s @ ..] => super::t8::n15254(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t8::n8546(s, out),
		['e', s @ ..] => super::t8::n8547(s, out),
		['i', s @ ..] => super::t8::n8548(s, out),
		['l', s @ ..] => super::t8::n8549(s, out),
		['o', s @ ..] => super::t8::n8550(s, out),
		['p', s @ ..] => super::t8::n8551(s, out),
		['s', s @ ..] => super::t8::n8552(s, out),
		['u', s @ ..] => super::t8::n8553(s, out),
		['x', s @ ..] => super::t8::n8554(s, out),
		_ => false,
	}
}

/// Words ending -atrius.
pub(crate) fn n3785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3205);
	s.is_empty()
}

/// Words ending -strius.
pub(crate) fn n3786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3206);
	s.is_empty()
}

/// Words ending -utrius.
pub(crate) fn n3787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3207);
	s.is_empty()
}

/// Words ending -urius.
pub(crate) fn n1345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1213);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3208);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3209);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrius.
pub(crate) fn n1346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1214);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6336);
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
		['s', s @ ..] => {
			super::push(out, 3212);
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

/// The word deridiculus.
pub(crate) fn n33192(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ridulus.
pub(crate) fn n8648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6401);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10097);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10098);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10099);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ndulus.
pub(crate) fn n3875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3276);
	match s {
		[] => true,
		['e', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10101);
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

/// Words ending -udulus.
pub(crate) fn n3876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3277);
	s.is_empty()
}

/// Words ending -fulus.
pub(crate) fn n1404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1272);
	s.is_empty()
}

/// Words ending -gulus.
pub(crate) fn n1405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1273);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3278);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6403);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3279);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10102);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 13401);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 13402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10103);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10104);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 13403);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 15699);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 13404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lulus.
pub(crate) fn n1406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1274);
	match s {
		[] => true,
		['l', 'e', 'b', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 3280);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 6405);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mulus.
pub(crate) fn n1407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1275);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3281);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3282);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nulus.
pub(crate) fn n1408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1276);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3284);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pulus.
pub(crate) fn n1409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1277);
	s.is_empty()
}

/// Words ending -rulus.
pub(crate) fn n1410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1278);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6406);
					match s {
						[] => true,
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6407);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sulus.
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1279);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3286);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tulus.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3288);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6408);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6409);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6410);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6411);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10107);
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
				['u', s @ ..] => {
					super::push(out, 6412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3289);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3290);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6413);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10108);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6414);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6415);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3291);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6417);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 10110);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10111);
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6418);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10112);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 10113);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3292);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vulus.
pub(crate) fn n1413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1281);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3294);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3295);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
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

/// The word verisimillimus.
pub(crate) fn n33193(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ruberrimus.
pub(crate) fn n33194(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -deterrimus.
pub(crate) fn n28639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15706);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teterrimus.
pub(crate) fn n28640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15707);
	s.is_empty()
}

/// Words ending -veterrimus.
pub(crate) fn n28641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15708);
	s.is_empty()
}

/// Words ending -urrimus.
pub(crate) fn n8688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6420);
	s.is_empty()
}

/// Words ending -trimus.
pub(crate) fn n3913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3304);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10126);
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

/// Words ending -urimus.
pub(crate) fn n3914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3305);
	s.is_empty()
}

/// Words ending -simus.
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1289);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3306);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3307);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6421);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10127);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6422);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3308);
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6424);
					s.is_empty()
				}
				['r', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t8::n3918(s, out),
		_ => false,
	}
}

/// Words ending -timus.
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1290);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3310);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6427);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6428);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3311);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 13473);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ximus.
pub(crate) fn n1425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1291);
	s.is_empty()
}

/// Words ending -ianus.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1313);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3336);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6448);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3337);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6450);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 10157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3338);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6451);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6452);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3339);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6454);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6455);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10158);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10159);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 15816);
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
						['s', s @ ..] => {
							super::push(out, 10160);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6456);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6457);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3340);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3341);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6459);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6460);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3342);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3343);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6462);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10163);
							s.is_empty()
						}
						['t', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6463);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6464);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10165);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 10166);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6465);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6466);
					match s {
						[] => true,
						['i', 'o', s @ ..] => {
							super::push(out, 10168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3344);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6467);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 10169);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 10170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6468);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10171);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 13476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'r', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 10172);
							s.is_empty()
						}
						['l', 's', 'i', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10173);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10174);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 13477);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15817);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 15818);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 13478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6469);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 13479);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 13480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6470);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3345);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3346);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3347);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6472);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10175);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 10176);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10177);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10178);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6473);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 10180);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 10181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6474);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10182);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10183);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 13481);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 13482);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 13483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6475);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6476);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10184);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6477);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6478);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10186);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t8::n3965(s, out),
		['v', s @ ..] => {
			super::push(out, 3350);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6487);
					match s {
						[] => true,
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 3351);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 6488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lanus.
pub(crate) fn n1448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1314);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3353);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6489);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3354);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6491);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 10191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3355);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6492);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6493);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3356);
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
		['u', s @ ..] => {
			super::push(out, 3357);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6495);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6496);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6497);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6498);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6499);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -manus.
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1315);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3358);
			match s {
				[] => true,
				['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3359);
			match s {
				[] => true,
				['b', 'l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6501);
					s.is_empty()
				}
				['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6502);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6503);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10192);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6504);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6505);
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 10194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3360);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3361);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3362);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6506);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6507);
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

/// Words ending -nanus.
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1316);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3363);
			match s {
				[] => true,
				['h', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3364);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -panus.
pub(crate) fn n1452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1318);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ranus.
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1319);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3368);
			match s {
				[] => true,
				['p', 'm', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 6508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3369);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sanus.
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1320);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3370);
			match s {
				[] => true,
				['c', 'e', 'o', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6509);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6510);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6511);
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
			super::push(out, 3371);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3372);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 6512);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tanus.
pub(crate) fn n1455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3373);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3374);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6514);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6515);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 13484);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 15821);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 15822);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 13485);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13486);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 15823);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 15824);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 17259);
													match s {
														[] => true,
														['i', s @ ..] => super::t8::n35701(s, out),
														_ => false,
													}
												}
												['i', s @ ..] => {
													super::push(out, 17260);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 15825);
											s.is_empty()
										}
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
					super::push(out, 6516);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10195);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 10196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6517);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6518);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6519);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10197);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 13487);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 13488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 10198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6520);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3375);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 10199);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13489);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 13490);
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
		['r', s @ ..] => {
			super::push(out, 3376);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 10200);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3377);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uanus.
pub(crate) fn n1456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1322);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3378);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10202);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 10203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3379);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3380);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3381);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3382);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3383);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vanus.
pub(crate) fn n1457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1323);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yanus.
pub(crate) fn n1458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1324);
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

/// Words ending -zanus.
pub(crate) fn n1459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1325);
	s.is_empty()
}

/// Words ending -elinus.
pub(crate) fn n4056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3430);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6569);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6570);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6571);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6572);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 6573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lilinus.
pub(crate) fn n8884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6575);
	s.is_empty()
}

/// Words ending -uilinus.
pub(crate) fn n8885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6576);
	s.is_empty()
}

/// Words ending -llinus.
pub(crate) fn n4059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3433);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6577);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10215);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 't', 's', s @ ..] => {
			super::push(out, 6578);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olinus.
pub(crate) fn n4060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3434);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 6579);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulinus.
pub(crate) fn n4061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3435);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6581);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10218);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6582);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10220);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6583);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -minus.
pub(crate) fn n1485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3436);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3437);
			match s {
				[] => true,
				['g', s @ ..] => {
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
						['r', 'e', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3438);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3439);
			match s {
				[] => true,
				['a', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ninus.
pub(crate) fn n1486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3441);
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
		['e', s @ ..] => {
			super::push(out, 3442);
			match s {
				[] => true,
				['b', 'e', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 3443);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3444);
			match s {
				[] => true,
				['m', 'e', 'f', s @ ..] => {
					super::push(out, 6586);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3445);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3446);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6588);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6589);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6590);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pinus.
pub(crate) fn n1487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1353);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3447);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', s @ ..] => {
							super::push(out, 10222);
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3448);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10223);
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

/// Words ending -rinus.
pub(crate) fn n1488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3449);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6591);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10224);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 10225);
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
					super::push(out, 6593);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6594);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3450);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3451);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3452);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6596);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3453);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6598);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6599);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6600);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6601);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6602);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6603);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6604);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6605);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10226);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6606);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6607);
					s.is_empty()
				}
				['v', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3454);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6609);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6610);
					match s {
						[] => true,
						['q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3456);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3457);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3458);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6612);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3459);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6614);
					match s {
						[] => true,
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 10228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6615);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3460);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6617);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sinus.
pub(crate) fn n1489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 3461);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6620);
					s.is_empty()
				}
				['i', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6621);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tinus.
pub(crate) fn n1490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3462);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6622);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6623);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6624);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10229);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 10230);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 10231);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 10232);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6625);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6626);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3463);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3464);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6628);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6629);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10233);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6630);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6631);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3465);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3466);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3467);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6633);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 10235);
							match s {
								[] => true,
								['p', 'e', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 13491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 10236);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 10237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6634);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10238);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 13492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6635);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6636);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10240);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10241);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 10242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3468);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6637);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6638);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 13493);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 13494);
									s.is_empty()
								}
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
			super::push(out, 3469);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6639);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10243);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3470);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6641);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6642);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6643);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3471);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6645);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3472);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6647);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6648);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6649);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uinus.
pub(crate) fn n1491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1357);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3473);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6650);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6651);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6652);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vinus.
pub(crate) fn n1492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1358);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3474);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3475);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3476);
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
		['r', s @ ..] => {
			super::push(out, 3477);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xinus.
pub(crate) fn n1493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1359);
	s.is_empty()
}

/// Words ending -yinus.
pub(crate) fn n1494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1360);
	s.is_empty()
}

/// Words ending -surus.
pub(crate) fn n1568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1427);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3524);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'o', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 6702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10273);
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
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6703);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
					super::push(out, 6704);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6705);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6706);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
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
								_ => false,
							}
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
					super::push(out, 6707);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6708);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6710);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6711);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6712);
					match s {
						[] => true,
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 6713);
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
		['i', s @ ..] => {
			super::push(out, 3526);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6714);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 10275);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10276);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 13506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10277);
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
				['l', s @ ..] => {
					super::push(out, 6716);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6717);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6718);
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
							super::push(out, 10280);
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
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
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
				['v', s @ ..] => super::t8::n9074(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t8::n4194(s, out),
		['n', s @ ..] => super::t8::n4195(s, out),
		['o', s @ ..] => super::t8::n4196(s, out),
		['p', s @ ..] => super::t8::n4197(s, out),
		['r', s @ ..] => super::t8::n4198(s, out),
		['s', s @ ..] => super::t8::n4199(s, out),
		['u', s @ ..] => super::t8::n4200(s, out),
		_ => false,
	}
}

/// Words ending -turus.
pub(crate) fn n1569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1428);
	match s {
		[] => true,
		['a', s @ ..] => super::t8::n4201(s, out),
		['c', s @ ..] => super::t8::n4202(s, out),
		['e', s @ ..] => {
			super::push(out, 3536);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6768);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
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
					super::push(out, 6769);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10518);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 10519);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14051);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 10520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14052);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 16399);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 16400);
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
								['p', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 6770);
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
									super::push(out, 14053);
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
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6771);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14054);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 14055);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 16401);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 16402);
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
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t8::n4204(s, out),
		['l', s @ ..] => super::t8::n4205(s, out),
		['m', s @ ..] => {
			super::push(out, 3539);
			s.is_empty()
		}
		['n', s @ ..] => super::t8::n4207(s, out),
		['o', s @ ..] => super::t8::n4208(s, out),
		['p', s @ ..] => super::t8::n4209(s, out),
		['r', s @ ..] => super::t8::n4210(s, out),
		['s', s @ ..] => super::t8::n4211(s, out),
		['u', s @ ..] => super::t8::n4212(s, out),
		['x', s @ ..] => super::t8::n4213(s, out),
		_ => false,
	}
}

/// The word deflexurus.
pub(crate) fn n34758(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word reflexurus.
pub(crate) fn n34759(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ixurus.
pub(crate) fn n4215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3548);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10662);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14229);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14230);
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
					super::push(out, 10663);
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
				['i', 'c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10664);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14232);
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
				['s', s @ ..] => {
					super::push(out, 10665);
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
			super::push(out, 6828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10667);
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
		_ => false,
	}
}

/// Words ending -uxurus.
pub(crate) fn n4216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3549);
	match s {
		[] => true,
		['l', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10668);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14233);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14234);
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
					super::push(out, 10669);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10670);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10671);
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

/// Words ending -ffensus.
pub(crate) fn n9276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6845);
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hensus.
pub(crate) fn n4280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3580);
	match s {
		[] => true,
		['e', 'r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10682);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14240);
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
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -mensus.
pub(crate) fn n4281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3581);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 10683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10684);
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
			super::push(out, 6848);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6849);
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
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pensus.
pub(crate) fn n4282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3582);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6850);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 6851);
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
		['s', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rensus.
pub(crate) fn n4283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3583);
	match s {
		[] => true,
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10685);
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
							super::push(out, 14241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -sensus.
pub(crate) fn n4284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3584);
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
		['n', s @ ..] => {
			super::push(out, 6852);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
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

/// Words ending -tensus.
pub(crate) fn n4285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3585);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 6853);
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
		['s', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onsus.
pub(crate) fn n1598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1456);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3586);
			match s {
				[] => true,
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3587);
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
									super::push(out, 14242);
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3588);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6855);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10687);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 10688);
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
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unsus.
pub(crate) fn n1599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1457);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -iosus.
pub(crate) fn n1607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1464);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3605);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3606);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6875);
					match s {
						[] => true,
						['d', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
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
						['l', s @ ..] => {
							super::push(out, 10698);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 14246);
									s.is_empty()
								}
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
			super::push(out, 3607);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6876);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 10699);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10700);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14247);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10701);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 10702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6877);
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
				['u', s @ ..] => {
					super::push(out, 6878);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 10703);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3608);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6879);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10705);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10706);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6880);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10708);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 10709);
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
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6881);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3609);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6883);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6884);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6885);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10711);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 10712);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3610);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6887);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6888);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10714);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6889);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6890);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 14249);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6891);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3611);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3612);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6893);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6894);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 6895);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10719);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10720);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10721);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6896);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6897);
					match s {
						[] => true,
						['l', 'g', s @ ..] => {
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
						['m', s @ ..] => {
							super::push(out, 10723);
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 14251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6898);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6899);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => super::t8::n23955(s, out),
								_ => false,
							}
						}
						['f', s @ ..] => super::t8::n16287(s, out),
						['i', s @ ..] => super::t8::n16288(s, out),
						['x', s @ ..] => super::t8::n16289(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t8::n4318(s, out),
		['t', s @ ..] => super::t8::n4319(s, out),
		['u', s @ ..] => super::t8::n4320(s, out),
		['v', s @ ..] => super::t8::n4321(s, out),
		_ => false,
	}
}

/// Words ending -losus.
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1465);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3617);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 6909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3618);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6910);
					match s {
						[] => true,
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10733);
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
			super::push(out, 3619);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6911);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 14252);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 16535);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 't', s @ ..] => {
					super::push(out, 6912);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3620);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10734);
							match s {
								[] => true,
								['m', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 14253);
									s.is_empty()
								}
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
			super::push(out, 3621);
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
				['p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3622);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6913);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 14254);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 14255);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 14256);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14257);
									match s {
										[] => true,
										['e', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 16536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 10735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10736);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 10737);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10738);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 14258);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 14259);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14260);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14261);
									match s {
										[] => true,
										['b', 'e', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 14262);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 16537);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 14263);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 16538);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 16539);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 10739);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10740);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10741);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14264);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14265);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6915);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10743);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10744);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6916);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10746);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6918);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10747);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6919);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6920);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10750);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10751);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6921);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6922);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10753);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omosus.
pub(crate) fn n4333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3628);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmosus.
pub(crate) fn n4334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3629);
	s.is_empty()
}

/// Words ending -umosus.
pub(crate) fn n4335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3630);
	s.is_empty()
}

/// Words ending -ymosus.
pub(crate) fn n4336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3631);
	s.is_empty()
}

/// Words ending -nosus.
pub(crate) fn n1610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1467);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3632);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3633);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6927);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6928);
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
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3634);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3635);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6929);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6930);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10759);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10760);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6931);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10762);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10763);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14268);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 14269);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 16540);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 16541);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 14270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10764);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14271);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14272);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6932);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6933);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10765);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10766);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14274);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 14275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10767);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10768);
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
						['r', s @ ..] => {
							super::push(out, 10769);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6934);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10771);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10772);
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
											super::push(out, 16542);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													super::push(out, 17649);
													s.is_empty()
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
										['t', 'l', s @ ..] => {
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'm', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 6935);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6936);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6937);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6938);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 10773);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10774);
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
					super::push(out, 6939);
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
			super::push(out, 3636);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3637);
			match s {
				[] => true,
				['a', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 3638);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -posus.
pub(crate) fn n1611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1468);
	s.is_empty()
}

/// Words ending -rosus.
pub(crate) fn n1612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1469);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3640);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', s @ ..] => {
					super::push(out, 6940);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 6941);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 10775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'u', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6942);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6943);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10776);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10777);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10778);
							match s {
								[] => true,
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 10779);
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
				['i', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'e', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6946);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3642);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3643);
			match s {
				[] => true,
				['c', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6949);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6950);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10780);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6951);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 14276);
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
				['l', s @ ..] => {
					super::push(out, 6952);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10782);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10783);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10784);
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
				['n', s @ ..] => {
					super::push(out, 6954);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10785);
							s.is_empty()
						}
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 10786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6955);
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
			super::push(out, 3644);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3645);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10787);
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 14277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tosus.
pub(crate) fn n1613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1470);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3646);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3647);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3648);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3649);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3650);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 10788);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10789);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14278);
									match s {
										[] => true,
										['l', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 16543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 14279);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14280);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 14281);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 10790);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 10791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3651);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3652);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uosus.
pub(crate) fn n1614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1471);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3654);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3655);
			match s {
				[] => true,
				['e', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 6959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 3656);
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
		['r', s @ ..] => {
			super::push(out, 3657);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3658);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6960);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14283);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14284);
									match s {
										[] => true,
										['f', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 16544);
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
						['e', s @ ..] => {
							super::push(out, 10793);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10794);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10795);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14285);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16545);
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
				['i', s @ ..] => {
					super::push(out, 6961);
					match s {
						[] => true,
						['r', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 10796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6962);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6963);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10797);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16546);
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
							super::push(out, 10798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6964);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
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
					super::push(out, 6965);
					match s {
						[] => true,
						['e', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3659);
			match s {
				[] => true,
				['e', 'l', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10799);
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

/// Words ending -vosus.
pub(crate) fn n1615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1472);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3660);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6966);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3661);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xosus.
pub(crate) fn n1616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1473);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rversus.
pub(crate) fn n9500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6981);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
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
		_ => false,
	}
}

/// Words ending -orsus.
pub(crate) fn n1620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1476);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3673);
			s.is_empty()
		}
		['b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3675);
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
				['m', s @ ..] => {
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
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3676);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 3678);
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

/// Words ending -ursus.
pub(crate) fn n1621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1477);
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
					super::push(out, 6984);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 10811);
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
					super::push(out, 6985);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -messus.
pub(crate) fn n4409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3682);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6995);
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

/// Words ending -ressus.
pub(crate) fn n4411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3683);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 6996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14286);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14287);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14288);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14290);
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
				['g', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10815);
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
					super::push(out, 10816);
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
					super::push(out, 10817);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
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
				['r', s @ ..] => {
					super::push(out, 10818);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 10819);
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
		['p', s @ ..] => {
			super::push(out, 6997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10820);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14291);
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
							super::push(out, 14292);
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
				['p', s @ ..] => {
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sessus.
pub(crate) fn n4412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3684);
	match s {
		[] => true,
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
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6998);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -issus.
pub(crate) fn n1624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1480);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3685);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10821);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 10822);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3687);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6999);
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
				['b', s @ ..] => {
					super::push(out, 7000);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7001);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 10823);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 10824);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10825);
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
					super::push(out, 7003);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7004);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 7005);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10826);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10827);
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
										['m', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['p', 'a', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 7007);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
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
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7008);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7009);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ossus.
pub(crate) fn n1625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1481);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 3688);
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
					super::push(out, 7010);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10828);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 10829);
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
					super::push(out, 7011);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7012);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10830);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 10831);
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
					super::push(out, 7013);
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
				['s', s @ ..] => {
					super::push(out, 7014);
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
		['l', s @ ..] => {
			super::push(out, 3689);
			match s {
				[] => true,
				['g', 'o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10832);
							match s {
								[] => true,
								['n', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 14293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10833);
							match s {
								[] => true,
								['a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14294);
									s.is_empty()
								}
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

/// Words ending -ussus.
pub(crate) fn n1626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1482);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3690);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7015);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10834);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10835);
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
				['r', 'e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 3691);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7016);
					match s {
						[] => true,
						['d', 'i', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7017);
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
		['t', s @ ..] => {
			super::push(out, 3692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etrusus.
pub(crate) fn n9609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7023);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10840);
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

/// The word abstrusus.
pub(crate) fn n9612(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word extrusus.
pub(crate) fn n9613(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -tusus.
pub(crate) fn n1637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1488);
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', '\u{1}', s @ ..] => {
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

/// The word masturbatus.
pub(crate) fn n24101(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word disturbatus.
pub(crate) fn n24102(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word exturbatus.
pub(crate) fn n16572(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ubatus.
pub(crate) fn n4457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3710);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7028);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10846);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10847);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -catus.
pub(crate) fn n1639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1490);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3711);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3712);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7030);
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
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7031);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10848);
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
				['v', s @ ..] => {
					super::push(out, 7032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3713);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3714);
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
							super::push(out, 10849);
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
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3715);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7033);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10850);
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
							super::push(out, 10851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7034);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10852);
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
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7035);
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
									super::push(out, 14295);
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
				_ => false,
			}
		}
		['i', s @ ..] => super::t8::n4463(s, out),
		['l', s @ ..] => {
			super::push(out, 3717);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
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
						['f', '\u{1}', s @ ..] => {
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
							super::push(out, 10888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14346);
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
		['n', s @ ..] => {
			super::push(out, 3718);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10890);
							match s {
								[] => true,
								['e', 'v', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10891);
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
		['o', s @ ..] => super::t8::n4466(s, out),
		['r', s @ ..] => {
			super::push(out, 3720);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7052);
					match s {
						[] => true,
						['m', s @ ..] => super::t8::n16652(s, out),
						['t', 'l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t8::n9658(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t8::n4468(s, out),
		['u', s @ ..] => super::t8::n4469(s, out),
		_ => false,
	}
}

/// Words ending -datus.
pub(crate) fn n1640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1491);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3723);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3724);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3725);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7060);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14351);
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
					super::push(out, 7061);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7062);
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
				['r', s @ ..] => {
					super::push(out, 7063);
					match s {
						[] => true,
						['e', 'h', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10906);
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
			super::push(out, 3726);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7065);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14352);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14353);
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
				['d', s @ ..] => {
					super::push(out, 7066);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7067);
					match s {
						[] => true,
						['i', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7068);
					match s {
						[] => true,
						['o', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14354);
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
					super::push(out, 7069);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10907);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10908);
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
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7070);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'u', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7071);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7072);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7073);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3727);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10909);
							match s {
								[] => true,
								['r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 10910);
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
			super::push(out, 3728);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7074);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 14355);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 14356);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 16588);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => super::t8::n34806(s, out),
												_ => false,
											}
										}
										['d', s @ ..] => super::t8::n30766(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => super::t8::n16684(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t8::n9683(s, out),
				['u', s @ ..] => super::t8::n9684(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t8::n4476(s, out),
		['r', s @ ..] => super::t8::n4477(s, out),
		['u', s @ ..] => super::t8::n4478(s, out),
		['y', s @ ..] => super::t8::n4479(s, out),
		_ => false,
	}
}

/// Words ending -eatus.
pub(crate) fn n1641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1492);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3733);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3734);
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
			super::push(out, 3735);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7084);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7085);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14365);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7086);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7087);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7088);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 10922);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 10923);
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
				['u', s @ ..] => {
					super::push(out, 7089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 7090);
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
			super::push(out, 3737);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 14366);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14367);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14368);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 16599);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', '\u{1}', s @ ..] => {
													super::push(out, 17660);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14369);
									match s {
										[] => true,
										['b', 'l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['b', 'u', 'r', '\u{1}', s @ ..] => {
													super::push(out, 17661);
													s.is_empty()
												}
												['g', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 16600);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3738);
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
							super::push(out, 10926);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14370);
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
						['s', s @ ..] => {
							super::push(out, 10927);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14371);
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
				['h', s @ ..] => {
					super::push(out, 7092);
					match s {
						[] => true,
						['c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3739);
			match s {
				[] => true,
				['i', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3740);
			s.is_empty()
		}
		['u', s @ ..] => super::t8::n4488(s, out),
		_ => false,
	}
}

/// Words ending -fatus.
pub(crate) fn n1642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1493);
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
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gatus.
pub(crate) fn n1643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1494);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3742);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7094);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7095);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7096);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7097);
					match s {
						[] => true,
						['f', 'u', s @ ..] => {
							super::push(out, 10929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7098);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10931);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10932);
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
			super::push(out, 3743);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7100);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7101);
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
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14372);
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
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10933);
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
					super::push(out, 7103);
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
									super::push(out, 14373);
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
			super::push(out, 3744);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7104);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 10934);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 10935);
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
							super::push(out, 10936);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14374);
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
						['f', s @ ..] => super::t8::n16760(s, out),
						['l', s @ ..] => super::t8::n16761(s, out),
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => super::t8::n16763(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t8::n9730(s, out),
				['r', s @ ..] => super::t8::n9731(s, out),
				['t', s @ ..] => super::t8::n9732(s, out),
				['v', s @ ..] => super::t8::n9733(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t8::n4498(s, out),
		['n', s @ ..] => super::t8::n4499(s, out),
		['o', s @ ..] => super::t8::n4500(s, out),
		['r', s @ ..] => super::t8::n4501(s, out),
		['u', s @ ..] => super::t8::n4502(s, out),
		_ => false,
	}
}

/// Words ending -hatus.
pub(crate) fn n1644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1495);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3750);
			match s {
				[] => true,
				['c', 'a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10962);
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
		['p', s @ ..] => {
			super::push(out, 3751);
			match s {
				[] => true,
				['m', 'e', s @ ..] => {
					super::push(out, 7118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iatus.
pub(crate) fn n1645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1496);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3752);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7119);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10963);
							match s {
								[] => true,
								['g', s @ ..] => {
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
						['m', s @ ..] => {
							super::push(out, 10964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7120);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 10965);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10966);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 10967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7121);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14391);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16604);
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7122);
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
									super::push(out, 14393);
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
				['s', s @ ..] => {
					super::push(out, 7123);
					match s {
						[] => true,
						['a', s @ ..] => {
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
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 16605);
											match s {
												[] => true,
												['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
													super::push(out, 17663);
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
												['n', s @ ..] => {
													super::push(out, 17664);
													match s {
														[] => true,
														['u', '\u{1}', s @ ..] => super::t8::n36468(s, out),
														_ => false,
													}
												}
												['r', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['b', 'l', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', s @ ..] => {
													match s {
														[] => true,
														['b', 'u', 'r', '\u{1}', s @ ..] => super::t8::n36469(s, out),
														['g', 'i', 'n', '\u{1}', s @ ..] => super::t8::n36470(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['x', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7124);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10968);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16606);
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
							super::push(out, 10969);
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
			super::push(out, 3753);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7125);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10970);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 10971);
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
				['i', s @ ..] => super::t8::n9753(s, out),
				['o', s @ ..] => {
					super::push(out, 7127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t8::n16818(s, out),
						['n', 'i', '\u{1}', s @ ..] => super::t8::n16819(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t8::n9755(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t8::n4507(s, out),
		['h', s @ ..] => super::t8::n4508(s, out),
		['l', s @ ..] => super::t8::n4509(s, out),
		['m', s @ ..] => super::t8::n4510(s, out),
		['n', s @ ..] => super::t8::n4511(s, out),
		['p', s @ ..] => super::t8::n4512(s, out),
		['r', s @ ..] => super::t8::n4513(s, out),
		['s', s @ ..] => super::t8::n4514(s, out),
		['t', s @ ..] => super::t8::n4515(s, out),
		['v', s @ ..] => super::t9::n4516(s, out),
		_ => false,
	}
}

