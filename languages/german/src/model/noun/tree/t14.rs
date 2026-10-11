use flexion::Path;

/// Words ending -umen.
pub(crate) fn n8203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17904);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 17905);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 27920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 17906);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17907);
			match s {
				[] => true,
				['o', 'v', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 35483);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 35484);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'g', s @ ..] => {
							super::push(out, 35485);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35486);
							s.is_empty()
						}
						['s', 'l', s @ ..] => {
							super::push(out, 35487);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40288);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 40289);
									s.is_empty()
								}
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
			super::push(out, 17908);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 17909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ymen.
pub(crate) fn n8204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8107);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 17910);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ämen.
pub(crate) fn n8205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8108);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 17911);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ömen.
pub(crate) fn n8206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8109);
	s.is_empty()
}

/// Words ending -erfahren.
pub(crate) fn n42815(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 42949);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 44187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 42950);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 42951);
			s.is_empty()
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 44188);
					s.is_empty()
				}
				['e', 'l', s @ ..] => {
					super::push(out, 44189);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 44190);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 44736);
							s.is_empty()
						}
						['h', 'a', 'w', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 44947);
									s.is_empty()
								}
								['s', 'l', s @ ..] => {
									super::push(out, 44948);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 44949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 44737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'm', s @ ..] => {
					super::push(out, 44191);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 44192);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 44738);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 44739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mitfahren.
pub(crate) fn n42818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40302);
	s.is_empty()
}

/// Words ending -zfahren.
pub(crate) fn n37489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35520);
	s.is_empty()
}

/// Words ending -wahren.
pub(crate) fn n29161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27962);
	s.is_empty()
}

/// Words ending -ehren.
pub(crate) fn n18441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17950);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 27963);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 27964);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ohren.
pub(crate) fn n18442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17951);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 27965);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 27966);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ähren.
pub(crate) fn n18443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17952);
	s.is_empty()
}

/// Words ending -ühren.
pub(crate) fn n18444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17953);
	s.is_empty()
}

/// Words ending -iren.
pub(crate) fn n8232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8135);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17954);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kren.
pub(crate) fn n8233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8136);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17956);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 17957);
			s.is_empty()
		}
		['l', 'e', 'f', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 17958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oren.
pub(crate) fn n8234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8137);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 17959);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17960);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 17961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rren.
pub(crate) fn n8235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17962);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27967);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 35522);
							match s {
								[] => true,
								['t', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 27968);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 27969);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 35523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27970);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27971);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 27972);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27973);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 35524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17963);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17964);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27974);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 27975);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 35525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'n', s @ ..] => {
			super::push(out, 17965);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 17966);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27976);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uren.
pub(crate) fn n8236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8139);
	s.is_empty()
}

/// Words ending -ären.
pub(crate) fn n8237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8140);
	s.is_empty()
}

/// Words ending -ören.
pub(crate) fn n8238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8141);
	s.is_empty()
}

/// Words ending -üren.
pub(crate) fn n8239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8142);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 17967);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 17968);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 17969);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üreisen.
pub(crate) fn n37513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35542);
	s.is_empty()
}

/// Words ending -seisen.
pub(crate) fn n29205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28003);
	s.is_empty()
}

/// Words ending -teisen.
pub(crate) fn n29206(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 35543);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 35544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weisen.
pub(crate) fn n29207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28004);
	s.is_empty()
}

/// Words ending -misen.
pub(crate) fn n18475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17983);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 28005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ksen.
pub(crate) fn n8246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8149);
	s.is_empty()
}

/// Words ending -lsen.
pub(crate) fn n8247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8150);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17984);
			s.is_empty()
		}
		['ü', 'h', 't', s @ ..] => {
			super::push(out, 17985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -msen.
pub(crate) fn n8248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8151);
	match s {
		[] => true,
		['i', 'r', s @ ..] => {
			super::push(out, 17986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nsen.
pub(crate) fn n8249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8152);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17987);
			match s {
				[] => true,
				['k', 'n', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 28006);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 28007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osen.
pub(crate) fn n8250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8153);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 17989);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17990);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 17991);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 28008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 17992);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 17993);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28009);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -psen.
pub(crate) fn n8251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8154);
	s.is_empty()
}

/// Words ending -rsen.
pub(crate) fn n8252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8155);
	match s {
		[] => true,
		['a', 's', s @ ..] => {
			super::push(out, 17994);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssen.
pub(crate) fn n8253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8156);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17995);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 28010);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28011);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 35545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17996);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 28012);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35546);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 28013);
					match s {
						[] => true,
						['h', 'c', 's', 'n', 'e', 'b', s @ ..] => {
							super::push(out, 35548);
							match s {
								[] => true,
								['e', 'n', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 40314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 35549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 28014);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 28015);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28016);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 28017);
					match s {
						[] => true,
						['i', 'e', s @ ..] => {
							super::push(out, 35551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 28018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17997);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 28019);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 28020);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 28021);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 28022);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35552);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 40315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'k', s @ ..] => {
							super::push(out, 35553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17998);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 28023);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 28024);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 28025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 17999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usen.
pub(crate) fn n8254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18000);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 28026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 18001);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ösen.
pub(crate) fn n8255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8158);
	s.is_empty()
}

/// Words ending -mungsverhalten.
pub(crate) fn n45769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42953);
	s.is_empty()
}

/// Words ending -ksverhalten.
pub(crate) fn n45770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42954);
	s.is_empty()
}

/// Words ending -nsverhalten.
pub(crate) fn n45771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42955);
	s.is_empty()
}

/// Words ending -elten.
pub(crate) fn n18511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18017);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 28049);
			s.is_empty()
		}
		['z', 'b', s @ ..] => {
			super::push(out, 28050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nten.
pub(crate) fn n8264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8167);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18018);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 28051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18019);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 28052);
					s.is_empty()
				}
				['t', 'a', 'm', 'i', 's', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 28053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oten.
pub(crate) fn n8265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8168);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 18021);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 18022);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 28054);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 35576);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 28055);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 18023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pten.
pub(crate) fn n8266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8169);
	s.is_empty()
}

/// Words ending -rten.
pub(crate) fn n8267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8170);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18024);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 28057);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 28058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 28059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18025);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 28060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 18026);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18027);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 18028);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 18029);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 28061);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sten.
pub(crate) fn n8268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8171);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18030);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 28062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 28063);
					match s {
						[] => true,
						['-', 'e', 'f', 'l', 'i', 'h', '-', 'e', 't', 's', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 35579);
							s.is_empty()
						}
						['t', 't', 'ü', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 35580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 28064);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 28065);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28066);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 28067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18031);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28068);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 28069);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 28070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 18032);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18033);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28072);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 28073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 18034);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18035);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 28074);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 28075);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 35581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 28076);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 28077);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35582);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 35583);
							match s {
								[] => true,
								['c', 'a', s @ ..] => {
									super::push(out, 40323);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 35584);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35585);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40324);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 42956);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'u', 'f', 'b', 'a', 'l', s @ ..] => {
									super::push(out, 40325);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 40326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 35586);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42957);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42958);
											match s {
												[] => true,
												['u', s @ ..] => {
													match s {
														[] => true,
														['f', s @ ..] => super::t15::n47778(s, out),
														['h', s @ ..] => super::t15::n47779(s, out),
														['t', 'l', 'a', 'w', s @ ..] => super::t15::n47780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'a', s @ ..] => {
									super::push(out, 40327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', s @ ..] => {
							super::push(out, 35587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 28078);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 35588);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 35589);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40329);
									match s {
										[] => true,
										['e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 42959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 35590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 28079);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 35591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18036);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 18037);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 28080);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 28081);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 18038);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 28083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 18039);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 18040);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tten.
pub(crate) fn n8269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8172);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18041);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 28086);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 40330);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 40331);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 42960);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42961);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 40332);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 40333);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', 'u', s @ ..] => {
													super::push(out, 44193);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 44194);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 40334);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42962);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 44195);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 44196);
													s.is_empty()
												}
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
					super::push(out, 28087);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 28088);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 28089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18042);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 28090);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 28091);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 28092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18043);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 28093);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 28094);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28095);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 28096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 18044);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 28097);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 28098);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 28099);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 18045);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 28101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 18046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uten.
pub(crate) fn n8270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8173);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 18047);
			s.is_empty()
		}
		['l', 'g', s @ ..] => {
			super::push(out, 18048);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 18049);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 28102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18050);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 18051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äten.
pub(crate) fn n8271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8174);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 18052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öten.
pub(crate) fn n8272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8175);
	match s {
		[] => true,
		['l', 'k', s @ ..] => {
			super::push(out, 18053);
			s.is_empty()
		}
		['r', 'k', s @ ..] => {
			super::push(out, 18054);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 18055);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üten.
pub(crate) fn n8273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8176);
	s.is_empty()
}

/// Words ending -enschein.
pub(crate) fn n42873(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 42969);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 42970);
			match s {
				[] => true,
				['i', 'l', 'i', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 44198);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 44199);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 44743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 42971);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 42972);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 44200);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 44201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 42973);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 42974);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 42975);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -merschein.
pub(crate) fn n45793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42976);
	s.is_empty()
}

/// Words ending -rerschein.
pub(crate) fn n45794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42977);
	s.is_empty()
}

/// Words ending -uerschein.
pub(crate) fn n45795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42978);
	s.is_empty()
}

/// Words ending -orschein.
pub(crate) fn n42875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40354);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 42979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sschein.
pub(crate) fn n37616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35640);
	s.is_empty()
}

/// Words ending -tschein.
pub(crate) fn n37617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35641);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 40355);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 42980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 40356);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44202);
							s.is_empty()
						}
						['s', 'g', 'n', 'u', s @ ..] => {
							match s {
								[] => true,
								['d', 'l', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 44744);
									s.is_empty()
								}
								['l', 't', 't', 'i', 'm', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 44745);
									s.is_empty()
								}
								['r', 'e', 'i', 'v', 'i', 't', 'k', 'a', '\u{1}', s @ ..] => {
									super::push(out, 44746);
									s.is_empty()
								}
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

/// Words ending -rhein.
pub(crate) fn n18688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18187);
	s.is_empty()
}

/// Words ending -thein.
pub(crate) fn n18689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18188);
	s.is_empty()
}

/// Words ending -jein.
pub(crate) fn n8362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8264);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lein.
pub(crate) fn n8363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8265);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18190);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 18191);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 28188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 18192);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 28189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 18193);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					super::push(out, 28190);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 35642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 18194);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 18195);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 18196);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 28192);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 28193);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 28194);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 28195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 18197);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 28196);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 35643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 18198);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18199);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18200);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 18201);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 18202);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 18203);
			match s {
				[] => true,
				['ä', 'r', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35644);
							s.is_empty()
						}
						['s', 't', 'f', 'i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 35645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mein.
pub(crate) fn n8364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8266);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 28198);
					s.is_empty()
				}
				['k', 'n', 'e', 'd', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 28199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nein.
pub(crate) fn n8365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8267);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18204);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pein.
pub(crate) fn n8366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8268);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 18206);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rein.
pub(crate) fn n8367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8269);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18207);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 18208);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 35646);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'h', s @ ..] => {
							super::push(out, 35647);
							match s {
								[] => true,
								['a', 'w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42981);
											s.is_empty()
										}
										['r', 'e', 'l', 'z', 'n', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 35648);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									super::push(out, 40358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 35649);
							match s {
								[] => true,
								['r', 'o', 'p', 's', 'l', s @ ..] => {
									super::push(out, 40359);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 40360);
									s.is_empty()
								}
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

/// Words ending -sein.
pub(crate) fn n8368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8270);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18209);
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					super::push(out, 28201);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 28202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 18210);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 18211);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 28203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18212);
			match s {
				[] => true,
				['s', 's', 'u', 'w', 'e', 'b', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 35650);
							s.is_empty()
						}
						['s', 't', s @ ..] => {
							super::push(out, 35651);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40361);
									s.is_empty()
								}
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

/// Words ending -tein.
pub(crate) fn n8369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8271);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18213);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 18214);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 28204);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 35652);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 40362);
									s.is_empty()
								}
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
			super::push(out, 18215);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 18216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28206);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 28207);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 35653);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 40363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 28208);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 35654);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 35655);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 35656);
							match s {
								[] => true,
								['d', 'n', 'u', s @ ..] => {
									super::push(out, 40364);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42983);
											s.is_empty()
										}
										['u', 'r', s @ ..] => {
											super::push(out, 42984);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'i', 's', s @ ..] => {
									super::push(out, 40365);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 40366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 35657);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 28209);
					match s {
						[] => true,
						['i', 'e', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 35659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 28210);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 35660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 28211);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40367);
									s.is_empty()
								}
								['e', 'p', s @ ..] => {
									super::push(out, 40368);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 42985);
											s.is_empty()
										}
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
					super::push(out, 28212);
					match s {
						[] => true,
						['e', 'k', s @ ..] => {
							super::push(out, 35661);
							match s {
								[] => true,
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 40369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 28213);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35662);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 40370);
									match s {
										[] => true,
										['i', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 42986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 40371);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42987);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 40372);
									match s {
										[] => true,
										['c', 'i', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 42988);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 42989);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 40373);
									s.is_empty()
								}
								['n', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 40374);
									s.is_empty()
								}
								['r', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 40375);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 40376);
									match s {
										[] => true,
										['a', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42990);
											s.is_empty()
										}
										['i', 'e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44203);
													s.is_empty()
												}
												['t', 'e', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 44204);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'k', 'i', 'r', 'p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42991);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 35663);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 35664);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42992);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 42993);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 44205);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => super::t15::n42902(s, out),
								['r', s @ ..] => super::t15::n42903(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t15::n37642(s, out),
						['o', 't', '\u{1}', s @ ..] => super::t15::n37643(s, out),
						['r', s @ ..] => super::t15::n37644(s, out),
						['u', s @ ..] => super::t15::n37645(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t15::n29430(s, out),
				['s', s @ ..] => super::t15::n29431(s, out),
				['t', s @ ..] => super::t15::n29432(s, out),
				['u', s @ ..] => super::t15::n29433(s, out),
				['y', s @ ..] => super::t15::n29434(s, out),
				['z', s @ ..] => super::t15::n29435(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n18719(s, out),
		_ => false,
	}
}

/// Words ending -wein.
pub(crate) fn n8370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8272);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18218);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 18219);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 18220);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 28221);
					match s {
						[] => true,
						['n', 'ä', 'g', '\u{1}', s @ ..] => {
							super::push(out, 35675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 18221);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					super::push(out, 28222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 18222);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 28223);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35676);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 35677);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 35678);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 35679);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40386);
									match s {
										[] => true,
										['c', 'u', 'a', 'b', 'e', 'g', 'n', 'ä', 'h', '\u{1}', s @ ..] => {
											super::push(out, 42996);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 40387);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 42997);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 'h', 'c', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 40388);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 40389);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 44206);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 44207);
													s.is_empty()
												}
												['s', 'a', 'r', 'h', 'p', '\u{1}', s @ ..] => {
													super::push(out, 44208);
													s.is_empty()
												}
												['t', 's', 'i', s @ ..] => {
													match s {
														[] => true,
														['h', s @ ..] => super::t15::n47785(s, out),
														['n', 'i', s @ ..] => super::t15::n47786(s, out),
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
										['a', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 42998);
											s.is_empty()
										}
										['e', 't', s @ ..] => {
											super::push(out, 42999);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 43000);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 43001);
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 44209);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'u', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 40390);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40391);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 40392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 28224);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 28225);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 35680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18223);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 18224);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 18225);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 28226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 18226);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 18227);
			match s {
				[] => true,
				['e', 'k', s @ ..] => {
					super::push(out, 28227);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 35681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18228);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 18229);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 18230);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28228);
					match s {
						[] => true,
						['n', 'a', 'r', 'b', 'z', s @ ..] => {
							super::push(out, 35682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ß', s @ ..] => {
			super::push(out, 18231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rlin.
pub(crate) fn n8405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8307);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18312);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 28278);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 28279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulin.
pub(crate) fn n8406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8308);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 18313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28280);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 28281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 18314);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 18315);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 18316);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35706);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 35707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ylin.
pub(crate) fn n8407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8309);
	match s {
		[] => true,
		['b', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 18317);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 18318);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 18319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l-lehrerin.
pub(crate) fn n47186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44210);
	s.is_empty()
}

/// Words ending -alllehrerin.
pub(crate) fn n47187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44211);
	s.is_empty()
}

/// Words ending -opterin.
pub(crate) fn n37717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35737);
	match s {
		[] => true,
		['d', 'b', 'y', 'l', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 40405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -esterin.
pub(crate) fn n29576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28349);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 35738);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 40406);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -verin.
pub(crate) fn n18900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18394);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 28350);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 35739);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 35740);
					s.is_empty()
				}
				['t', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 35741);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zerin.
pub(crate) fn n18901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18395);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 28351);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 28352);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 35742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßerin.
pub(crate) fn n18902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18396);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 28353);
			match s {
				[] => true,
				['t', 's', 'h', s @ ..] => {
					super::push(out, 35743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -grin.
pub(crate) fn n8448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8350);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrin.
pub(crate) fn n8449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8351);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 18398);
			match s {
				[] => true,
				['y', 'r', 'e', 'm', 'ä', 'h', '\u{1}', s @ ..] => {
					super::push(out, 28354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irin.
pub(crate) fn n8450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8352);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18399);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 18400);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 28355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orin.
pub(crate) fn n8451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8353);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 18401);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 28356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35744);
							s.is_empty()
						}
						['-', 'a', 'b', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 35745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18402);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 28357);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 35746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrin.
pub(crate) fn n8452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8354);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 28358);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35747);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 35748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 28359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trin.
pub(crate) fn n8453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8355);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18403);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 18404);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 28360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 18405);
			match s {
				[] => true,
				['o', 'd', '-', s @ ..] => {
					super::push(out, 28361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 18406);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urin.
pub(crate) fn n8454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18407);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 18408);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 28363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 18409);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 28364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18410);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 28365);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 35749);
							match s {
								[] => true,
								['f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40407);
									s.is_empty()
								}
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
			super::push(out, 18411);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28366);
					match s {
						[] => true,
						['g', 'r', s @ ..] => {
							super::push(out, 35750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18412);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18413);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yrin.
pub(crate) fn n8455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8357);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 18414);
			match s {
				[] => true,
				['p', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 28369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -contin.
pub(crate) fn n29643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28413);
	s.is_empty()
}

/// Words ending -otin.
pub(crate) fn n8477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8379);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 18457);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 18458);
			s.is_empty()
		}
		['g', 'r', s @ ..] => {
			super::push(out, 18459);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 28414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 18460);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 28416);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 28417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 18461);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 18462);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 18463);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 28418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptin.
pub(crate) fn n8478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8380);
	match s {
		[] => true,
		['e', 'p', s @ ..] => {
			super::push(out, 18464);
			match s {
				[] => true,
				['o', 'i', 'r', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 28419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtin.
pub(crate) fn n8479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8381);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18465);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 28420);
					match s {
						[] => true,
						['s', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 35773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 28421);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 28422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18466);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 28423);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 28424);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 28425);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stin.
pub(crate) fn n8480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8382);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18468);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 28426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18469);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 28427);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 28428);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35777);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40413);
									match s {
										[] => true,
										['e', 'l', '-', s @ ..] => {
											super::push(out, 43009);
											s.is_empty()
										}
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
					super::push(out, 28429);
					match s {
						[] => true,
						['i', 'l', 'b', s @ ..] => {
							super::push(out, 35778);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40414);
									s.is_empty()
								}
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
			super::push(out, 18470);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 28430);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 28431);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttin.
pub(crate) fn n8481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8383);
	s.is_empty()
}

/// Words ending -utin.
pub(crate) fn n8482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8384);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 18471);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18472);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 18473);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 28433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18474);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 28434);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 35779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ztin.
pub(crate) fn n8483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8385);
	s.is_empty()
}

/// Words ending -ätin.
pub(crate) fn n8484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8386);
	s.is_empty()
}

/// Words ending -ndsmann.
pub(crate) fn n37803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35822);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 40431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ngsmann.
pub(crate) fn n29705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28468);
	s.is_empty()
}

/// Words ending -lsmann.
pub(crate) fn n29706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28469);
	s.is_empty()
}

/// Words ending -nsmann.
pub(crate) fn n29707(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 40432);
					match s {
						[] => true,
						['c', 'e', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 43017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 40433);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 40434);
					match s {
						[] => true,
						['a', 'r', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 43018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 35824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -psmann.
pub(crate) fn n29708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28470);
	s.is_empty()
}

/// Words ending -rsmann.
pub(crate) fn n29709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28471);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 40435);
					s.is_empty()
				}
				['g', 'ä', 'j', '\u{1}', s @ ..] => {
					super::push(out, 40436);
					s.is_empty()
				}
				['t', 't', s @ ..] => {
					super::push(out, 40437);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsmann.
pub(crate) fn n29710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28472);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 35825);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 35826);
			match s {
				[] => true,
				['ä', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 40438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 35827);
			match s {
				[] => true,
				['c', 'a', 'n', 'h', 'i', 'e', 'w', 'n', 'e', 'd', 'a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 40439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 40440);
					s.is_empty()
				}
				['l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 40441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 35828);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 35829);
			match s {
				[] => true,
				['o', 'b', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 43019);
							s.is_empty()
						}
						['s', 'b', 'a', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 44213);
									s.is_empty()
								}
								['r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 44214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 43020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'o', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 35830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tmann.
pub(crate) fn n19037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18524);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 28473);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 28474);
			s.is_empty()
		}
		['n', 'u', s @ ..] => {
			super::push(out, 28475);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 28476);
			match s {
				[] => true,
				['u', 'a', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40442);
							s.is_empty()
						}
						['s', 'e', 'd', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 40443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'o', s @ ..] => {
			super::push(out, 28477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35831);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zmann.
pub(crate) fn n19038(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 28478);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 28479);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word spielbeginn.
pub(crate) fn n47194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44218);
	s.is_empty()
}

/// Words ending -sbildungsbeginn.
pub(crate) fn n47195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44219);
	s.is_empty()
}

/// Words ending -kungsbeginn.
pub(crate) fn n47196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44220);
	s.is_empty()
}

/// Words ending -winn.
pub(crate) fn n8533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8435);
	match s {
		[] => true,
		['e', 'g', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28503);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35842);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 43024);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 43025);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 't', 't', s @ ..] => {
					super::push(out, 28504);
					match s {
						[] => true,
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 35843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', 'n', 'u', 'r', 'e', 'ß', s @ ..] => {
							super::push(out, 35844);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 35845);
							match s {
								[] => true,
								['a', 't', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 40459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'f', s @ ..] => {
					super::push(out, 28505);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lzinn.
pub(crate) fn n19058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18542);
	s.is_empty()
}

/// Words ending -tzinn.
pub(crate) fn n19059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18543);
	s.is_empty()
}

/// Words ending -pion.
pub(crate) fn n8587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8489);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18627);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 18628);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 28574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18629);
			match s {
				[] => true,
				['o', 'k', 's', 'r', 'e', 's', s @ ..] => {
					super::push(out, 28575);
					match s {
						[] => true,
						['s', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 35869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 18630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rion.
pub(crate) fn n8588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8490);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 18631);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 18632);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 28576);
					s.is_empty()
				}
				['t', 't', s @ ..] => {
					super::push(out, 28577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 18633);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28578);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 35870);
							s.is_empty()
						}
						['p', 'o', 't', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 35871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 28579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 18635);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 18636);
			match s {
				[] => true,
				['s', 'e', 'h', 'c', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 28580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 18637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sion.
pub(crate) fn n8589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8491);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18638);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 28581);
					match s {
						[] => true,
						['m', 'y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 35872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 28582);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 28583);
					match s {
						[] => true,
						['s', 'o', 'n', 'o', 'd', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 35873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 28584);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35874);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18639);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 35876);
							s.is_empty()
						}
						['o', 'r', 'u', s @ ..] => {
							super::push(out, 35877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 28585);
					match s {
						[] => true,
						['ä', s @ ..] => {
							super::push(out, 35878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			super::push(out, 18640);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 35879);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 35880);
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
				['l', 'p', 'x', 'e', 'z', s @ ..] => {
					super::push(out, 28586);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 28587);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 35881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 28588);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 35882);
							s.is_empty()
						}
						['r', 'o', 'k', 'ß', s @ ..] => {
							super::push(out, 35883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 28589);
					match s {
						[] => true,
						['s', 'i', 'd', 'r', s @ ..] => {
							super::push(out, 35884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 28590);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35885);
							match s {
								[] => true,
								['r', 't', 'n', 'e', s @ ..] => {
									super::push(out, 40469);
									s.is_empty()
								}
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
			super::push(out, 18641);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28591);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 35886);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 35887);
							match s {
								[] => true,
								['-', 'm', 'a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 40470);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 40471);
									s.is_empty()
								}
								['m', 'a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 40472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 35888);
							match s {
								[] => true,
								['ä', s @ ..] => {
									super::push(out, 40473);
									match s {
										[] => true,
										['r', 'p', 'r', s @ ..] => {
											super::push(out, 43026);
											s.is_empty()
										}
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
					super::push(out, 28592);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 35889);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['m', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 43027);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 44221);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 44222);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', 'e', s @ ..] => {
											super::push(out, 43028);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 40474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'f', 'f', s @ ..] => {
			super::push(out, 18642);
			match s {
				[] => true,
				['i', 'd', 's', s @ ..] => {
					super::push(out, 28593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', 'h', s @ ..] => {
			super::push(out, 18643);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 28594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tion.
pub(crate) fn n8590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8492);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18644);
			match s {
				[] => true,
				['b', 'r', 'u', 'n', 'o', 'c', s @ ..] => {
					super::push(out, 28595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 28596);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 28597);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 35891);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 40475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 35892);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35893);
							match s {
								[] => true,
								['u', 'o', s @ ..] => {
									super::push(out, 40476);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 43029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 35894);
							match s {
								[] => true,
								['a', 't', 'e', 'r', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 43030);
											s.is_empty()
										}
										['s', 'l', s @ ..] => {
											super::push(out, 43031);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 35895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 28598);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40477);
									s.is_empty()
								}
								['r', 'g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43032);
											s.is_empty()
										}
										['n', 'o', 'k', 's', s @ ..] => {
											super::push(out, 43033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 35896);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40478);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40479);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 40480);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 43034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'r', 'e', s @ ..] => {
							super::push(out, 35897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 28599);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							super::push(out, 35898);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35899);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 35900);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 43035);
											s.is_empty()
										}
										['e', 'r', 'e', 'f', 'f', 'i', 'd', 's', s @ ..] => {
											super::push(out, 43036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'n', 'e', s @ ..] => super::t15::n37894(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t15::n29845(s, out),
				['l', s @ ..] => super::t15::n29846(s, out),
				['m', s @ ..] => super::t15::n29847(s, out),
				['n', s @ ..] => super::t15::n29848(s, out),
				['p', 'i', 'z', 'n', 'a', 'm', 'e', 'n', s @ ..] => super::t15::n29849(s, out),
				['r', s @ ..] => super::t15::n29850(s, out),
				['s', s @ ..] => super::t15::n29851(s, out),
				['t', s @ ..] => super::t15::n29852(s, out),
				['u', s @ ..] => super::t15::n29853(s, out),
				['v', s @ ..] => super::t15::n29854(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t15::n19165(s, out),
		['e', s @ ..] => super::t15::n19166(s, out),
		['i', s @ ..] => super::t15::n19167(s, out),
		['k', s @ ..] => super::t15::n19168(s, out),
		['n', 'e', 'v', s @ ..] => super::t15::n19169(s, out),
		['o', s @ ..] => super::t15::n19170(s, out),
		['p', s @ ..] => super::t15::n19171(s, out),
		['r', s @ ..] => super::t15::n19172(s, out),
		['s', s @ ..] => super::t15::n19173(s, out),
		['u', s @ ..] => super::t15::n19174(s, out),
		_ => false,
	}
}

/// Words ending -plexion.
pub(crate) fn n19175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18655);
	match s {
		[] => true,
		['m', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 28635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zion.
pub(crate) fn n8592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8494);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ston.
pub(crate) fn n8680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8582);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18819);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 28757);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 28758);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 28759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 18820);
			match s {
				[] => true,
				['n', 'a', 'g', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 28760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 18821);
			match s {
				[] => true,
				['c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 28761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 18822);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 28762);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 28763);
					match s {
						[] => true,
						['r', 'ü', 't', '\u{1}', s @ ..] => {
							super::push(out, 35993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 28764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 18823);
			match s {
				[] => true,
				['h', 'e', 'f', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 28765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 18824);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 18825);
			match s {
				[] => true,
				['e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 28766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 18826);
			match s {
				[] => true,
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 28767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 18827);
			match s {
				[] => true,
				['a', 'k', 'o', 'l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 28768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tton.
pub(crate) fn n8681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8583);
	match s {
		[] => true,
		['i', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 18828);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 28769);
					s.is_empty()
				}
				['u', 'q', 'i', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 28770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 18829);
			s.is_empty()
		}
		['t', 'e', 'l', 'o', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 18830);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 18831);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 28771);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 28772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 18832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -auton.
pub(crate) fn n19359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18833);
	match s {
		[] => true,
		['l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 28773);
			s.is_empty()
		}
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 28774);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -muton.
pub(crate) fn n19361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18835);
	s.is_empty()
}

/// Words ending -wton.
pub(crate) fn n8683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8585);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yton.
pub(crate) fn n8684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8586);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 18837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zton.
pub(crate) fn n8685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8587);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 28775);
					s.is_empty()
				}
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 28776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßton.
pub(crate) fn n8686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8588);
	match s {
		[] => true,
		['i', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 18838);
			s.is_empty()
		}
		['o', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 18839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ûton.
pub(crate) fn n8687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8589);
	match s {
		[] => true,
		['o', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 18840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rkorn.
pub(crate) fn n19445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18911);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 36043);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 36044);
					s.is_empty()
				}
				['m', 'm', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 36045);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 36046);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kreiskorn.
pub(crate) fn n30128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28863);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tkorn.
pub(crate) fn n19447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18913);
	match s {
		[] => true,
		['a', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 28864);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 28865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -porn.
pub(crate) fn n8739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8641);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 18914);
			match s {
				[] => true,
				['o', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 28866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 18915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28867);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 28868);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36048);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 28869);
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 36050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zorn.
pub(crate) fn n8740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8642);
	s.is_empty()
}

/// Words ending -nder.
pub(crate) fn n9365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9256);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19400);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 29110);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29111);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29112);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36137);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 43093);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 43094);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 36138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19401);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29113);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 29114);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36139);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 36140);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36141);
							match s {
								[] => true,
								['ö', 'w', 'z', '\u{1}', s @ ..] => {
									super::push(out, 40588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 29116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36142);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 36143);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36144);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 40589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 29117);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29118);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29119);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29120);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 36145);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 29121);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29122);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29123);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36147);
							match s {
								[] => true,
								['e', 'm', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 40590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'a', s @ ..] => {
							super::push(out, 36148);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 36149);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 36150);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36151);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36152);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 36153);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36154);
							s.is_empty()
						}
						['s', 'i', s @ ..] => {
							super::push(out, 36155);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 36156);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 36157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36158);
							s.is_empty()
						}
						['e', 'k', s @ ..] => {
							super::push(out, 36159);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 36160);
							s.is_empty()
						}
						['r', 'a', 'b', s @ ..] => {
							super::push(out, 36161);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 29125);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 29126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 19402);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19403);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29127);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 36163);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 36164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 19404);
			match s {
				[] => true,
				['e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 29128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19405);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 29129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19406);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19407);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 29131);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29132);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29133);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36165);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 36166);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36167);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 36168);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 36169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 29134);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 29135);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19408);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29136);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29137);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36171);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36172);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40593);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 40594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 36173);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36174);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36175);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36176);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 19409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -loder.
pub(crate) fn n19965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19412);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 29140);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 29141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rder.
pub(crate) fn n9367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9258);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19415);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19416);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 19417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29142);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 29143);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29144);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29145);
					match s {
						[] => true,
						['a', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 36178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29146);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 19418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uder.
pub(crate) fn n9368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9259);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19419);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29147);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19420);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 29149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 19421);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 19422);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 19423);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 19424);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 19425);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 19426);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 29150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19427);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29151);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 29152);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29153);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 29154);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29155);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 36182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äder.
pub(crate) fn n9369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9260);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öder.
pub(crate) fn n9370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9261);
	s.is_empty()
}

/// Words ending -üder.
pub(crate) fn n9371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9262);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 19429);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssopfer.
pub(crate) fn n38232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36217);
	s.is_empty()
}

/// Words ending -stopfer.
pub(crate) fn n30493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29208);
	s.is_empty()
}

/// Words ending -upfer.
pub(crate) fn n20019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19464);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 29209);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 29210);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 36218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 29211);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 29212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öpfer.
pub(crate) fn n20020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19465);
	s.is_empty()
}

/// Words ending -üpfer.
pub(crate) fn n20021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19466);
	s.is_empty()
}

/// Words ending -rfer.
pub(crate) fn n9391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9282);
	s.is_empty()
}

/// Words ending -sfer.
pub(crate) fn n9392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9283);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 19467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ufer.
pub(crate) fn n9393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9284);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 19468);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 19469);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 19470);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 19471);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19472);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 19473);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 29213);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 19474);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 19475);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 19476);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 19477);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 19478);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29215);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19479);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 19480);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 19481);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 19482);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äfer.
pub(crate) fn n9394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9285);
	match s {
		[] => true,
		['k', 'l', s @ ..] => {
			super::push(out, 19483);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 29217);
					match s {
						[] => true,
						['e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 36219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üfer.
pub(crate) fn n9395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9286);
	s.is_empty()
}

/// The word kunstverständiger.
pub(crate) fn n47220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44243);
	s.is_empty()
}

/// Words ending -nniger.
pub(crate) fn n30525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29238);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 36232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uniger.
pub(crate) fn n30526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29239);
	s.is_empty()
}

/// Words ending -riger.
pub(crate) fn n20057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19501);
	s.is_empty()
}

/// Words ending -siger.
pub(crate) fn n20058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19502);
	match s {
		[] => true,
		['s', 'i', 'e', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 29240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiger.
pub(crate) fn n20059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19503);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29241);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 29242);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 36233);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 36234);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 36235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 29243);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 40607);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 43096);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 43097);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43098);
									s.is_empty()
								}
								['s', 'e', s @ ..] => {
									super::push(out, 43099);
									s.is_empty()
								}
								['w', 'h', s @ ..] => {
									super::push(out, 43100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'e', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 40608);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 40609);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 40610);
							match s {
								[] => true,
								['s', 'f', s @ ..] => {
									super::push(out, 43101);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 44244);
											s.is_empty()
										}
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
			super::push(out, 29244);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29245);
			match s {
				[] => true,
				['a', 'w', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['h', 'e', 's', 'n', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 40611);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 40612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 29246);
			match s {
				[] => true,
				['e', 'z', 'o', 'r', 'p', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 36236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			super::push(out, 29247);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 29248);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 36237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', s @ ..] => {
			super::push(out, 29249);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 29250);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 29251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wiger.
pub(crate) fn n20060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19504);
	s.is_empty()
}

/// Words ending -ziger.
pub(crate) fn n20061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19505);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 29252);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 29253);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 29254);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 36238);
					s.is_empty()
				}
				['i', 'w', 'n', 'h', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 36239);
					s.is_empty()
				}
				['u', 't', 's', 's', 'f', 'f', 'i', 'r', 'g', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 36240);
					s.is_empty()
				}
				['ä', 's', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 36241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßiger.
pub(crate) fn n20062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19506);
	match s {
		[] => true,
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 36242);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 40613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 36243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lger.
pub(crate) fn n9401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9292);
	s.is_empty()
}

/// Words ending -nger.
pub(crate) fn n9402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9293);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19507);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 29255);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19508);
			match s {
				[] => true,
				['r', 'ü', s @ ..] => {
					super::push(out, 29256);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29257);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 36245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 29258);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 36246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19509);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29259);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29260);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19510);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 29262);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36247);
							match s {
								[] => true,
								['d', 'n', 'u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 40615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'd', s @ ..] => {
					super::push(out, 29263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rger.
pub(crate) fn n9403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9294);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19511);
			match s {
				[] => true,
				['b', 'e', 'n', 'a', s @ ..] => {
					super::push(out, 29264);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19512);
			match s {
				[] => true,
				['b', 's', s @ ..] => {
					super::push(out, 29266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19513);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 19514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uger.
pub(crate) fn n9404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9295);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19515);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 19516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29267);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äger.
pub(crate) fn n9405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9296);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 19518);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 29270);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 36248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üger.
pub(crate) fn n9406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9297);
	s.is_empty()
}

/// Words ending -ischer.
pub(crate) fn n30572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29284);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 36259);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 36260);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 36261);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 36262);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 40625);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 40626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 36263);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 36264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lscher.
pub(crate) fn n30573(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 36265);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 40627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 36266);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 40628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 36267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mscher.
pub(crate) fn n30574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29285);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 36268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nscher.
pub(crate) fn n30575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29286);
	s.is_empty()
}

/// Words ending -rscher.
pub(crate) fn n30576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29287);
	s.is_empty()
}

/// Words ending -tscher.
pub(crate) fn n30577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29288);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 36269);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 36270);
			match s {
				[] => true,
				['l', 'g', 'h', s @ ..] => {
					super::push(out, 40629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 36271);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 40630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 36272);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 40631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 40632);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 40633);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 40634);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 40635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 36273);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 40636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uscher.
pub(crate) fn n30578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29289);
	s.is_empty()
}

/// Words ending -äscher.
pub(crate) fn n30579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29290);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 36274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öscher.
pub(crate) fn n30580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29291);
	s.is_empty()
}

/// Words ending -ucher.
pub(crate) fn n20080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19524);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 29292);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 29293);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 36275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 29294);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 29295);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 36276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eher.
pub(crate) fn n9408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9299);
	match s {
		[] => true,
		['i', 'w', s @ ..] => {
			super::push(out, 19525);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19526);
			match s {
				[] => true,
				['n', 'r', 'e', 'f', 'n', s @ ..] => {
					super::push(out, 29297);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36277);
							match s {
								[] => true,
								['r', 'h', 'ö', 'r', '\u{1}', s @ ..] => {
									super::push(out, 40637);
									s.is_empty()
								}
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

/// Words ending -iher.
pub(crate) fn n9409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9300);
	s.is_empty()
}

/// Words ending -pher.
pub(crate) fn n9410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9301);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19527);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 19528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ther.
pub(crate) fn n9411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9302);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19529);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19530);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 29298);
					match s {
						[] => true,
						['o', 't', '-', 't', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 29299);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36279);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 36280);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40638);
									match s {
										[] => true,
										['t', 'e', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 44245);
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 44246);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 40639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19531);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 29301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 19532);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19533);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 19534);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19535);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 19536);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 29304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äher.
pub(crate) fn n9412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9303);
	s.is_empty()
}

/// Words ending -rmeier.
pub(crate) fn n30621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29327);
	match s {
		[] => true,
		['e', 'd', 'e', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 36300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -peier.
pub(crate) fn n20121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19562);
	s.is_empty()
}

/// Words ending -reier.
pub(crate) fn n20122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19563);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 29328);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 36301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 29329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -seier.
pub(crate) fn n20123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19564);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 29330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weier.
pub(crate) fn n20124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19565);
	s.is_empty()
}

/// The word polizeier.
pub(crate) fn n20125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19566);
	s.is_empty()
}

/// Words ending -fier.
pub(crate) fn n9418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9309);
	s.is_empty()
}

/// Words ending -gier.
pub(crate) fn n9419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9310);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19567);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 29331);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 19568);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 19569);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19570);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 19571);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 19572);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19573);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 29334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19574);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 19575);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 19576);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 19577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hier.
pub(crate) fn n9420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9311);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29335);
					s.is_empty()
				}
				['b', 'u', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 29336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kier.
pub(crate) fn n9421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9312);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 19578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lier.
pub(crate) fn n9422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9313);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19579);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 29337);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29338);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29339);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 29340);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 29341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19580);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29342);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29343);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29344);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 36302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29345);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36303);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 36304);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 29346);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 36306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 19581);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 29347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19582);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 19583);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29348);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 29349);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19584);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 29351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19585);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 29352);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29353);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mier.
pub(crate) fn n9423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9314);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19586);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19587);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 19588);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36307);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36308);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 40644);
									s.is_empty()
								}
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

/// Words ending -nier.
pub(crate) fn n9424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9315);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19589);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 29355);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36309);
							s.is_empty()
						}
						['s', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 36310);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 29357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36312);
							s.is_empty()
						}
						['n', 'e', 's', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 36313);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 29358);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 36315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19590);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 29360);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29361);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 19591);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19592);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29363);
					match s {
						[] => true,
						['a', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 36316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29364);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36317);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 36318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 19593);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29365);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19594);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29367);
					s.is_empty()
				}
				['g', 'a', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 29368);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 29369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36319);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 36320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 29370);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 29371);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29372);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19595);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29374);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 36321);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 36322);
							match s {
								[] => true,
								['c', 's', 'l', s @ ..] => {
									super::push(out, 40645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29375);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 29376);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36324);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40646);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 40647);
									s.is_empty()
								}
								['h', 'c', 'a', 'h', 'c', 's', 'l', s @ ..] => {
									super::push(out, 40648);
									s.is_empty()
								}
								['r', 'e', 't', s @ ..] => {
									super::push(out, 40649);
									s.is_empty()
								}
								['t', 'r', s @ ..] => {
									super::push(out, 40650);
									s.is_empty()
								}
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
			super::push(out, 19596);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 19597);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36326);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oier.
pub(crate) fn n9425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9316);
	s.is_empty()
}

/// Words ending -pier.
pub(crate) fn n9426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9317);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19598);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36328);
							s.is_empty()
						}
						['-', 'c', 'w', '\u{1}', s @ ..] => {
							super::push(out, 36329);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 36330);
							s.is_empty()
						}
						['b', 'i', 'e', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 36331);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40651);
									s.is_empty()
								}
								['n', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 40652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40653);
									s.is_empty()
								}
								['s', 'i', 'e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43108);
											s.is_empty()
										}
										['n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 43109);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['k', 'n', 'u', 'p', 'k', 'c', 'e', '\u{1}', s @ ..] => {
											super::push(out, 43110);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 43111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', 'i', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40654);
									s.is_empty()
								}
								['i', 'e', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 40655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'l', 'c', 'y', 'c', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 40656);
									s.is_empty()
								}
								['u', 'e', 'z', 'r', 'h', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 40657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 36332);
							s.is_empty()
						}
						['i', 'm', 'm', 'u', 'g', 'u', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 36333);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 36334);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 43112);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 43113);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 43114);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 40658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 36335);
							match s {
								[] => true,
								['a', 'n', 'o', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40659);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 43115);
											s.is_empty()
										}
										['k', 'c', 'i', 'w', 'n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 43116);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 40660);
									s.is_empty()
								}
								['o', 'i', 'n', 'n', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 40661);
									s.is_empty()
								}
								['ö', '\u{1}', s @ ..] => {
									super::push(out, 40662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', 'e', 'h', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36336);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36337);
							match s {
								[] => true,
								['e', 't', 't', 'e', s @ ..] => {
									super::push(out, 40663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 36338);
							match s {
								[] => true,
								['k', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40664);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 40665);
									s.is_empty()
								}
								['r', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40666);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40667);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'p', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 36339);
							s.is_empty()
						}
						['r', 'e', s @ ..] => super::t15::n38365(s, out),
						['s', s @ ..] => super::t15::n38366(s, out),
						['t', s @ ..] => super::t15::n38367(s, out),
						['z', s @ ..] => {
							super::push(out, 36342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 29377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t15::n20159(s, out),
		['m', s @ ..] => super::t15::n20160(s, out),
		['n', s @ ..] => super::t15::n20161(s, out),
		['o', s @ ..] => super::t15::n20162(s, out),
		['u', s @ ..] => super::t15::n20163(s, out),
		['z', s @ ..] => super::t15::n20164(s, out),
		_ => false,
	}
}

/// Words ending -rier.
pub(crate) fn n9427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9318);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19605);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29384);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 36343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19606);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 19607);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29385);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29386);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29388);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29389);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 19608);
			match s {
				[] => true,
				['t', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 29390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sier.
pub(crate) fn n9428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9319);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19609);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19610);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19611);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 29391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36346);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36347);
							s.is_empty()
						}
						['x', 'e', 'l', 'f', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 36348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19612);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29392);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 36349);
							s.is_empty()
						}
						['r', 'u', s @ ..] => {
							super::push(out, 36350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 29393);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29394);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19613);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 19614);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 19615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tier.
pub(crate) fn n9429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9320);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19616);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 19617);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29396);
					s.is_empty()
				}
				['d', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 29397);
					s.is_empty()
				}
				['h', 'p', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 29398);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29399);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 29400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 19618);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 19619);
			match s {
				[] => true,
				['l', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 29401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19620);
			match s {
				[] => true,
				['b', 'e', 'w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 29402);
					s.is_empty()
				}
				['d', 'i', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 29403);
					s.is_empty()
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 29404);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29405);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 29406);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 36353);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 36354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 29407);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 36355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29408);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29409);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 29410);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 19621);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 29411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 19622);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 19623);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19624);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29412);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 19625);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 19626);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 36357);
							s.is_empty()
						}
						['t', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 36358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 29414);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40676);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40677);
									s.is_empty()
								}
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
			super::push(out, 19627);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 19628);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29415);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 36359);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40678);
									s.is_empty()
								}
								['u', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 36360);
							match s {
								[] => true,
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 40680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 36361);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29416);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19629);
			match s {
				[] => true,
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 29417);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 29418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'y', 'g', 'e', 'a', '\u{1}', s @ ..] => {
			super::push(out, 19630);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 19631);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29419);
					match s {
						[] => true,
						['u', 'q', 't', 'p', 'u', 'a', 'h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40682);
									s.is_empty()
								}
								['r', 'ä', 't', 'i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 40683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29420);
					match s {
						[] => true,
						['s', 's', 'a', 'w', 'r', s @ ..] => {
							super::push(out, 36363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29421);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 36364);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 36365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 29422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19632);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29423);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29424);
					s.is_empty()
				}
				['g', 'n', 'u', 'j', s @ ..] => {
					super::push(out, 29425);
					s.is_empty()
				}
				['h', 'c', 'u', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 29426);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 29427);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 36368);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 36369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 29428);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36370);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 36371);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40684);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 40685);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'r', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 36372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 19633);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 29429);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 29430);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 36373);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19634);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29431);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'i', 't', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 19635);
			s.is_empty()
		}
		['z', 'l', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 19636);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 19637);
			match s {
				[] => true,
				['f', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 29433);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vier.
pub(crate) fn n9430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9321);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19638);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 19639);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 36376);
							match s {
								[] => true,
								['n', 'e', 't', s @ ..] => {
									super::push(out, 40687);
									match s {
										[] => true,
										['u', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 43132);
											s.is_empty()
										}
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
					super::push(out, 29434);
					match s {
						[] => true,
						['i', 'd', 'n', 'a', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 36377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 19640);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 29435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19641);
			match s {
				[] => true,
				['r', 'e', 'd', s @ ..] => {
					super::push(out, 29436);
					match s {
						[] => true,
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 36378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wier.
pub(crate) fn n9431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9322);
	match s {
		[] => true,
		['a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 19643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xier.
pub(crate) fn n9432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9323);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 19644);
			match s {
				[] => true,
				['l', 'e', 'n', s @ ..] => {
					super::push(out, 29437);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zier.
pub(crate) fn n9433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9324);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19645);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19646);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 29438);
					match s {
						[] => true,
						['f', 'o', 'e', s @ ..] => {
							super::push(out, 36379);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									super::push(out, 40688);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 43133);
											s.is_empty()
										}
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
					super::push(out, 29439);
					match s {
						[] => true,
						['ö', s @ ..] => {
							super::push(out, 36380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 29440);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 36381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 19647);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29441);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36382);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 29442);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nker.
pub(crate) fn n9439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9330);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19660);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29470);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 29471);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 29472);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29473);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36401);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36402);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 40696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 36403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19661);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19662);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 29476);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19663);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 29478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -poker.
pub(crate) fn n20226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19666);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 29480);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 29481);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 29482);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -woker.
pub(crate) fn n20228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19668);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29483);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rker.
pub(crate) fn n9441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9332);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19669);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 29484);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 36404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19670);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 29485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ommer.
pub(crate) fn n20273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19711);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 29538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 29539);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 36439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 29540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ummer.
pub(crate) fn n20274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19712);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 29541);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 29542);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 29543);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29544);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 36441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 29545);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 29546);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 36442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 29547);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 29548);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 36443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 29549);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ämmer.
pub(crate) fn n20275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19713);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 29550);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 29551);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ümmer.
pub(crate) fn n20276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19714);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 29552);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 36445);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 36446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -omer.
pub(crate) fn n9466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9357);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 19715);
			s.is_empty()
		}
		['g', 'i', 'l', 'o', '\u{1}', s @ ..] => {
			super::push(out, 19716);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 19717);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29553);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 19718);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 29555);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19719);
			s.is_empty()
		}
		['r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 19720);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 19721);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 19722);
			match s {
				[] => true,
				['s', 'a', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 29557);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rmer.
pub(crate) fn n9467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9358);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29558);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29559);
					s.is_empty()
				}
				['n', 'h', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 29560);
					s.is_empty()
				}
				['s', 'u', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 29561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19724);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 19725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -umer.
pub(crate) fn n9468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9359);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 19726);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 29562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ymer.
pub(crate) fn n9469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9360);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 19728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ömer.
pub(crate) fn n9471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9362);
	s.is_empty()
}

/// Words ending -ümer.
pub(crate) fn n9472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9363);
	s.is_empty()
}

/// The word gießener.
pub(crate) fn n30903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29598);
	s.is_empty()
}

/// Words ending -oßener.
pub(crate) fn n30904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29599);
	s.is_empty()
}

/// Words ending -fner.
pub(crate) fn n9477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9368);
	s.is_empty()
}

/// Words ending -gner.
pub(crate) fn n9478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9369);
	s.is_empty()
}

/// Words ending -hner.
pub(crate) fn n9479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9370);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iner.
pub(crate) fn n9480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9371);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19744);
			match s {
				[] => true,
				['r', 'k', s @ ..] => {
					super::push(out, 29600);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36462);
							match s {
								[] => true,
								['s', 'ä', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40716);
									s.is_empty()
								}
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
			super::push(out, 19745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29601);
					s.is_empty()
				}
				['a', 'l', s @ ..] => {
					super::push(out, 29602);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19746);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 29603);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 36464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 19747);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 29605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kner.
pub(crate) fn n9481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9372);
	s.is_empty()
}

/// Words ending -lner.
pub(crate) fn n9482(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19748);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 29606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 19749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nner.
pub(crate) fn n9483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9373);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19750);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29607);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 29608);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19751);
			match s {
				[] => true,
				['r', 'b', 'n', 'e', 'l', s @ ..] => {
					super::push(out, 29610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19752);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29611);
					match s {
						[] => true,
						['-', 't', 'h', 'g', 'i', 'l', '-', 'e', 'l', 'd', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 36465);
							s.is_empty()
						}
						['a', 'l', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36466);
							s.is_empty()
						}
						['s', 's', 'u', 'l', 'h', 'c', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 36467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 29612);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 29613);
					match s {
						[] => true,
						['e', 'g', 'n', 'e', 's', s @ ..] => {
							super::push(out, 36468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19753);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29614);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 36469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19754);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 19755);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 19756);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36470);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oner.
pub(crate) fn n9484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9374);
	match s {
		[] => true,
		['h', 't', s @ ..] => {
			super::push(out, 19757);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rner.
pub(crate) fn n9485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9375);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19758);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 19759);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36472);
							match s {
								[] => true,
								['g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 40717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 36473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 19760);
			match s {
				[] => true,
				['k', 's', s @ ..] => {
					super::push(out, 29616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sner.
pub(crate) fn n9486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9376);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 19761);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 29617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tner.
pub(crate) fn n9487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9377);
	s.is_empty()
}

/// Words ending -uner.
pub(crate) fn n9488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9378);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19762);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 29618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19763);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 29619);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 36474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -höner.
pub(crate) fn n9489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9379);
	s.is_empty()
}

/// Words ending -üner.
pub(crate) fn n9490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9380);
	match s {
		[] => true,
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 19764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -asser.
pub(crate) fn n20381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19816);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 29648);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 29649);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 29650);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29651);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 29652);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 29653);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 29654);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 36489);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 36490);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40725);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43138);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 43139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 36491);
					match s {
						[] => true,
						['n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 40726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'd', s @ ..] => {
					super::push(out, 36492);
					s.is_empty()
				}
				['f', 'p', s @ ..] => {
					super::push(out, 36493);
					s.is_empty()
				}
				['g', 'i', 'r', s @ ..] => {
					super::push(out, 36494);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43140);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 43141);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 44249);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 44250);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 43142);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 44251);
											s.is_empty()
										}
										['i', 'n', s @ ..] => {
											super::push(out, 44252);
											match s {
												[] => true,
												['l', 'ö', 'k', '\u{1}', s @ ..] => {
													super::push(out, 44751);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 44253);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 40727);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 43143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 36495);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40728);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 40729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 36496);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40730);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 40731);
							match s {
								[] => true,
								['f', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 43144);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 43145);
									s.is_empty()
								}
								['v', 'a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 43146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 40732);
							s.is_empty()
						}
						['i', 'n', s @ ..] => {
							super::push(out, 40733);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 40734);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 40735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40736);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 43147);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 43148);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 43149);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 43150);
									match s {
										[] => true,
										['c', 'ä', 'l', 'f', 'r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
											super::push(out, 44254);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'r', 'i', s @ ..] => {
									super::push(out, 43151);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 44255);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44256);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 43152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 40737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40738);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43153);
									match s {
										[] => true,
										['s', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 44257);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 43154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 40739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 36497);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40740);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 40741);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40742);
							match s {
								[] => true,
								['e', s @ ..] => super::t15::n45986(s, out),
								['o', 'i', 't', 'a', 'n', 'g', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 43155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 40743);
							s.is_empty()
						}
						['r', s @ ..] => super::t15::n43310(s, out),
						['s', s @ ..] => super::t15::n43311(s, out),
						['t', s @ ..] => super::t15::n43312(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t15::n38533(s, out),
				['u', s @ ..] => super::t15::n38534(s, out),
				['z', s @ ..] => super::t15::n38535(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -esser.
pub(crate) fn n20382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19817);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 29655);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 29656);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 29657);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 29658);
			match s {
				[] => true,
				['b', 'l', s @ ..] => {
					super::push(out, 36500);
					s.is_empty()
				}
				['d', 'a', s @ ..] => {
					super::push(out, 36501);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 36502);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 40753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 36503);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 36504);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 36505);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43157);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43158);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 43159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 40754);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43160);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 43161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 40755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 36506);
					match s {
						[] => true,
						['e', 'k', s @ ..] => {
							super::push(out, 40756);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 40757);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 40758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 43162);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 43163);
									s.is_empty()
								}
								['h', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 44260);
											s.is_empty()
										}
										['ö', s @ ..] => {
											super::push(out, 44261);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 44262);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 44263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 40759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 36507);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 36508);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43164);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 43165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 40760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 36509);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 40761);
							s.is_empty()
						}
						['g', 'n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43166);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 43167);
									match s {
										[] => true,
										['t', 'u', s @ ..] => {
											super::push(out, 44264);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 40762);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 40763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 40764);
							s.is_empty()
						}
						['r', 'h', s @ ..] => {
							super::push(out, 40765);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 40766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 29659);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 29660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weisser.
pub(crate) fn n30969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29661);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osser.
pub(crate) fn n20384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19819);
	match s {
		[] => true,
		['l', 'o', s @ ..] => {
			super::push(out, 29662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usser.
pub(crate) fn n20385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19820);
	s.is_empty()
}

/// Words ending -ässer.
pub(crate) fn n20386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19821);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 29663);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 29664);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 29665);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 29666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -össer.
pub(crate) fn n20387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19822);
	s.is_empty()
}

/// Words ending -üsser.
pub(crate) fn n20388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19823);
	s.is_empty()
}

/// Words ending -user.
pub(crate) fn n9519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9409);
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			super::push(out, 19824);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19825);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wser.
pub(crate) fn n9520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9410);
	s.is_empty()
}

/// Words ending -yser.
pub(crate) fn n9521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9411);
	s.is_empty()
}

/// Words ending -äser.
pub(crate) fn n9522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9412);
	match s {
		[] => true,
		['l', 'b', 'n', 'e', 'l', s @ ..] => {
			super::push(out, 19826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öser.
pub(crate) fn n9523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9413);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 19827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 19828);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 19829);
			match s {
				[] => true,
				['i', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 29670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -meter.
pub(crate) fn n20412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19847);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 29686);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 36514);
					s.is_empty()
				}
				['i', 'k', s @ ..] => {
					super::push(out, 36515);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 36516);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 36517);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 36518);
					match s {
						[] => true,
						['a', 'p', 'e', 'g', s @ ..] => {
							super::push(out, 40767);
							match s {
								[] => true,
								['a', 'l', 'n', s @ ..] => {
									super::push(out, 43168);
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
						['e', s @ ..] => {
							super::push(out, 40768);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 40769);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40771);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 40772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 29687);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 29688);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 36519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 29689);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 29690);
			match s {
				[] => true,
				['d', 'u', s @ ..] => {
					super::push(out, 36520);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 36521);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 36522);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 40773);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 40774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 36523);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 36524);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 40775);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 36525);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 29691);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 29692);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 29693);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 29694);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36527);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 36528);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					super::push(out, 36529);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 36530);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 36531);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 40778);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 40779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 36532);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 40780);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43169);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 43170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 40781);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 40782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 36533);
					match s {
						[] => true,
						['r', 'e', 'h', 't', 'n', 'e', s @ ..] => {
							super::push(out, 40783);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 43171);
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
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 43172);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 40784);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 43174);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 43175);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 36534);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40785);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 44265);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 44266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 40786);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 40787);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 40788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					super::push(out, 36535);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', 'e', s @ ..] => {
							super::push(out, 40789);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 43178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 40790);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 40791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 29695);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36536);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 36537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 29696);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 36538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 29697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fter.
pub(crate) fn n9529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9419);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19853);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29707);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29708);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29709);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 36546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19854);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 19855);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29710);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 29711);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36547);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 29712);
					match s {
						[] => true,
						['w', 'z', 'n', 'e', 'l', 'l', 'e', 'b', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 36549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 19856);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 29713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 19857);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 29714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19858);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 19859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gter.
pub(crate) fn n9530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9420);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19860);
			s.is_empty()
		}
		['i', 't', 'h', 'c', 'e', 'r', 'e', 'b', 's', s @ ..] => {
			super::push(out, 19861);
			match s {
				[] => true,
				['g', 'n', 'u', 't', 'e', s @ ..] => {
					super::push(out, 29715);
					match s {
						[] => true,
						['r', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 36550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hter.
pub(crate) fn n9531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9421);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 29716);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 36551);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36552);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36553);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 36554);
							match s {
								[] => true,
								['f', 'e', s @ ..] => {
									super::push(out, 40794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29717);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36555);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36556);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40795);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 29718);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36557);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29719);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 36558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 29720);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36559);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36560);
							match s {
								[] => true,
								['f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 40798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 36561);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 29721);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36562);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40800);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 40801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 36563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 29722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19862);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29723);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iter.
pub(crate) fn n9532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9422);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19863);
			match s {
				[] => true,
				['s', 'f', s @ ..] => {
					super::push(out, 29724);
					match s {
						[] => true,
						['l', 'ö', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 36564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19864);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29725);
					match s {
						[] => true,
						['r', 'a', 'r', s @ ..] => {
							super::push(out, 36565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 29726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36566);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 36567);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 36568);
							match s {
								[] => true,
								['n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40803);
									match s {
										[] => true,
										['h', 'c', 'r', 'a', 'r', 'e', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 43179);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 40804);
									match s {
										[] => true,
										['e', 'i', 'r', 'r', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 43180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 36569);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 36570);
							match s {
								[] => true,
								['e', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 40805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 36571);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 43181);
											s.is_empty()
										}
										['o', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 43182);
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
								['a', 'a', s @ ..] => {
									super::push(out, 40806);
									s.is_empty()
								}
								['h', 'a', s @ ..] => {
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 43183);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													super::push(out, 44267);
													s.is_empty()
												}
												_ => false,
											}
										}
										['w', s @ ..] => {
											super::push(out, 43184);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'a', s @ ..] => {
									super::push(out, 40807);
									match s {
										[] => true,
										['t', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 43185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'u', 'i', s @ ..] => {
							super::push(out, 36572);
							match s {
								[] => true,
								['n', 'i', 'm', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 40808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 36573);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40809);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 43186);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 40810);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 43187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 40811);
									match s {
										[] => true,
										['a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 43188);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'd', s @ ..] => {
							super::push(out, 36574);
							match s {
								[] => true,
								['n', 'a', 'm', 'm', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 36575);
							match s {
								[] => true,
								['p', 'a', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 43189);
											match s {
												[] => true,
												['u', 'ä', 'r', '\u{1}', s @ ..] => {
													super::push(out, 44268);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 43190);
											match s {
												[] => true,
												['h', 'ü', 'h', '\u{1}', s @ ..] => {
													super::push(out, 44269);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 43191);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 40814);
									match s {
										[] => true,
										['e', 'w', 'r', 'e', 'u', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 43192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 36576);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 40815);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43193);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 43194);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 40816);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 40817);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43195);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 't', s @ ..] => {
							super::push(out, 36577);
							s.is_empty()
						}
						['u', 'l', s @ ..] => super::t15::n38626(s, out),
						['z', 'l', s @ ..] => super::t15::n38627(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t15::n31036(s, out),
				['w', s @ ..] => super::t15::n31037(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t15::n20431(s, out),
		['r', s @ ..] => {
			super::push(out, 19866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kter.
pub(crate) fn n9533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9423);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19867);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29731);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 36585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 29732);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29733);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29734);
					match s {
						[] => true,
						['a', 'h', 'c', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 40821);
									s.is_empty()
								}
								['l', 'l', 'e', s @ ..] => {
									super::push(out, 40822);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 36586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 19868);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 19869);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 29736);
					match s {
						[] => true,
						['h', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 36587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lter.
pub(crate) fn n9534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9424);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19871);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29737);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36588);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40824);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 40825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29738);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 36589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 29739);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 29740);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 40826);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 43196);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', s @ ..] => {
									super::push(out, 40827);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 43197);
											s.is_empty()
										}
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
					super::push(out, 29741);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36590);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 36591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29742);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29743);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36592);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36593);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 40828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 29744);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29745);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36594);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36595);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 36596);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36597);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 36598);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29746);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36600);
							match s {
								[] => true,
								['e', 'z', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 43198);
											s.is_empty()
										}
										['s', 'i', s @ ..] => {
											super::push(out, 43199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 36601);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 40829);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 40830);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 40831);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 43200);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 43201);
											s.is_empty()
										}
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
					super::push(out, 29747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29748);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 29749);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29750);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29751);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36602);
							s.is_empty()
						}
						['s', 'e', 'f', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 29753);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 36604);
							match s {
								[] => true,
								['u', 'w', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 40832);
									s.is_empty()
								}
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
			super::push(out, 19873);
			match s {
				[] => true,
				['ä', 'w', 'r', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 29754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19874);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29755);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29756);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36605);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 43202);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 43203);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 36606);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36607);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 36608);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40833);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 40834);
									match s {
										[] => true,
										['l', 'l', 'e', 'w', s @ ..] => {
											super::push(out, 43204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 36609);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 36610);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36611);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 40836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 36612);
							s.is_empty()
						}
						['z', 't', 'u', s @ ..] => {
							super::push(out, 36613);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 36614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 19875);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19876);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 29758);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 29759);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 29760);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29761);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n20443(s, out),
		['ä', s @ ..] => super::t15::n20444(s, out),
		_ => false,
	}
}

/// Words ending -mter.
pub(crate) fn n9535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9425);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19879);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19880);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 29765);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nter.
pub(crate) fn n9536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9426);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19881);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19882);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 29766);
					match s {
						[] => true,
						['-', 'g', s @ ..] => {
							super::push(out, 36618);
							match s {
								[] => true,
								['n', 'i', 'p', 'p', 'o', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 40838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 36619);
							match s {
								[] => true,
								['n', 'i', 'p', 'p', 'o', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 40839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', s @ ..] => {
							super::push(out, 36620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 29767);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 29768);
					match s {
						[] => true,
						['d', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 36621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29769);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29770);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 19883);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19884);
			match s {
				[] => true,
				['w', 'n', s @ ..] => {
					super::push(out, 29773);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 36623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 19885);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 19886);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 19887);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 19888);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 29774);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'n', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 29775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oter.
pub(crate) fn n9537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9427);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 19889);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36625);
							match s {
								[] => true,
								['p', 'm', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 't', s @ ..] => {
							super::push(out, 36626);
							match s {
								[] => true,
								['i', 'e', 'h', 'r', 'e', 'h', 'c', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 40841);
									s.is_empty()
								}
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
			super::push(out, 19890);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 29776);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 36627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19891);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 19892);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 19893);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 19894);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 29777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 19895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 19896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pter.
pub(crate) fn n9538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9428);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 19897);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 19898);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 29779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19899);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 29780);
					s.is_empty()
				}
				['k', 'o', s @ ..] => {
					super::push(out, 29781);
					match s {
						[] => true,
						['r', 'y', s @ ..] => {
							super::push(out, 36628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 19900);
			match s {
				[] => true,
				['o', 'l', 'k', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 29782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 19901);
			match s {
				[] => true,
				['r', 'r', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 29783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rter.
pub(crate) fn n9539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9429);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19902);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29784);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 36629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29785);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19903);
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36630);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 36631);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 29787);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 29788);
					match s {
						[] => true,
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 36632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 29789);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 36633);
							match s {
								[] => true,
								['r', 'e', 'u', 'a', 'd', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40843);
									s.is_empty()
								}
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
			super::push(out, 19904);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36634);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 36635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19905);
			match s {
				[] => true,
				['p', 'e', s @ ..] => {
					super::push(out, 29790);
					match s {
						[] => true,
						['r', 'n', s @ ..] => {
							super::push(out, 36636);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'p', 's', 'e', 'g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 19906);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 19907);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 19908);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 19909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ster.
pub(crate) fn n9540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9430);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29791);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 29792);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 29793);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 29794);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 29795);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 29796);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 29797);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 29798);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29799);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 36637);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 36638);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 36639);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36640);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36641);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29800);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29801);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29802);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36643);
							match s {
								[] => true,
								['m', 'm', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 40845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 36644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29803);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29804);
					match s {
						[] => true,
						['a', 'k', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40846);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 40847);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 40848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 29805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19911);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29806);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 29807);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 36646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 29808);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 29809);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 29810);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 36648);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40849);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 43205);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 43206);
											match s {
												[] => true,
												['w', '\u{1}', s @ ..] => {
													super::push(out, 44270);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 40850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 29811);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 36649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 29812);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 36650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29813);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 29814);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29815);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29816);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29817);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36652);
							s.is_empty()
						}
						['r', 'e', 'w', 'r', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 36653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 29818);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 29819);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36654);
							match s {
								[] => true,
								['i', 's', 'n', s @ ..] => {
									super::push(out, 40851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 36655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 29820);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36656);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 40852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 36657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 29821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 19912);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 29822);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36658);
							s.is_empty()
						}
						['ü', 'j', '\u{1}', s @ ..] => {
							super::push(out, 36659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 19913);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 29823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19914);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29824);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 29825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29826);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 36661);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 40853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 36662);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36663);
							match s {
								[] => true,
								['k', 'n', s @ ..] => {
									super::push(out, 40854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 36664);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36665);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 29827);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36666);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 40856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 36667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 29828);
					s.is_empty()
				}
				['w', s @ ..] => super::t15::n31142(s, out),
				['z', s @ ..] => {
					super::push(out, 29830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t15::n20481(s, out),
		['m', s @ ..] => {
			super::push(out, 19916);
			s.is_empty()
		}
		['n', s @ ..] => super::t15::n20483(s, out),
		['o', s @ ..] => super::t15::n20484(s, out),
		['p', s @ ..] => super::t15::n20485(s, out),
		['r', s @ ..] => super::t15::n20486(s, out),
		['s', s @ ..] => super::t15::n20487(s, out),
		['u', s @ ..] => super::t15::n20488(s, out),
		['ö', s @ ..] => super::t15::n20489(s, out),
		['ü', s @ ..] => super::t15::n20490(s, out),
		_ => false,
	}
}

/// Words ending -tter.
pub(crate) fn n9541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9431);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19925);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 29860);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 36682);
							s.is_empty()
						}
						['z', 't', s @ ..] => {
							super::push(out, 36683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 29861);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29862);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29863);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 36684);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 40860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 29864);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 36685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 29865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19926);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 29866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 29867);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29868);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36687);
							s.is_empty()
						}
						['i', 'e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 36688);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36689);
							match s {
								[] => true,
								['w', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 40861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 29869);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 36690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 29870);
					match s {
						[] => true,
						['ü', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 36691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 29871);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 29872);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29873);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 29874);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 29875);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 36692);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'e', s @ ..] => {
							super::push(out, 36693);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 36694);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 36695);
							s.is_empty()
						}
						['l', 'e', 'b', s @ ..] => {
							super::push(out, 36696);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40864);
									s.is_empty()
								}
								['r', 'u', s @ ..] => {
									super::push(out, 40865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 40866);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 40867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 40868);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'i', s @ ..] => {
							super::push(out, 36697);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 36698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19927);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29876);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36699);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 36700);
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									super::push(out, 40871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 29877);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29878);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 36701);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 36702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 29879);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29880);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36703);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29881);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 29882);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36705);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 36706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19928);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 29883);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 36707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29884);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29885);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 29886);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29887);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 36708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 29888);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 29889);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29890);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36709);
							match s {
								[] => true,
								['g', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 40872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29891);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29892);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 36710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 29893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n20495(s, out),
		['ö', s @ ..] => {
			super::push(out, 19930);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 19931);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 29901);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 29902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uter.
pub(crate) fn n9542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9432);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19932);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 29903);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 36724);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 19933);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 29904);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 29905);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 29906);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 29907);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 29908);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 29909);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 36726);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40884);
									s.is_empty()
								}
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
			super::push(out, 19934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 19935);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 29911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zter.
pub(crate) fn n9544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9434);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 19936);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äter.
pub(crate) fn n9545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9435);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 19937);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 19938);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 29912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üter.
pub(crate) fn n9547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9437);
	match s {
		[] => true,
		['h', 'n', s @ ..] => {
			super::push(out, 19939);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 29913);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mbauer.
pub(crate) fn n31236(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 36735);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 36736);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'm', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 43219);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 43220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nbauer.
pub(crate) fn n31237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29920);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 40891);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 40892);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 40893);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', s @ ..] => {
					super::push(out, 40894);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 40895);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 40896);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 40897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 36737);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -obauer.
pub(crate) fn n31238(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'k', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 36738);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 36739);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 36740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbauer.
pub(crate) fn n31239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29921);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40898);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 40899);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 40900);
					s.is_empty()
				}
				['i', 'v', 'a', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 40901);
					s.is_empty()
				}
				['p', 'r', 'ö', 'k', '\u{1}', s @ ..] => {
					super::push(out, 40902);
					s.is_empty()
				}
				['t', 's', 'n', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 40903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sbauer.
pub(crate) fn n31240(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'r', 'e', 'w', 'r', 'e', 'n', 'e', 'b', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 36741);
			s.is_empty()
		}
		['f', 'f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 36742);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 40904);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 40905);
					s.is_empty()
				}
				['u', 'z', s @ ..] => {
					match s {
						[] => true,
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 43222);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 43223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 36743);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 36744);
			s.is_empty()
		}
		['s', 'o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 36745);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['f', 'a', 'h', 'c', 's', 'n', 'e', 's', 's', 'o', 'n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 40906);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 40907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tbauer.
pub(crate) fn n31241(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 36746);
			s.is_empty()
		}
		['s', 'ü', 'r', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 36747);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word neubauer.
pub(crate) fn n31242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29922);
	s.is_empty()
}

/// Words ending -ßbauer.
pub(crate) fn n31243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29923);
	s.is_empty()
}

/// Words ending -dauer.
pub(crate) fn n20507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19941);
	match s {
		[] => true,
		['e', 's', s @ ..] => {
			super::push(out, 29924);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29925);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 36748);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 40908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 29926);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 36749);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 36750);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 36751);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40909);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 40910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 29927);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40911);
							s.is_empty()
						}
						['n', 'u', 't', s @ ..] => {
							super::push(out, 40912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 36752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 36753);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							super::push(out, 40913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 36754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gauer.
pub(crate) fn n20508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19942);
	s.is_empty()
}

/// Words ending -hauer.
pub(crate) fn n20509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19943);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 29928);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 36755);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kauer.
pub(crate) fn n20510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19944);
	s.is_empty()
}

/// Words ending -lauer.
pub(crate) fn n20511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19945);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29929);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 29930);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 29931);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mauer.
pub(crate) fn n20512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19946);
	match s {
		[] => true,
		['d', 'i', s @ ..] => {
			super::push(out, 29932);
			match s {
				[] => true,
				['e', 'h', 't', s @ ..] => {
					super::push(out, 36757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 29933);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 29934);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 36758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nauer.
pub(crate) fn n20513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19947);
	s.is_empty()
}

/// Words ending -rauer.
pub(crate) fn n20514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19948);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 29935);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 29936);
			match s {
				[] => true,
				['d', 'l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 36759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 29937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sauer.
pub(crate) fn n20515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19949);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 29938);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 29939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tauer.
pub(crate) fn n20516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19950);
	s.is_empty()
}

/// Words ending -euer.
pub(crate) fn n9549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9439);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 19951);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					super::push(out, 29940);
					s.is_empty()
				}
				['n', 'e', 'h', s @ ..] => {
					super::push(out, 29941);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 36760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', s @ ..] => {
					super::push(out, 29942);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 29943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 19952);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29944);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 29945);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 29946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 19953);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 29947);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							super::push(out, 36762);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 36763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 19954);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 29948);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 29949);
					match s {
						[] => true,
						['n', 'e', 'n', s @ ..] => {
							super::push(out, 36765);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 36766);
							s.is_empty()
						}
						['t', 'h', s @ ..] => {
							super::push(out, 36767);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 40915);
									s.is_empty()
								}
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

/// Words ending -luer.
pub(crate) fn n9550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9440);
	s.is_empty()
}

/// Words ending -quer.
pub(crate) fn n9551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9441);
	s.is_empty()
}

/// Words ending -ruer.
pub(crate) fn n9552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9442);
	s.is_empty()
}

/// Words ending -tuer.
pub(crate) fn n9553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9443);
	s.is_empty()
}

/// Words ending -äuer.
pub(crate) fn n9554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9444);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 19955);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 19956);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 19957);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rkehr.
pub(crate) fn n20584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20016);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 36787);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 36788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40922);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 40923);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 43224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 40924);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43225);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43227);
									s.is_empty()
								}
								['g', 'e', 'i', 'b', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 43228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 40925);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 40926);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 44273);
											match s {
												[] => true,
												['f', 't', s @ ..] => {
													super::push(out, 44753);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => super::t15::n47251(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => super::t15::n43509(s, out),
						['i', s @ ..] => super::t15::n43510(s, out),
						['k', s @ ..] => super::t15::n43511(s, out),
						['l', s @ ..] => super::t15::n43512(s, out),
						['n', s @ ..] => super::t15::n43513(s, out),
						['o', s @ ..] => super::t15::n43514(s, out),
						['r', s @ ..] => super::t15::n43515(s, out),
						['s', s @ ..] => super::t15::n43516(s, out),
						['t', s @ ..] => super::t15::n43517(s, out),
						['z', s @ ..] => super::t15::n43518(s, out),
						['ß', s @ ..] => super::t15::n43519(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sektor.
pub(crate) fn n31373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30040);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 36822);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 40947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 36823);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 36824);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 36825);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 40948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'u', 't', 's', 'i', 'e', 'l', 't', 's', 'n', 'e', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 40949);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 40950);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 40951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 36826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nduktor.
pub(crate) fn n20686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20118);
	match s {
		[] => true,
		['i', 'm', s @ ..] => {
			super::push(out, 30043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltor.
pub(crate) fn n9678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9566);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20119);
			match s {
				[] => true,
				['i', 'p', 's', 'r', 'e', 'd', 'n', 'ä', 'l', '\u{1}', s @ ..] => {
					super::push(out, 30044);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 30045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 20120);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 20121);
			s.is_empty()
		}
		['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 20122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mtor.
pub(crate) fn n9679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9567);
	match s {
		[] => true,
		['o', 't', 'n', 'a', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 20123);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 30046);
					s.is_empty()
				}
				['i', 'n', 'i', 'm', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 30047);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntor.
pub(crate) fn n9680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9568);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 20124);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 20125);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 30048);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 36828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 30049);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 30050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 's', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 36830);
							s.is_empty()
						}
						['u', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 36831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 36832);
							s.is_empty()
						}
						['u', 'e', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 36833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'r', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 30051);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 30052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 20126);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 30053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otor.
pub(crate) fn n9681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9569);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 30054);
					s.is_empty()
				}
				['r', 'e', 'x', s @ ..] => {
					super::push(out, 30055);
					match s {
						[] => true,
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 36834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptor.
pub(crate) fn n9682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9570);
	match s {
		[] => true,
		['e', 'z', 'e', 'r', 'l', s @ ..] => {
			super::push(out, 20127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtor.
pub(crate) fn n9683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9571);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20128);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stor.
pub(crate) fn n9684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9572);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 20129);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 20130);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 20131);
			match s {
				[] => true,
				['o', 'h', 'd', 'e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 30056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 20132);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 36835);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', 'h', 'ü', 'f', '\u{1}', s @ ..] => {
									super::push(out, 40952);
									s.is_empty()
								}
								['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 40953);
									s.is_empty()
								}
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
			super::push(out, 20133);
			match s {
				[] => true,
				['c', 'i', 'e', 'l', 'g', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 30057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 20134);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 30058);
					match s {
						[] => true,
						['g', 'n', 'ä', 'f', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 20135);
			match s {
				[] => true,
				['l', 'a', 'f', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 30059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 20136);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 30060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 20137);
			match s {
				[] => true,
				['u', 'l', 'h', 'c', 's', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 30061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 20138);
			s.is_empty()
		}
		['u', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 20139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttor.
pub(crate) fn n9685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9573);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 20140);
			s.is_empty()
		}
		['h', 'c', 'u', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 20141);
			s.is_empty()
		}
		['p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 20142);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 20143);
			match s {
				[] => true,
				['b', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 30062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 20144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utor.
pub(crate) fn n9686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9574);
	s.is_empty()
}

/// Words ending -ytor.
pub(crate) fn n9687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9575);
	match s {
		[] => true,
		['e', 'k', 'c', 'o', 'h', 's', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 20145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ztor.
pub(crate) fn n9688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9576);
	match s {
		[] => true,
		['l', 'o', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30063);
					s.is_empty()
				}
				['n', 'e', 'h', 'c', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 30064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', 'h', 'c', 's', 'd', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 20146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßtor.
pub(crate) fn n9689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9577);
	match s {
		[] => true,
		['o', 't', 's', 'i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 20147);
			s.is_empty()
		}
		['u', 'l', 'h', 'c', 's', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 20148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erliteratur.
pub(crate) fn n47290(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 44765);
			s.is_empty()
		}
		['u', 'e', s @ ..] => {
			super::push(out, 44766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orliteratur.
pub(crate) fn n47291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44310);
	s.is_empty()
}

/// Words ending -skultur.
pub(crate) fn n31455(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 36857);
			s.is_empty()
		}
		['g', 'e', s @ ..] => {
			super::push(out, 36858);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 36859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ktkultur.
pub(crate) fn n31456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30116);
	s.is_empty()
}

/// Words ending -mtur.
pub(crate) fn n9757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9644);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 20221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntur.
pub(crate) fn n9758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9645);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 30117);
					match s {
						[] => true,
						['a', 'z', s @ ..] => {
							super::push(out, 36860);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 40960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 30118);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 36861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 20222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptur.
pub(crate) fn n9759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9646);
	s.is_empty()
}

/// Words ending -stur.
pub(crate) fn n9760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9647);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20223);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utur.
pub(crate) fn n9761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9648);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 20224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -n-gefängnis.
pub(crate) fn n47292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44311);
	s.is_empty()
}

/// Words ending -hmensverzeichnis.
pub(crate) fn n47293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44312);
	s.is_empty()
}

/// Words ending -rnis.
pub(crate) fn n9998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9878);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 20452);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 30211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 20453);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 30212);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 30213);
					s.is_empty()
				}
				['m', 'm', 'ü', 'k', '\u{1}', s @ ..] => {
					super::push(out, 30214);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 30215);
					match s {
						[] => true,
						['s', 'n', 'i', 'f', 's', s @ ..] => {
							super::push(out, 36904);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 40978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 30216);
					match s {
						[] => true,
						['h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 36905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'a', 'f', 'r', 'e', 'd', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 20454);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 20455);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 30217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 20456);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 30218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 20457);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tnis.
pub(crate) fn n9999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9879);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 20458);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 20459);
			match s {
				[] => true,
				['c', 'ä', 'd', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36906);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36907);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40979);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 40980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 36908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 20460);
			match s {
				[] => true,
				['ä', 'h', 'r', 'e', 'v', 's', 't', s @ ..] => {
					super::push(out, 30219);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 36909);
							match s {
								[] => true,
								['a', 'h', 'c', 's', 't', 'd', 'n', 'a', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 40981);
									s.is_empty()
								}
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
			super::push(out, 20461);
			match s {
				[] => true,
				['n', 'e', 'k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36910);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 40982);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 40983);
									match s {
										[] => true,
										['m', 'e', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 43267);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 36911);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 36912);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 36913);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 43268);
											match s {
												[] => true,
												['a', 'd', 'l', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 44313);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 43269);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 44314);
													s.is_empty()
												}
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

/// Words ending -ndungsbeschluss.
pub(crate) fn n47296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44315);
	s.is_empty()
}

/// The word börsenschluss.
pub(crate) fn n47297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44316);
	s.is_empty()
}

/// Words ending -anuss.
pub(crate) fn n21309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20717);
	s.is_empty()
}

/// Words ending -enuss.
pub(crate) fn n21310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20718);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 30353);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 36967);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 36968);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 36969);
					s.is_empty()
				}
				['n', 'e', 'g', s @ ..] => {
					super::push(out, 36970);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 36971);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 41000);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 41001);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 41002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hnuss.
pub(crate) fn n21311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20719);
	s.is_empty()
}

/// Words ending -lnuss.
pub(crate) fn n21312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20720);
	s.is_empty()
}

/// Words ending -nnuss.
pub(crate) fn n21313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20721);
	s.is_empty()
}

/// Words ending -rnuss.
pub(crate) fn n21314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20722);
	s.is_empty()
}

/// Words ending -snuss.
pub(crate) fn n21315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20723);
	s.is_empty()
}

/// Words ending -russ.
pub(crate) fn n10243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10122);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 20724);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 36972);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 36973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 20725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tuss.
pub(crate) fn n10244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10123);
	s.is_empty()
}

/// Words ending -smus.
pub(crate) fn n10339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10218);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 20847);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 30427);
					s.is_empty()
				}
				['i', 'l', s @ ..] => {
					super::push(out, 30428);
					s.is_empty()
				}
				['l', 'k', 'o', 'i', s @ ..] => {
					super::push(out, 30429);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 30430);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 30431);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t15::n21447(s, out),
		['s', s @ ..] => {
			super::push(out, 20849);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 30450);
					match s {
						[] => true,
						['n', 'd', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 37021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 20850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onat.
pub(crate) fn n10477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10354);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 21042);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41140);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 41141);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 43349);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44346);
													s.is_empty()
												}
												['m', 'u', 'i', s @ ..] => {
													match s {
														[] => true,
														['c', 'l', 'a', 'c', '\u{1}', s @ ..] => super::t15::n47817(s, out),
														['n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => super::t15::n47818(s, out),
														['r', 't', 'a', 'n', '\u{1}', s @ ..] => super::t15::n47819(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 43350);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 41142);
									match s {
										[] => true,
										['u', 'i', 'r', s @ ..] => {
											super::push(out, 43351);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 44347);
													match s {
														[] => true,
														['a', 'n', '\u{1}', s @ ..] => super::t15::n47820(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'e', 'g', 'o', 'r', 'd', 'y', 'h', 'm', 'u', 'i', s @ ..] => {
									match s {
										[] => true,
										['n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => {
											super::push(out, 43352);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 43353);
											match s {
												[] => true,
												['t', 'a', 'n', '\u{1}', s @ ..] => {
													super::push(out, 44348);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 41143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41144);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 41145);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 41146);
									match s {
										[] => true,
										['e', 'g', 'o', 'r', 'd', 'y', 'h', 'm', 'u', 'i', 'r', 't', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 43354);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 41147);
									s.is_empty()
								}
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
			super::push(out, 21043);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 21044);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 30570);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 30571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 21045);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 30572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 21046);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 30573);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 37115);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 37116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 30574);
					match s {
						[] => true,
						['n', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41148);
									s.is_empty()
								}
								['n', 'e', 'h', 'c', 'd', 'ä', 'm', '\u{1}', s @ ..] => {
									super::push(out, 41149);
									s.is_empty()
								}
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
			super::push(out, 21047);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30575);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 30576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 21048);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 30577);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 41150);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 41151);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'g', s @ ..] => {
					super::push(out, 30578);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 30579);
					match s {
						[] => true,
						['e', 'h', 'c', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 37117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37118);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 30580);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37120);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'e', 'l', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41152);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['s', 'o', '\u{1}', s @ ..] => {
											super::push(out, 43355);
											s.is_empty()
										}
										['ä', 'v', '\u{1}', s @ ..] => {
											super::push(out, 43356);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 41153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 37121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', s @ ..] => {
					super::push(out, 30581);
					s.is_empty()
				}
				['t', 's', 'o', s @ ..] => {
					super::push(out, 30582);
					s.is_empty()
				}
				['u', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 30583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 21049);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 21050);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 30584);
					match s {
						[] => true,
						['i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 37122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'e', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 30585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word kombipräparat.
pub(crate) fn n47333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44349);
	s.is_empty()
}

/// The word morphinpräparat.
pub(crate) fn n47334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44350);
	s.is_empty()
}

/// The word kombinationspräparat.
pub(crate) fn n47335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44351);
	s.is_empty()
}

/// Words ending -frat.
pub(crate) fn n10490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10367);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 21078);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 21079);
			match s {
				[] => true,
				['h', 's', s @ ..] => {
					super::push(out, 30605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -grat.
pub(crate) fn n10491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10368);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 21080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrat.
pub(crate) fn n10492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10369);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 21081);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37137);
							s.is_empty()
						}
						['d', 'l', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 37138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 21082);
			match s {
				[] => true,
				['s', 'n', 'r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 30606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 21083);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 37139);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 37140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41163);
									s.is_empty()
								}
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

/// Words ending -irat.
pub(crate) fn n10493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10370);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21084);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 30607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37141);
							s.is_empty()
						}
						['n', 'r', 'e', 't', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 37142);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 37143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 30608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 21085);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 30609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37144);
							s.is_empty()
						}
						['f', 'l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 37145);
							s.is_empty()
						}
						['n', 'e', 't', 's', 'ü', 'w', '\u{1}', s @ ..] => {
							super::push(out, 37146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 21086);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 21087);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 30610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -krat.
pub(crate) fn n10494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10371);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 21088);
			match s {
				[] => true,
				['h', 't', 'e', '\u{1}', s @ ..] => {
					super::push(out, 30611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21089);
			match s {
				[] => true,
				['u', 'f', 'd', 'n', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 30612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 21090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lrat.
pub(crate) fn n10495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10372);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21091);
			match s {
				[] => true,
				['i', 'z', 'o', s @ ..] => {
					super::push(out, 30613);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 30614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21092);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 21093);
			match s {
				[] => true,
				['h', 'c', 's', 't', s @ ..] => {
					super::push(out, 30615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nrat.
pub(crate) fn n10497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10374);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21094);
			match s {
				[] => true,
				['i', 'd', s @ ..] => {
					super::push(out, 30616);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 37147);
							s.is_empty()
						}
						['u', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41164);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 41165);
									s.is_empty()
								}
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
			super::push(out, 21095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orat.
pub(crate) fn n10498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10375);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 21096);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30617);
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 37148);
							match s {
								[] => true,
								['t', 'e', 't', 'm', 'u', 'i', 'h', s @ ..] => {
									super::push(out, 41166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', s @ ..] => {
							super::push(out, 37149);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 41167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 30618);
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 30619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 21097);
			match s {
				[] => true,
				['n', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 30620);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 30621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21098);
			match s {
				[] => true,
				['h', 'c', 'r', 'e', 'p', 'm', s @ ..] => {
					super::push(out, 30622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21099);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 21100);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 41168);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43363);
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 43364);
											s.is_empty()
										}
										['r', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 43365);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 41169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 37150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 30623);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 30624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrat.
pub(crate) fn n10499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10376);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21101);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 30625);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 30626);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 37151);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 41170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 37152);
							match s {
								[] => true,
								['n', 'a', 'y', 'c', 'a', 'x', 'e', 'h', 'm', s @ ..] => {
									super::push(out, 41171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 30627);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 37153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 30628);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 30629);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 30630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37154);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 37155);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 37156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 21102);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', 'e', s @ ..] => {
							super::push(out, 37157);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 't', s @ ..] => {
									super::push(out, 41172);
									s.is_empty()
								}
								['i', 'e', 'b', s @ ..] => {
									super::push(out, 41173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 37158);
							match s {
								[] => true,
								['e', 's', 's', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 41174);
									s.is_empty()
								}
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
			super::push(out, 21103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -srat.
pub(crate) fn n10500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10377);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 21104);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 21105);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 21106);
			match s {
				[] => true,
				['n', 'u', 't', s @ ..] => {
					super::push(out, 30631);
					match s {
						[] => true,
						['l', 'a', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 37159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 21107);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 30632);
					s.is_empty()
				}
				['h', 'c', 'e', s @ ..] => {
					super::push(out, 30633);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 30634);
					match s {
						[] => true,
						['e', 'h', 'r', 'e', 'h', 'c', 'i', 's', '-', s @ ..] => {
							super::push(out, 37160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 21108);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trat.
pub(crate) fn n10501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10378);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 21109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 21110);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 30637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37161);
							s.is_empty()
						}
						['m', 'u', 'i', 'n', s @ ..] => {
							super::push(out, 37162);
							match s {
								[] => true,
								['o', 'm', 'm', 'a', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 41175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 37163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 30638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37164);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 37165);
							match s {
								[] => true,
								['u', 'i', 'r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 41176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 37166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 30639);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21111);
			match s {
				[] => true,
				['i', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37168);
							s.is_empty()
						}
						['r', 'e', 'f', 'u', '\u{1}', s @ ..] => {
							super::push(out, 37169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21112);
			match s {
				[] => true,
				['e', 'z', 'n', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37170);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 37171);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 37172);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['f', 'a', 's', s @ ..] => {
									match s {
										[] => true,
										['l', 'e', 'f', 'p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 43366);
											s.is_empty()
										}
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'a', 'r', 'o', '\u{1}', s @ ..] => {
													super::push(out, 44352);
													s.is_empty()
												}
												['n', 'o', 'r', 't', 'i', 'z', '\u{1}', s @ ..] => {
													super::push(out, 44353);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'o', 'm', 'n', 'e', 'b', 'u', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 41177);
									s.is_empty()
								}
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
			super::push(out, 21113);
			match s {
				[] => true,
				['a', 't', 'n', 'e', 'g', 'o', 'r', 'd', 'y', 'h', 'm', s @ ..] => {
					super::push(out, 30640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 21114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30641);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 30642);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 30643);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 30644);
					match s {
						[] => true,
						['g', 'a', 'm', 'o', s @ ..] => {
							super::push(out, 37173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 30645);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 30646);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urat.
pub(crate) fn n10502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10379);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21115);
			s.is_empty()
		}
		['g', 'g', 'i', 'z', '\u{1}', s @ ..] => {
			super::push(out, 21116);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 21117);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30647);
					s.is_empty()
				}
				['d', 'l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 30648);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 30649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', 'q', 'i', 'z', '\u{1}', s @ ..] => {
			super::push(out, 21118);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 21119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vrat.
pub(crate) fn n10503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10380);
	s.is_empty()
}

/// Words ending -yrat.
pub(crate) fn n10504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10381);
	s.is_empty()
}

/// Words ending -chaft.
pub(crate) fn n21862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21250);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 30722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37219);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 37220);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43370);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 43371);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44355);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 44356);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 44779);
													s.is_empty()
												}
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
				['k', s @ ..] => {
					super::push(out, 37221);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 37222);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41193);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 43372);
									match s {
										[] => true,
										['e', 'i', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 44357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 41194);
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									super::push(out, 43373);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 44358);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 41195);
							match s {
								[] => true,
								['e', 's', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44359);
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 44780);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'e', 't', 'm', s @ ..] => {
											super::push(out, 44360);
											s.is_empty()
										}
										['s', 't', s @ ..] => {
											super::push(out, 44361);
											match s {
												[] => true,
												['r', 'h', 'a', 'f', 'f', 'f', 'i', 'h', 'c', 's', 'f', s @ ..] => {
													super::push(out, 44781);
													match s {
														[] => true,
														['p', 'm', 'a', 'd', 'u', s @ ..] => super::t15::n48018(s, out),
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
				['n', s @ ..] => super::t15::n39349(s, out),
				['r', s @ ..] => super::t15::n39350(s, out),
				['s', s @ ..] => {
					super::push(out, 37225);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41202);
							match s {
								[] => true,
								['l', 'g', 'r', 'e', 'b', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 43384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t15::n39352(s, out),
				['u', s @ ..] => {
					super::push(out, 37227);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 37228);
					match s {
						[] => true,
						['l', s @ ..] => super::t15::n43824(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -raft.
pub(crate) fn n10630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10507);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 21260);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 30728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 21261);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 21262);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 30730);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 37230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'g', s @ ..] => {
					super::push(out, 30731);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 30732);
					match s {
						[] => true,
						['p', 'm', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 37232);
							s.is_empty()
						}
						['u', 'a', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41211);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 41212);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 41213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'a', 'l', s @ ..] => {
					super::push(out, 30733);
					s.is_empty()
				}
				['h', 'e', 's', s @ ..] => {
					super::push(out, 30734);
					s.is_empty()
				}
				['l', 'h', s @ ..] => {
					super::push(out, 30735);
					s.is_empty()
				}
				['n', 'e', 'h', 'c', 's', s @ ..] => {
					super::push(out, 30736);
					s.is_empty()
				}
				['r', 'e', 's', s @ ..] => {
					super::push(out, 30737);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 37233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'z', s @ ..] => {
							super::push(out, 37234);
							s.is_empty()
						}
						['g', 'n', s @ ..] => {
							super::push(out, 37235);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 44399);
													s.is_empty()
												}
												['n', 'i', 'f', 'p', s @ ..] => {
													super::push(out, 44400);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 43389);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 44401);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 44402);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 43390);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 43391);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 44403);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 43392);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 43393);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'e', s @ ..] => {
							super::push(out, 37236);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 41214);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 41215);
									match s {
										[] => true,
										['i', 't', 'a', 'v', s @ ..] => {
											super::push(out, 43394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', 'l', 'h', 'c', 's', 'e', s @ ..] => {
							super::push(out, 37237);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 41216);
									match s {
										[] => true,
										['a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 43395);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 41217);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 43396);
											s.is_empty()
										}
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
						['a', s @ ..] => {
							super::push(out, 37238);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 41218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 37239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 30738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -racht.
pub(crate) fn n21937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21320);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 30807);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 30808);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 30809);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					super::push(out, 37291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37292);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 41239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43404);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', s @ ..] => {
					super::push(out, 37293);
					s.is_empty()
				}
				['r', 'e', 'd', s @ ..] => {
					super::push(out, 37294);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 37295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -echt.
pub(crate) fn n10660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10537);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 21326);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37296);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 37297);
							s.is_empty()
						}
						['r', 'e', 'u', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 37298);
							s.is_empty()
						}
						['t', 'r', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 37299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 21327);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 21328);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 30811);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37300);
							match s {
								[] => true,
								['g', 'r', s @ ..] => {
									super::push(out, 41240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 30812);
					match s {
						[] => true,
						['c', 's', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41241);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 41242);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 43406);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 41243);
									s.is_empty()
								}
								['s', 'l', 'e', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 41244);
									s.is_empty()
								}
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
			super::push(out, 21329);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 30813);
					match s {
						[] => true,
						['n', 'e', 'd', s @ ..] => {
							super::push(out, 37301);
							match s {
								[] => true,
								['u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 41245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'b', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 37302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 21330);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 30814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37303);
							s.is_empty()
						}
						['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 37304);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 37305);
							s.is_empty()
						}
						['g', 'r', 'e', 'w', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41246);
									s.is_empty()
								}
								['n', 'e', 'p', 'p', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 41247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 37306);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 41248);
									s.is_empty()
								}
								['i', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 41249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 43407);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 43408);
											s.is_empty()
										}
										['k', 'c', 'ü', 'r', 'ß', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 43409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', 'n', 'e', 'f', 'l', 'e', '\u{1}', s @ ..] => {
											super::push(out, 43410);
											s.is_empty()
										}
										['l', 'k', '\u{1}', s @ ..] => {
											super::push(out, 43411);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ü', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 41250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'u', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 37307);
							s.is_empty()
						}
						['s', 's', 'u', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 37308);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 37309);
							s.is_empty()
						}
						['u', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 37310);
							s.is_empty()
						}
						['z', 'r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t15::n21948(s, out),
		_ => false,
	}
}

