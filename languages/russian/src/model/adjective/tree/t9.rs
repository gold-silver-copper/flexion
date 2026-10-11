use flexion::Path;

/// Words ending -ькавший.
pub(crate) fn n5237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5018);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11015);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18310);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18311);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11016);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -якавший.
pub(crate) fn n5239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5020);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11018);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11019);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11020);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 18312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11021);
			match s {
				[] => true,
				['б', 'а', s @ ..] => {
					super::push(out, 18313);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11022);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мутавший.
pub(crate) fn n11667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11116);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -путавший.
pub(crate) fn n11668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11117);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 18412);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24886);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтавший.
pub(crate) fn n5296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5076);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11118);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытавший.
pub(crate) fn n5298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5078);
	match s {
		[] => true,
		['п', 'ы', s @ ..] => {
			super::push(out, 11120);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятавший.
pub(crate) fn n5299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5079);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11121);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дничавший.
pub(crate) fn n19392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18443);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24895);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24896);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29243);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 31601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29244);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31602);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 31603);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24897);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24898);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 24899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еничавший.
pub(crate) fn n19393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18444);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 24900);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жничавший.
pub(crate) fn n19394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18445);
	s.is_empty()
}

/// Words ending -зничавший.
pub(crate) fn n19395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18446);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24901);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24902);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24903);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24904);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24905);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24906);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 29248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йничавший.
pub(crate) fn n19396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18447);
	s.is_empty()
}

/// Words ending -мничавший.
pub(crate) fn n19397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18448);
	s.is_empty()
}

/// Words ending -нничавший.
pub(crate) fn n19398(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24907);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24908);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29249);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29250);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31606);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 29251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рничавший.
pub(crate) fn n19399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18449);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24910);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24911);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29252);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'у', s @ ..] => {
					super::push(out, 29253);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24912);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29254);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 29255);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31609);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 29256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24913);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24914);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31611);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31612);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32769);
									s.is_empty()
								}
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
			super::push(out, 24915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сничавший.
pub(crate) fn n19400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18450);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24916);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 24917);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тничавший.
pub(crate) fn n19401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18451);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24919);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24920);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24921);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 29258);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24922);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29259);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31615);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 31616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чничавший.
pub(crate) fn n19402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18452);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24923);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'т', s @ ..] => {
			super::push(out, 24924);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 29261);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шничавший.
pub(crate) fn n19403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18453);
	s.is_empty()
}

/// Words ending -ьничавший.
pub(crate) fn n19404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18454);
	s.is_empty()
}

/// Words ending -ричавший.
pub(crate) fn n11715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11164);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 18455);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лчавший.
pub(crate) fn n5317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5095);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мчавший.
pub(crate) fn n5318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5096);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11166);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11167);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18456);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчавший.
pub(crate) fn n5319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5097);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -очавший.
pub(crate) fn n5320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5098);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 11169);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11170);
			match s {
				[] => true,
				['о', 'в', 'а', s @ ..] => {
					super::push(out, 18457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11171);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18458);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пчавший.
pub(crate) fn n5321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5099);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рчавший.
pub(crate) fn n5322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5100);
	s.is_empty()
}

/// Words ending -учавший.
pub(crate) fn n5323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5101);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11173);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11174);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11175);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 11176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ычавший.
pub(crate) fn n5324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5102);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьчавший.
pub(crate) fn n5325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5103);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ючавший.
pub(crate) fn n5326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5104);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11179);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ячавший.
pub(crate) fn n5327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5105);
	s.is_empty()
}

/// Words ending -ловивший.
pub(crate) fn n11884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11327);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18578);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 18579);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 18580);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18581);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 18582);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18583);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 18584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24990);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 29276);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 29277);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 29278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18585);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 18586);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 18587);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отовивший.
pub(crate) fn n19546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18591);
	s.is_empty()
}

/// Words ending -свивший.
pub(crate) fn n5437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5214);
	s.is_empty()
}

/// Words ending -твивший.
pub(crate) fn n5438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5215);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 11332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -увивший.
pub(crate) fn n5439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5216);
	s.is_empty()
}

/// Words ending -ывивший.
pub(crate) fn n5440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5217);
	s.is_empty()
}

/// Words ending -явивший.
pub(crate) fn n5441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5218);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11333);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11334);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11335);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11336);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11337);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 18593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11338);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11339);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 11340);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одивший.
pub(crate) fn n5449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5226);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11365);
			match s {
				[] => true,
				['о', 'в', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25009);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25010);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29282);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18622);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18623);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29283);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18624);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25013);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29284);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31620);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 25014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18625);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25015);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 25016);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 25017);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25018);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29286);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 31621);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29287);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31622);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 31623);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32771);
													s.is_empty()
												}
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
							super::push(out, 25020);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11366);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11367);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 18627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11368);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18628);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25021);
							s.is_empty()
						}
						['с', 'ы', s @ ..] => {
							super::push(out, 25022);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', 'ы', s @ ..] => {
							super::push(out, 25023);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18629);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25024);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29291);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 31624);
											s.is_empty()
										}
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
		['р', s @ ..] => {
			super::push(out, 11369);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18630);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 18631);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25025);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29292);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 29293);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31625);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29294);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29296);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29297);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 31626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25027);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18632);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29299);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31627);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 31628);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 32772);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 31629);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29300);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 25028);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29301);
									s.is_empty()
								}
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
			super::push(out, 11370);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18633);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25029);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18634);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удивший.
pub(crate) fn n5451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5228);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11371);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11372);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11373);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18639);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18640);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11374);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18641);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18642);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18643);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11375);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18644);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18645);
					match s {
						[] => true,
						['а', 'р', 'о', s @ ..] => {
							super::push(out, 25035);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29305);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31631);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 25036);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18646);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11376);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25038);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25039);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29308);
									s.is_empty()
								}
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
			super::push(out, 11377);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11378);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аруживший.
pub(crate) fn n19638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18678);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 25054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -друживший.
pub(crate) fn n19639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18679);
	s.is_empty()
}

/// Words ending -круживший.
pub(crate) fn n19640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18680);
	s.is_empty()
}

/// Words ending -оруживший.
pub(crate) fn n19641(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 25055);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29313);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 29314);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пруживший.
pub(crate) fn n19642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18681);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25057);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 25058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атуживший.
pub(crate) fn n11967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11404);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 18682);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыживший.
pub(crate) fn n5468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5244);
	s.is_empty()
}

/// Words ending -юживший.
pub(crate) fn n5469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5245);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11405);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18683);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11406);
			match s {
				[] => true,
				['у', 'ы', s @ ..] => {
					super::push(out, 18684);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яживший.
pub(crate) fn n5470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5246);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -безобразивший.
pub(crate) fn n34397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32773);
	s.is_empty()
}

/// Words ending -агообразивший.
pub(crate) fn n34398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32774);
	s.is_empty()
}

/// Words ending -знообразивший.
pub(crate) fn n34399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32775);
	s.is_empty()
}

/// Words ending -узивший.
pub(crate) fn n5479(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11424);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18708);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18710);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11425);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11426);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18713);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25070);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29323);
									s.is_empty()
								}
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
			super::push(out, 11427);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18714);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18715);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18716);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 18717);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11428);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11429);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еливший.
pub(crate) fn n5486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5259);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11450);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18754);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11451);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 11452);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18755);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18756);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18757);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25094);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18758);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'т', s @ ..] => {
			super::push(out, 11453);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18759);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18760);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25096);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29331);
									s.is_empty()
								}
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
			super::push(out, 11454);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18761);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18762);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18763);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25098);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18764);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25099);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29333);
									s.is_empty()
								}
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
			super::push(out, 11455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18765);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18766);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18767);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18768);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иливший.
pub(crate) fn n5488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5261);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11456);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11457);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11458);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18770);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18771);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18772);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							super::push(out, 25102);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18773);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 18774);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18775);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11460);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кливший.
pub(crate) fn n5490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11462);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мливший.
pub(crate) fn n5491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5264);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11464);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11465);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оливший.
pub(crate) fn n5492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5265);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11466);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 11467);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18779);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18780);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25104);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25105);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25106);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18781);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11468);
			match s {
				[] => true,
				['а', 'л', s @ ..] => {
					super::push(out, 18783);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18785);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18786);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18787);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18788);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18789);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18790);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11469);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 18792);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25111);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29336);
									s.is_empty()
								}
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
			super::push(out, 11470);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18793);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25112);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18794);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11471);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18795);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11472);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18797);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 18798);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18799);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18800);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 18801);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18802);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25114);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25116);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18804);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11473);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11474);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11475);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18807);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пливший.
pub(crate) fn n5493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5266);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18808);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рливший.
pub(crate) fn n5494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5267);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11476);
			match s {
				[] => true,
				['в', 'с', 'ы', s @ ..] => {
					super::push(out, 18810);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сливший.
pub(crate) fn n5495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5268);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11478);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11479);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18811);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25122);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25123);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25124);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 18812);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25125);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25126);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29339);
									s.is_empty()
								}
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
			super::push(out, 11480);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18813);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11481);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 18814);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11482);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 18815);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тливший.
pub(crate) fn n5496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5269);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11483);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25130);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25131);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29340);
									s.is_empty()
								}
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

/// Words ending -уливший.
pub(crate) fn n5497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5270);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11484);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11485);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11486);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11487);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11488);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 11489);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11490);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18818);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11491);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18819);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11492);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18822);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11493);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 11494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фливший.
pub(crate) fn n5498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5271);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хливший.
pub(crate) fn n5499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5272);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11496);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11497);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шливший.
pub(crate) fn n5500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5273);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыливший.
pub(crate) fn n5501(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11500);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11501);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18824);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11502);
			s.is_empty()
		}
		['р', 'к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18825);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25137);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юливший.
pub(crate) fn n5502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5274);
	s.is_empty()
}

/// Words ending -яливший.
pub(crate) fn n5503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5275);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11504);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18826);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'ы', s @ ..] => {
			super::push(out, 11505);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18827);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енивший.
pub(crate) fn n5522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5294);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11536);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18877);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18878);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11537);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11538);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 11539);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18879);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11540);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18880);
					match s {
						[] => true,
						['з', 'ы', s @ ..] => {
							super::push(out, 25179);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 18881);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25180);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29350);
									s.is_empty()
								}
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
			super::push(out, 11541);
			match s {
				[] => true,
				['е', 'р', 'в', s @ ..] => {
					super::push(out, 18882);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							super::push(out, 25181);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18883);
					s.is_empty()
				}
				['е', 'т', s @ ..] => {
					super::push(out, 18884);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11543);
			s.is_empty()
		}
		['т', 'е', s @ ..] => {
			super::push(out, 11544);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18885);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18886);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18887);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25184);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25185);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29352);
									s.is_empty()
								}
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

/// Words ending -порожнивший.
pub(crate) fn n26451(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29353);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 29354);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знивший.
pub(crate) fn n5524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5296);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11545);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11546);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11547);
			match s {
				[] => true,
				['р', 'г', 'ы', s @ ..] => {
					super::push(out, 18891);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инивший.
pub(crate) fn n5525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5297);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18892);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18893);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18894);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11548);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11549);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11550);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11551);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18897);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11552);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18899);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18900);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 25189);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18901);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11553);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 11554);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18903);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18904);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 11555);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнивший.
pub(crate) fn n5526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5298);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11556);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25192);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25193);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25194);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25195);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25196);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29356);
									s.is_empty()
								}
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

/// Words ending -мнивший.
pub(crate) fn n5527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5299);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11557);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 18906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11558);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18907);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онивший.
pub(crate) fn n5528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5300);
	match s {
		[] => true,
		['в', 'з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18909);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 18910);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18911);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29357);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31641);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 29358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25198);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25199);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18912);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11559);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11560);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11561);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11562);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11563);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18917);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18918);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25201);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11564);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18919);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11565);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18922);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 25204);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18923);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25205);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29362);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31642);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 25206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29363);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 25207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18925);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11568);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 18927);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пнивший.
pub(crate) fn n5529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5301);
	s.is_empty()
}

/// Words ending -рнивший.
pub(crate) fn n5530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5302);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11569);
			match s {
				[] => true,
				['ч', 'ы', s @ ..] => {
					super::push(out, 18928);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снивший.
pub(crate) fn n5531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5303);
	match s {
		[] => true,
		['е', 'т', 'ы', s @ ..] => {
			super::push(out, 11570);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11571);
			match s {
				[] => true,
				['т', 'ы', s @ ..] => {
					super::push(out, 18930);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25211);
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
				['о', s @ ..] => {
					super::push(out, 18931);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 18932);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18933);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнивший.
pub(crate) fn n5532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5304);
	s.is_empty()
}

/// Words ending -унивший.
pub(crate) fn n5533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5305);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11572);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 11573);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18935);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18936);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнивший.
pub(crate) fn n5534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5306);
	s.is_empty()
}

/// Words ending -шнивший.
pub(crate) fn n5535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5307);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынивший.
pub(crate) fn n5536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5308);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юнивший.
pub(crate) fn n5537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5309);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11576);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11577);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янивший.
pub(crate) fn n5538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5310);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 11578);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11579);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11580);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11581);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 11582);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18940);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обагривший.
pub(crate) fn n30828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29372);
	s.is_empty()
}

/// Words ending -угривший.
pub(crate) fn n12221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11636);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дривший.
pub(crate) fn n5559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5331);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11637);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18998);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11638);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11639);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11640);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19000);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еривший.
pub(crate) fn n5560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5332);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19002);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19003);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19004);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19005);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11642);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11643);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19007);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19008);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25251);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19009);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11644);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11645);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11646);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19012);
					match s {
						[] => true,
						['м', 'ы', s @ ..] => {
							super::push(out, 25253);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11647);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11648);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 11649);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11650);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 11651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зривший.
pub(crate) fn n5561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5333);
	s.is_empty()
}

/// Words ending -иривший.
pub(crate) fn n5562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5334);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11652);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11653);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11654);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19018);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25254);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 25255);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11655);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кривший.
pub(crate) fn n5563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5335);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 11656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оривший.
pub(crate) fn n5564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5336);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11657);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19020);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25256);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25257);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19021);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25258);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29377);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29378);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31643);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25259);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19022);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25260);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19023);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25261);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25262);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29381);
									s.is_empty()
								}
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
			super::push(out, 11658);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11659);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19026);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19027);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19028);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19029);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19030);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 19031);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11661);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19032);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11662);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19033);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19034);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25269);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25270);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19035);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 19036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11663);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19037);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19038);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19039);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 19040);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11664);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19041);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19042);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25274);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29383);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'о', s @ ..] => {
					super::push(out, 19043);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19044);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25278);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 25279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 19045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11665);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19046);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тривший.
pub(crate) fn n5565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5337);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11666);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19047);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25280);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25281);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25282);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29384);
									s.is_empty()
								}
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
			super::push(out, 11667);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11668);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уривший.
pub(crate) fn n5566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5338);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11670);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					super::push(out, 19049);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19050);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19051);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19052);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11672);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11673);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19054);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19055);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19056);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11674);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19058);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19059);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19060);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19061);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25287);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29385);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 31644);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25288);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19062);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 19063);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19064);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11676);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 19065);
					s.is_empty()
				}
				['х', 'о', s @ ..] => {
					super::push(out, 19066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11677);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19067);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11678);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19069);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19070);
					match s {
						[] => true,
						['к', 'у', 'т', 'ш', 'ы', s @ ..] => {
							super::push(out, 25293);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19071);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19072);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25294);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19073);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25295);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19074);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11679);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 11680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фривший.
pub(crate) fn n5567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5339);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11681);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хривший.
pub(crate) fn n5568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5340);
	match s {
		[] => true,
		['и', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19075);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11682);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щривший.
pub(crate) fn n5569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5341);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыривший.
pub(crate) fn n5570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5342);
	s.is_empty()
}

/// Words ending -яривший.
pub(crate) fn n5571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5343);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11684);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11685);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11686);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 19077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 11687);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усивший.
pub(crate) fn n5579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5351);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 11709);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19102);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11710);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11711);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19103);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25320);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29397);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 29398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 25321);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25322);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 25323);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25324);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29400);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25326);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29402);
									s.is_empty()
								}
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

/// Words ending -ысивший.
pub(crate) fn n5580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5352);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11712);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -итивший.
pub(crate) fn n5584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5356);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11729);
			s.is_empty()
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19131);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25349);
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 25350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 11730);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтивший.
pub(crate) fn n5585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5357);
	match s {
		[] => true,
		['е', 'ж', 'ы', s @ ..] => {
			super::push(out, 11731);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19133);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтивший.
pub(crate) fn n5586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5358);
	match s {
		[] => true,
		['и', 'в', 'ы', s @ ..] => {
			super::push(out, 11732);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отивший.
pub(crate) fn n5587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5359);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11733);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25351);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25352);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29411);
									s.is_empty()
								}
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
			super::push(out, 11734);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11735);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29412);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31647);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'ы', s @ ..] => {
							super::push(out, 25354);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'ы', s @ ..] => {
							super::push(out, 25355);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'ы', s @ ..] => {
							super::push(out, 25356);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19136);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25357);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19137);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25358);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19138);
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 25360);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29418);
									s.is_empty()
								}
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
			super::push(out, 11736);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25361);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25362);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25363);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птивший.
pub(crate) fn n5588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5360);
	match s {
		[] => true,
		['о', 'к', 'ы', s @ ..] => {
			super::push(out, 11737);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19139);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртивший.
pub(crate) fn n5589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5361);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11738);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19140);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25365);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19141);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25366);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29421);
									s.is_empty()
								}
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
			super::push(out, 11739);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стивший.
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11740);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19144);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25368);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25369);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25370);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19145);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19146);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25371);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19148);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25372);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25373);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25374);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29425);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31648);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29426);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31649);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19149);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25375);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19150);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 25376);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19151);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25377);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29430);
									s.is_empty()
								}
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
			super::push(out, 11742);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 19152);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25378);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25379);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11743);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11744);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11745);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19153);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 19154);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25381);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19156);
					match s {
						[] => true,
						['о', 'х', 'ы', s @ ..] => {
							super::push(out, 25383);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19157);
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
							super::push(out, 25384);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 29434);
									match s {
										[] => true,
										['а', 'з', '\u{1}', s @ ..] => {
											super::push(out, 31650);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25385);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19158);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11746);
			match s {
				[] => true,
				['п', 'ы', s @ ..] => {
					super::push(out, 19160);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утивший.
pub(crate) fn n5591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5363);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11748);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19161);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11749);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11750);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19162);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25388);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 25389);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 25390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19163);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11751);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 19165);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11752);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19166);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11753);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19167);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтивший.
pub(crate) fn n5592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5364);
	s.is_empty()
}

/// Words ending -ытивший.
pub(crate) fn n5593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5365);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19168);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19169);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19170);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютивший.
pub(crate) fn n5594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5366);
	s.is_empty()
}

/// Words ending -ятивший.
pub(crate) fn n5595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5367);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11754);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11755);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19172);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19173);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 19174);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19175);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25394);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29437);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31651);
											s.is_empty()
										}
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
					super::push(out, 19176);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19177);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19178);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очивший.
pub(crate) fn n5606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5378);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11779);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11780);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 19209);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11781);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19210);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25418);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19211);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25419);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25420);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11782);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19212);
					match s {
						[] => true,
						['щ', 'ы', s @ ..] => {
							super::push(out, 25421);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19213);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19214);
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 25423);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29444);
									s.is_empty()
								}
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
			super::push(out, 11783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19215);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19216);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25424);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 29445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19217);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11784);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11785);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19218);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19219);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25426);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 25427);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25428);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25429);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29446);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31652);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19220);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19221);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19222);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25430);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29447);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31653);
											s.is_empty()
										}
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
					super::push(out, 19223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11786);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19224);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19225);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19226);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11787);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19227);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25433);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25434);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19228);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25435);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29449);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29450);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31654);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 25437);
							s.is_empty()
						}
						['р', 'и', s @ ..] => {
							super::push(out, 25438);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19229);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29452);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31655);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 29453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19230);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -корчивший.
pub(crate) fn n20214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19234);
	s.is_empty()
}

/// Words ending -учивший.
pub(crate) fn n5608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5380);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11791);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 11792);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19235);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19236);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25441);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 25442);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 29454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19237);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11793);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11794);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11795);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11796);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 11797);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11798);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11799);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11800);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11801);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19241);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11802);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 11803);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19242);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19243);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11804);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19244);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 19245);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19246);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19247);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25446);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19248);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19249);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19250);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11805);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19251);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11806);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19252);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19253);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 19254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 11807);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11808);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычивший.
pub(crate) fn n5609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5381);
	s.is_empty()
}

/// Words ending -ьчивший.
pub(crate) fn n5610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5382);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11809);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ючивший.
pub(crate) fn n5611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5383);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11810);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19256);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25449);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19257);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25450);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25451);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29457);
									s.is_empty()
								}
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
			super::push(out, 11811);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19258);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 25452);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29458);
									s.is_empty()
								}
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
			super::push(out, 11812);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11813);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячивший.
pub(crate) fn n5612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11814);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11815);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11816);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11817);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25453);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 25454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11818);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word срыгнувший.
pub(crate) fn n30927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29471);
	s.is_empty()
}

/// Words ending -трыгнувший.
pub(crate) fn n26768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25498);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ягнувший.
pub(crate) fn n12469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11874);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 19323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -днувший.
pub(crate) fn n5642(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11875);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11876);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -знувший.
pub(crate) fn n5643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5413);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11878);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19325);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 19326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11879);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25499);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11880);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11881);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25501);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25502);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29474);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31658);
											s.is_empty()
										}
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
		['у', s @ ..] => {
			super::push(out, 11882);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11883);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25503);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29475);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31659);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 25504);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29476);
									s.is_empty()
								}
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
			super::push(out, 11884);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19329);
					match s {
						[] => true,
						['о', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29477);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29478);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31660);
											s.is_empty()
										}
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
		['я', s @ ..] => {
			super::push(out, 11885);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инувший.
pub(crate) fn n5644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5414);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11886);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25505);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25506);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29479);
									s.is_empty()
								}
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
			super::push(out, 11887);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11888);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19331);
					match s {
						[] => true,
						['р', 'п', 'ы', s @ ..] => {
							super::push(out, 25507);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19332);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19333);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19334);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кнувший.
pub(crate) fn n5645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5415);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11891);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19336);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19337);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19338);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11892);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11893);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19340);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19341);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19342);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25512);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25513);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11894);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19344);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19345);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 25515);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19346);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19347);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 25516);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29483);
									s.is_empty()
								}
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
			super::push(out, 11895);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11896);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19348);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 25517);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29484);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31661);
											s.is_empty()
										}
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
					super::push(out, 19349);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 25518);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25519);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25520);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29485);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29486);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31662);
											s.is_empty()
										}
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
		['м', s @ ..] => {
			super::push(out, 11897);
			match s {
				[] => true,
				['а', 'ж', s @ ..] => {
					super::push(out, 19350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11898);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25522);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 25523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11899);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19352);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25524);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29488);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31663);
											s.is_empty()
										}
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
					super::push(out, 19353);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25525);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25526);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 25527);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29490);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31664);
											s.is_empty()
										}
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
					super::push(out, 19354);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19355);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25528);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25529);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29491);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 31665);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19356);
					s.is_empty()
				}
				['ы', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25531);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 29492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 25532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 19357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25534);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11900);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11901);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11902);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25536);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 25537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25538);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 19361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11903);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19362);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25540);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25541);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t10::n20353(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 11904);
			s.is_empty()
		}
		['я', s @ ..] => super::t10::n12502(s, out),
		_ => false,
	}
}

/// Words ending -мнувший.
pub(crate) fn n5646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5416);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онувший.
pub(crate) fn n5647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5417);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11907);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11908);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 19364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пнувший.
pub(crate) fn n5648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11910);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19365);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25543);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 25544);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 19366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19367);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19368);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 19369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11911);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19370);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19371);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19372);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 19373);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25545);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25546);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29495);
									s.is_empty()
								}
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
			super::push(out, 11912);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19374);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19375);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19376);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11913);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 25549);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29497);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31666);
											s.is_empty()
										}
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
		['у', s @ ..] => {
			super::push(out, 11914);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19378);
					match s {
						[] => true,
						['о', 'к', 'ы', s @ ..] => {
							super::push(out, 25550);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19379);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11915);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 11916);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 11917);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнувший.
pub(crate) fn n5649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5419);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11918);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19381);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 19382);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25553);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25554);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19383);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25555);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25556);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11919);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11920);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11921);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19385);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25557);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 29501);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31667);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25558);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 29502);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31668);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 25559);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29503);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31669);
											s.is_empty()
										}
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
					super::push(out, 19386);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19387);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25560);
							s.is_empty()
						}
						['ш', 'ы', s @ ..] => {
							super::push(out, 25561);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25562);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29505);
									s.is_empty()
								}
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

/// Words ending -снувший.
pub(crate) fn n5650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5420);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11922);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19388);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 19389);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19390);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11923);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19392);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29506);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29507);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31670);
											s.is_empty()
										}
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
					super::push(out, 19393);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11924);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 19394);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25565);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29508);
									s.is_empty()
								}
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
			super::push(out, 11925);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19395);
					match s {
						[] => true,
						['о', 'п', 'ы', s @ ..] => {
							super::push(out, 25566);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29509);
									s.is_empty()
								}
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
			super::push(out, 11926);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11927);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19396);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 25567);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 11928);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тнувший.
pub(crate) fn n5651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5421);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11930);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11931);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 19398);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11932);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11933);
			match s {
				[] => true,
				['о', 'б', 'ы', s @ ..] => {
					super::push(out, 19400);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11934);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11935);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19401);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19402);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25570);
							match s {
								[] => true,
								['х', 'ы', s @ ..] => {
									super::push(out, 29511);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31671);
											s.is_empty()
										}
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
					super::push(out, 19403);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19404);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29512);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 29513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19405);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 19406);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унувший.
pub(crate) fn n5652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5422);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11936);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11937);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11938);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 19408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнувший.
pub(crate) fn n5653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5423);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19409);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 19410);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 19411);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19412);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25574);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25575);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 25576);
							s.is_empty()
						}
						['о', 'р', s @ ..] => {
							super::push(out, 25577);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25578);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19413);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 19414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11940);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11941);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19415);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25580);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25581);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19416);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25582);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19417);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 19418);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25583);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29518);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19419);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25584);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25585);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25586);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25587);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25588);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25589);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19421);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25590);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25591);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 29520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25593);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11942);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11943);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19423);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 25595);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29521);
									s.is_empty()
								}
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
			super::push(out, 11944);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19424);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25596);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 19425);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19426);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25597);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 11945);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11946);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19429);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25600);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25601);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19431);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'л', 'у', s @ ..] => {
					super::push(out, 19432);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19433);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11947);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25605);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25606);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29525);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29526);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31672);
											s.is_empty()
										}
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

/// Words ending -цнувший.
pub(crate) fn n5654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5424);
	s.is_empty()
}

/// Words ending -чнувший.
pub(crate) fn n5655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5425);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11948);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынувший.
pub(crate) fn n5656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5426);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьнувший.
pub(crate) fn n5657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5427);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11950);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юнувший.
pub(crate) fn n5658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5428);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11951);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 19435);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'ы', s @ ..] => {
					super::push(out, 19436);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янувший.
pub(crate) fn n5659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5429);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11952);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11953);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25609);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25610);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25611);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25612);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25613);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 19438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25614);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 29528);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31673);
											s.is_empty()
										}
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
		['м', s @ ..] => {
			super::push(out, 11954);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11955);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19440);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11956);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19442);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19443);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еживающий.
pub(crate) fn n20546(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 25668);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25669);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29544);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29545);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29546);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зживающий.
pub(crate) fn n20547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19547);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оживающий.
pub(crate) fn n20549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19549);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25671);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 25672);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 25673);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 25674);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 25675);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25676);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рживающий.
pub(crate) fn n20550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19550);
	s.is_empty()
}

/// Words ending -тживающий.
pub(crate) fn n20551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19551);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25678);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уживающий.
pub(crate) fn n20552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19552);
	s.is_empty()
}

/// Words ending -ыживающий.
pub(crate) fn n20553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19553);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кивающий.
pub(crate) fn n12729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12124);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19554);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19555);
			match s {
				[] => true,
				['а', 'т', 'т', s @ ..] => {
					super::push(out, 25680);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31676);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31677);
									s.is_empty()
								}
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
			super::push(out, 19556);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25681);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29548);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 31678);
									s.is_empty()
								}
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

/// Words ending -ливающий.
pub(crate) fn n12730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12125);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25682);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25683);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 25684);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 25685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', 'о', s @ ..] => {
			super::push(out, 19557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25687);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29550);
							match s {
								[] => true,
								['а', 'т', 'с', 'о', 'о', s @ ..] => {
									super::push(out, 31679);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32776);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33302);
													s.is_empty()
												}
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
		['д', s @ ..] => {
			super::push(out, 19559);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25688);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25689);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19560);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25691);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 25692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19561);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19562);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19563);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25693);
					match s {
						[] => true,
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 29553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25694);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25695);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29554);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 29555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19564);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19565);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19566);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 19567);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25697);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нивающий.
pub(crate) fn n12731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12126);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19568);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 19569);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пивающий.
pub(crate) fn n12732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12127);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ривающий.
pub(crate) fn n12733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12128);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19572);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 19573);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25698);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25699);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25700);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19574);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25701);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19575);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 19576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хивающий.
pub(crate) fn n12734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12129);
	s.is_empty()
}

/// Words ending -чивающий.
pub(crate) fn n12735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12130);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19577);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19578);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19579);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25703);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 29559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25704);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25705);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шивающий.
pub(crate) fn n12736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12131);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19581);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25707);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 25708);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 19582);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19583);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25711);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19584);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19585);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25712);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19586);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25713);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19587);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25714);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19588);
			match s {
				[] => true,
				['л', 'с', 'д', s @ ..] => {
					super::push(out, 25715);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 29565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19589);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щивающий.
pub(crate) fn n12737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12132);
	s.is_empty()
}

/// Words ending -овающий.
pub(crate) fn n5873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5634);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 12133);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 19590);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увающий.
pub(crate) fn n5874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5635);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывающий.
pub(crate) fn n5875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5636);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19591);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19592);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25717);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25718);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29566);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19593);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19594);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19595);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19596);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 19597);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 12136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19599);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19600);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25720);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19601);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19602);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29569);
									s.is_empty()
								}
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
			super::push(out, 12137);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 12138);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19603);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25722);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 25723);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25724);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 25725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 19604);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 19605);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19606);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19607);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19608);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25728);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 25729);
							match s {
								[] => true,
								['о', 'р', 'п', 'е', s @ ..] => {
									super::push(out, 29570);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31680);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25730);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19609);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19610);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19611);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19612);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 19613);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 19614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12139);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19615);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19616);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12140);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19618);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 19619);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19620);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19621);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19622);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19623);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19624);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25739);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19626);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12141);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19627);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19628);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 19629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12142);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19630);
					match s {
						[] => true,
						['е', 'ч', 'с', 'и', '\u{1}', s @ ..] => {
							super::push(out, 25742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19631);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25743);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25744);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 25745);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 25746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 19632);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25747);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19633);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19634);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19635);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19636);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19637);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19638);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19639);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12144);
			s.is_empty()
		}
		['т', s @ ..] => super::t10::n12750(s, out),
		_ => false,
	}
}

/// The word приемо-передающий.
pub(crate) fn n34404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32780);
	s.is_empty()
}

/// Words ending -ныкающий.
pub(crate) fn n12849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12240);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 19704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тыкающий.
pub(crate) fn n12851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12242);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лькающий.
pub(crate) fn n5926(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 12243);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 12244);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экающий.
pub(crate) fn n5927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5687);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 12245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юкающий.
pub(crate) fn n5928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5688);
	s.is_empty()
}

/// Words ending -якающий.
pub(crate) fn n5929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5689);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12246);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 12247);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 12248);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['б', 'а', s @ ..] => {
					super::push(out, 19707);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12249);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -севозрастающий.
pub(crate) fn n34405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32781);
	s.is_empty()
}

/// Words ending -ачальствующий.
pub(crate) fn n34414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32790);
	s.is_empty()
}

/// Words ending -ельствующий.
pub(crate) fn n31086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29625);
	s.is_empty()
}

/// Words ending -ольствующий.
pub(crate) fn n31087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29626);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 31708);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лирующий.
pub(crate) fn n13004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12392);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19814);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25851);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 29639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19815);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25852);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29640);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31711);
									s.is_empty()
								}
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
			super::push(out, 19816);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25853);
					match s {
						[] => true,
						['и', 'о', 'л', s @ ..] => {
							super::push(out, 29641);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25854);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19817);
			match s {
				[] => true,
				['м', 'и', s @ ..] => {
					super::push(out, 25856);
					match s {
						[] => true,
						['т', 'с', 'е', s @ ..] => {
							super::push(out, 29643);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31713);
									s.is_empty()
								}
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

/// Words ending -мирующий.
pub(crate) fn n13005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19818);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19819);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29644);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31715);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31716);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 31717);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31718);
									s.is_empty()
								}
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

/// Words ending -нирующий.
pub(crate) fn n13006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12394);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 19820);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19821);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25857);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29645);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19822);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25858);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пирующий.
pub(crate) fn n13007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12395);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19823);
			s.is_empty()
		}
		['и', 'к', s @ ..] => {
			super::push(out, 19824);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 25860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19825);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 19826);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19827);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рирующий.
pub(crate) fn n13008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12396);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19828);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25862);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сирующий.
pub(crate) fn n13009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12397);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 19829);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25863);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							super::push(out, 29648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тирующий.
pub(crate) fn n13010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12398);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19830);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19831);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25866);
					match s {
						[] => true,
						['и', 'б', 'а', 'е', 'р', 'е', s @ ..] => {
							super::push(out, 29650);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31720);
									s.is_empty()
								}
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
			super::push(out, 19832);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19833);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25867);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29651);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цирующий.
pub(crate) fn n13011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12399);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19834);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 19835);
			match s {
				[] => true,
				['е', 'р', 'о', 'ф', s @ ..] => {
					super::push(out, 25868);
					match s {
						[] => true,
						['с', 'о', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 29653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ширующий.
pub(crate) fn n13012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12400);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19836);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19837);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25869);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ворующий.
pub(crate) fn n13013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12401);
	s.is_empty()
}

/// Words ending -трующий.
pub(crate) fn n6051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5808);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 12402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урующий.
pub(crate) fn n6052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5809);
	s.is_empty()
}

/// Words ending -фрующий.
pub(crate) fn n6053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5810);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -арниковый.
pub(crate) fn n21132(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 26000);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 26001);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сниковый.
pub(crate) fn n13545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12911);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 20112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тниковый.
pub(crate) fn n13546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12912);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20113);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 20114);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 20115);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20116);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26002);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26003);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20117);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20118);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26005);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31729);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31730);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32794);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31731);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31732);
									s.is_empty()
								}
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
			super::push(out, 20119);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26006);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чниковый.
pub(crate) fn n13547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12913);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20120);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 20121);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26007);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 26008);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 26009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20122);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шниковый.
pub(crate) fn n13548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12914);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 20124);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20125);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 20126);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29707);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 29708);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31733);
									s.is_empty()
								}
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

/// Words ending -ьниковый.
pub(crate) fn n13549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12915);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пиковый.
pub(crate) fn n6727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6449);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12916);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12917);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 12918);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риковый.
pub(crate) fn n6728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6450);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12919);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 12920);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12921);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20129);
					match s {
						[] => true,
						['а', 'м', 'д', s @ ..] => {
							super::push(out, 26012);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29710);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31734);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29711);
									s.is_empty()
								}
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
			super::push(out, 12922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20130);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12923);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 12924);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиковый.
pub(crate) fn n6730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6452);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12925);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12926);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12927);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 12928);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20135);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26013);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29712);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 31735);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29713);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31736);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 32795);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31737);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 26014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20136);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 26015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12929);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 12930);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20138);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 20139);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чиковый.
pub(crate) fn n6731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6453);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 12931);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20140);
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 20141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12932);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word трехсантиметровый.
pub(crate) fn n34420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32796);
	s.is_empty()
}

/// The word трехметровый.
pub(crate) fn n34421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32797);
	s.is_empty()
}

/// Words ending -итрехметровый.
pub(crate) fn n34422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32798);
	match s {
		[] => true,
		['т', 'а', 'ц', 'д', 'и', 'р', 'т', '\u{1}', s @ ..] => {
			super::push(out, 33306);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word четырехметровый.
pub(crate) fn n34423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32799);
	s.is_empty()
}

/// Words ending -кестровый.
pub(crate) fn n21365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20336);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утровый.
pub(crate) fn n6974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6682);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 13215);
			match s {
				[] => true,
				['а', 'л', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 20338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтровый.
pub(crate) fn n6975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6683);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -леживаемый.
pub(crate) fn n27437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26144);
	match s {
		[] => true,
		['с', 'т', s @ ..] => {
			super::push(out, 29757);
			match s {
				[] => true,
				['о', 'е', s @ ..] => {
					super::push(out, 31754);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реживаемый.
pub(crate) fn n27438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26145);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29758);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 29759);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зживаемый.
pub(crate) fn n21541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20506);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иживаемый.
pub(crate) fn n21542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20507);
	s.is_empty()
}

/// Words ending -оживаемый.
pub(crate) fn n21543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20508);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26147);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 26148);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 26149);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 26150);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29761);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рживаемый.
pub(crate) fn n21544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20509);
	s.is_empty()
}

/// Words ending -уживаемый.
pub(crate) fn n21545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20510);
	s.is_empty()
}

/// Words ending -ыживаемый.
pub(crate) fn n21546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20511);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -киваемый.
pub(crate) fn n14159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13499);
	s.is_empty()
}

/// Words ending -ливаемый.
pub(crate) fn n14160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13500);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26152);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26153);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 26154);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', 'о', s @ ..] => {
			super::push(out, 20512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 20513);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 20514);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26158);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26159);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 20515);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26161);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20516);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20517);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26163);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 26164);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 26165);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20518);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 20519);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 26167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 20520);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20521);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 20522);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ниваемый.
pub(crate) fn n14161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13501);
	s.is_empty()
}

/// Words ending -пиваемый.
pub(crate) fn n14162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13502);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 20523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -риваемый.
pub(crate) fn n14163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13503);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20524);
			match s {
				[] => true,
				['в', 'о', 'г', 'ы', s @ ..] => {
					super::push(out, 26170);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 29766);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 20525);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26171);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 26172);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20526);
			match s {
				[] => true,
				['м', 'а', s @ ..] => {
					super::push(out, 26174);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20527);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26175);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 20528);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26177);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26178);
					match s {
						[] => true,
						['в', 'о', 'р', 'п', 'е', s @ ..] => {
							super::push(out, 29769);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31757);
									s.is_empty()
								}
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
			super::push(out, 20529);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хиваемый.
pub(crate) fn n14164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13504);
	s.is_empty()
}

/// Words ending -чиваемый.
pub(crate) fn n14165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13505);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20530);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26179);
					match s {
						[] => true,
						['п', 'о', 'о', s @ ..] => {
							super::push(out, 29770);
							match s {
								[] => true,
								['к', 'з', s @ ..] => {
									super::push(out, 31758);
									s.is_empty()
								}
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
			super::push(out, 20531);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26180);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 29771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шиваемый.
pub(crate) fn n14166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13506);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20533);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 26183);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 20534);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 20535);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 20536);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26186);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20537);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 20538);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26187);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20539);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26188);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20540);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 20541);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 20542);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26189);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щиваемый.
pub(crate) fn n14167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13507);
	s.is_empty()
}

/// Words ending -уваемый.
pub(crate) fn n7304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6983);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ываемый.
pub(crate) fn n7305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6984);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13509);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20543);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20544);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13510);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20546);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26192);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20547);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26193);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29777);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31759);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 32801);
													s.is_empty()
												}
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
		['д', s @ ..] => {
			super::push(out, 13511);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 13512);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20548);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 26195);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 26196);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26197);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29779);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31760);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 20549);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20551);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26200);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26201);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20553);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20554);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13513);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20556);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20557);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20558);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26205);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20559);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 20560);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20561);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20562);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20563);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20564);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20565);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26212);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 26213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20567);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13515);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					super::push(out, 20568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20569);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26216);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 26217);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26218);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20570);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26220);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20571);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20572);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20573);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20574);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20575);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26223);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20576);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13517);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 13518);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20578);
					match s {
						[] => true,
						['ч', 'с', 'о', s @ ..] => {
							super::push(out, 26224);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29782);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31761);
											s.is_empty()
										}
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

/// Words ending -родегенеративный.
pub(crate) fn n34437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32813);
	s.is_empty()
}

/// Words ending -громелиоративный.
pub(crate) fn n34438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32814);
	s.is_empty()
}

/// Words ending -есомелиоративный.
pub(crate) fn n34439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32815);
	s.is_empty()
}

/// Words ending -истративный.
pub(crate) fn n27651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26350);
	match s {
		[] => true,
		['н', 'и', 'м', 'д', 'а', '-', s @ ..] => {
			super::push(out, 29849);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31804);
					match s {
						[] => true,
						['н', 'н', 'е', 'з', 'а', 'к', '\u{1}', s @ ..] => {
							super::push(out, 32816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уративный.
pub(crate) fn n21841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20790);
	s.is_empty()
}

/// Words ending -сативный.
pub(crate) fn n14536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13844);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 20791);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тативный.
pub(crate) fn n14537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13845);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 20792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'л', 'у', 'к', s @ ..] => {
			super::push(out, 20793);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26354);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итивный.
pub(crate) fn n7713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7368);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13846);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 13847);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20794);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26355);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26356);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20795);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 26357);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29852);
									s.is_empty()
								}
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
			super::push(out, 13848);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13849);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20796);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13850);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20797);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктивный.
pub(crate) fn n7714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7369);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13851);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20798);
					match s {
						[] => true,
						['о', 'н', 'р', s @ ..] => {
							super::push(out, 26360);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20799);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26361);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 26362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'т', s @ ..] => {
					super::push(out, 20800);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26363);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29855);
									s.is_empty()
								}
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
			super::push(out, 13852);
			match s {
				[] => true,
				['л', 'л', s @ ..] => {
					super::push(out, 20801);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20802);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29856);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31805);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 29857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20803);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26364);
							s.is_empty()
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29858);
									s.is_empty()
								}
								['э', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31806);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31807);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32817);
													s.is_empty()
												}
												['г', s @ ..] => {
													super::push(out, 32818);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 32819);
													s.is_empty()
												}
												['к', s @ ..] => {
													match s {
														[] => true,
														['з', s @ ..] => super::t10::n34961(s, out),
														['о', s @ ..] => super::t10::n34962(s, out),
														['э', s @ ..] => super::t10::n34963(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 32820);
													s.is_empty()
												}
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
				['ъ', s @ ..] => {
					super::push(out, 20804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13853);
			match s {
				[] => true,
				['д', 'о', 'р', 'п', 'е', 'р', s @ ..] => {
					super::push(out, 20805);
					s.is_empty()
				}
				['р', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26365);
							match s {
								[] => true,
								['д', 'а', s @ ..] => {
									super::push(out, 29859);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтивный.
pub(crate) fn n7715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7370);
	match s {
		[] => true,
		['е', 'г', s @ ..] => {
			super::push(out, 13854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отивный.
pub(crate) fn n7716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7371);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 13855);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птивный.
pub(crate) fn n7717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7372);
	match s {
		[] => true,
		['а', 'д', 'а', 'з', s @ ..] => {
			super::push(out, 13857);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13858);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13859);
			match s {
				[] => true,
				['р', 'э', '\u{1}', s @ ..] => {
					super::push(out, 20807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ортивный.
pub(crate) fn n7718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7373);
	match s {
		[] => true,
		['п', 'с', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20808);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26367);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 29860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20809);
					match s {
						[] => true,
						['т', 'о', 'м', '\u{1}', s @ ..] => {
							super::push(out, 26368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утивный.
pub(crate) fn n7719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7374);
	s.is_empty()
}

/// Words ending -ятивный.
pub(crate) fn n7720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7375);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20810);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20811);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26369);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29861);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 26370);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 26371);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29862);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31810);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 29863);
									match s {
										[] => true,
										['к', 'к', s @ ..] => {
											super::push(out, 31811);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32821);
													s.is_empty()
												}
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
		_ => false,
	}
}

/// The word углеводородный.
pub(crate) fn n34447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32822);
	s.is_empty()
}

/// Words ending --углеводородный.
pub(crate) fn n34448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32823);
	s.is_empty()
}

/// Words ending -дистоводородный.
pub(crate) fn n34449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32824);
	s.is_empty()
}

/// Words ending -ристоводородный.
pub(crate) fn n34450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32825);
	s.is_empty()
}

/// Words ending -тистоводородный.
pub(crate) fn n34451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32826);
	s.is_empty()
}

/// Words ending -кислородный.
pub(crate) fn n22002(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', 'о', 'н', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29899);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 29900);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26450);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26451);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29901);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нородный.
pub(crate) fn n14737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14031);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 20940);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 20941);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26453);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -породный.
pub(crate) fn n14738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14032);
	s.is_empty()
}

/// Words ending -сродный.
pub(crate) fn n7876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7521);
	s.is_empty()
}

/// Words ending -тродный.
pub(crate) fn n7877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7522);
	match s {
		[] => true,
		['к', 'е', 'л', 'э', 'х', s @ ..] => {
			super::push(out, 14033);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20942);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юродный.
pub(crate) fn n7878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7523);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14034);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20943);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20944);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26454);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26455);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', 'п', s @ ..] => {
							super::push(out, 26456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14035);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20945);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 26457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ованный.
pub(crate) fn n8387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7996);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14475);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21213);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26574);
							match s {
								[] => true,
								['т', 'с', s @ ..] => {
									super::push(out, 29964);
									match s {
										[] => true,
										['о', 'в', 'е', s @ ..] => {
											super::push(out, 31845);
											s.is_empty()
										}
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
					super::push(out, 21214);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26575);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31846);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 31847);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 31848);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 31849);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32839);
													s.is_empty()
												}
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
				['о', s @ ..] => {
					super::push(out, 21215);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21216);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 21217);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14476);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26579);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29965);
									match s {
										[] => true,
										['у', 'ч', 'о', 'р', s @ ..] => {
											super::push(out, 31850);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 29966);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29967);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31851);
											s.is_empty()
										}
										['ж', s @ ..] => {
											super::push(out, 31852);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32840);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ч', s @ ..] => {
											super::push(out, 31853);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 29968);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 29969);
									match s {
										[] => true,
										['и', 'а', 'з', 'о', s @ ..] => {
											super::push(out, 31854);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 29970);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29971);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 29972);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 31855);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32841);
													match s {
														[] => true,
														['т', 'е', 'д', 'и', 'в', 'с', 'а', 'з', 'е', s @ ..] => super::t10::n34965(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 32842);
													match s {
														[] => true,
														['в', s @ ..] => super::t10::n34966(s, out),
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
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14477);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21218);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 21219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21220);
					match s {
						[] => true,
						['р', 'б', s @ ..] => {
							super::push(out, 26580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21221);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26581);
							match s {
								[] => true,
								['с', 'с', s @ ..] => {
									super::push(out, 29973);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31856);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26582);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29974);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 31857);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21222);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21223);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26584);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 26585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21224);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21225);
					match s {
						[] => true,
						['р', 'о', 'б', 'о', 'е', 'н', s @ ..] => {
							super::push(out, 26586);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t10::n15214(s, out),
		['к', s @ ..] => super::t10::n15215(s, out),
		['л', s @ ..] => super::t10::n15216(s, out),
		['м', s @ ..] => {
			super::push(out, 14482);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21242);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26605);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 29997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t10::n22320(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t10::n15218(s, out),
		['п', s @ ..] => super::t10::n15219(s, out),
		['р', s @ ..] => super::t10::n15220(s, out),
		['с', s @ ..] => super::t10::n15221(s, out),
		['т', s @ ..] => super::t10::n15222(s, out),
		['ф', s @ ..] => super::t10::n15223(s, out),
		['х', s @ ..] => super::t10::n15224(s, out),
		['ц', s @ ..] => super::t10::n15225(s, out),
		_ => false,
	}
}

/// Words ending -созданный.
pub(crate) fn n22401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21322);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26685);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26686);
			s.is_empty()
		}
		['с', 'о', 'в', '\u{1}', s @ ..] => {
			super::push(out, 26687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -узданный.
pub(crate) fn n15257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14521);
	match s {
		[] => true,
		['б', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26688);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26689);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иданный.
pub(crate) fn n8408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8017);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14522);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21323);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14523);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14524);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14525);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 21325);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йданный.
pub(crate) fn n8409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8018);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14526);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21326);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оданный.
pub(crate) fn n8410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8019);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14527);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 14528);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14529);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21328);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 26692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14530);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21329);
					s.is_empty()
				}
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 21330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14531);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26693);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26694);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26695);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32010);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32011);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32992);
													s.is_empty()
												}
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
		_ => false,
	}
}

/// Words ending -рданный.
pub(crate) fn n8411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8020);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14532);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сданный.
pub(crate) fn n8412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8021);
	s.is_empty()
}

/// Words ending -тданный.
pub(crate) fn n8413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8022);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыданный.
pub(crate) fn n8414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8023);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14534);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юзанный.
pub(crate) fn n8423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8032);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14546);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21363);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язанный.
pub(crate) fn n8424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8033);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14547);
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21364);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21365);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26721);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21366);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экранный.
pub(crate) fn n15368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14624);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21442);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21443);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21444);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26782);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26783);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торанный.
pub(crate) fn n15370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14626);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 21445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пранный.
pub(crate) fn n8482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8091);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сранный.
pub(crate) fn n8483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8092);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -транный.
pub(crate) fn n8484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8093);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14629);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 14630);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21446);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21447);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26784);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 30175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уранный.
pub(crate) fn n8485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8094);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -франный.
pub(crate) fn n8486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8095);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хранный.
pub(crate) fn n8487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8096);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14633);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21449);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21450);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21451);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еревоспитанный.
pub(crate) fn n34633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32993);
	s.is_empty()
}

/// Words ending -станный.
pub(crate) fn n8504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14652);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14653);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21481);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 26816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21482);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26817);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30190);
									s.is_empty()
								}
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
			super::push(out, 14654);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14655);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14656);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 14657);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 14658);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21485);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 26818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утанный.
pub(crate) fn n8505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8114);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14659);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21487);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21488);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21489);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтанный.
pub(crate) fn n8508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8117);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14662);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твенный.
pub(crate) fn n8543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8152);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14714);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 21528);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14715);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21530);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26839);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									super::push(out, 30197);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21531);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14716);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21532);
					match s {
						[] => true,
						['ж', 'о', s @ ..] => {
							super::push(out, 26841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t10::n15467(s, out),
		['у', s @ ..] => {
			super::push(out, 14718);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -свобожденный.
pub(crate) fn n28243(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30258);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32076);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 30259);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -можденный.
pub(crate) fn n22678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21586);
	s.is_empty()
}

/// Words ending -рожденный.
pub(crate) fn n22679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21587);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26909);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 26910);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26911);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30260);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 26912);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26913);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26914);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30263);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 30264);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30265);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32080);
							s.is_empty()
						}
						['н', 'о', 'к', 'а', 'з', 'е', s @ ..] => {
							super::push(out, 32081);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26916);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржденный.
pub(crate) fn n15502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14751);
	s.is_empty()
}

/// Words ending -ужденный.
pub(crate) fn n15503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14752);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21588);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21589);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26918);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30269);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33062);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33063);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33466);
													s.is_empty()
												}
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
				['о', s @ ..] => {
					super::push(out, 26919);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26920);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30271);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32082);
									s.is_empty()
								}
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
			super::push(out, 21590);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 21591);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иденный.
pub(crate) fn n8555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8164);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14753);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21592);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21593);
					match s {
						[] => true,
						['е', 'р', 'п', 'е', s @ ..] => {
							super::push(out, 26921);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30272);
									s.is_empty()
								}
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

/// Words ending -йденный.
pub(crate) fn n8556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8165);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14754);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14755);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14756);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21596);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21597);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26922);
							match s {
								[] => true,
								['е', 'р', 'п', 'е', s @ ..] => {
									super::push(out, 30273);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32083);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 26923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21598);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 26924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лденный.
pub(crate) fn n8557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8166);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14757);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оденный.
pub(crate) fn n8558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8167);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14758);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уденный.
pub(crate) fn n8559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8168);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14760);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыденный.
pub(crate) fn n8560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8169);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14761);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юденный.
pub(crate) fn n8561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8170);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14762);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яденный.
pub(crate) fn n8562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8171);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14763);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26925);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 26926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -безображенный.
pub(crate) fn n34709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33064);
	s.is_empty()
}

/// Words ending -леженный.
pub(crate) fn n15528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14777);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26940);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26941);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26942);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26943);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снеженный.
pub(crate) fn n22723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21629);
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			super::push(out, 26944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реженный.
pub(crate) fn n15531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14780);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21630);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21631);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 21632);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21633);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жженный.
pub(crate) fn n8568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8177);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14782);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14783);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14784);
			match s {
				[] => true,
				['б', 'о', 'е', s @ ..] => {
					super::push(out, 21634);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14785);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зженный.
pub(crate) fn n8569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8178);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14786);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					super::push(out, 21637);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21638);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26948);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32088);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32089);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30285);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											super::push(out, 32090);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30286);
									s.is_empty()
								}
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
			super::push(out, 14787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иженный.
pub(crate) fn n8570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8179);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14788);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 14789);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26949);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21639);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21640);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14791);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26951);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26952);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30287);
									s.is_empty()
								}
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
			super::push(out, 14792);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лженный.
pub(crate) fn n8571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8180);
	match s {
		[] => true,
		['о', 'д', 'о', 'р', s @ ..] => {
			super::push(out, 14793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нженный.
pub(crate) fn n8572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8181);
	s.is_empty()
}

/// Words ending -оженный.
pub(crate) fn n8573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8182);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14794);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21642);
					match s {
						[] => true,
						['р', 'т', 'с', 'в', s @ ..] => {
							super::push(out, 26953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21643);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26954);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26955);
							match s {
								[] => true,
								['п', 'ы', s @ ..] => {
									super::push(out, 30288);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32091);
											s.is_empty()
										}
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
					super::push(out, 21644);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14795);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21645);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21646);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21647);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21648);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26957);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26958);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30289);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 32092);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 30290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21649);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26960);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 26961);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26962);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30291);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32093);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30292);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32094);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 32095);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 33065);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26963);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26964);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 26965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21650);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21651);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21652);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14796);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21653);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21654);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21655);
					match s {
						[] => true,
						['о', 'т', 'а', s @ ..] => {
							super::push(out, 26969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14797);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14798);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21656);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21657);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26971);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 26972);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 26973);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 30295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 26974);
							match s {
								[] => true,
								['а', 'з', 'о', s @ ..] => {
									super::push(out, 30296);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32096);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 30297);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 30298);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26975);
							match s {
								[] => true,
								['с', 'а', 'н', '\u{1}', s @ ..] => {
									super::push(out, 30300);
									s.is_empty()
								}
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
			super::push(out, 14799);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 14800);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21658);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21659);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21660);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рженный.
pub(crate) fn n8574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8183);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21661);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21662);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21663);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21664);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26977);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26978);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30301);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30302);
									s.is_empty()
								}
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
			super::push(out, 14801);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уженный.
pub(crate) fn n8575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8184);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14802);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 14803);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 14804);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 14805);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21667);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21668);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26979);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32098);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32099);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33066);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 26980);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26981);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26982);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26983);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30303);
									s.is_empty()
								}
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
			super::push(out, 14806);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21669);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 21670);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26985);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30304);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30305);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32100);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33067);
													s.is_empty()
												}
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
							super::push(out, 26986);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26987);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26988);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21671);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26990);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 26991);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21673);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26993);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30307);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 30308);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32102);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26994);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32103);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33068);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 33069);
													match s {
														[] => true,
														['е', s @ ..] => super::t10::n35129(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21674);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21675);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14807);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 14808);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14809);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14810);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыженный.
pub(crate) fn n8576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8185);
	match s {
		[] => true,
		['т', 'о', s @ ..] => {
			super::push(out, 14811);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юженный.
pub(crate) fn n8577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8186);
	s.is_empty()
}

/// Words ending -яженный.
pub(crate) fn n8578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8187);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21679);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21680);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26996);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30311);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32105);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 32106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26997);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30312);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30313);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21681);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21682);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26998);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21683);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26999);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30315);
									s.is_empty()
								}
								['н', 'е', s @ ..] => {
									super::push(out, 30316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14812);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лабленный.
pub(crate) fn n22805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21708);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30322);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 30323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рабленный.
pub(crate) fn n22806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21709);
	s.is_empty()
}

/// Words ending -ебленный.
pub(crate) fn n15584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14831);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21710);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ибленный.
pub(crate) fn n15585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14832);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 21712);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27016);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27017);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лбленный.
pub(crate) fn n15586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14833);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21713);
			match s {
				[] => true,
				['д', 'ы', s @ ..] => {
					super::push(out, 27018);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27019);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбленный.
pub(crate) fn n15587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14834);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обленный.
pub(crate) fn n15588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14835);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21715);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21716);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30326);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30327);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 30328);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21717);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 21718);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21719);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27021);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 27022);
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 30329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27023);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21720);
			match s {
				[] => true,
				['о', 'п', 'с', 'и', 'р', 'п', 'е', s @ ..] => {
					super::push(out, 27024);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбленный.
pub(crate) fn n15589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14836);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21721);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 27025);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 30332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21722);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27026);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 27027);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убленный.
pub(crate) fn n15590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14837);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 21723);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27028);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 27029);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21724);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21725);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21726);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27031);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27032);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27033);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбленный.
pub(crate) fn n15591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14838);
	s.is_empty()
}

/// Words ending -юбленный.
pub(crate) fn n15592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14839);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30337);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30338);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 32109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30339);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 30340);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 30341);
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 30342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27036);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябленный.
pub(crate) fn n15593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14840);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 21727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вленный.
pub(crate) fn n8589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8198);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14841);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21728);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21729);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21730);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27037);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27038);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27039);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27040);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21731);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27041);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27042);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 30345);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21732);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30346);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32111);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 32112);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33070);
													s.is_empty()
												}
												['е', 'р', s @ ..] => {
													super::push(out, 33071);
													match s {
														[] => true,
														['е', s @ ..] => super::t10::n35130(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 33072);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 30347);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30348);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30349);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30350);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30351);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30352);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30353);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32113);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27044);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30354);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30355);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30356);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32114);
											s.is_empty()
										}
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
					super::push(out, 21733);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30357);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30358);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33073);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 33074);
													match s {
														[] => true,
														['о', s @ ..] => super::t10::n35131(s, out),
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
									super::push(out, 30359);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32115);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 32116);
											match s {
												[] => true,
												['о', s @ ..] => {
													match s {
														[] => true,
														['к', s @ ..] => super::t10::n35132(s, out),
														['с', s @ ..] => super::t10::n35133(s, out),
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
									super::push(out, 30360);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32117);
											s.is_empty()
										}
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
			super::push(out, 14842);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21734);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 21735);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27045);
							s.is_empty()
						}
						['у', 'д', 'о', 'е', s @ ..] => {
							super::push(out, 27046);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30361);
									s.is_empty()
								}
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
			super::push(out, 14843);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21736);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 21737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n15597(s, out),
		['л', s @ ..] => {
			super::push(out, 14845);
			match s {
				[] => true,
				['о', s @ ..] => super::t10::n22842(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t10::n15599(s, out),
		['т', s @ ..] => super::t10::n15600(s, out),
		['щ', s @ ..] => {
			super::push(out, 14848);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t10::n15602(s, out),
		_ => false,
	}
}

/// Words ending -гленный.
pub(crate) fn n8590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8199);
	match s {
		[] => true,
		['у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21757);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 21758);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21759);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дленный.
pub(crate) fn n8591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8200);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14850);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27075);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27076);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 30380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27077);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 30381);
									s.is_empty()
								}
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
			super::push(out, 14851);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21760);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еленный.
pub(crate) fn n8592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8201);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14852);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21761);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27078);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21762);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21763);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14853);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21765);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27081);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 21766);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21767);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30383);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32123);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33079);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 33080);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 21768);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21769);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30385);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30386);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32124);
											s.is_empty()
										}
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
					super::push(out, 21770);
					match s {
						[] => true,
						['п', 'е', s @ ..] => {
							super::push(out, 27084);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21771);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21772);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21773);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14855);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 21774);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 27086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14856);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 21775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14857);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21776);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 27087);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30388);
									s.is_empty()
								}
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
			super::push(out, 14858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21777);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 27088);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 27089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30389);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30390);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32125);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21778);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21779);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 21780);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27090);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 30392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21781);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27091);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 30393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21782);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21783);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21784);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21785);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27093);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 30394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21786);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14859);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 14860);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21788);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зленный.
pub(crate) fn n8593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8202);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иленный.
pub(crate) fn n8594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8203);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14862);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14863);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 21791);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14864);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кленный.
pub(crate) fn n8595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8204);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14865);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14866);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 21794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мленный.
pub(crate) fn n8596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8205);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14867);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 27097);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14868);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21795);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27099);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27100);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21796);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21797);
					match s {
						[] => true,
						['т', 'с', 'у', 'е', s @ ..] => {
							super::push(out, 27102);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 21798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14869);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14870);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21800);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14871);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21801);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27104);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 32126);
											match s {
												[] => true,
												['о', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n35136(s, out),
														['е', s @ ..] => super::t10::n35137(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 32127);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21803);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27106);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 27107);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 27108);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27109);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30397);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 32128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27110);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32129);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33081);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 32130);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 30398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27111);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27112);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27113);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27114);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21804);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21805);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21806);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14872);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21807);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30400);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30401);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32131);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													super::push(out, 33082);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 30402);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30403);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32132);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 27117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14873);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14874);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27118);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32133);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33083);
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 32134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21809);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 21810);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t10::n15629(s, out),
		['ь', s @ ..] => super::t10::n15630(s, out),
		['я', s @ ..] => super::t10::n15631(s, out),
		_ => false,
	}
}

/// Words ending -оленный.
pub(crate) fn n8597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8206);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14878);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 14879);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21815);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27122);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30406);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32136);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32137);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33084);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27123);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14880);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14881);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21818);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14882);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 14883);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21820);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21821);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 27126);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 30409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 21822);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21823);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21824);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21825);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21826);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14885);
			match s {
				[] => true,
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 21827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14886);
			match s {
				[] => true,
				['о', 'д', s @ ..] => {
					super::push(out, 21828);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14887);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пленный.
pub(crate) fn n8598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8207);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14889);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14890);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21831);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14891);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21833);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27132);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27133);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30410);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27135);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21834);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27136);
							match s {
								[] => true,
								['у', 'е', s @ ..] => {
									super::push(out, 30413);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32138);
											s.is_empty()
										}
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
					super::push(out, 21835);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 21836);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 21837);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27138);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27139);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30415);
									s.is_empty()
								}
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
			super::push(out, 14892);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21838);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21839);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27141);
							match s {
								[] => true,
								['з', 'е', s @ ..] => {
									super::push(out, 30416);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27142);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27143);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27144);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30417);
									s.is_empty()
								}
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
			super::push(out, 14893);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21840);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27146);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27147);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27148);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30418);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32140);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33085);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33086);
													s.is_empty()
												}
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
							super::push(out, 27149);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27150);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21842);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27151);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27152);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21843);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21844);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27153);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 30421);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27154);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27155);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27156);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30422);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30423);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 32141);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 30424);
									s.is_empty()
								}
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
			super::push(out, 14894);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рленный.
pub(crate) fn n8599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8208);
	match s {
		[] => true,
		['е', 'в', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21846);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21847);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21848);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

