use flexion::Path;

/// Words ending -picatus.
pub(crate) fn n9644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7042);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10870);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14327);
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 14328);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14329);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14330);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 14331);
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

/// Words ending -ricatus.
pub(crate) fn n9645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7043);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10871);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14332);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16582);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10872);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14333);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17659);
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
					super::push(out, 14334);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14335);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14336);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16584);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10873);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 10874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14337);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16586);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16587);
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
					super::push(out, 14338);
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
					super::push(out, 14339);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10875);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10876);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10877);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 10878);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14340);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14341);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10879);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 14342);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14343);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicatus.
pub(crate) fn n9646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7044);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 10880);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ticatus.
pub(crate) fn n9647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7045);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10882);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10883);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10884);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14344);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vicatus.
pub(crate) fn n9648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7046);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 10886);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10887);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word distillatus.
pub(crate) fn n30885(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nstillatus.
pub(crate) fn n30886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16616);
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rstillatus.
pub(crate) fn n30887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16617);
	match s {
		[] => true,
		['e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xstillatus.
pub(crate) fn n30888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16618);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word cavillatus.
pub(crate) fn n16924(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ollatus.
pub(crate) fn n9834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7193);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14419);
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

/// Words ending -ullatus.
pub(crate) fn n9835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7194);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 11025);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// Words ending -eculatus.
pub(crate) fn n16962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11045);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14432);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 16622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16623);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16624);
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

/// Words ending -iculatus.
pub(crate) fn n16963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11046);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 14433);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 14434);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
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
				['i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 14435);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16625);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
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

/// The word calculatus.
pub(crate) fn n16964(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -oculatus.
pub(crate) fn n16965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11047);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -rculatus.
pub(crate) fn n16966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11048);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14436);
			match s {
				[] => true,
				['b', 'u', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 14437);
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

/// Words ending -sculatus.
pub(crate) fn n16967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11049);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14438);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 14439);
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

/// Words ending -dulatus.
pub(crate) fn n9853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7205);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11051);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11052);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11053);
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

/// Words ending -gulatus.
pub(crate) fn n9854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7206);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11054);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14440);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11055);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11056);
			match s {
				[] => true,
				['a', s @ ..] => {
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
								['r', 't', '\u{1}', s @ ..] => {
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
								['t', 's', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iulatus.
pub(crate) fn n9855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7207);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11057);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11058);
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

/// Words ending -lulatus.
pub(crate) fn n9856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7208);
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

/// Words ending -mulatus.
pub(crate) fn n9857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7209);
	match s {
		[] => true,
		['e', 'a', s @ ..] => {
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
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 14442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16626);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
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
				['t', 's', s @ ..] => {
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -nulatus.
pub(crate) fn n9858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7210);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11060);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14443);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pulatus.
pub(crate) fn n9859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11062);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11063);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
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
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11064);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14445);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16627);
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

/// Words ending -serrulatus.
pub(crate) fn n9860(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -sulatus.
pub(crate) fn n9861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7212);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 11065);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11066);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 14447);
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

/// Words ending -tulatus.
pub(crate) fn n9862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7213);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11067);
			match s {
				[] => true,
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 14448);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14449);
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 11068);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11069);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11070);
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
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16628);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 17672);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 16629);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -vulatus.
pub(crate) fn n9863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7214);
	s.is_empty()
}

/// The word exulatus.
pub(crate) fn n9864(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word albomarginatus.
pub(crate) fn n36903(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word proclinatus.
pub(crate) fn n31032(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -plinatus.
pub(crate) fn n17082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11109);
	match s {
		[] => true,
		['i', 'c', 's', 'i', 'd', s @ ..] => {
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

/// Words ending -minatus.
pub(crate) fn n9928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7264);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 11110);
			match s {
				[] => true,
				['l', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['t', 'a', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 11111);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14490);
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
				['g', s @ ..] => {
					super::push(out, 14491);
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
					super::push(out, 14492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 16649);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
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
		['g', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11112);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14493);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14494);
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
			super::push(out, 11113);
			match s {
				[] => true,
				['u', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11114);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11115);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 14495);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 14496);
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
				['n', s @ ..] => {
					super::push(out, 14497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16650);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'o', 'c', '\u{1}', s @ ..] => {
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
			super::push(out, 11116);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14498);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
									super::push(out, 17675);
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
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11117);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
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
				['l', s @ ..] => {
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
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pinatus.
pub(crate) fn n9929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7265);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11118);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11119);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11120);
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
					super::push(out, 14499);
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
			super::push(out, 11121);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rinatus.
pub(crate) fn n9930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7266);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11122);
			match s {
				[] => true,
				['c', s @ ..] => {
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
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16651);
							match s {
								[] => true,
								['t', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 17676);
									s.is_empty()
								}
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
			super::push(out, 11123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tinatus.
pub(crate) fn n9931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7267);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11124);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 11125);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11126);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14500);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 16652);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 14501);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14502);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16654);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17677);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17678);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 16655);
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
		['u', s @ ..] => {
			super::push(out, 11127);
			match s {
				[] => true,
				['l', 'g', s @ ..] => {
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
						['g', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -uinatus.
pub(crate) fn n9932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7268);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14503);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14504);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14505);
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

/// Words ending -vinatus.
pub(crate) fn n9933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7269);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11128);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -migeratus.
pub(crate) fn n24770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14538);
	s.is_empty()
}

/// Words ending -rigeratus.
pub(crate) fn n24771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14539);
	match s {
		[] => true,
		['f', s @ ..] => {
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

/// Words ending -ieratus.
pub(crate) fn n9986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7309);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14540);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 14541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14542);
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
			super::push(out, 11170);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leratus.
pub(crate) fn n9987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7310);
	match s {
		[] => true,
		['e', 'c', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 14543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16664);
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

/// Words ending -meratus.
pub(crate) fn n9988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7311);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11171);
			match s {
				[] => true,
				['t', s @ ..] => {
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
		['o', 'l', 'g', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 11172);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 16666);
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
								['a', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -neratus.
pub(crate) fn n9989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7312);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11173);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
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
							super::push(out, 16667);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17679);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17680);
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
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
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
		['g', 'i', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'v', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 11174);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11175);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
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

/// Words ending -peratus.
pub(crate) fn n9990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7313);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 11176);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
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
						['t', 'a', '\u{1}', s @ ..] => {
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
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11177);
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
				['r', 'p', s @ ..] => {
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
						['n', 'i', '\u{1}', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 11178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14544);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 14545);
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
				['e', s @ ..] => {
					super::push(out, 14546);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14547);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14548);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14549);
					match s {
						[] => true,
						['e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11179);
			match s {
				[] => true,
				['s', s @ ..] => {
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

/// Words ending -seratus.
pub(crate) fn n9991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7314);
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
		['i', 'm', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -teratus.
pub(crate) fn n9992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7315);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11180);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16668);
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
		['i', s @ ..] => {
			super::push(out, 11181);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16669);
							s.is_empty()
						}
						['b', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17681);
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
			super::push(out, 11182);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ueratus.
pub(crate) fn n9993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7316);
	s.is_empty()
}

/// Words ending -veratus.
pub(crate) fn n9994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7317);
	s.is_empty()
}

/// The word inhonoratus.
pub(crate) fn n24857(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word exhonoratus.
pub(crate) fn n24858(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -poratus.
pub(crate) fn n10018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7336);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11200);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16679);
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
			super::push(out, 11201);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11202);
			match s {
				[] => true,
				['o', 'c', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -roratus.
pub(crate) fn n10019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7337);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -toratus.
pub(crate) fn n10020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7338);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['o', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'a', s @ ..] => {
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

/// Words ending -voratus.
pub(crate) fn n10021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7339);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11203);
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

/// Words ending -xoratus.
pub(crate) fn n10022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7340);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word demonstratus.
pub(crate) fn n36907(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -saturatus.
pub(crate) fn n24910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14572);
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

/// Words ending -bturatus.
pub(crate) fn n17337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11228);
	match s {
		[] => true,
		['o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ituratus.
pub(crate) fn n17339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11230);
	s.is_empty()
}

/// Words ending -pturatus.
pub(crate) fn n17340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11231);
	s.is_empty()
}

/// Words ending -flagitatus.
pub(crate) fn n31319(s: &[char], _out: &mut Path<'_>) -> bool {
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
		_ => false,
	}
}

/// Words ending -oagitatus.
pub(crate) fn n25023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14603);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xagitatus.
pub(crate) fn n25024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14604);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -igitatus.
pub(crate) fn n17425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11275);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ogitatus.
pub(crate) fn n17426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11276);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 14605);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14606);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugitatus.
pub(crate) fn n17428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11278);
	s.is_empty()
}

/// Words ending -litatus.
pub(crate) fn n10088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7396);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 11279);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11280);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11281);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14607);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16703);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 16704);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16705);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14608);
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
			super::push(out, 11282);
			match s {
				[] => true,
				['e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mitatus.
pub(crate) fn n10089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7397);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11283);
			match s {
				[] => true,
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17686);
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
		['i', s @ ..] => {
			super::push(out, 11284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14609);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14610);
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
		['o', s @ ..] => {
			super::push(out, 11285);
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
		['r', s @ ..] => {
			super::push(out, 11286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nitatus.
pub(crate) fn n10090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7398);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11287);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11288);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pitatus.
pub(crate) fn n10091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7399);
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
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
		['e', 'r', 'c', s @ ..] => {
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
		['i', 't', s @ ..] => {
			super::push(out, 11290);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'u', 'n', 'e', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'o', 'h', s @ ..] => {
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

/// Words ending -ritatus.
pub(crate) fn n10092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7400);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11291);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11292);
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 11293);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11294);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11295);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14611);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14612);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sitatus.
pub(crate) fn n10093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7401);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11296);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11297);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11298);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11299);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11300);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'c', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -titatus.
pub(crate) fn n10094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7402);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 11302);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16706);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11303);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uitatus.
pub(crate) fn n10095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7403);
	s.is_empty()
}

/// Words ending -vitatus.
pub(crate) fn n10096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7404);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 11305);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11306);
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
		['n', s @ ..] => {
			super::push(out, 11307);
			match s {
				[] => true,
				['i', s @ ..] => {
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

/// Words ending -narrabilis.
pub(crate) fn n18727(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 14863);
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

/// Words ending -trabilis.
pub(crate) fn n11391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8087);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11838);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11839);
			match s {
				[] => true,
				['n', 'e', 'p', s @ ..] => {
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
			super::push(out, 11840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urabilis.
pub(crate) fn n11392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8088);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 11841);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 11842);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11843);
			match s {
				[] => true,
				['n', 'e', 'm', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 11844);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16792);
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

/// Words ending -mutabilis.
pub(crate) fn n18760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11866);
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
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ficandus.
pub(crate) fn n14176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9226);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15078);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16840);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 15079);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16842);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 16843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 15080);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16844);
							match s {
								[] => true,
								['a', s @ ..] => {
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
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
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
							super::push(out, 16845);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 16846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15081);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15082);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16847);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16848);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16849);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 16850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15083);
					match s {
						[] => true,
						['g', 'i', 's', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15084);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16851);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 16852);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16853);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16854);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 16855);
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
						['u', s @ ..] => {
							super::push(out, 16856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 15085);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 15086);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16857);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 16858);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17709);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17710);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 17711);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 18014);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 18015);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 16859);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16860);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16861);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17712);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17713);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 17714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 15087);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16862);
							match s {
								[] => true,
								['v', s @ ..] => {
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
							super::push(out, 16863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -plicandus.
pub(crate) fn n20639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12490);
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
			super::push(out, 15088);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16865);
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
			super::push(out, 15089);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16866);
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
		['m', s @ ..] => {
			super::push(out, 15090);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 15091);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16867);
					match s {
						[] => true,
						['n', 'e', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -rnicandus.
pub(crate) fn n20643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12493);
	s.is_empty()
}

/// Words ending -unicandus.
pub(crate) fn n20644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12494);
	s.is_empty()
}

/// Words ending -picandus.
pub(crate) fn n14180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9230);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 15094);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ricandus.
pub(crate) fn n14181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9231);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12495);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 15096);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16868);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 12496);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15097);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15098);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16870);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 12497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15099);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16873);
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
					super::push(out, 15100);
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
					super::push(out, 15101);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12498);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 12499);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12500);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 12501);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15102);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15103);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicandus.
pub(crate) fn n14182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9232);
	s.is_empty()
}

/// Words ending -ticandus.
pub(crate) fn n14183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9233);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12502);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12503);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12504);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15104);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vicandus.
pub(crate) fn n14184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9234);
	s.is_empty()
}

/// Words ending -cumulandus.
pub(crate) fn n20933(s: &[char], _out: &mut Path<'_>) -> bool {
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
		['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pulandus.
pub(crate) fn n14351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9363);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12638);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 12639);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
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
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12640);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15166);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 15167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16892);
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

/// Words ending -xsulandus.
pub(crate) fn n20938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12642);
	s.is_empty()
}

/// Words ending -tulandus.
pub(crate) fn n14353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9365);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12643);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
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
			super::push(out, 12644);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12645);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 15168);
					match s {
						[] => true,
						['p', s @ ..] => {
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16893);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 17719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 16894);
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
		_ => false,
	}
}

/// Words ending -feminandus.
pub(crate) fn n27115(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16912);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 16913);
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

/// Words ending -geminandus.
pub(crate) fn n27116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15203);
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

/// Words ending -seminandus.
pub(crate) fn n27117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15204);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16914);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iminandus.
pub(crate) fn n21004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12675);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 15205);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15206);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lminandus.
pub(crate) fn n21005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12676);
	match s {
		[] => true,
		['u', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mminandus.
pub(crate) fn n21006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12677);
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ominandus.
pub(crate) fn n21007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12678);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 15207);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 15208);
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
		['n', s @ ..] => {
			super::push(out, 15209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16915);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rminandus.
pub(crate) fn n21008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12679);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
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
							super::push(out, 17720);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uminandus.
pub(crate) fn n21009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12680);
	match s {
		[] => true,
		['c', 'a', s @ ..] => {
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
		['l', s @ ..] => {
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
		['r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pinandus.
pub(crate) fn n14402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9410);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12681);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 12682);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 12683);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 15210);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15211);
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
			super::push(out, 12684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rinandus.
pub(crate) fn n14403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9411);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 12685);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12686);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tinandus.
pub(crate) fn n14404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9412);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12687);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 12688);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12689);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15212);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 16916);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15213);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17721);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17722);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 16919);
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
		['u', s @ ..] => {
			super::push(out, 12690);
			match s {
				[] => true,
				['l', 'g', s @ ..] => {
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
						['g', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -uinandus.
pub(crate) fn n14405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9413);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 15214);
					match s {
						[] => true,
						['i', s @ ..] => {
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 15215);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word accelerandus.
pub(crate) fn n27200(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word adcelerandus.
pub(crate) fn n27201(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -scelerandus.
pub(crate) fn n27202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15247);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 16926);
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

/// Words ending -tolerandus.
pub(crate) fn n21099(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -merandus.
pub(crate) fn n14455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9448);
	match s {
		[] => true,
		['e', 't', s @ ..] => {
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
		['o', 'l', 'g', s @ ..] => {
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
		['u', 'n', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15249);
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
						['a', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -nerandus.
pub(crate) fn n14456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9449);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12727);
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
							super::push(out, 16927);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17724);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17725);
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
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
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
		['g', 'i', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'v', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 12728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12729);
			match s {
				[] => true,
				['m', s @ ..] => {
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

/// Words ending -perandus.
pub(crate) fn n14457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9450);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 12730);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
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
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'o', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 12731);
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
				['r', 'p', s @ ..] => {
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
			super::push(out, 12732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15250);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 15251);
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
				['e', s @ ..] => {
					super::push(out, 15252);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 15253);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15254);
					match s {
						[] => true,
						['e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12733);
			match s {
				[] => true,
				['s', s @ ..] => {
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

/// Words ending -serandus.
pub(crate) fn n14458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9451);
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
		['i', 'm', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -terandus.
pub(crate) fn n14459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9452);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12734);
			s.is_empty()
		}
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
		['i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uerandus.
pub(crate) fn n14460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9453);
	s.is_empty()
}

/// Words ending -verandus.
pub(crate) fn n14461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9454);
	s.is_empty()
}

/// Words ending -rorandus.
pub(crate) fn n14480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9472);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -torandus.
pub(crate) fn n14481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9473);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['o', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'a', s @ ..] => {
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

/// Words ending -vorandus.
pub(crate) fn n14482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9474);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12751);
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

/// The word exorandus.
pub(crate) fn n21158(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -digitandus.
pub(crate) fn n27407(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ogitandus.
pub(crate) fn n21282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12809);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 15300);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugitandus.
pub(crate) fn n21284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12811);
	s.is_empty()
}

/// Words ending -litandus.
pub(crate) fn n14541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9528);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 12812);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 12813);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 12814);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15302);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16953);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 16954);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16955);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 15303);
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
			super::push(out, 12815);
			match s {
				[] => true,
				['e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mitandus.
pub(crate) fn n14542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9529);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12816);
			match s {
				[] => true,
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17730);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17731);
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
		['i', s @ ..] => {
			super::push(out, 12817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15304);
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
		['o', s @ ..] => {
			super::push(out, 12818);
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
		['r', s @ ..] => {
			super::push(out, 12819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nitandus.
pub(crate) fn n14543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9530);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12820);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 12821);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12822);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pitandus.
pub(crate) fn n14544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9531);
	match s {
		[] => true,
		['e', 'r', 'c', s @ ..] => {
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
		['s', 'o', 'h', s @ ..] => {
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

/// Words ending -ritandus.
pub(crate) fn n14545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9532);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12823);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12824);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12825);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 12826);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15305);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sitandus.
pub(crate) fn n14546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9533);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12827);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 12828);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12829);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12830);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12831);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -titandus.
pub(crate) fn n14547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9534);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16956);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 12833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uitandus.
pub(crate) fn n14548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9535);
	match s {
		[] => true,
		['q', 'e', s @ ..] => {
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
				['n', 'i', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -vitandus.
pub(crate) fn n14549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9536);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 12834);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 12835);
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
		['n', s @ ..] => {
			super::push(out, 12836);
			match s {
				[] => true,
				['i', s @ ..] => {
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

/// Words ending -miscendus.
pub(crate) fn n21535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12936);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 15369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16978);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16979);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16980);
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

/// Words ending -iniscendus.
pub(crate) fn n27647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15370);
	s.is_empty()
}

/// Words ending -uniscendus.
pub(crate) fn n27648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15371);
	s.is_empty()
}

/// Words ending -piscendus.
pub(crate) fn n21537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12938);
	s.is_empty()
}

/// Words ending -riscendus.
pub(crate) fn n21538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12939);
	s.is_empty()
}

/// Words ending -tiscendus.
pub(crate) fn n21539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12940);
	s.is_empty()
}

/// Words ending -viscendus.
pub(crate) fn n21540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12941);
	s.is_empty()
}

/// Words ending -oscendus.
pub(crate) fn n14653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9614);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 12942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 15372);
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
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17740);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18017);
											s.is_empty()
										}
										_ => false,
									}
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15373);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
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
			super::push(out, 12943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16982);
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

/// The word superinpendendus.
pub(crate) fn n32655(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word appendendus.
pub(crate) fn n27729(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word perpendendus.
pub(crate) fn n27730(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word suspendendus.
pub(crate) fn n27731(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word expendendus.
pub(crate) fn n27732(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rendendus.
pub(crate) fn n21619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12983);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 15394);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 15395);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16990);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 17747);
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

/// Words ending -tendendus.
pub(crate) fn n21620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12984);
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
			super::push(out, 15396);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
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
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
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

/// Words ending -vendendus.
pub(crate) fn n21621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12985);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 15397);
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

/// Words ending -indendus.
pub(crate) fn n14679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9637);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', 's', s @ ..] => {
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
					super::push(out, 15398);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 15399);
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
		['f', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ondendus.
pub(crate) fn n14680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9638);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 12986);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 12987);
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
									super::push(out, 17748);
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
			super::push(out, 12988);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15400);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16991);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 16992);
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

/// Words ending -undendus.
pub(crate) fn n14681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9639);
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
				['f', s @ ..] => {
					super::push(out, 15401);
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
						['o', '\u{1}', s @ ..] => {
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
					super::push(out, 15402);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16993);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16994);
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
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['s', s @ ..] => {
					super::push(out, 15403);
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
			super::push(out, 12989);
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

/// Words ending -tringendus.
pub(crate) fn n27872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15425);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17753);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 18018);
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
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17754);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 18019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 18020);
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

/// The word pertingendus.
pub(crate) fn n27877(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word attingendus.
pub(crate) fn n27878(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ungendus.
pub(crate) fn n14707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9659);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 13023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15426);
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
			super::push(out, 13024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 15427);
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
					super::push(out, 15428);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15429);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 17003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15430);
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
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13025);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13026);
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
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
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

/// Words ending -cuefaciendus.
pub(crate) fn n35471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17764);
	match s {
		[] => true,
		['a', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -quefaciendus.
pub(crate) fn n35472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17765);
	match s {
		[] => true,
		['i', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -suefaciendus.
pub(crate) fn n35473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17766);
	match s {
		[] => true,
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 18024);
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
		_ => false,
	}
}

/// Words ending -vefaciendus.
pub(crate) fn n32783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17016);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lfaciendus.
pub(crate) fn n27942(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word satisfaciendus.
pub(crate) fn n27943(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -iaciendus.
pub(crate) fn n21743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13038);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
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

/// The word laciendus.
pub(crate) fn n21744(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -spiciendus.
pub(crate) fn n27979(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 17026);
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
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17027);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 17767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17768);
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
			super::push(out, 17028);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 17029);
			match s {
				[] => true,
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 17030);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17769);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17771);
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
			super::push(out, 17031);
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
			super::push(out, 17032);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 17033);
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

/// The word obaudiendus.
pub(crate) fn n32861(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word subaudiendus.
pub(crate) fn n32862(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inaudiendus.
pub(crate) fn n28012(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word exaudiendus.
pub(crate) fn n28013(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rudiendus.
pub(crate) fn n21777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13060);
	s.is_empty()
}

/// The word infelicissimus.
pub(crate) fn n36921(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lissimus.
pub(crate) fn n15378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13423);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 15730);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15731);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 17156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 15732);
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
			super::push(out, 13424);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17157);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 18026);
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
							super::push(out, 17158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13425);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15733);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17159);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17160);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17161);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15734);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 15735);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 17163);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17164);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 15736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13426);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 13427);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nfirmissimus.
pub(crate) fn n33228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17166);
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nissimus.
pub(crate) fn n15380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10136);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13431);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15737);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15738);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15739);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 15740);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17167);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17168);
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
				['n', s @ ..] => {
					super::push(out, 15741);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15742);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 15743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13432);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15744);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15745);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15746);
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
					super::push(out, 15747);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15748);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 15749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13433);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15750);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17169);
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
							super::push(out, 17170);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13434);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 15751);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15752);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 15753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13435);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pissimus.
pub(crate) fn n15381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10137);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13437);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rissimus.
pub(crate) fn n15382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13439);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17172);
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
							super::push(out, 17173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 15754);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 13440);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13441);
			match s {
				[] => true,
				['v', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 13442);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 15756);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 15757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13443);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13444);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13445);
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
				['t', s @ ..] => {
					super::push(out, 15758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sissimus.
pub(crate) fn n15383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10139);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 13446);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13447);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15759);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13448);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15761);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 15762);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15763);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17174);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 17175);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									super::push(out, 17797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 17176);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17177);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17178);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17798);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 17799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17179);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17180);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 17800);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 17801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 17181);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17802);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17803);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 17804);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 18027);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 18028);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 18029);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15764);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 17182);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17183);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 17806);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 15765);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17184);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17185);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15766);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17187);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17808);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 17809);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 17188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15767);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							super::push(out, 17189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 15768);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15769);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 17811);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 17812);
									s.is_empty()
								}
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
			super::push(out, 13449);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13450);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17190);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 17191);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13451);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13452);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 17193);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tissimus.
pub(crate) fn n15384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10140);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13453);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15770);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 15771);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17195);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17196);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 17813);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 17814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 15772);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15773);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 17198);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 15774);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17200);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15775);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 15776);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15777);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17202);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17203);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 17815);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 18030);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 18031);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 17816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17204);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 15778);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15779);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17206);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['b', s @ ..] => {
							super::push(out, 17207);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17208);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17817);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 18032);
											match s {
												[] => true,
												['s', 'n', 'o', 'c', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n36975(s, out),
														['n', 'i', '\u{1}', s @ ..] => super::t13::n36976(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 18033);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 17818);
									match s {
										[] => true,
										['e', 'c', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 18087);
													match s {
														[] => true,
														['o', 'c', '\u{1}', s @ ..] => super::t13::n36977(s, out),
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
									super::push(out, 17819);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 18034);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 18035);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 17820);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 18036);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 18037);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 17209);
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
							super::push(out, 17210);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 15780);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							super::push(out, 17212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 13454);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15781);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15782);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 17213);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17214);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => super::t13::n35660(s, out),
								['i', s @ ..] => super::t13::n35661(s, out),
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17216);
							s.is_empty()
						}
						['r', s @ ..] => super::t13::n33286(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t13::n28730(s, out),
				['n', s @ ..] => super::t13::n28731(s, out),
				['o', s @ ..] => super::t13::n28732(s, out),
				['r', s @ ..] => super::t13::n28733(s, out),
				['u', s @ ..] => super::t13::n28734(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t13::n22500(s, out),
		['i', s @ ..] => super::t13::n22501(s, out),
		['l', s @ ..] => super::t13::n22502(s, out),
		['n', s @ ..] => super::t13::n22503(s, out),
		['o', s @ ..] => super::t13::n22504(s, out),
		['p', s @ ..] => super::t13::n22505(s, out),
		['r', s @ ..] => super::t13::n22506(s, out),
		['s', s @ ..] => super::t13::n22507(s, out),
		['u', s @ ..] => super::t13::n22508(s, out),
		_ => false,
	}
}

/// Words ending -uissimus.
pub(crate) fn n15385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10141);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 13464);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 13465);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13466);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 13467);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15811);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17257);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15812);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vissimus.
pub(crate) fn n15386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10142);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13468);
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
				['r', 'g', s @ ..] => {
					super::push(out, 15813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13469);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13470);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13471);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xissimus.
pub(crate) fn n15387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10143);
	s.is_empty()
}

/// Words ending -icaturus.
pub(crate) fn n15727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10345);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13594);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 13595);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 15887);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17851);
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
					super::push(out, 15888);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15889);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17279);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 17852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17853);
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
							super::push(out, 17280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 17281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17282);
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
					super::push(out, 15890);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17283);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 18061);
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
							super::push(out, 17284);
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
					super::push(out, 15891);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 17285);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 17286);
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
					super::push(out, 15892);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17287);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 17288);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 17854);
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
									super::push(out, 17855);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 18062);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 18063);
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
									super::push(out, 17856);
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
							super::push(out, 17289);
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
		['f', s @ ..] => super::t13::n22847(s, out),
		['l', s @ ..] => {
			super::push(out, 13597);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15894);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15895);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15896);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15897);
					s.is_empty()
				}
				['p', s @ ..] => super::t13::n29050(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13598);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15899);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15900);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15901);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17305);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13599);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15902);
					s.is_empty()
				}
				['r', s @ ..] => super::t13::n29055(s, out),
				['u', s @ ..] => super::t13::n29056(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t13::n22851(s, out),
		['r', s @ ..] => super::t13::n22852(s, out),
		['s', s @ ..] => super::t13::n22853(s, out),
		['t', s @ ..] => super::t13::n22854(s, out),
		['v', s @ ..] => super::t13::n22855(s, out),
		_ => false,
	}
}

/// The word allocaturus.
pub(crate) fn n33507(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -collocaturus.
pub(crate) fn n33508(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word conlocaturus.
pub(crate) fn n29081(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rocaturus.
pub(crate) fn n22862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13608);
	s.is_empty()
}

/// Words ending -vocaturus.
pub(crate) fn n22863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13609);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 15922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 15923);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17320);
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
			super::push(out, 15925);
			match s {
				[] => true,
				['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15926);
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
			super::push(out, 15927);
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

/// Words ending -rcaturus.
pub(crate) fn n15731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10349);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13610);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17321);
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
			super::push(out, 13611);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17322);
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

/// Words ending -scaturus.
pub(crate) fn n15732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10350);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13612);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13613);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 15928);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 15929);
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
			super::push(out, 13614);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17323);
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

/// Words ending -ucaturus.
pub(crate) fn n15733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10351);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15930);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15931);
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

/// Words ending -emendaturus.
pub(crate) fn n33542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17331);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmendaturus.
pub(crate) fn n33543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17332);
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -undaturus.
pub(crate) fn n22886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13629);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 15933);
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
			super::push(out, 15934);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17333);
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
					super::push(out, 17334);
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
			super::push(out, 15935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17335);
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
			super::push(out, 15936);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17336);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17337);
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

/// Words ending -odaturus.
pub(crate) fn n15740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10358);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 13630);
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
			super::push(out, 13631);
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
					super::push(out, 15937);
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

/// Words ending -rdaturus.
pub(crate) fn n15741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10359);
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
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udaturus.
pub(crate) fn n15742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10360);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13632);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 15938);
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
					super::push(out, 15939);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17894);
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
			super::push(out, 13633);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15940);
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
			super::push(out, 13635);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15941);
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

/// Words ending -nligaturus.
pub(crate) fn n29179(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -migaturus.
pub(crate) fn n22916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13647);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 15956);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 15957);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rigaturus.
pub(crate) fn n22917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13648);
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
			super::push(out, 15958);
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

/// Words ending -tigaturus.
pub(crate) fn n22918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13649);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15959);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17895);
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
			super::push(out, 15960);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 17344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 15961);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 15962);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17345);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17346);
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
					super::push(out, 17347);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vigaturus.
pub(crate) fn n22919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13650);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15963);
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
							super::push(out, 17896);
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
							super::push(out, 17897);
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
							super::push(out, 17898);
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
			super::push(out, 15964);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgaturus.
pub(crate) fn n15757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10369);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 13651);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17349);
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
			super::push(out, 13652);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 15965);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 15966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17350);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17351);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17352);
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

/// Words ending -ngaturus.
pub(crate) fn n15758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10370);
	match s {
		[] => true,
		['o', 'l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15967);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ogaturus.
pub(crate) fn n15759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10371);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 15969);
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
					super::push(out, 15970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 17353);
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
					super::push(out, 15971);
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
					super::push(out, 15972);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15973);
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

/// Words ending -rgaturus.
pub(crate) fn n15760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10372);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 15974);
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
							super::push(out, 17354);
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

/// Words ending -ugaturus.
pub(crate) fn n15761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10373);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 13653);
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
			super::push(out, 13654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 15975);
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
					super::push(out, 15976);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15977);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15978);
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
			super::push(out, 13655);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13656);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word expiaturus.
pub(crate) fn n22955(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -riaturus.
pub(crate) fn n15771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10383);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13675);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 13676);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13677);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 15988);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17359);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 13678);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13679);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 15989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13680);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15990);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15991);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 15992);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 15993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siaturus.
pub(crate) fn n15772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10384);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13681);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13682);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiaturus.
pub(crate) fn n15773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10385);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13683);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13684);
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
			super::push(out, 13685);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 15994);
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
			super::push(out, 13686);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15995);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15996);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15997);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 17901);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17902);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 17903);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18070);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 18071);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 18072);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 18073);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['b', s @ ..] => {
													super::push(out, 18090);
													match s {
														[] => true,
														['a', '\u{1}', s @ ..] => super::t13::n36982(s, out),
														_ => false,
													}
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
									super::push(out, 17904);
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
			super::push(out, 13687);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13688);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viaturus.
pub(crate) fn n15774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10386);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 13690);
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
			super::push(out, 13691);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 15998);
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
					super::push(out, 15999);
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

/// Words ending -ibilaturus.
pub(crate) fn n29283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16005);
	match s {
		[] => true,
		['s', s @ ..] => {
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

/// Words ending -cilaturus.
pub(crate) fn n23000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13704);
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

/// Words ending -dilaturus.
pub(crate) fn n23001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13705);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gilaturus.
pub(crate) fn n23002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13706);
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
					super::push(out, 17363);
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hilaturus.
pub(crate) fn n23003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13707);
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

/// Words ending -milaturus.
pub(crate) fn n23004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13708);
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
				['s', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pilaturus.
pub(crate) fn n23005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13709);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16007);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16008);
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

/// Words ending -tilaturus.
pub(crate) fn n23006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13710);
	match s {
		[] => true,
		['n', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 16009);
			match s {
				[] => true,
				['u', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 17364);
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

/// Words ending -llaturus.
pub(crate) fn n15784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10394);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16010);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 16011);
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
			super::push(out, 13712);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16012);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17906);
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
				['c', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 16013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 16014);
					match s {
						[] => true,
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
			super::push(out, 13713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 16015);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17365);
							match s {
								[] => true,
								['r', 'c', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 16016);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16017);
					match s {
						[] => true,
						['g', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17366);
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
				['t', s @ ..] => {
					super::push(out, 16018);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17367);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17368);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 17369);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17907);
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
									super::push(out, 17908);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17909);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 17910);
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
					super::push(out, 16019);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 16020);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13714);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17370);
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
		['u', s @ ..] => {
			super::push(out, 13715);
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

/// The word circumlaturus.
pub(crate) fn n15785(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nlaturus.
pub(crate) fn n15786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10395);
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			super::push(out, 13716);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olaturus.
pub(crate) fn n15787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10396);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13717);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 13718);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16021);
					match s {
						[] => true,
						['r', 'g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16022);
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
			super::push(out, 13719);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16023);
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
			super::push(out, 13720);
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
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13721);
			match s {
				[] => true,
				['m', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13722);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13723);
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
			super::push(out, 13724);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16024);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16025);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16026);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16027);
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
			super::push(out, 13725);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 13726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16028);
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
				['e', s @ ..] => {
					super::push(out, 16029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17371);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 17372);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 17373);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17374);
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
					super::push(out, 16030);
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
					super::push(out, 16031);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16032);
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
					super::push(out, 16033);
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

/// Words ending -platurus.
pub(crate) fn n15788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10397);
	match s {
		[] => true,
		['m', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rlaturus.
pub(crate) fn n15789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10398);
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

/// Words ending -slaturus.
pub(crate) fn n15790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10399);
	match s {
		[] => true,
		['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulaturus.
pub(crate) fn n15791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10400);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13727);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16034);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 17375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17911);
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
							super::push(out, 17376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 17377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16035);
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
					super::push(out, 16036);
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
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
			super::push(out, 13728);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16037);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
					super::push(out, 16038);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17912);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 17913);
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
				['i', s @ ..] => {
					super::push(out, 16039);
					match s {
						[] => true,
						['n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16040);
					s.is_empty()
				}
				['r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16041);
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
			super::push(out, 13729);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16043);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16044);
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
			super::push(out, 13730);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16045);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16046);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16047);
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
					super::push(out, 16048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13731);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16049);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16050);
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
			super::push(out, 13732);
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
		['m', s @ ..] => {
			super::push(out, 13733);
			match s {
				[] => true,
				['e', 'a', s @ ..] => {
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
				['i', 's', s @ ..] => super::t13::n29364(s, out),
				['u', 'c', s @ ..] => super::t13::n29365(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t13::n23034(s, out),
		['s', s @ ..] => super::t13::n23035(s, out),
		['t', s @ ..] => super::t13::n23036(s, out),
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -istimaturus.
pub(crate) fn n33787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17385);
	s.is_empty()
}

/// Words ending -ximaturus.
pub(crate) fn n23054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13752);
	match s {
		[] => true,
		['o', 'r', 'p', s @ ..] => {
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

/// Words ending -lmaturus.
pub(crate) fn n15796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10404);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmaturus.
pub(crate) fn n15797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10405);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13753);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17386);
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
			super::push(out, 13754);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13755);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13756);
			match s {
				[] => true,
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word inmaturus.
pub(crate) fn n15798(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -omaturus.
pub(crate) fn n15799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10406);
	s.is_empty()
}

/// Words ending -rmaturus.
pub(crate) fn n15800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10407);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13757);
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
			super::push(out, 13758);
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
							super::push(out, 17387);
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
							super::push(out, 17388);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17918);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17919);
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
			super::push(out, 13759);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17389);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17921);
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
							super::push(out, 17390);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17391);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17922);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17923);
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
							super::push(out, 17392);
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
					super::push(out, 16069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -smaturus.
pub(crate) fn n15801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10408);
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

/// Words ending -umaturus.
pub(crate) fn n15802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10409);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 13760);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 13761);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 13762);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13763);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13764);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17393);
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
			super::push(out, 13765);
			match s {
				[] => true,
				['r', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugnaturus.
pub(crate) fn n23085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13774);
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
					super::push(out, 17400);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17929);
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
					super::push(out, 17401);
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

/// Words ending -inaturus.
pub(crate) fn n15807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10414);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13775);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 13776);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16077);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16078);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 17402);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17403);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 17404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16079);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16080);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16081);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17405);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16082);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16083);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 13777);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16084);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17407);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16085);
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
			super::push(out, 13778);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16086);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17409);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17410);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 17411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16087);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 17930);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 17931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16088);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17412);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16089);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17932);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17933);
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
			super::push(out, 13779);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13780);
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
							super::push(out, 17414);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17934);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17935);
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
							super::push(out, 17415);
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
		['m', s @ ..] => {
			super::push(out, 13781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16090);
					match s {
						[] => true,
						['l', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => super::t13::n36000(s, out),
								['n', 'o', 'c', '\u{1}', s @ ..] => super::t13::n36001(s, out),
								['t', 'a', '\u{1}', s @ ..] => super::t13::n36002(s, out),
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => super::t13::n33859(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t13::n29443(s, out),
				['i', s @ ..] => super::t13::n29444(s, out),
				['l', s @ ..] => super::t13::n29445(s, out),
				['m', s @ ..] => super::t13::n29446(s, out),
				['o', s @ ..] => super::t13::n29447(s, out),
				['r', s @ ..] => super::t13::n29448(s, out),
				['u', s @ ..] => super::t13::n29449(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t13::n23093(s, out),
		['r', s @ ..] => super::t13::n23094(s, out),
		['t', s @ ..] => super::t13::n23095(s, out),
		['u', s @ ..] => super::t13::n23096(s, out),
		['v', s @ ..] => super::t13::n23097(s, out),
		_ => false,
	}
}

/// Words ending -damnaturus.
pub(crate) fn n23098(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -nnaturus.
pub(crate) fn n15809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10416);
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
		_ => false,
	}
}

/// Words ending -onaturus.
pub(crate) fn n15810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10417);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 13787);
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
			super::push(out, 13788);
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
			super::push(out, 13789);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16109);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16110);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17428);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17429);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13790);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13791);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13792);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13793);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13794);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnaturus.
pub(crate) fn n15811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10418);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13795);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13796);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16112);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17431);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17432);
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
					super::push(out, 16113);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16114);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 17433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13797);
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
					super::push(out, 16115);
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

/// Words ending -unaturus.
pub(crate) fn n15812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10419);
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
		_ => false,
	}
}

/// The word emancipaturus.
pub(crate) fn n36959(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lucraturus.
pub(crate) fn n29518(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -draturus.
pub(crate) fn n15824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10430);
	match s {
		[] => true,
		['a', 'u', 'q', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eraturus.
pub(crate) fn n15825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10431);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13813);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16129);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17944);
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
					super::push(out, 16130);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
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
								['s', s @ ..] => {
									super::push(out, 17945);
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
				['u', s @ ..] => {
					super::push(out, 16131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17447);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 17448);
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
		['c', s @ ..] => {
			super::push(out, 13814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16132);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16133);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 17449);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 17450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 17946);
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
						['m', s @ ..] => {
							super::push(out, 17451);
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
					super::push(out, 16134);
					match s {
						[] => true,
						['u', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13815);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16135);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17947);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 17948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16136);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 16137);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 17452);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 17453);
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
		['f', s @ ..] => {
			super::push(out, 13816);
			match s {
				[] => true,
				['c', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13817);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16139);
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
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
					super::push(out, 16140);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17454);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17455);
							match s {
								[] => true,
								['f', s @ ..] => {
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13818);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17456);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 17457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17458);
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
					super::push(out, 16141);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13819);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
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
						['s', s @ ..] => {
							super::push(out, 17459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => super::t13::n36076(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t13::n23146(s, out),
		['n', s @ ..] => super::t13::n23147(s, out),
		['p', s @ ..] => super::t13::n23148(s, out),
		['s', s @ ..] => super::t13::n23149(s, out),
		['t', s @ ..] => super::t13::n23150(s, out),
		['u', s @ ..] => super::t13::n23151(s, out),
		['v', s @ ..] => super::t13::n23152(s, out),
		_ => false,
	}
}

/// Words ending -graturus.
pub(crate) fn n15826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10432);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13827);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16150);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17952);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 17953);
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
					super::push(out, 16151);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17467);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 17468);
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
			super::push(out, 13828);
			match s {
				[] => true,
				['t', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'e', 'r', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 13829);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 16152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 17954);
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
						['m', s @ ..] => {
							super::push(out, 17470);
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
							super::push(out, 17471);
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
					super::push(out, 16153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17472);
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
					super::push(out, 16154);
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

/// Words ending -iraturus.
pub(crate) fn n15827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13830);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16155);
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
			super::push(out, 13832);
			match s {
				[] => true,
				['s', s @ ..] => {
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
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17473);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17955);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17956);
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
						['u', 's', '\u{1}', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 13833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 13834);
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

/// Words ending -oraturus.
pub(crate) fn n15828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10434);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 13835);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16156);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17957);
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16157);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 13836);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16158);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17958);
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
				['l', s @ ..] => {
					super::push(out, 16159);
					match s {
						[] => true,
						['u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16160);
					match s {
						[] => true,
						['e', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13837);
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
		['f', s @ ..] => {
			super::push(out, 13838);
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
		['g', s @ ..] => {
			super::push(out, 13839);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13840);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'e', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13841);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 16161);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17959);
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
				['p', s @ ..] => {
					super::push(out, 16162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17474);
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
			super::push(out, 13842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17475);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 17476);
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
								_ => false,
							}
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
		['n', s @ ..] => {
			super::push(out, 13843);
			match s {
				[] => true,
				['g', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'h', s @ ..] => {
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
		['p', s @ ..] => {
			super::push(out, 13844);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16163);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17960);
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
					super::push(out, 16164);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16165);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
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
				_ => false,
			}
		}
		['r', s @ ..] => super::t13::n23173(s, out),
		['t', s @ ..] => super::t13::n23174(s, out),
		['v', s @ ..] => super::t13::n23175(s, out),
		['x', s @ ..] => {
			super::push(out, 13848);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => super::t13::n29600(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -praturus.
pub(crate) fn n15829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10435);
	match s {
		[] => true,
		['u', 't', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rraturus.
pub(crate) fn n15830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10436);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13849);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17477);
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
			super::push(out, 13850);
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13851);
			match s {
				[] => true,
				['s', 'u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17478);
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

/// Words ending -traturus.
pub(crate) fn n15831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10437);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13852);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16167);
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
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16168);
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
			super::push(out, 13853);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
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
		['i', s @ ..] => {
			super::push(out, 13854);
			match s {
				[] => true,
				['c', 'l', 'a', 'c', s @ ..] => {
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
			super::push(out, 13855);
			match s {
				[] => true,
				['i', s @ ..] => {
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
			super::push(out, 13856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16169);
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
				['b', s @ ..] => {
					super::push(out, 16170);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16171);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17479);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 17480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16172);
					match s {
						[] => true,
						['n', 'i', 'm', s @ ..] => {
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
								['m', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16173);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17482);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
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
												['d', s @ ..] => {
													super::push(out, 18091);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n36985(s, out),
														_ => false,
													}
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16174);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16175);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
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
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uraturus.
pub(crate) fn n15832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10438);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13857);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16176);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17961);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 17962);
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
		['c', s @ ..] => {
			super::push(out, 13858);
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
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16177);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
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
		['f', s @ ..] => {
			super::push(out, 13860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13861);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16178);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17963);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 17964);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 17965);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 17966);
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
				['l', s @ ..] => {
					super::push(out, 16179);
					match s {
						[] => true,
						['u', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16180);
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13862);
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
					super::push(out, 16181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17483);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 17484);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17485);
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
				['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 13863);
			match s {
				[] => true,
				['r', 'u', 'm', s @ ..] => {
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
						['m', s @ ..] => {
							match s {
								[] => true,
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
		['p', s @ ..] => {
			super::push(out, 13864);
			match s {
				[] => true,
				['r', 'u', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13865);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13866);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16182);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17486);
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
						['n', s @ ..] => {
							super::push(out, 17487);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17488);
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
				['b', s @ ..] => {
					super::push(out, 16183);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yraturus.
pub(crate) fn n15833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10439);
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

/// The word desponsaturus.
pub(crate) fn n36964(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ssaturus.
pub(crate) fn n15840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10446);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
			super::push(out, 13875);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16191);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16193);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 17493);
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
		_ => false,
	}
}

/// Words ending -pausaturus.
pub(crate) fn n29678(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -cusaturus.
pub(crate) fn n23212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13877);
	match s {
		[] => true,
		['c', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word detractaturus.
pub(crate) fn n36965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18092);
	s.is_empty()
}

/// Words ending -ectaturus.
pub(crate) fn n23216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13881);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 16202);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17494);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17495);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 16203);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17497);
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
				['e', s @ ..] => {
					super::push(out, 17498);
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
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17499);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 16204);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17969);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 17970);
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
		['p', s @ ..] => {
			super::push(out, 16205);
			match s {
				[] => true,
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
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17971);
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
						['o', s @ ..] => {
							super::push(out, 17972);
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
				['x', 'e', s @ ..] => {
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
			super::push(out, 16206);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 17501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16207);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17502);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17974);
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
				['s', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 16208);
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

/// Words ending -arctaturus.
pub(crate) fn n29705(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -uctaturus.
pub(crate) fn n23221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13886);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16212);
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
			super::push(out, 16213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17977);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17978);
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
			super::push(out, 16214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17505);
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

/// Words ending -etaturus.
pub(crate) fn n15844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10450);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13887);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 13888);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 13889);
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
		['g', s @ ..] => {
			super::push(out, 13890);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 13891);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16215);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16216);
					match s {
						[] => true,
						['q', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 13892);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'u', 'c', 'o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13893);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'r', 't', 's', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13894);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 16217);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 16218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itaturus.
pub(crate) fn n15845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10451);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13896);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
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
				['u', s @ ..] => {
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
						['d', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 13897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16219);
					match s {
						[] => true,
						['l', 'l', 'o', s @ ..] => {
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
						['r', 'b', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16220);
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
					super::push(out, 16221);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17506);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 17507);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 17508);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17509);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 17510);
							match s {
								[] => true,
								['s', s @ ..] => {
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
												['n', 'o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'o', 'c', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13898);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16223);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16225);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17511);
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
						['p', s @ ..] => {
							super::push(out, 17512);
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17513);
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
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
					super::push(out, 16226);
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
		['g', s @ ..] => {
			super::push(out, 13899);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17514);
							match s {
								[] => true,
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17515);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 17516);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => super::t13::n36243(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t13::n29734(s, out),
				['o', s @ ..] => super::t13::n29735(s, out),
				['r', s @ ..] => super::t13::n29736(s, out),
				['u', s @ ..] => super::t13::n29737(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t13::n23236(s, out),
		['m', s @ ..] => super::t13::n23237(s, out),
		['n', s @ ..] => super::t13::n23238(s, out),
		['p', s @ ..] => super::t13::n23239(s, out),
		['r', s @ ..] => super::t13::n23240(s, out),
		['s', s @ ..] => super::t13::n23241(s, out),
		['t', s @ ..] => super::t13::n23242(s, out),
		['u', s @ ..] => super::t13::n23243(s, out),
		['v', s @ ..] => super::t13::n23244(s, out),
		_ => false,
	}
}

/// Words ending -ltaturus.
pub(crate) fn n15846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10452);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13909);
			match s {
				[] => true,
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
		['u', 's', s @ ..] => {
			super::push(out, 13910);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16259);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17526);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntaturus.
pub(crate) fn n15847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10453);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13911);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16260);
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
								['d', s @ ..] => {
									super::push(out, 17983);
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
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 16261);
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
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 17984);
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
		['e', s @ ..] => {
			super::push(out, 13912);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16262);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16263);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17528);
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
						['e', s @ ..] => {
							super::push(out, 17529);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 17530);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17531);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 17532);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17533);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16264);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 16265);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
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
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16266);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 16267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otaturus.
pub(crate) fn n15848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10454);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 13913);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 13914);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 16268);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16269);
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
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17535);
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
		['r', s @ ..] => {
			super::push(out, 13917);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16270);
					match s {
						[] => true,
						['e', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -ptaturus.
pub(crate) fn n15849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10455);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13919);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16271);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16272);
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
		['m', 'e', 't', s @ ..] => {
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13920);
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

