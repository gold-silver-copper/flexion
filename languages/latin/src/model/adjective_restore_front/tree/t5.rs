use flexion::Path;

/// Words ending -oearp.
pub(crate) fn n7447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6905);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9515);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 9516);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pearp.
pub(crate) fn n7448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6906);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9518);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9519);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 11056);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12145);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12146);
									s.is_empty()
								}
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
			super::push(out, 9520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rearp.
pub(crate) fn n7449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6907);
	s.is_empty()
}

/// Words ending -searp.
pub(crate) fn n7450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6908);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9521);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11057);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11058);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12244);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 12147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9522);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9523);
			match s {
				[] => true,
				['n', 's', s @ ..] => {
					super::push(out, 11059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12148);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9524);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11060);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12149);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 12150);
									s.is_empty()
								}
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
			super::push(out, 9525);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11061);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12151);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['o', 's', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 12153);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 12154);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11062);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tearp.
pub(crate) fn n7451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6909);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9526);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11063);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11839);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12246);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12247);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9527);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11841);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 11842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11843);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 12156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['q', s @ ..] => {
							super::push(out, 11844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uearp.
pub(crate) fn n7452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6910);
	match s {
		[] => true,
		['r', 'e', 'n', 'd', s @ ..] => {
			super::push(out, 9528);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9529);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vearp.
pub(crate) fn n7453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6911);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9530);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 11065);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9531);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ecidorp.
pub(crate) fn n10681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9564);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11857);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udorp.
pub(crate) fn n7482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6937);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', 'd', s @ ..] => {
					super::push(out, 11086);
					s.is_empty()
				}
				['i', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 11087);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11088);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eorp.
pub(crate) fn n3822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3669);
	s.is_empty()
}

/// Words ending -forp.
pub(crate) fn n3823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3670);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6938);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11089);
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11859);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11090);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11091);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11093);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6939);
			match s {
				[] => true,
				['c', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11095);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9565);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6940);
			match s {
				[] => true,
				['c', 'i', 'e', s @ ..] => {
					super::push(out, 9567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6941);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9568);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9569);
					match s {
						[] => true,
						['g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12163);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12164);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 12165);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12248);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12249);
													s.is_empty()
												}
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
					super::push(out, 9570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6942);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9571);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11097);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9572);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9573);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11099);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9574);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gorp.
pub(crate) fn n3824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3671);
	s.is_empty()
}

/// Words ending -horp.
pub(crate) fn n3825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3672);
	s.is_empty()
}

/// Words ending -iorp.
pub(crate) fn n3826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3673);
	s.is_empty()
}

/// Words ending -lorp.
pub(crate) fn n3827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3674);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6943);
			match s {
				[] => true,
				['b', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11101);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11103);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11104);
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
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12166);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12167);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11862);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11863);
									s.is_empty()
								}
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
			super::push(out, 6944);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9575);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6945);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6946);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9577);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -morp.
pub(crate) fn n3828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3675);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6947);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6948);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6949);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9579);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9580);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11864);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11865);
									s.is_empty()
								}
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
			super::push(out, 6950);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6951);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9581);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9582);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11866);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11867);
									s.is_empty()
								}
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

/// Words ending -norp.
pub(crate) fn n3829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3676);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6952);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9583);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9584);
					match s {
						[] => true,
						['t', 'i', 'a', 't', 'i', s @ ..] => {
							super::push(out, 11105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -porp.
pub(crate) fn n3830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3677);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6953);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9586);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11868);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9587);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6954);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9589);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9590);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11106);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11107);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 11870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6955);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 11871);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12168);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12169);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12170);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12171);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 11108);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11872);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 11873);
									s.is_empty()
								}
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
			super::push(out, 6956);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6957);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9592);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11874);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11875);
									s.is_empty()
								}
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
			super::push(out, 6958);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6959);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9593);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9594);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9595);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rorp.
pub(crate) fn n3831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3678);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6960);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9597);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9598);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11876);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11877);
									s.is_empty()
								}
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
			super::push(out, 6961);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sorp.
pub(crate) fn n3832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3679);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6963);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6964);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9599);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9600);
					match s {
						[] => true,
						['y', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9601);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 9602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6965);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6966);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11110);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
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
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 9603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9604);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11112);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -torp.
pub(crate) fn n3833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3680);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6969);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9605);
					match s {
						[] => true,
						['m', 'a', 'c', 'u', 'l', 'a', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 11114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9606);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11878);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9607);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9608);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11880);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11881);
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
						['d', s @ ..] => {
							super::push(out, 11115);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11116);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12172);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12173);
											s.is_empty()
										}
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
					super::push(out, 9609);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11117);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11118);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 11119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6970);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9611);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9612);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6971);
			match s {
				[] => true,
				['u', 'd', s @ ..] => {
					super::push(out, 9614);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11882);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11883);
									s.is_empty()
								}
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
			super::push(out, 6972);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9615);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vorp.
pub(crate) fn n3834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3681);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6973);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 9617);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11884);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11885);
									s.is_empty()
								}
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
			super::push(out, 6974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xorp.
pub(crate) fn n3835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3682);
	s.is_empty()
}

/// Words ending -necudortnibus.
pub(crate) fn n14304(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 12268);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utcudortnibus.
pub(crate) fn n14305(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 12270);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -snanimesrepus.
pub(crate) fn n14290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12252);
	s.is_empty()
}

/// Words ending -tanimesrepus.
pub(crate) fn n14291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12253);
	s.is_empty()
}

/// Words ending -utanalpmoc.
pub(crate) fn n13950(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 12212);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -elpmoc.
pub(crate) fn n8889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8127);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 10290);
			s.is_empty()
		}
		['x', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
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
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -ilpmoc.
pub(crate) fn n8890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8128);
	s.is_empty()
}

/// Words ending -olpmoc.
pub(crate) fn n8891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8129);
	match s {
		[] => true,
		['d', 'e', 'n', 'd', s @ ..] => {
			super::push(out, 10291);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['n', 'd', s @ ..] => {
					super::push(out, 11458);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 11459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10292);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulpmoc.
pub(crate) fn n8892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8130);
	s.is_empty()
}

/// Words ending -opmoc.
pub(crate) fn n5344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5061);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -rpmoc.
pub(crate) fn n5345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5062);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8131);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 10293);
					match s {
						[] => true,
						['e', 'n', 's', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'u', 'r', s @ ..] => {
									super::push(out, 12013);
									s.is_empty()
								}
								['i', 'b', s @ ..] => {
									super::push(out, 12014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10294);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11460);
							s.is_empty()
						}
						['s', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8132);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8133);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10296);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10297);
					s.is_empty()
				}
				['v', 'i', 'n', 'c', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 10298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tpmoc.
pub(crate) fn n5346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5063);
	s.is_empty()
}

/// Words ending -upmoc.
pub(crate) fn n5347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5064);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8134);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8135);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10299);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 10300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -psnoc.
pub(crate) fn n5412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5126);
	match s {
		[] => true,
		['i', 'r', s @ ..] => {
			super::push(out, 8251);
			match s {
				[] => true,
				['a', 't', 'u', 'r', s @ ..] => {
					super::push(out, 10367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsnoc.
pub(crate) fn n5413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5127);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8252);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10368);
					s.is_empty()
				}
				['n', 't', s @ ..] => {
					super::push(out, 10369);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							super::push(out, 11530);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12047);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 12048);
									s.is_empty()
								}
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
			super::push(out, 8253);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8254);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 10370);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12224);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12225);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12226);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12227);
											s.is_empty()
										}
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
					super::push(out, 10371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usnoc.
pub(crate) fn n5414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5128);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8256);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 10372);
					match s {
						[] => true,
						['c', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12049);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', 'd', s @ ..] => {
					super::push(out, 10373);
					match s {
						[] => true,
						['i', 'n', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8257);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 8258);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11532);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10374);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12051);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 12228);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 12229);
											s.is_empty()
										}
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
					super::push(out, 10375);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8259);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11534);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

