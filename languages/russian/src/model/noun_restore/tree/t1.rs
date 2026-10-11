use flexion::Path;

/// Words ending -ер.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1735);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3064);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 4393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 3065);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1740);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3067);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3068);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 4394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['и', 'в', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3069);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4395);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 4396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3071);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 4397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['л', 'б', s @ ..] => {
							super::push(out, 3072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1749);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3073);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 4398);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 3074);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 3076);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1751);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1753);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3078);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3079);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3080);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4400);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4401);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1754);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3082);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3084);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4403);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 4404);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 4405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3085);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 3086);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4406);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 4407);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1755);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 1757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 767);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 3087);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1759);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1760);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1761);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3089);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3090);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 4409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 3091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t1::n4669(s, out),
								['н', s @ ..] => super::t1::n4670(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t1::n1830(s, out),
				['и', s @ ..] => super::t1::n1831(s, out),
				['й', s @ ..] => super::t1::n1832(s, out),
				['о', 'п', s @ ..] => super::t1::n1833(s, out),
				['у', s @ ..] => super::t1::n1834(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t1::n788(s, out),
		['к', s @ ..] => super::t1::n789(s, out),
		['л', s @ ..] => super::t1::n790(s, out),
		['м', s @ ..] => super::t1::n791(s, out),
		['н', s @ ..] => super::t1::n792(s, out),
		['п', s @ ..] => super::t1::n793(s, out),
		['р', s @ ..] => super::t1::n794(s, out),
		['с', s @ ..] => super::t1::n795(s, out),
		['т', s @ ..] => super::t1::n796(s, out),
		['у', s @ ..] => super::t1::n797(s, out),
		['ф', s @ ..] => super::t1::n798(s, out),
		['х', s @ ..] => super::t1::n799(s, out),
		['ц', s @ ..] => super::t1::n800(s, out),
		['ч', s @ ..] => super::t1::n801(s, out),
		['ш', s @ ..] => super::t1::n802(s, out),
		['щ', s @ ..] => super::t1::n803(s, out),
		['ь', s @ ..] => super::t1::n804(s, out),
		_ => false,
	}
}

/// Words ending -алет.
pub(crate) fn n1973(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3263);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3264);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3265);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 3266);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3267);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3268);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3269);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3270);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аблет.
pub(crate) fn n3457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3272);
	s.is_empty()
}

/// Words ending -облет.
pub(crate) fn n3458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3273);
	s.is_empty()
}

/// Words ending -ублет.
pub(crate) fn n3459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3274);
	s.is_empty()
}

/// Words ending -глет.
pub(crate) fn n1975(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 3275);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -длет.
pub(crate) fn n1976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1891);
	s.is_empty()
}

/// Words ending -елет.
pub(crate) fn n1977(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3277);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -злет.
pub(crate) fn n1978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1892);
	s.is_empty()
}

/// Words ending -илет.
pub(crate) fn n1979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1893);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3279);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3280);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -клет.
pub(crate) fn n1980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1894);
	s.is_empty()
}

/// Words ending -ллет.
pub(crate) fn n1981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1895);
	s.is_empty()
}

/// Words ending -млет.
pub(crate) fn n1982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1896);
	s.is_empty()
}

/// Words ending -нлет.
pub(crate) fn n1983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1897);
	s.is_empty()
}

/// Words ending -олет.
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1898);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3282);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3283);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4478);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4479);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3284);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3285);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4481);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3286);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3287);
			match s {
				[] => true,
				['а', 'с', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 5252);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3288);
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4483);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4484);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4485);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 4486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3289);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3290);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4487);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4488);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плет.
pub(crate) fn n1985(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3292);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3293);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3294);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3295);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слет.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1899);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3297);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тлет.
pub(crate) fn n1987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1900);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3299);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улет.
pub(crate) fn n1988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1901);
	s.is_empty()
}

/// Words ending -флет.
pub(crate) fn n1989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1902);
	s.is_empty()
}

/// Words ending -ылет.
pub(crate) fn n1990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1903);
	match s {
		[] => true,
		['в', '-', s @ ..] => {
			super::push(out, 3301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мет.
pub(crate) fn n838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1904);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3302);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3303);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3304);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3306);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4490);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1908);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1910);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 1911);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3311);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 3312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1913);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1914);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1915);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1916);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 1917);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нет.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 817);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1919);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1920);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1921);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пет.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 818);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1923);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рет.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	s.is_empty()
}

/// Words ending -сет.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	s.is_empty()
}

/// Words ending -тет.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	s.is_empty()
}

/// Words ending -фет.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	s.is_empty()
}

/// Words ending -цет.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	s.is_empty()
}

/// Words ending -чет.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1924);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['р', 'е', s @ ..] => {
					super::push(out, 3314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1926);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3316);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1927);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1928);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1929);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шет.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	s.is_empty()
}

/// Words ending -ьет.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	s.is_empty()
}

/// Words ending -ит.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 827);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1931);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 3318);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3320);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3321);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1933);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3323);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 1934);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4493);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1935);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 3325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1936);
					match s {
						[] => true,
						['е', 'л', 'к', s @ ..] => {
							super::push(out, 3326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1937);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3328);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1943);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1945);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1946);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3339);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'л', s @ ..] => {
					super::push(out, 1949);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						['о', 'х', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1957);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3346);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4496);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4497);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4499);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 3348);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4501);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 4502);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 3349);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4504);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3350);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3352);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 5254);
											s.is_empty()
										}
										['м', s @ ..] => super::t1::n5579(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t1::n2049(s, out),
				['о', s @ ..] => super::t1::n2050(s, out),
				['р', s @ ..] => super::t1::n2051(s, out),
				['т', s @ ..] => super::t1::n2052(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t1::n863(s, out),
		['т', s @ ..] => super::t1::n864(s, out),
		['ф', s @ ..] => super::t1::n865(s, out),
		['х', s @ ..] => super::t1::n866(s, out),
		['ц', s @ ..] => super::t1::n867(s, out),
		['ч', s @ ..] => super::t1::n868(s, out),
		_ => false,
	}
}

/// Words ending -йт.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['е', 'р', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 1977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кт.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	s.is_empty()
}

/// Words ending -лт.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 849);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нт.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1978);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3362);
							match s {
								[] => true,
								['л', 'к', '-', s @ ..] => {
									super::push(out, 4510);
									s.is_empty()
								}
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

/// Words ending -от.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['б', 'е', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 1979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1980);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3363);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 3365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1981);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 855);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1983);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1984);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1985);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пт.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	s.is_empty()
}

/// Words ending -рт.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 858);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1987);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1988);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3366);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3367);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3368);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3369);
							match s {
								[] => true,
								['к', 'к', s @ ..] => {
									super::push(out, 4511);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4512);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1989);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1990);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 1991);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 1992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ст.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 862);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1994);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3370);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3371);
							s.is_empty()
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4514);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4515);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1995);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 3372);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4517);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4518);
									match s {
										[] => true,
										['л', s @ ..] => {
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 5683);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 5684);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 5256);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 5257);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 5258);
											s.is_empty()
										}
										['ф', s @ ..] => {
											super::push(out, 5259);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3374);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3375);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 5260);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 5261);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 4519);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 4520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 3376);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2001);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2002);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2003);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тт.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	s.is_empty()
}

/// Words ending -ут.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2005);
					match s {
						[] => true,
						['с', 'л', s @ ..] => {
							super::push(out, 3377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фт.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	s.is_empty()
}

/// Words ending -хт.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	s.is_empty()
}

/// Words ending -ыт.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	s.is_empty()
}

/// Words ending -ьт.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	s.is_empty()
}

/// Words ending -эт.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	s.is_empty()
}

/// Words ending -ют.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	s.is_empty()
}

/// Words ending -ят.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	s.is_empty()
}

/// Words ending -ревец.
pub(crate) fn n3579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3384);
	s.is_empty()
}

/// Words ending -чевец.
pub(crate) fn n3580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3385);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5266);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 5267);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 4527);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щевец.
pub(crate) fn n3581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3386);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4528);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4529);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьевец.
pub(crate) fn n3582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3387);
	s.is_empty()
}

/// Words ending -ивец.
pub(crate) fn n2106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2015);
	s.is_empty()
}

/// Words ending -овец.
pub(crate) fn n2107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2016);
	match s {
		[] => true,
		['д', 'е', s @ ..] => {
			super::push(out, 3388);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3389);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4531);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4532);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5269);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'е', s @ ..] => {
			super::push(out, 3390);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 4533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'е', s @ ..] => {
			super::push(out, 3391);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'е', s @ ..] => {
			super::push(out, 3392);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3393);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4535);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5271);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4536);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5272);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 5273);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4537);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4538);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3394);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3395);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гец.
pub(crate) fn n904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 882);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['и', 'п', s @ ..] => {
					super::push(out, 3397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2018);
			match s {
				[] => true,
				['о', 'б', 'е', s @ ..] => {
					super::push(out, 3398);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дец.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 883);
	s.is_empty()
}

/// Words ending -еец.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 884);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2019);
			match s {
				[] => true,
				['р', 'а', 'о', 'н', s @ ..] => {
					super::push(out, 3399);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4540);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жец.
pub(crate) fn n907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 885);
	match s {
		[] => true,
		['р', 'о', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['б', 'е', s @ ..] => {
					super::push(out, 3400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зец.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 886);
	s.is_empty()
}

/// Words ending -иец.
pub(crate) fn n909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 887);
	s.is_empty()
}

/// Words ending -лец.
pub(crate) fn n910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 888);
	s.is_empty()
}

/// Words ending -мец.
pub(crate) fn n911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 889);
	s.is_empty()
}

/// Words ending -нец.
pub(crate) fn n912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 890);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2021);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3401);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3402);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3403);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3404);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 3405);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3406);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 4546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 3407);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4547);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5275);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 5276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 4548);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2023);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3408);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'г', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4551);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рец.
pub(crate) fn n913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2026);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3410);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3411);
					match s {
						[] => true,
						['о', 'о', 'н', s @ ..] => {
							super::push(out, 4553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3412);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3413);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4554);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сец.
pub(crate) fn n914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	match s {
		[] => true,
		['о', 'н', 'о', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3414);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4556);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 5277);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 5278);
									s.is_empty()
								}
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

/// Words ending -тец.
pub(crate) fn n915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2030);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['л', 'п', 'е', s @ ..] => {
					super::push(out, 3415);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2032);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4557);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2033);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зователь.
pub(crate) fn n6026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5686);
	match s {
		[] => true,
		['а', 'р', 'б', 'о', 'о', s @ ..] => {
			super::push(out, 5885);
			match s {
				[] => true,
				['к', 'н', s @ ..] => {
					super::push(out, 6071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 5886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тыватель.
pub(crate) fn n6027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5687);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5887);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ткодержатель.
pub(crate) fn n6028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5688);
	s.is_empty()
}

/// Words ending -к-испытатель.
pub(crate) fn n6029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5689);
	s.is_empty()
}

/// Words ending -итель.
pub(crate) fn n3628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3431);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4568);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5290);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5889);
									s.is_empty()
								}
								['с', 'и', '-', s @ ..] => {
									super::push(out, 5890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4569);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4570);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4571);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5291);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							super::push(out, 5690);
							match s {
								[] => true,
								['р', 'п', 'с', 'а', 'р', '-', s @ ..] => {
									super::push(out, 5891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5292);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5892);
									match s {
										[] => true,
										['о', 'н', s @ ..] => {
											super::push(out, 6072);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 5893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4572);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4573);
			match s {
				[] => true,
				['р', 'т', 'с', 'о', 'т', s @ ..] => {
					super::push(out, 5293);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5691);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 5894);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 5895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4574);
			match s {
				[] => true,
				['о', 'н', 'о', 'б', s @ ..] => {
					super::push(out, 5294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нь.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2077);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2078);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 2079);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пь.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	s.is_empty()
}

/// Words ending -рь.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['а', 'б', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 930);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сь.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2082);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4576);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 4577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ть.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 931);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 932);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 933);
			s.is_empty()
		}
		['о', 'г', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 935);
			s.is_empty()
		}
		['с', s @ ..] => super::t1::n963(s, out),
		['у', s @ ..] => {
			super::push(out, 937);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 938);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чь.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 941);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3448);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 4650);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 4651);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шь.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	s.is_empty()
}

/// Words ending -щь.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	s.is_empty()
}

/// Words ending -тная.
pub(crate) fn n2198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2102);
	match s {
		[] => true,
		['е', 'л', s @ ..] => {
			super::push(out, 3467);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4668);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3468);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4670);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5383);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ечная.
pub(crate) fn n3671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3469);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 4672);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шная.
pub(crate) fn n2200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2104);
	s.is_empty()
}

/// Words ending -ьная.
pub(crate) fn n2201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2105);
	s.is_empty()
}

/// Words ending -тая.
pub(crate) fn n975(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2106);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чая.
pub(crate) fn n976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	s.is_empty()
}

/// Words ending -щая.
pub(crate) fn n977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	s.is_empty()
}

/// Words ending -ея.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	s.is_empty()
}

/// Words ending -ия.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 949);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2108);
					match s {
						[] => true,
						['ф', 'м', 'а', '-', 'т', s @ ..] => {
							super::push(out, 3471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'ф', 'о', 'т', s @ ..] => {
					super::push(out, 2109);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3472);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4673);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 4674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 950);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 951);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2111);
					match s {
						[] => true,
						['о', 'м', '-', s @ ..] => {
							super::push(out, 3473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 952);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2113);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 3474);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2114);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3475);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3476);
							match s {
								[] => true,
								['и', 'ф', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5387);
											match s {
												[] => true,
												['е', s @ ..] => {
													match s {
														[] => true,
														['д', s @ ..] => super::t1::n6400(s, out),
														['з', s @ ..] => super::t1::n6401(s, out),
														_ => false,
													}
												}
												['л', s @ ..] => {
													super::push(out, 5800);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 5388);
											s.is_empty()
										}
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

/// Words ending -ля.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ня.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 2116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2117);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2119);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3478);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							super::push(out, 3480);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 4676);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4677);
									match s {
										[] => true,
										['м', 'о', s @ ..] => {
											super::push(out, 5389);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'д', s @ ..] => {
							super::push(out, 3481);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 4678);
									match s {
										[] => true,
										['р', 'п', 'о', 'к', s @ ..] => {
											super::push(out, 5390);
											s.is_empty()
										}
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

/// Words ending -тя.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 2120);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2121);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ья.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2122);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 3482);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ддевка.
pub(crate) fn n3695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3493);
	s.is_empty()
}

/// Words ending -здевка.
pub(crate) fn n3696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3494);
	s.is_empty()
}

/// The word членодевка.
pub(crate) fn n3697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3495);
	s.is_empty()
}

/// Words ending -рдевка.
pub(crate) fn n3698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3496);
	s.is_empty()
}

/// Words ending -еевка.
pub(crate) fn n2248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2147);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жевка.
pub(crate) fn n2249(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3498);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3499);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3500);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рязевка.
pub(crate) fn n2250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2148);
	s.is_empty()
}

/// Words ending -иевка.
pub(crate) fn n2251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2149);
	s.is_empty()
}

/// Words ending -левка.
pub(crate) fn n2252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2150);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3502);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3503);
			match s {
				[] => true,
				['у', 'р', 'х', s @ ..] => {
					super::push(out, 4679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5391);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4680);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 5393);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3505);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3506);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -невка.
pub(crate) fn n2253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2151);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3508);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3509);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3510);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4682);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4683);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 4684);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5395);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4685);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', 'е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4687);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3511);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3512);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3513);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3514);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 3515);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оевка.
pub(crate) fn n2254(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3516);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3517);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -певка.
pub(crate) fn n2255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2152);
	s.is_empty()
}

/// Words ending -ревка.
pub(crate) fn n2256(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3519);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3520);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -севка.
pub(crate) fn n2257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2153);
	s.is_empty()
}

/// Words ending -тевка.
pub(crate) fn n2258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2154);
	s.is_empty()
}

/// Words ending -уевка.
pub(crate) fn n2259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2155);
	s.is_empty()
}

/// Words ending -цевка.
pub(crate) fn n2260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2156);
	s.is_empty()
}

/// Words ending -чевка.
pub(crate) fn n2261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2157);
	s.is_empty()
}

/// Words ending -шевка.
pub(crate) fn n2262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2158);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 3522);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щевка.
pub(crate) fn n2263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2159);
	s.is_empty()
}

/// Words ending -ьевка.
pub(crate) fn n2264(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3524);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юевка.
pub(crate) fn n2265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2160);
	s.is_empty()
}

/// Words ending -ивка.
pub(crate) fn n1040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1009);
	s.is_empty()
}

/// Words ending -лвка.
pub(crate) fn n1041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1010);
	s.is_empty()
}

/// Words ending -овка.
pub(crate) fn n1042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1011);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2161);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3526);
					match s {
						[] => true,
						['х', 'х', s @ ..] => {
							super::push(out, 4689);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5397);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 5398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2163);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2164);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2165);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 3528);
					match s {
						[] => true,
						['ю', 'д', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 5399);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 5400);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5801);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 5802);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3529);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2166);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3531);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3532);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2167);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3533);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3534);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4691);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3535);
					match s {
						[] => true,
						['е', 'м', 'о', s @ ..] => {
							super::push(out, 4693);
							match s {
								[] => true,
								['л', 'и', 'к', 'х', 'е', s @ ..] => {
									super::push(out, 5401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2168);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -увка.
pub(crate) fn n1043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1012);
	s.is_empty()
}

/// Words ending -ывка.
pub(crate) fn n1044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1013);
	s.is_empty()
}

/// Words ending -явка.
pub(crate) fn n1045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1014);
	s.is_empty()
}

/// Words ending -дка.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 3536);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1016);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3537);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2173);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2174);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2176);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1018);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1020);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['р', 'х', s @ ..] => {
					super::push(out, 2178);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3539);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ека.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1024);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жка.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['э', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4695);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 4696);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5402);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 5403);
											s.is_empty()
										}
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
		['е', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2186);
					s.is_empty()
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 4697);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 3541);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 3542);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 3543);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3544);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3546);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3547);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4699);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3548);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3549);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3550);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3551);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 3552);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3554);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3556);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3557);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3558);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 4701);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2191);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3559);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4703);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1028);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['р', 'е', 'д', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3561);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2195);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3563);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1032);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1033);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зка.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['л', 'г', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3565);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2199);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3567);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3569);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2201);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 2202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1036);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ика.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	s.is_empty()
}

/// Words ending -йка.
pub(crate) fn n360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['и', 'л', 'х', s @ ..] => {
							super::push(out, 3571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кка.
pub(crate) fn n361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 359);
	s.is_empty()
}

/// Words ending -лка.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 3572);
							match s {
								[] => true,
								['ж', 'ы', 'в', 'о', 'з', s @ ..] => {
									super::push(out, 4706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2206);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3573);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 4707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				['п', 'е', 'р', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2209);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3574);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2211);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3576);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3577);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4708);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 4709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2213);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 3579);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2215);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 3581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2216);
					match s {
						[] => true,
						['а', 'д', s @ ..] => {
							super::push(out, 3582);
							match s {
								[] => true,
								['о', 'з', s @ ..] => {
									super::push(out, 4710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2217);
					match s {
						[] => true,
						['т', 'с', 'х', s @ ..] => {
							super::push(out, 3583);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4711);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', 'у', s @ ..] => {
					super::push(out, 2218);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 3584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мка.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 5404);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 5405);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 4713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2224);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3585);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3587);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1049);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1052);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1053);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1054);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 1055);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нка.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3589);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 4714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2236);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 3590);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3591);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3592);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 5406);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 5407);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5408);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 5409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 4715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2237);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2238);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3593);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3594);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4716);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 4717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', 'о', s @ ..] => {
							super::push(out, 3595);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3596);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2240);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 3598);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3599);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 3600);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3601);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3602);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3603);
							s.is_empty()
						}
						['я', 'р', 'п', s @ ..] => {
							super::push(out, 3604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['л', s @ ..] => super::t1::n2353(s, out),
				['м', s @ ..] => super::t1::n2354(s, out),
				['н', s @ ..] => super::t1::n2355(s, out),
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => super::t1::n3833(s, out),
						['д', s @ ..] => super::t1::n3834(s, out),
						['л', s @ ..] => super::t1::n3835(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t1::n2357(s, out),
				['р', s @ ..] => super::t1::n2358(s, out),
				['с', s @ ..] => super::t1::n2359(s, out),
				['т', s @ ..] => super::t1::n2360(s, out),
				['ц', s @ ..] => super::t1::n2361(s, out),
				['ч', s @ ..] => super::t1::n2362(s, out),
				['ш', s @ ..] => super::t1::n2363(s, out),
				['щ', s @ ..] => super::t1::n2364(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n1091(s, out),
		['о', s @ ..] => super::t1::n1092(s, out),
		['у', s @ ..] => super::t1::n1093(s, out),
		['ы', s @ ..] => super::t1::n1094(s, out),
		['я', s @ ..] => super::t1::n1095(s, out),
		_ => false,
	}
}

/// Words ending -ока.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	s.is_empty()
}

/// Words ending -пка.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1064);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3662);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3663);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3664);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3665);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3666);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3667);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3668);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 4766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3669);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3670);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3671);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3673);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3674);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 4767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1068);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1069);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рка.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2258);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3676);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 4768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3677);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2259);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3679);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3680);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4769);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 5422);
											s.is_empty()
										}
										['э', s @ ..] => {
											super::push(out, 5423);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 4770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 3682);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3683);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3684);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2261);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4771);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 3686);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2262);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3688);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3690);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2264);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							super::push(out, 3692);
							s.is_empty()
						}
						['и', 'р', 'г', s @ ..] => {
							super::push(out, 3693);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3694);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 3695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2265);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3696);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4773);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 4774);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 4775);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 4776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 3697);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3698);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3699);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 4777);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2266);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 3700);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 3701);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2267);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3703);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 3704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2268);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							super::push(out, 3705);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3706);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4779);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 4780);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3707);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									super::push(out, 4782);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 3708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2271);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2272);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2273);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 2274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1074);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1075);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1077);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ска.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1080);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2275);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2276);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2277);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3709);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3710);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4784);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2278);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 3711);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3712);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3713);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1083);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тка.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1085);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 3715);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 4786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3716);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3717);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3718);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3719);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3720);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3721);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4787);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 4788);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4789);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 4790);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 4791);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4792);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 3722);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3723);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3724);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3726);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3727);
							s.is_empty()
						}
						['д', 'о', s @ ..] => {
							super::push(out, 3728);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3729);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 4794);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3730);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2297);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3733);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3734);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1087);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1088);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2300);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 3737);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4796);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 3738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3739);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 4798);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3740);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 3741);
							s.is_empty()
						}
						['р', 'е', s @ ..] => {
							super::push(out, 3742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3743);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5424);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 5425);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 5803);
													s.is_empty()
												}
												['ф', s @ ..] => {
													super::push(out, 5804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['в', 'х', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4800);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 4801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t1::n2427(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t1::n1124(s, out),
		['ы', s @ ..] => super::t1::n1125(s, out),
		['я', s @ ..] => super::t1::n1126(s, out),
		_ => false,
	}
}

/// Words ending -ука.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	s.is_empty()
}

/// Words ending -фка.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	s.is_empty()
}

/// Words ending -цка.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1095);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1096);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чка.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1098);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['ж', 'е', s @ ..] => {
					super::push(out, 2311);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3748);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2314);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3751);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3752);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4806);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 4807);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 4808);
									s.is_empty()
								}
								['п', 'е', s @ ..] => {
									super::push(out, 4809);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 2316);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1100);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3755);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3756);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3758);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3759);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4811);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 5429);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 5430);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 5431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 4812);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4813);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4814);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4815);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 4816);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 4817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 3760);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t1::n2437(s, out),
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3769);
							s.is_empty()
						}
						['е', s @ ..] => super::t1::n3995(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t1::n2439(s, out),
				['м', s @ ..] => super::t1::n2440(s, out),
				['н', s @ ..] => super::t1::n2441(s, out),
				['п', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['р', s @ ..] => super::t1::n2443(s, out),
				['с', s @ ..] => super::t1::n2444(s, out),
				['т', s @ ..] => super::t1::n2445(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1102);
			s.is_empty()
		}
		['ы', s @ ..] => super::t1::n1135(s, out),
		['ю', s @ ..] => super::t1::n1136(s, out),
		['я', s @ ..] => super::t1::n1137(s, out),
		_ => false,
	}
}

/// Words ending -шка.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1106);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2321);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2322);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3798);
							s.is_empty()
						}
						['е', 'ц', '\u{1}', s @ ..] => {
							super::push(out, 3799);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3800);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3802);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3803);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3804);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3806);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'и', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3808);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3810);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4870);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 4871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3811);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3812);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3813);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 3814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2328);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 3815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1108);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3816);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2333);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3817);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3818);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4873);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 4874);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4875);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 4876);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 4877);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 4878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1109);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['б', 'е', s @ ..] => {
					super::push(out, 2335);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3821);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'е', s @ ..] => {
					super::push(out, 2336);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3823);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2337);
					s.is_empty()
				}
				['л', 'е', s @ ..] => {
					super::push(out, 2338);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2339);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2340);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3826);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 4879);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5452);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 5453);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 4880);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 3827);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2342);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2343);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3829);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4882);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3830);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 5454);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 5455);
											s.is_empty()
										}
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
		['ы', s @ ..] => {
			super::push(out, 1111);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1112);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыка.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	s.is_empty()
}

/// Words ending -ька.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1114);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 3831);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4884);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4885);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 3832);
							s.is_empty()
						}
						['ш', 'е', s @ ..] => {
							super::push(out, 3833);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 3834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3835);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4886);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3836);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4888);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юка.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	s.is_empty()
}

/// Words ending -яка.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	s.is_empty()
}

/// Words ending -етина.
pub(crate) fn n2516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2385);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4894);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ечина.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1151);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2386);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 2387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щина.
pub(crate) fn n1188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1153);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3860);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4895);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4896);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4897);
							s.is_empty()
						}
						['о', 'р', s @ ..] => {
							super::push(out, 4898);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'д', s @ ..] => {
							super::push(out, 4899);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 4900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2388);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3862);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3863);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4901);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5457);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5458);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 5459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 4902);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2389);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -она.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	s.is_empty()
}

/// Words ending -рна.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	s.is_empty()
}

/// Words ending -уна.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					super::push(out, 2391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ница.
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1186);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2418);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3875);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4906);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 4907);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4908);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 4909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2420);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2421);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 2422);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3877);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3878);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4910);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4911);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4912);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4913);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4914);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5460);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 5461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 5462);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 5463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2423);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3881);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3882);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5464);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 5465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2424);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3884);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2425);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3886);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4917);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 5466);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 4918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2427);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3888);
					match s {
						[] => true,
						['а', 'л', 'к', 'х', s @ ..] => {
							super::push(out, 4919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2428);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3889);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 4920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3890);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3891);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4921);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5468);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5469);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 4922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3892);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3893);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4923);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 4924);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 4925);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4926);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5470);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 4927);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5472);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5474);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4928);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4929);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5476);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5477);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 5807);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 5808);
											s.is_empty()
										}
										['щ', s @ ..] => {
											super::push(out, 5809);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 5478);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3894);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3895);
					match s {
						[] => true,
						['с', 'ы', 'т', 'х', s @ ..] => {
							super::push(out, 4930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2430);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3896);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4931);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 5480);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -едчица.
pub(crate) fn n4124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3897);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4932);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зчица.
pub(crate) fn n2569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2437);
	s.is_empty()
}

/// Words ending -счица.
pub(crate) fn n2570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2438);
	s.is_empty()
}

/// Words ending -тчица.
pub(crate) fn n2571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2439);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3898);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4934);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 5482);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 5810);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4935);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4936);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4937);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							super::push(out, 5483);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5484);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5486);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4938);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4939);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 4940);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3900);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3901);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3902);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3903);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щица.
pub(crate) fn n1230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1191);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2440);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3906);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2441);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3908);
					match s {
						[] => true,
						['ь', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 5488);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5489);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2442);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2443);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3910);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3911);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4943);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 4944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3912);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3913);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3915);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3916);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4945);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4946);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 4947);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4948);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5491);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 4949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3917);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2444);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3919);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5493);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 5494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 4951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3921);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3922);
					match s {
						[] => true,
						['в', 'о', 'к', s @ ..] => {
							super::push(out, 4952);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4953);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4954);
							s.is_empty()
						}
						['н', 'и', s @ ..] => {
							super::push(out, 4955);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4956);
							s.is_empty()
						}
						['т', 'р', s @ ..] => {
							super::push(out, 4957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3923);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3924);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2446);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							super::push(out, 4958);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5495);
									match s {
										[] => true,
										['м', 'о', s @ ..] => {
											super::push(out, 5812);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'д', s @ ..] => {
							super::push(out, 4959);
							match s {
								[] => true,
								['я', 'р', 'п', 'о', s @ ..] => {
									super::push(out, 5496);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 5813);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 5814);
											s.is_empty()
										}
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

/// Words ending -йца.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	s.is_empty()
}

/// Words ending -ние.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2500);
					s.is_empty()
				}
				['в', s @ ..] => super::t1::n2640(s, out),
				['г', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3983);
							match s {
								[] => true,
								['е', 'д', s @ ..] => {
									super::push(out, 4997);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2505);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3984);
							match s {
								[] => true,
								['е', 'м', s @ ..] => {
									super::push(out, 4999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3985);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5000);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2510);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2511);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2513);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3986);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 5002);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											super::push(out, 5554);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 2514);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 2515);
					s.is_empty()
				}
				['ч', 'и', 'н', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3987);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => super::t1::n4219(s, out),
						['р', s @ ..] => {
							super::push(out, 3989);
							match s {
								[] => true,
								['е', s @ ..] => super::t1::n5306(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => super::t1::n2656(s, out),
				['щ', s @ ..] => super::t1::n2657(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t1::n1316(s, out),
		['я', s @ ..] => super::t1::n1317(s, out),
		_ => false,
	}
}

/// Words ending -мелье.
pub(crate) fn n2743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2596);
	s.is_empty()
}

/// Words ending -телье.
pub(crate) fn n2744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2597);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4032);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4033);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5023);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5024);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илье.
pub(crate) fn n1386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1341);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2598);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2600);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4035);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4036);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олье.
pub(crate) fn n1387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1342);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2601);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2602);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 4037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2603);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2604);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2605);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4038);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4040);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улье.
pub(crate) fn n1388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1343);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2607);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 2608);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ылье.
pub(crate) fn n1389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1344);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2610);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мье.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2611);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2612);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нье.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2614);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 4041);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 5025);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 5567);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', 'р', s @ ..] => {
									super::push(out, 5026);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5568);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 5569);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 5855);
													s.is_empty()
												}
												['ч', s @ ..] => {
													super::push(out, 5856);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4042);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4043);
							s.is_empty()
						}
						['ы', 'в', s @ ..] => {
							super::push(out, 4044);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 5027);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2615);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4045);
							s.is_empty()
						}
						['и', 'л', s @ ..] => {
							super::push(out, 4046);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4047);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4048);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5029);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2616);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2617);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2618);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2619);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4050);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4051);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4052);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2620);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 4054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2621);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4055);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 5033);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4056);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4058);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 4059);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 4061);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4062);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2623);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2624);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 2625);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4064);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 5036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 2626);
					match s {
						[] => true,
						['и', 'н', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5037);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5570);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 5038);
									match s {
										[] => true,
										['е', 'ь', s @ ..] => {
											super::push(out, 5571);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 5039);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5572);
											s.is_empty()
										}
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
		['е', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2627);
					match s {
						[] => true,
						['р', 'т', 'с', 'о', s @ ..] => {
							super::push(out, 4065);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2630);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пье.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2632);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1353);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2634);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рье.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1355);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2636);
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 4067);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2637);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2638);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2639);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2640);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2641);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2642);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4069);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'у', s @ ..] => {
					super::push(out, 2644);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 2645);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2647);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 4071);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 4072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2648);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2649);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2650);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2652);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1358);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1359);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сье.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4075);
							s.is_empty()
						}
						['г', 'о', s @ ..] => {
							super::push(out, 4076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тье.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2655);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2658);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2660);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 2661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'з', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2663);
					s.is_empty()
				}
				['ж', 'е', s @ ..] => {
					super::push(out, 2664);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4078);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2665);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4080);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'б', 'о', s @ ..] => {
					super::push(out, 2666);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2668);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2669);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							super::push(out, 4082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4083);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 4084);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 4085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2670);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1366);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2672);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4086);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2673);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2674);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2675);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2677);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4089);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4090);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4091);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4092);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4093);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'о', s @ ..] => {
					super::push(out, 2678);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2679);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2680);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2681);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2682);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2683);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2684);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2686);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чье.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1370);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2689);
					s.is_empty()
				}
				['р', 'б', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1371);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2692);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2694);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2696);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2699);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ушье.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['д', 'о', s @ ..] => {
			super::push(out, 1373);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1374);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ник.
pub(crate) fn n648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1537);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1538);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1539);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2825);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4147);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4148);
							s.is_empty()
						}
						['ч', 'и', s @ ..] => {
							super::push(out, 4149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1540);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 2827);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4150);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4151);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5074);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 5075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2828);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4152);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									super::push(out, 5076);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 5588);
											match s {
												[] => true,
												['д', 'ж', 'е', 'м', '-', s @ ..] => {
													super::push(out, 5859);
													s.is_empty()
												}
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
		['е', s @ ..] => {
			super::push(out, 1541);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2829);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2830);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 4154);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4155);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4156);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4157);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5077);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 5079);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4158);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 5081);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 5082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2831);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2832);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2833);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1543);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2835);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2836);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4159);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 5083);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 5084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4160);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5085);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5086);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1544);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					super::push(out, 2839);
					match s {
						[] => true,
						['а', 'л', 'х', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2841);
					match s {
						[] => true,
						['а', 'н', 'о', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4165);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 5589);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 5590);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 4170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2842);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2843);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2844);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 5088);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 5089);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5591);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 5592);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2846);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t1::n1604(s, out),
		['о', s @ ..] => {
			super::push(out, 1547);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1548);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2852);
					s.is_empty()
				}
				['е', s @ ..] => super::t1::n3010(s, out),
				['о', s @ ..] => {
					super::push(out, 2854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t1::n1608(s, out),
		['т', s @ ..] => super::t1::n1609(s, out),
		['х', s @ ..] => {
			super::push(out, 1552);
			s.is_empty()
		}
		['ч', s @ ..] => super::t1::n1611(s, out),
		['ш', s @ ..] => super::t1::n1612(s, out),
		['щ', s @ ..] => {
			super::push(out, 1555);
			s.is_empty()
		}
		['ь', s @ ..] => super::t1::n1614(s, out),
		['я', s @ ..] => {
			super::push(out, 1557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чик.
pub(crate) fn n654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2887);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4224);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1567);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2889);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2890);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4226);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 4227);
							match s {
								[] => true,
								['з', 'а', 'р', '-', s @ ..] => {
									super::push(out, 5129);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 5607);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 1570);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2892);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2893);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4228);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4229);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4230);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4231);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 4232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2894);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2895);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 2896);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1572);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1573);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1574);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2899);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 4233);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4234);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4235);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4236);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4237);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 4238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2901);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1575);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1576);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4239);
							match s {
								[] => true,
								['х', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5608);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '-', s @ ..] => {
											super::push(out, 5609);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 5863);
													s.is_empty()
												}
												['ь', s @ ..] => {
													super::push(out, 5864);
													match s {
														[] => true,
														['л', s @ ..] => super::t1::n6425(s, out),
														['р', s @ ..] => super::t1::n6426(s, out),
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
						['д', s @ ..] => {
							super::push(out, 4240);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5130);
									match s {
										[] => true,
										['р', 'е', 'п', 'о', 'м', s @ ..] => {
											super::push(out, 5610);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 5131);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5132);
									match s {
										[] => true,
										['о', 'м', s @ ..] => {
											super::push(out, 5611);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 4241);
							match s {
								[] => true,
								['о', 'р', 'п', 'о', s @ ..] => {
									super::push(out, 5133);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 5612);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t2::n3063(s, out),
				['и', s @ ..] => {
					super::push(out, 2905);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2906);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2907);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2908);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1577);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4253);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4254);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5149);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											super::push(out, 5623);
											s.is_empty()
										}
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

/// Words ending -щик.
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 642);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2913);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4255);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4256);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 4257);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4258);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 5150);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5151);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4260);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 4261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2914);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2915);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2917);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							super::push(out, 4262);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 5624);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 5625);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2918);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5153);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 5154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 4264);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 5155);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5156);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 5626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 5157);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4265);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4266);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4267);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 4268);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 4269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2919);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2920);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 4270);
							match s {
								[] => true,
								['а', 'т', 'с', 'о', s @ ..] => {
									super::push(out, 5159);
									match s {
										[] => true,
										['п', '-', 'р', s @ ..] => {
											super::push(out, 5627);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 5868);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 5869);
													s.is_empty()
												}
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
		['й', s @ ..] => {
			super::push(out, 1582);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2921);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4271);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4272);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4273);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4274);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5160);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 5161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4275);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 4276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2923);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2924);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2925);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2926);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1584);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2928);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2929);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4277);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 4278);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4279);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5163);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5164);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 5165);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 4280);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5167);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 5628);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 5629);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2930);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t2::n1644(s, out),
		['р', s @ ..] => super::t2::n1645(s, out),
		['ь', s @ ..] => super::t2::n1646(s, out),
		_ => false,
	}
}

/// Words ending -енок.
pub(crate) fn n1662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1602);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2962);
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4302);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'р', 'о', s @ ..] => {
			super::push(out, 2963);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4304);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4305);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5185);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 5186);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 5187);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5188);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4306);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4307);
					s.is_empty()
				}
				['к', '\u{1}', s @ ..] => {
					super::push(out, 4308);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5190);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 5191);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 5192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4309);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'с', s @ ..] => {
			super::push(out, 2964);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2965);
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4311);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4313);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4314);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5193);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4316);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5195);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 5196);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 5197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4317);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 4318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2966);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2967);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4319);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4320);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4321);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4322);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4323);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4324);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5198);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5199);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5635);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 5636);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4325);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 2968);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 2969);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 2970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осок.
pub(crate) fn n1674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1614);
	s.is_empty()
}

/// Words ending -ток.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1615);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4331);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4332);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4333);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 4334);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5201);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 5202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1617);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2978);
					s.is_empty()
				}
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4335);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4336);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2979);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4338);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4339);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 4340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2980);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2981);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1621);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чок.
pub(crate) fn n672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	s.is_empty()
}

/// Words ending -шок.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	s.is_empty()
}

/// Words ending -лкоткачество.
pub(crate) fn n6427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6062);
	s.is_empty()
}

/// Words ending -пкоткачество.
pub(crate) fn n6428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6063);
	s.is_empty()
}

/// Words ending -емничество.
pub(crate) fn n6429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6064);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6188);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омничество.
pub(crate) fn n6430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6065);
	s.is_empty()
}

/// Words ending -ерничество.
pub(crate) fn n6431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6066);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 6190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -четничество.
pub(crate) fn n6432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6067);
	s.is_empty()
}

/// Words ending -чничество.
pub(crate) fn n6226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5874);
	match s {
		[] => true,
		['о', 'т', s @ ..] => {
			super::push(out, 6068);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6191);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 6192);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нство.
pub(crate) fn n3212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3042);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4361);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4362);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5214);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5215);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5639);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5875);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'о', 'д', '\u{1}', s @ ..] => {
							super::push(out, 5640);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5216);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 5217);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 5218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4363);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -епство.
pub(crate) fn n3213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3043);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 4365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рство.
pub(crate) fn n3214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3044);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4366);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4367);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5219);
					s.is_empty()
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5642);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5644);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5645);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 5646);
							s.is_empty()
						}
						['н', 'о', s @ ..] => {
							super::push(out, 5647);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 5220);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5649);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5650);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 5651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5221);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5652);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 5653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5222);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5654);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5655);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 5877);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 5878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 5656);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5658);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 5659);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5660);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5223);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5224);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5662);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 5879);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 6069);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 6070);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 5880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 5663);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 5664);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5665);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 5881);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 5882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5667);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 5669);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 5670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5671);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 5672);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 5673);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									super::push(out, 5883);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 5674);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5675);
							s.is_empty()
						}
						['р', 'о', s @ ..] => {
							super::push(out, 5676);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5225);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 5226);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 5227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4368);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4369);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4370);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 4371);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тство.
pub(crate) fn n3215(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4372);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4373);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5228);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5229);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5678);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5679);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4374);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьство.
pub(crate) fn n3216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3045);
	s.is_empty()
}

/// The word тазер.
pub(crate) fn n4669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4410);
	s.is_empty()
}

/// Words ending -нтазер.
pub(crate) fn n4670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4411);
	s.is_empty()
}

/// Words ending -езер.
pub(crate) fn n1830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1765);
	s.is_empty()
}

/// Words ending -изер.
pub(crate) fn n1831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1766);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3092);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4412);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4413);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 4414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4415);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4416);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5234);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 5235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йзер.
pub(crate) fn n1832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1767);
	s.is_empty()
}

/// Words ending -позер.
pub(crate) fn n1833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1768);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -узер.
pub(crate) fn n1834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1769);
	s.is_empty()
}

/// Words ending -йер.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 770);
	s.is_empty()
}

/// Words ending -кер.
pub(crate) fn n789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 771);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3095);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3096);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4419);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1770);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1771);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4421);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4423);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'с', s @ ..] => {
					super::push(out, 3098);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3099);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1772);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1773);
			s.is_empty()
		}
		['н', 'а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1774);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['а', 'м', 'о', 'и', 'д', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лер.
pub(crate) fn n790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', 'а', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['т', 'а', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4425);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1780);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1782);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1783);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4427);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1785);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1786);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1787);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4429);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 1788);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 1789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мер.
pub(crate) fn n791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1790);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['р', 'г', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['а', 'т', '-', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 4431);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1792);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 1793);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нер.
pub(crate) fn n792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1794);
			match s {
				[] => true,
				['в', 'ь', 'л', 'а', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3140);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 4433);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3141);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5236);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 5681);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 5682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 4435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4436);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1795);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 5237);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 4440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 3150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1796);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1797);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1798);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1799);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3152);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3156);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 4442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1800);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4443);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 4444);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 5239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1801);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'п', s @ ..] => {
			super::push(out, 1802);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1803);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пер.
pub(crate) fn n793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 775);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1805);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1806);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1807);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1808);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1809);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3165);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				['т', 'с', 'е', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4447);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1810);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1811);
			s.is_empty()
		}
		['с', 'е', s @ ..] => {
			super::push(out, 1812);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1813);
			s.is_empty()
		}
		['ш', 'и', s @ ..] => {
			super::push(out, 1814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рер.
pub(crate) fn n794(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1815);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1816);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сер.
pub(crate) fn n795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 776);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 1818);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1819);
			s.is_empty()
		}
		['и', 'в', s @ ..] => {
			super::push(out, 1820);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 1821);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3175);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 5240);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						['л', 'а', s @ ..] => {
							super::push(out, 4450);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'ф', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3180);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4452);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 4453);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1823);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тер.
pub(crate) fn n796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3182);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3184);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4455);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3186);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1825);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3189);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3190);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3191);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4457);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3192);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 3193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3194);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3195);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1826);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 1827);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4459);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4460);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4461);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4462);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3198);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1828);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1829);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3200);
					s.is_empty()
				}
				['е', 'м', s @ ..] => {
					super::push(out, 3201);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3202);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4464);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 3203);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3204);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3206);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3207);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3208);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3210);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4466);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 4467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3211);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3213);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4468);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4469);
							match s {
								[] => true,
								['с', 'к', s @ ..] => {
									super::push(out, 5242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 4470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3214);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3215);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5243);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5244);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 5245);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 3217);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3218);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4472);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 5247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1833);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3220);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 5248);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 5249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3223);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				['и', 'л', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1835);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 1836);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уер.
pub(crate) fn n797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 778);
	s.is_empty()
}

/// Words ending -фер.
pub(crate) fn n798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 779);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1839);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 1840);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1841);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1842);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1844);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1845);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 1846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хер.
pub(crate) fn n799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 780);
	s.is_empty()
}

/// Words ending -цер.
pub(crate) fn n800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 781);
	s.is_empty()
}

/// Words ending -чер.
pub(crate) fn n801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 782);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1847);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1848);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1849);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3238);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шер.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3240);
					match s {
						[] => true,
						['и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5250);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'т', s @ ..] => {
			super::push(out, 1852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щер.
pub(crate) fn n803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 783);
	s.is_empty()
}

/// Words ending -ьер.
pub(crate) fn n804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1853);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['е', 'р', 'п', '-', 'н', s @ ..] => {
					super::push(out, 3242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3243);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1856);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -махерит.
pub(crate) fn n5579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5255);
	s.is_empty()
}

/// Words ending -ирит.
pub(crate) fn n2049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1958);
	s.is_empty()
}

/// Words ending -орит.
pub(crate) fn n2050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1959);
	s.is_empty()
}

/// Words ending -ррит.
pub(crate) fn n2051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1960);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4506);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 4507);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трит.
pub(crate) fn n2052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1961);
	s.is_empty()
}

/// Words ending -сит.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 841);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1962);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1963);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1964);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1965);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'е', s @ ..] => {
			super::push(out, 1966);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3355);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1967);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тит.
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1968);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1969);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3357);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3358);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1970);
			match s {
				[] => true,
				['а', 'д', s @ ..] => {
					super::push(out, 3360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фит.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1972);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1973);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', 'н', s @ ..] => {
							super::push(out, 4508);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хит.
pub(crate) fn n866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	s.is_empty()
}

/// Words ending -цит.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['о', 'б', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чит.
pub(crate) fn n868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1975);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1976);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сть.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2086);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2087);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2088);
			s.is_empty()
		}
		['о', s @ ..] => super::t2::n2182(s, out),
		_ => false,
	}
}

/// Words ending -деефикация.
pub(crate) fn n6400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6035);
	s.is_empty()
}

/// Words ending -зеефикация.
pub(crate) fn n6401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6036);
	s.is_empty()
}

/// Words ending -ленка.
pub(crate) fn n2353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2242);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3605);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 5410);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5411);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 5412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4718);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4720);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5413);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5414);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4721);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4722);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3606);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3607);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3608);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4724);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4725);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4726);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4727);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4728);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4730);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4731);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 4732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3609);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3610);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3611);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', 'м', '-', s @ ..] => {
					super::push(out, 4733);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4734);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3612);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 3613);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -менка.
pub(crate) fn n2354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2243);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3615);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4736);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5416);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненка.
pub(crate) fn n2355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2244);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3616);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3617);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4737);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3618);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 3619);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 3620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -военка.
pub(crate) fn n3833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3621);
	s.is_empty()
}

/// Words ending -доенка.
pub(crate) fn n3834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3622);
	s.is_empty()
}

/// Words ending -лоенка.
pub(crate) fn n3835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3623);
	s.is_empty()
}

/// Words ending -пенка.
pub(crate) fn n2357(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3624);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3625);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ренка.
pub(crate) fn n2358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2245);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3627);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3628);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4739);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'м', s @ ..] => {
			super::push(out, 3629);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3630);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4741);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3631);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сенка.
pub(crate) fn n2359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2246);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3633);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 4743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3634);
			s.is_empty()
		}
		['о', 'р', s @ ..] => {
			super::push(out, 3635);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенка.
pub(crate) fn n2360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2247);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4744);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3637);
			match s {
				[] => true,
				['л', 'п', 'с', s @ ..] => {
					super::push(out, 4746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4747);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3638);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4749);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3639);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 3640);
			s.is_empty()
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4751);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ценка.
pub(crate) fn n2361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2248);
	s.is_empty()
}

/// Words ending -ченка.
pub(crate) fn n2362(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3642);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4753);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4754);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4755);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3643);
			s.is_empty()
		}
		['л', 'о', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4757);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3644);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3645);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шенка.
pub(crate) fn n2363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2249);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3647);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3648);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3649);
			s.is_empty()
		}
		['о', 'р', 'к', s @ ..] => {
			super::push(out, 3650);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3651);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3652);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щенка.
pub(crate) fn n2364(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3654);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5418);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 5419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4760);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5420);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3655);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3656);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инка.
pub(crate) fn n1091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1059);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2250);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3658);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 4761);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 4762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3659);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 4763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онка.
pub(crate) fn n1092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1060);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 2252);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 3660);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4764);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унка.
pub(crate) fn n1093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1061);
	s.is_empty()
}

/// Words ending -ынка.
pub(crate) fn n1094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1062);
	s.is_empty()
}

/// Words ending -янка.
pub(crate) fn n1095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1063);
	s.is_empty()
}

/// Words ending -рстка.
pub(crate) fn n2427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2309);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3745);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4802);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 5426);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5805);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 5806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4803);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 5427);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 5428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утка.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1092);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2310);
			match s {
				[] => true,
				['и', 'м', 'х', s @ ..] => {
					super::push(out, 3747);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4804);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытка.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1093);
	s.is_empty()
}

/// Words ending -ятка.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1094);
	s.is_empty()
}

/// Words ending -дочка.
pub(crate) fn n2437(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3762);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3763);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4818);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4819);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3764);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4821);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5432);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3765);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3766);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4823);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3767);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 3768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -езочка.
pub(crate) fn n3995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3770);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4825);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лочка.
pub(crate) fn n2439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2317);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3771);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3772);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4827);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4828);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4829);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4830);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4831);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 4832);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4833);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4834);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 4835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3773);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3774);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3775);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мочка.
pub(crate) fn n2440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2318);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4836);
					s.is_empty()
				}
				['с', 'е', s @ ..] => {
					super::push(out, 4837);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ночка.
pub(crate) fn n2441(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3778);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3779);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4838);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4839);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4840);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4841);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5434);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5435);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5436);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4842);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4843);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4844);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4845);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4846);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 4847);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3780);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3781);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 3782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рочка.
pub(crate) fn n2443(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3783);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3784);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5438);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4849);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4850);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4851);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5441);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4852);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3785);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3786);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3787);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сочка.
pub(crate) fn n2444(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3789);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4854);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точка.
pub(crate) fn n2445(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3791);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3792);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4856);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5443);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'д', s @ ..] => {
					super::push(out, 4857);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4858);
					s.is_empty()
				}
				['р', 'е', s @ ..] => {
					super::push(out, 4859);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 5445);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4860);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 4861);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4862);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5447);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 4863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3793);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3794);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3795);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4864);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4865);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 5449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4866);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'е', s @ ..] => {
			super::push(out, 3796);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4868);
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 5450);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ычка.
pub(crate) fn n1135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1103);
	s.is_empty()
}

/// Words ending -ючка.
pub(crate) fn n1136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1104);
	s.is_empty()
}

/// Words ending -ячка.
pub(crate) fn n1137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	s.is_empty()
}

/// Words ending -вание.
pub(crate) fn n2640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2501);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3976);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3977);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4970);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4971);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4972);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4973);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5499);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5500);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 5501);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5502);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5503);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5815);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5504);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 5505);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 5506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4974);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5507);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5508);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 5817);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 5818);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 6037);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 6038);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 6039);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 6040);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 6041);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 6042);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 5819);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 6043);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 6181);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 6182);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 5820);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 5821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5509);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4976);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5511);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5512);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 5513);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5822);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5824);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5825);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 6044);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 6045);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 5826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5827);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5828);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 6046);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 6183);
													match s {
														[] => true,
														['о', 'п', s @ ..] => super::t2::n6629(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 6184);
													match s {
														[] => true,
														['с', s @ ..] => super::t2::n6630(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 6047);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 5829);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 5830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5515);
							s.is_empty()
						}
						['ю', s @ ..] => super::t2::n5850(s, out),
						['я', s @ ..] => super::t2::n5851(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t2::n5277(s, out),
				['н', s @ ..] => super::t2::n5278(s, out),
				['р', s @ ..] => super::t2::n5279(s, out),
				['х', s @ ..] => super::t2::n5280(s, out),
				['ч', s @ ..] => super::t2::n5281(s, out),
				['ш', s @ ..] => super::t2::n5282(s, out),
				['щ', s @ ..] => super::t2::n5283(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t2::n4211(s, out),
		['у', s @ ..] => super::t2::n4212(s, out),
		['ы', s @ ..] => super::t2::n4213(s, out),
		_ => false,
	}
}

/// Words ending -зничание.
pub(crate) fn n4219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3988);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5555);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 5556);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерничание.
pub(crate) fn n5306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5004);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 5557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шание.
pub(crate) fn n2656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2516);
	s.is_empty()
}

/// Words ending -щание.
pub(crate) fn n2657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2517);
	s.is_empty()
}

/// Words ending -ение.
pub(crate) fn n1316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1274);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2518);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3990);
					s.is_empty()
				}
				['ж', 'о', 'в', s @ ..] => {
					super::push(out, 3991);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 5558);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5559);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 5849);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 5850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 3992);
					match s {
						[] => true,
						['р', 'п', 'о', 'к', s @ ..] => {
							super::push(out, 5005);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 5560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2519);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3993);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5006);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2520);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2521);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2522);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3995);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 3996);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5561);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['р', 'е', s @ ..] => {
											super::push(out, 5851);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 6055);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 6056);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 5852);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 6057);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 6058);
													match s {
														[] => true,
														['к', s @ ..] => super::t2::n6566(s, out),
														['л', s @ ..] => super::t2::n6567(s, out),
														_ => false,
													}
												}
												['ф', s @ ..] => {
													super::push(out, 6059);
													s.is_empty()
												}
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
		['р', 'у', s @ ..] => {
			super::push(out, 2523);
			match s {
				[] => true,
				['к', 'е', s @ ..] => {
					super::push(out, 3997);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 5008);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2524);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5010);
							match s {
								[] => true,
								['д', 'е', 'р', 'с', '\u{1}', s @ ..] => {
									super::push(out, 5562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2525);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					super::push(out, 3999);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5011);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 5563);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 5853);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 5854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'р', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5012);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2526);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 2527);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яние.
pub(crate) fn n1317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1275);
	s.is_empty()
}

/// Words ending -нник.
pub(crate) fn n1604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1546);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2848);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4171);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 5593);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 5594);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5860);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 5861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2849);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4173);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4174);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5090);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'е', s @ ..] => {
					super::push(out, 4175);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4176);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4177);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5595);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 5596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2850);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2851);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					super::push(out, 4178);
					match s {
						[] => true,
						['к', 'х', s @ ..] => {
							super::push(out, 5093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4179);
					match s {
						[] => true,
						['о', 'т', 'с', 'х', s @ ..] => {
							super::push(out, 5094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерник.
pub(crate) fn n3010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2853);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4180);
			match s {
				[] => true,
				['я', 'ъ', s @ ..] => {
					super::push(out, 5095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4181);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 5096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сник.
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1550);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2855);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 4183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2856);
			match s {
				[] => true,
				['л', 'о', 'п', 'х', 'е', s @ ..] => {
					super::push(out, 4184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тник.
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1551);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2857);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2858);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4185);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4186);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4187);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5097);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4188);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4189);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4190);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5100);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5101);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2859);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2860);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чник.
pub(crate) fn n1611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1553);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2862);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4191);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5103);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 5104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2863);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4192);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4193);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4194);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4195);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4196);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4197);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4198);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5106);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5107);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 4199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2864);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2865);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4200);
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5109);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5110);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4201);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4202);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5114);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									super::push(out, 5597);
									match s {
										[] => true,
										['с', 'х', s @ ..] => {
											super::push(out, 5862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4203);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5117);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 5598);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 5599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4204);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5119);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5120);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5121);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4205);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4206);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5123);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 5600);
									s.is_empty()
								}
								['ч', 'е', s @ ..] => {
									super::push(out, 5601);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 5602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'р', s @ ..] => {
							super::push(out, 5124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2866);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2867);
			match s {
				[] => true,
				['с', 'ы', 'т', 'т', s @ ..] => {
					super::push(out, 4207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шник.
pub(crate) fn n1612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1554);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2868);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2869);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4209);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4210);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 4211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2870);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2871);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4212);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5125);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2872);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4213);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							super::push(out, 5127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 2873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьник.
pub(crate) fn n1614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1556);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4215);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4216);
					match s {
						[] => true,
						['г', 'у', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 5603);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 5604);
									s.is_empty()
								}
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

/// Words ending -ль-перехватчик.
pub(crate) fn n6425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6060);
	s.is_empty()
}

/// Words ending -рь-перехватчик.
pub(crate) fn n6426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6061);
	s.is_empty()
}

