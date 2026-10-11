use flexion::Path;

/// Words ending -тавливать.
pub(crate) fn n29221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20388);
	match s {
		[] => true,
		['о', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'а', 'к', 'д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word осчастливливать.
pub(crate) fn n14831(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лвливать.
pub(crate) fn n14832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11742);
	match s {
		[] => true,
		['а', 'м', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'м', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 28687);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34577);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овливать.
pub(crate) fn n14833(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', 'к', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'й', 'а', 'н', s @ ..] => {
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -государствливать.
pub(crate) fn n14834(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явливать.
pub(crate) fn n14835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11743);
	match s {
		[] => true,
		['н', 'ю', 'л', 'с', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28688);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34578);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'ы', 'д', s @ ..] => {
			match s {
				[] => true,
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алепливать.
pub(crate) fn n46811(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38411);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40738);
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
		['н', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38412);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40739);
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

/// Words ending -облепливать.
pub(crate) fn n46812(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 34584);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38413);
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

/// Words ending -влепливать.
pub(crate) fn n46813(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 34585);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38414);
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

/// The word надлепливать.
pub(crate) fn n46814(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перелепливать.
pub(crate) fn n46815(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -злепливать.
pub(crate) fn n46816(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38415);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40740);
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
		['и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -прилепливать.
pub(crate) fn n46817(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 34586);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38416);
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

/// Words ending -олепливать.
pub(crate) fn n46818(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38417);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40741);
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
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слепливать.
pub(crate) fn n46819(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 34587);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38418);
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

/// Words ending -отлепливать.
pub(crate) fn n46820(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 34588);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38419);
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

/// The word улепливать.
pub(crate) fn n46821(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -вылепливать.
pub(crate) fn n46822(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 38420);
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

/// The word затепливать.
pub(crate) fn n29313(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -опливать.
pub(crate) fn n14881(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', 'к', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'и', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рпливать.
pub(crate) fn n14882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11763);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упливать.
pub(crate) fn n14883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11764);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'к', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -икатуривать.
pub(crate) fn n62453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34611);
	match s {
		[] => true,
		['р', 'а', 'к', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40744);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укатуривать.
pub(crate) fn n62454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34612);
	match s {
		[] => true,
		['т', 'ш', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -литературивать.
pub(crate) fn n47134(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лтуривать.
pub(crate) fn n29701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20480);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28744);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 34613);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтуривать.
pub(crate) fn n29702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20481);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28745);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34614);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обмишуривать.
pub(crate) fn n15044(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -щуривать.
pub(crate) fn n15045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11833);
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сколачивать.
pub(crate) fn n62507(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word уколачивать.
pub(crate) fn n62509(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выколачивать.
pub(crate) fn n62510(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -молачивать.
pub(crate) fn n47248(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плачивать.
pub(crate) fn n29821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20517);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28771);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 28773);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34625);
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word всхохлачивать.
pub(crate) fn n29822(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -мачивать.
pub(crate) fn n15085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11846);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'л', 'о', 'п', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['х', 'о', 'л', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', 'о', 'л', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -начивать.
pub(crate) fn n15086(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'х', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['х', 'о', 'м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пачивать.
pub(crate) fn n15087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11847);
	match s {
		[] => true,
		['л', 'о', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 34626);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 38428);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40745);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рачивать.
pub(crate) fn n15088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11848);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 20518);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 28774);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34627);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 20519);
			match s {
				[] => true,
				['м', 'о', 'д', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'ю', 'б', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 34628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38429);
									s.is_empty()
								}
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
			super::push(out, 20520);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
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
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 20521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28775);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28776);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'д', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сачивать.
pub(crate) fn n15089(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тачивать.
pub(crate) fn n15090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11849);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20522);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word наохачивать.
pub(crate) fn n15091(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word позаканчивать.
pub(crate) fn n85601(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -сучивать.
pub(crate) fn n15139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11871);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тучивать.
pub(crate) fn n15140(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', 'д', 'н', 'у', 'м', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шучивать.
pub(crate) fn n15141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11872);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20541);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 20542);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28796);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20543);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20544);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28799);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 34638);
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
		['ы', s @ ..] => {
			super::push(out, 20545);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word прищучивать.
pub(crate) fn n29973(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ущучивать.
pub(crate) fn n29974(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -выучивать.
pub(crate) fn n15143(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word повыспрашивать.
pub(crate) fn n87974(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ствовать.
pub(crate) fn n15223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11917);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20577);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28832);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 38438);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 40751);
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
								['о', s @ ..] => {
									match s {
										[] => true,
										['м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', 'и', 'д', 'е', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 40752);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 28833);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 38439);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40753);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 40754);
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 20578);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 28834);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34651);
							match s {
								[] => true,
								['о', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40755);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42163);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n87976(s, out),
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
							super::push(out, 34652);
							match s {
								[] => true,
								['б', 'л', 'о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['г', 'з', 'о', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['л', 'о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28835);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['ж', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40756);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42164);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n87977(s, out),
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
						['н', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 34653);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38440);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 40757);
											match s {
												[] => true,
												['е', 'л', 'х', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 34654);
							match s {
								[] => true,
								['о', 'п', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40758);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42165);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n87978(s, out),
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
						['ф', s @ ..] => super::t28::n62754(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t28::n30172(s, out),
		['г', s @ ..] => {
			super::push(out, 20580);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28842);
					match s {
						[] => true,
						['г', 'а', 'м', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t28::n30174(s, out),
		['е', s @ ..] => super::t28::n30175(s, out),
		['и', s @ ..] => super::t28::n30176(s, out),
		['й', s @ ..] => super::t28::n30177(s, out),
		['м', s @ ..] => super::t28::n30178(s, out),
		['н', s @ ..] => super::t28::n30179(s, out),
		['п', s @ ..] => super::t28::n30180(s, out),
		['р', s @ ..] => super::t28::n30181(s, out),
		['т', s @ ..] => super::t28::n30182(s, out),
		['ф', s @ ..] => super::t28::n30183(s, out),
		['ь', s @ ..] => super::t28::n30184(s, out),
		_ => false,
	}
}

/// The word недообследовать.
pub(crate) fn n88143(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word нарасследовать.
pub(crate) fn n88144(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перерасследовать.
pub(crate) fn n88145(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word дорасследовать.
pub(crate) fn n88146(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word дооборудовать.
pub(crate) fn n85904(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -едооборудовать.
pub(crate) fn n85905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42285);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 43079);
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

/// Words ending -лизовать.
pub(crate) fn n15275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11967);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20661);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28974);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 34897);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 38690);
									match s {
										[] => true,
										['г', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 34898);
							match s {
								[] => true,
								['е', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 28975);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 28976);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 34899);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['к', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 28977);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 28978);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 28979);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 34900);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38691);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 38692);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 40963);
											match s {
												[] => true,
												['в', 'д', 'а', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 34901);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 38693);
									match s {
										[] => true,
										['т', 'а', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 42286);
													match s {
														[] => true,
														['д', s @ ..] => super::t28::n88148(s, out),
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
									super::push(out, 38694);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 40964);
											match s {
												[] => true,
												['у', 'д', 'н', 'и', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 34902);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38695);
									match s {
										[] => true,
										['е', 'м', 'м', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 28980);
					match s {
						[] => true,
						['и', s @ ..] => super::t28::n63087(s, out),
						['о', s @ ..] => super::t28::n63088(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t28::n47818(s, out),
				['н', s @ ..] => super::t28::n47819(s, out),
				['р', s @ ..] => super::t28::n47820(s, out),
				['т', s @ ..] => super::t28::n47821(s, out),
				['у', s @ ..] => super::t28::n47822(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t28::n30278(s, out),
		['и', s @ ..] => super::t28::n30279(s, out),
		['к', s @ ..] => super::t28::n30280(s, out),
		['л', s @ ..] => super::t28::n30281(s, out),
		['о', s @ ..] => super::t28::n30282(s, out),
		_ => false,
	}
}

/// Words ending -еорганизовать.
pub(crate) fn n74610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38717);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 42294);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43082);
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

/// The word дезорганизовать.
pub(crate) fn n85935(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -сорганизовать.
pub(crate) fn n74612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ианизовать.
pub(crate) fn n47839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28999);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 34929);
			match s {
				[] => true,
				['с', 'и', 'р', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -канизовать.
pub(crate) fn n47840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29000);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34930);
			match s {
				[] => true,
				['у', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -манизовать.
pub(crate) fn n47841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29001);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34931);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранизовать.
pub(crate) fn n47842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29002);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34932);
			match s {
				[] => true,
				['э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енизовать.
pub(crate) fn n30287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20671);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29003);
			match s {
				[] => true,
				['о', 'м', 'о', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29004);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34933);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онизовать.
pub(crate) fn n30288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20672);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29006);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 38720);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 40985);
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
		['м', s @ ..] => {
			super::push(out, 29007);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 29008);
			match s {
				[] => true,
				['а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29009);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 34934);
					match s {
						[] => true,
						['н', 'и', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнизовать.
pub(crate) fn n30289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20673);
	match s {
		[] => true,
		['е', 'д', 'о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оизовать.
pub(crate) fn n15278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11970);
	match s {
		[] => true,
		['р', 'е', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ризовать.
pub(crate) fn n15279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11971);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20674);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29010);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34935);
							match s {
								[] => true,
								['а', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29011);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 34936);
							match s {
								[] => true,
								['и', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29012);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34937);
							match s {
								[] => true,
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29013);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34938);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 38721);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40986);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 38722);
									match s {
										[] => true,
										['о', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 34939);
							match s {
								[] => true,
								['л', 'и', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40987);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 40988);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'е', 'в', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38723);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 38724);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 20675);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29014);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 34940);
							match s {
								[] => true,
								['л', 'у', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 29015);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34941);
							match s {
								[] => true,
								['в', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29016);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34942);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 38725);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 34943);
							match s {
								[] => true,
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29017);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 34944);
							match s {
								[] => true,
								['а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29018);
					match s {
						[] => true,
						['н', 'а', 'п', 'с', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29019);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34945);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 38726);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 40989);
											match s {
												[] => true,
												['к', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 34946);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 40990);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 40991);
											match s {
												[] => true,
												['а', 'х', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88160(s, out),
														['о', s @ ..] => super::t28::n88161(s, out),
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
						['с', s @ ..] => {
							super::push(out, 34947);
							match s {
								[] => true,
								['а', 'л', s @ ..] => {
									super::push(out, 38727);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 34948);
							match s {
								[] => true,
								['ь', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29020);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 34949);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t28::n30293(s, out),
		['т', s @ ..] => super::t28::n30294(s, out),
		['у', s @ ..] => super::t28::n30295(s, out),
		['ю', s @ ..] => super::t28::n30296(s, out),
		['я', s @ ..] => super::t28::n30297(s, out),
		_ => false,
	}
}

/// Words ending -тизовать.
pub(crate) fn n15280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11972);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20681);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29027);
					match s {
						[] => true,
						['е', 'л', 'б', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29028);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 34958);
							match s {
								[] => true,
								['б', 'р', 'а', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 20682);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29029);
					match s {
						[] => true,
						['к', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20683);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29030);
					match s {
						[] => true,
						['д', 'н', 'а', 'т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29031);
					match s {
						[] => true,
						['п', 'с', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -физовать.
pub(crate) fn n15281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11973);
	match s {
		[] => true,
		['р', 'о', 'м', 'а', 'т', 'е', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хизовать.
pub(crate) fn n15282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11974);
	match s {
		[] => true,
		['р', 'а', 'р', 'е', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word републиковать.
pub(crate) fn n85947(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word сполитиковать.
pub(crate) fn n85948(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -критиковать.
pub(crate) fn n63199(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38743);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 38744);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 38745);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктиковать.
pub(crate) fn n30349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20719);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29062);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38746);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41000);
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
		_ => false,
	}
}

/// Words ending -фиковать.
pub(crate) fn n15312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12002);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20720);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34973);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 34974);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 38747);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// Words ending -мчиковать.
pub(crate) fn n30351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20721);
	match s {
		[] => true,
		['и', 'ч', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шиковать.
pub(crate) fn n15314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12004);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щиковать.
pub(crate) fn n15315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12005);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 20722);
			match s {
				[] => true,
				['я', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word наконфисковать.
pub(crate) fn n85950(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word предзнаменовать.
pub(crate) fn n85954(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -футеровать.
pub(crate) fn n30564(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29180);
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

/// Words ending -церовать.
pub(crate) fn n15448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12135);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20860);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29181);
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

/// Words ending -черовать.
pub(crate) fn n15449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12136);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 20861);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29182);
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

/// The word пролоббировать.
pub(crate) fn n85956(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word опломбировать.
pub(crate) fn n85957(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word распломбировать.
pub(crate) fn n85958(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -обировать.
pub(crate) fn n30571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20866);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 29189);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38775);
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
				['т', s @ ..] => {
					super::push(out, 35040);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38776);
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

/// Words ending -рбировать.
pub(crate) fn n30572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20867);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29190);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38777);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 38778);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42295);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29191);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35042);
					match s {
						[] => true,
						['с', 'а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41012);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42296);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убировать.
pub(crate) fn n30573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20868);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29192);
			match s {
				[] => true,
				['н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38779);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41013);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['т', s @ ..] => {
			super::push(out, 29193);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35043);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38780);
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
				['с', s @ ..] => {
					super::push(out, 35044);
					match s {
						[] => true,
						['к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вировать.
pub(crate) fn n15451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12138);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20869);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29194);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35045);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35046);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38781);
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
						['к', s @ ..] => {
							super::push(out, 35047);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38782);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 41014);
											match s {
												[] => true,
												['в', 'а', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 35048);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 38784);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41015);
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
						['с', s @ ..] => {
							super::push(out, 35049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35050);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38785);
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
						['ы', s @ ..] => {
							super::push(out, 35051);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38786);
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
				['р', s @ ..] => {
					super::push(out, 29195);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35052);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 35053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38787);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 38788);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 38789);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 41016);
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
								['з', s @ ..] => {
									super::push(out, 38790);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 38791);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 38792);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 38793);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 41017);
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20870);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29196);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35054);
							match s {
								[] => true,
								['ц', 'е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29197);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35055);
							match s {
								[] => true,
								['а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t28::n48196(s, out),
				['х', s @ ..] => {
					super::push(out, 29198);
					match s {
						[] => true,
						['р', 'а', s @ ..] => super::t28::n63384(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t28::n30576(s, out),
		['р', s @ ..] => super::t28::n30577(s, out),
		['ь', s @ ..] => super::t28::n30578(s, out),
		_ => false,
	}
}

/// Words ending -гировать.
pub(crate) fn n15452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12139);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20874);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29202);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35063);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38808);
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 38809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 38810);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29203);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35064);
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
				['п', s @ ..] => {
					super::push(out, 29204);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35065);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29205);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35066);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35067);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38811);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41031);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 38812);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 41032);
											match s {
												[] => true,
												['э', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 35068);
							match s {
								[] => true,
								['и', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 20875);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 35069);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29207);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35070);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 41033);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 42305);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88175(s, out),
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
							super::push(out, 35071);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38814);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 35072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29208);
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
		['и', s @ ..] => {
			super::push(out, 20876);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29209);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35073);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29211);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 38816);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 38817);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['н', s @ ..] => {
			super::push(out, 20877);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29212);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35074);
							match s {
								[] => true,
								['е', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41034);
											s.is_empty()
										}
										['а', s @ ..] => super::t28::n81927(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t28::n48213(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t28::n30583(s, out),
		['р', s @ ..] => super::t28::n30584(s, out),
		['у', s @ ..] => super::t28::n30585(s, out),
		['ь', s @ ..] => super::t28::n30586(s, out),
		['ю', s @ ..] => super::t28::n30587(s, out),
		_ => false,
	}
}

/// Words ending -дировать.
pub(crate) fn n15453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12140);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20883);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29218);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35082);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38821);
									match s {
										[] => true,
										['р', 'а', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42307);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 42308);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t28::n88177(s, out),
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
						['с', s @ ..] => {
							super::push(out, 35083);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38822);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29219);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35084);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 41037);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 42309);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88178(s, out),
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
				['р', s @ ..] => {
					super::push(out, 29220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35085);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35086);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38823);
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
						['б', s @ ..] => {
							super::push(out, 35087);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38824);
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
						['г', s @ ..] => {
							super::push(out, 35088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38825);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 38826);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 41038);
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
						['р', s @ ..] => {
							super::push(out, 35089);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 38827);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 38828);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35090);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 38829);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 41039);
											match s {
												[] => true,
												['к', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 38830);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 41040);
											s.is_empty()
										}
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
			super::push(out, 20884);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29221);
					match s {
						[] => true,
						['р', 'о', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 29222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'б', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20885);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29223);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29224);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35092);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35093);
							s.is_empty()
						}
						['а', s @ ..] => super::t28::n63435(s, out),
						['л', s @ ..] => super::t28::n63436(s, out),
						['о', s @ ..] => super::t28::n63437(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t28::n48229(s, out),
				['о', s @ ..] => super::t28::n48230(s, out),
				['с', s @ ..] => super::t28::n48231(s, out),
				['ф', s @ ..] => super::t28::n48232(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t28::n30591(s, out),
		['л', s @ ..] => super::t28::n30592(s, out),
		['н', s @ ..] => super::t28::n30593(s, out),
		['о', s @ ..] => super::t28::n30594(s, out),
		['р', s @ ..] => super::t28::n30595(s, out),
		['у', s @ ..] => super::t28::n30596(s, out),
		['ь', s @ ..] => super::t28::n30597(s, out),
		['ю', s @ ..] => super::t28::n30598(s, out),
		_ => false,
	}
}

/// Words ending -жировать.
pub(crate) fn n15454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12141);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20894);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 20895);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29250);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35133);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 41054);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 42319);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88187(s, out),
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
					super::push(out, 29251);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35134);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29252);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35135);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 38859);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 41055);
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
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29253);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 35136);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29254);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38860);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 38861);
									match s {
										[] => true,
										['а', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 35138);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38862);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 38863);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 38864);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41056);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 41057);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 41058);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 35139);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 35140);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 38866);
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
				['с', s @ ..] => {
					super::push(out, 29255);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35141);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41059);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 42320);
													match s {
														[] => true,
														['а', 'с', '\u{1}', s @ ..] => super::t28::n88188(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 42321);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88189(s, out),
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
				['т', s @ ..] => {
					super::push(out, 29256);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35142);
							match s {
								[] => true,
								['а', 'ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41060);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41061);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42322);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88190(s, out),
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
		['д', s @ ..] => {
			super::push(out, 20896);
			match s {
				[] => true,
				['е', s @ ..] => super::t28::n48261(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20897);
			match s {
				[] => true,
				['т', s @ ..] => super::t28::n48262(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t28::n30603(s, out),
		['н', s @ ..] => super::t28::n30604(s, out),
		['о', s @ ..] => super::t28::n30605(s, out),
		['р', s @ ..] => super::t28::n30606(s, out),
		['т', s @ ..] => super::t28::n30607(s, out),
		['у', s @ ..] => super::t28::n30608(s, out),
		['я', s @ ..] => super::t28::n30609(s, out),
		_ => false,
	}
}

/// Words ending -зировать.
pub(crate) fn n15455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20905);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35149);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38871);
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
						['е', s @ ..] => {
							super::push(out, 35150);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38872);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 35151);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29273);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35152);
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
				['р', s @ ..] => {
					super::push(out, 29274);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 35153);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38873);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 38874);
									match s {
										[] => true,
										['р', 'а', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 38875);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 38876);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29275);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35154);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38877);
									match s {
										[] => true,
										['ф', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42324);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 42325);
													match s {
														[] => true,
														['з', s @ ..] => super::t28::n88192(s, out),
														['н', s @ ..] => super::t28::n88193(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 42326);
													match s {
														[] => true,
														['д', s @ ..] => super::t28::n88194(s, out),
														['п', s @ ..] => super::t28::n88195(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 42327);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88196(s, out),
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
						['с', s @ ..] => {
							super::push(out, 35155);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38878);
									match s {
										[] => true,
										['т', 'е', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 38879);
									match s {
										[] => true,
										['п', 'и', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['г', s @ ..] => {
													super::push(out, 42328);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n88197(s, out),
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
				['ф', s @ ..] => {
					super::push(out, 29276);
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
		['е', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29277);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 35156);
							match s {
								[] => true,
								['л', 'а', 'н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29278);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35157);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t28::n48283(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t28::n30612(s, out),
		['н', s @ ..] => super::t28::n30613(s, out),
		['о', s @ ..] => super::t28::n30614(s, out),
		['у', s @ ..] => {
			super::push(out, 20909);
			match s {
				[] => true,
				['а', 'к', '\u{1}', s @ ..] => super::t28::n48312(s, out),
				['б', 'е', 'к', 'р', 'а', '\u{1}', s @ ..] => super::t28::n48313(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t28::n30616(s, out),
		_ => false,
	}
}

/// Words ending -иировать.
pub(crate) fn n15456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12143);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20911);
			match s {
				[] => true,
				['е', 'р', 'б', 'б', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 35226);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39012);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 20912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29309);
					match s {
						[] => true,
						['л', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39013);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39014);
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
		['д', s @ ..] => {
			super::push(out, 20913);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29310);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 20914);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29311);
					match s {
						[] => true,
						['ф', 'ф', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20915);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29312);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35227);
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
				['п', s @ ..] => {
					super::push(out, 29313);
					match s {
						[] => true,
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39015);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 39016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35228);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'к', 'э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['ц', s @ ..] => {
			super::push(out, 20916);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29315);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 39017);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 39018);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 41194);
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
				['о', s @ ..] => {
					super::push(out, 29316);
					match s {
						[] => true,
						['с', 'с', 'и', s @ ..] => {
							super::push(out, 35229);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -кировать.
pub(crate) fn n15457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12144);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29317);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35230);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35231);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35232);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 35233);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 35234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35235);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29319);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35236);
							match s {
								[] => true,
								['и', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 20918);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29320);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20919);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35237);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 35238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39019);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39020);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41195);
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
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20920);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29322);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35239);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 39021);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 39022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 35240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39023);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41196);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 35241);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 39024);
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
		['о', s @ ..] => {
			super::push(out, 20921);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29323);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35242);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29324);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39025);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 41197);
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
								['е', s @ ..] => {
									super::push(out, 39026);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 41198);
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
								['з', s @ ..] => {
									super::push(out, 39027);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 39028);
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
				['р', s @ ..] => {
					super::push(out, 29325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35243);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 35244);
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
				['ш', s @ ..] => {
					super::push(out, 29326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20922);
			match s {
				[] => true,
				['а', s @ ..] => super::t28::n48334(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t28::n30629(s, out),
		['ь', s @ ..] => super::t28::n30630(s, out),
		_ => false,
	}
}

/// Words ending -лировать.
pub(crate) fn n15458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12145);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20925);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29330);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35246);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 29331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35247);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29332);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39040);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39041);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 41205);
											match s {
												[] => true,
												['и', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 35248);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 39042);
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
						['ш', s @ ..] => {
							super::push(out, 35249);
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
				['м', s @ ..] => {
					super::push(out, 29333);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 35250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29334);
					match s {
						[] => true,
						['р', 'у', 'ж', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29335);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29336);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29337);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35251);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 35252);
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
				['у', s @ ..] => {
					super::push(out, 29338);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39043);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39044);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 41206);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 42426);
													match s {
														[] => true,
														['д', s @ ..] => super::t28::n88352(s, out),
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
		['б', s @ ..] => {
			super::push(out, 20926);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29339);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35253);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 35254);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 39045);
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
				['е', s @ ..] => {
					super::push(out, 29340);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39046);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 39047);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39048);
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
				['м', s @ ..] => {
					super::push(out, 29341);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35256);
							match s {
								[] => true,
								['с', 'с', 'а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41207);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 41208);
											match s {
												[] => true,
												['и', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 29342);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35257);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29343);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35258);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39049);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39050);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t28::n30633(s, out),
		['д', s @ ..] => {
			super::push(out, 20928);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29345);
					match s {
						[] => true,
						['й', '\u{1}', s @ ..] => super::t28::n63656(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t28::n30635(s, out),
		['и', s @ ..] => super::t28::n30636(s, out),
		['к', s @ ..] => super::t28::n30637(s, out),
		['л', s @ ..] => super::t29::n30638(s, out),
		['о', s @ ..] => super::t29::n30639(s, out),
		['п', s @ ..] => super::t29::n30640(s, out),
		['с', s @ ..] => super::t29::n30641(s, out),
		['у', s @ ..] => super::t29::n30642(s, out),
		['ф', s @ ..] => super::t29::n30643(s, out),
		_ => false,
	}
}

/// Words ending -мировать.
pub(crate) fn n15459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12146);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20938);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29394);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 35338);
							match s {
								[] => true,
								['л', 'а', 'м', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29395);
					match s {
						[] => true,
						['ь', 'л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39109);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39110);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29396);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39111);
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42455);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 42456);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t29::n88379(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 42457);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n88380(s, out),
														['р', s @ ..] => super::t29::n88381(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42458);
													s.is_empty()
												}
												['з', s @ ..] => {
													super::push(out, 42459);
													match s {
														[] => true,
														['а', 'р', '\u{1}', s @ ..] => super::t29::n88382(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 42460);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t29::n88383(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 42461);
													match s {
														[] => true,
														['о', '\u{1}', s @ ..] => super::t29::n88384(s, out),
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
									super::push(out, 39112);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41252);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42462);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88385(s, out),
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
				['р', s @ ..] => {
					super::push(out, 29397);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35339);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39113);
									match s {
										[] => true,
										['а', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42463);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 42464);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88386(s, out),
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
						['ф', s @ ..] => {
							super::push(out, 35340);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 39114);
									match s {
										[] => true,
										['л', 'о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 35341);
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
		['в', s @ ..] => {
			super::push(out, 20939);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 20940);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29399);
					match s {
						[] => true,
						['р', 'ф', 'а', 'и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39115);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39116);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 20941);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29400);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t29::n75297(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t29::n63756(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t29::n30648(s, out),
		['и', s @ ..] => super::t29::n30649(s, out),
		['м', s @ ..] => super::t29::n30650(s, out),
		['о', s @ ..] => super::t29::n30651(s, out),
		['р', s @ ..] => super::t29::n30652(s, out),
		['т', s @ ..] => super::t29::n30653(s, out),
		['у', s @ ..] => super::t29::n30654(s, out),
		['ф', s @ ..] => super::t29::n30655(s, out),
		['ь', s @ ..] => super::t29::n30656(s, out),
		['ю', s @ ..] => super::t29::n30657(s, out),
		_ => false,
	}
}

/// Words ending -нировать.
pub(crate) fn n15460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12147);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20952);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29423);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 35367);
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
				['к', s @ ..] => {
					super::push(out, 29424);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35368);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 39141);
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
						['н', s @ ..] => {
							super::push(out, 35369);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39142);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 35370);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39143);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39144);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29425);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35371);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39145);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39146);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 39147);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 41267);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 41268);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 39148);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 35372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39149);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39150);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41269);
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
				['н', s @ ..] => {
					super::push(out, 29426);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35373);
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
				['п', s @ ..] => {
					super::push(out, 29427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35374);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35375);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39151);
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
						['е', s @ ..] => {
							super::push(out, 35376);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39152);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 35377);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39153);
									match s {
										[] => true,
										['к', s @ ..] => super::t29::n82419(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t29::n48438(s, out),
				['с', s @ ..] => super::t29::n48439(s, out),
				['т', s @ ..] => super::t29::n48440(s, out),
				['ф', s @ ..] => super::t29::n48441(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t29::n30659(s, out),
		['е', s @ ..] => super::t29::n30660(s, out),
		['и', s @ ..] => super::t29::n30661(s, out),
		['й', s @ ..] => super::t29::n30662(s, out),
		['л', s @ ..] => super::t29::n30663(s, out),
		['н', s @ ..] => super::t29::n30664(s, out),
		['о', s @ ..] => super::t29::n30665(s, out),
		['р', s @ ..] => super::t29::n30666(s, out),
		['у', s @ ..] => super::t29::n30667(s, out),
		_ => false,
	}
}

/// Words ending -оировать.
pub(crate) fn n15461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12148);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20962);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29479);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39226);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39227);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 20963);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пировать.
pub(crate) fn n15462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12149);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20964);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 20965);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29482);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35460);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29483);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39228);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29484);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 35462);
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
		['е', s @ ..] => {
			super::push(out, 20966);
			match s {
				[] => true,
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20967);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29485);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 35463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29486);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 39229);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 41324);
											match s {
												[] => true,
												['г', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 39230);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41325);
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
								['т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 29488);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35465);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 35466);
							match s {
								[] => true,
								['т', 'н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 35467);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 35468);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39231);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', 'а', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
				['ч', s @ ..] => {
					super::push(out, 29489);
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
		['й', s @ ..] => {
			super::push(out, 20968);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29490);
					match s {
						[] => true,
						['т', 'е', 'л', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 20969);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29491);
					match s {
						[] => true,
						['р', 'р', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20970);
			match s {
				[] => true,
				['к', s @ ..] => super::t29::n48505(s, out),
				['л', s @ ..] => {
					super::push(out, 29493);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35475);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 39241);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29494);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 35476);
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
				['п', s @ ..] => {
					super::push(out, 29495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29496);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35478);
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
		['п', s @ ..] => super::t29::n30677(s, out),
		['р', s @ ..] => super::t29::n30678(s, out),
		['т', s @ ..] => {
			super::push(out, 20973);
			match s {
				[] => true,
				['о', s @ ..] => super::t29::n48514(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t29::n30680(s, out),
		['ь', s @ ..] => super::t29::n30681(s, out),
		['ю', s @ ..] => super::t29::n30682(s, out),
		_ => false,
	}
}

/// Words ending -рировать.
pub(crate) fn n15463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12150);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20977);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29505);
					match s {
						[] => true,
						['к', 'е', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39244);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39245);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 35485);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41332);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 41333);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 42500);
													match s {
														[] => true,
														['о', '\u{1}', s @ ..] => super::t29::n88429(s, out),
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
									super::push(out, 39246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 41334);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 35486);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39247);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35487);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39248);
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
				['т', s @ ..] => {
					super::push(out, 29506);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 29507);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35488);
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
		['б', s @ ..] => {
			super::push(out, 20978);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29508);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35489);
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
				['и', s @ ..] => {
					super::push(out, 29509);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35490);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39249);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 41335);
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
						['л', s @ ..] => {
							super::push(out, 35491);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39250);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 41336);
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
								['и', s @ ..] => {
									super::push(out, 39251);
									match s {
										[] => true,
										['в', 'к', 'э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 35492);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39252);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['в', s @ ..] => {
			super::push(out, 20979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29510);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 35493);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39253);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35494);
							match s {
								[] => true,
								['с', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41337);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 41338);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 42501);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88430(s, out),
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
				['е', s @ ..] => super::t29::n48525(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t29::n30686(s, out),
		['д', s @ ..] => super::t29::n30687(s, out),
		['е', s @ ..] => super::t29::n30688(s, out),
		['и', s @ ..] => super::t29::n30689(s, out),
		['к', s @ ..] => {
			super::push(out, 20984);
			match s {
				[] => true,
				['и', s @ ..] => super::t29::n48544(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t29::n30691(s, out),
		['р', s @ ..] => {
			super::push(out, 20986);
			match s {
				[] => true,
				['е', s @ ..] => super::t29::n48556(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t29::n30693(s, out),
		['у', s @ ..] => super::t29::n30694(s, out),
		['ф', s @ ..] => super::t29::n30695(s, out),
		['э', s @ ..] => super::t29::n30696(s, out),
		['ю', s @ ..] => super::t29::n30697(s, out),
		_ => false,
	}
}

/// Words ending -сировать.
pub(crate) fn n15464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12151);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20992);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29557);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39297);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39298);
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
				['р', s @ ..] => {
					super::push(out, 29558);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35542);
							match s {
								[] => true,
								['е', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 20993);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29560);
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
		['й', s @ ..] => {
			super::push(out, 20994);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29561);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 39299);
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
		['к', s @ ..] => {
			super::push(out, 20995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29562);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35543);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39300);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 35544);
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
				['е', s @ ..] => {
					super::push(out, 29563);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35545);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39301);
									match s {
										[] => true,
										['и', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42526);
													s.is_empty()
												}
												['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 42527);
													match s {
														[] => true,
														['д', '\u{1}', s @ ..] => super::t29::n88458(s, out),
														['р', 'п', '\u{1}', s @ ..] => super::t29::n88459(s, out),
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
						['л', s @ ..] => {
							super::push(out, 35546);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39302);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 41372);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 42528);
													match s {
														[] => true,
														['ь', 'л', 'у', 'м', '\u{1}', s @ ..] => super::t29::n88460(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 41373);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 42529);
													match s {
														[] => true,
														['к', '\u{1}', s @ ..] => super::t29::n88461(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 39303);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41374);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 41375);
											match s {
												[] => true,
												['р', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88462(s, out),
														['о', s @ ..] => super::t29::n88463(s, out),
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
						['н', s @ ..] => {
							super::push(out, 35547);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29564);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35548);
							match s {
								[] => true,
								['е', s @ ..] => super::t29::n75617(s, out),
								_ => false,
							}
						}
						['ф', s @ ..] => super::t29::n64034(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t29::n48582(s, out),
				['у', s @ ..] => super::t29::n48583(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t29::n30702(s, out),
		['п', s @ ..] => super::t29::n30703(s, out),
		['р', s @ ..] => super::t29::n30704(s, out),
		['с', s @ ..] => super::t29::n30705(s, out),
		['у', s @ ..] => super::t29::n30706(s, out),
		['ь', s @ ..] => super::t29::n30707(s, out),
		['ю', s @ ..] => super::t29::n30708(s, out),
		_ => false,
	}
}

/// Words ending -тировать.
pub(crate) fn n15465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12152);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21003);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21004);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29583);
					match s {
						[] => true,
						['е', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39348);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39349);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41407);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 29584);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35583);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 39350);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 39351);
									match s {
										[] => true,
										['г', 'а', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 42550);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88483(s, out),
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
							super::push(out, 35584);
							match s {
								[] => true,
								['р', 'р', 'у', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 29585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35585);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39352);
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
						['и', s @ ..] => {
							super::push(out, 35586);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 39353);
									match s {
										[] => true,
										['н', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29586);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35587);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 39354);
									match s {
										[] => true,
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 35588);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 39355);
									match s {
										[] => true,
										['и', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29587);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 35589);
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
				['м', s @ ..] => {
					super::push(out, 29588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35590);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35591);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 35592);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39356);
									match s {
										[] => true,
										['а', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 35593);
							match s {
								[] => true,
								['о', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41408);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 41409);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 35594);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 39357);
									match s {
										[] => true,
										['о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t29::n48606(s, out),
				['о', s @ ..] => super::t29::n48607(s, out),
				['п', s @ ..] => super::t29::n48608(s, out),
				['р', s @ ..] => super::t29::n48609(s, out),
				['т', s @ ..] => super::t29::n48610(s, out),
				['у', s @ ..] => super::t29::n48611(s, out),
				['ф', s @ ..] => super::t29::n48612(s, out),
				['ш', s @ ..] => super::t29::n48613(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t29::n30711(s, out),
		['и', s @ ..] => super::t29::n30712(s, out),
		['й', s @ ..] => super::t29::n30713(s, out),
		['к', s @ ..] => super::t29::n30714(s, out),
		['н', s @ ..] => super::t29::n30715(s, out),
		['о', s @ ..] => super::t29::n30716(s, out),
		['п', s @ ..] => super::t29::n30717(s, out),
		['р', s @ ..] => super::t29::n30718(s, out),
		['с', s @ ..] => super::t29::n30719(s, out),
		['т', s @ ..] => super::t29::n30720(s, out),
		['у', s @ ..] => super::t29::n30721(s, out),
		['ь', s @ ..] => super::t29::n30722(s, out),
		['ю', s @ ..] => super::t29::n30723(s, out),
		_ => false,
	}
}

/// Words ending -уировать.
pub(crate) fn n15466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12153);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21018);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29668);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39489);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 41516);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
		['в', s @ ..] => {
			super::push(out, 21019);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29669);
					match s {
						[] => true,
						['з', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21020);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29670);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39490);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39491);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39492);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['ж', s @ ..] => {
			super::push(out, 21021);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29671);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35746);
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
		['к', s @ ..] => {
			super::push(out, 21022);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29672);
					match s {
						[] => true,
						['в', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 39493);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['н', s @ ..] => {
			super::push(out, 21023);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29673);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35747);
							match s {
								[] => true,
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 21024);
			match s {
				[] => true,
				['т', 'с', 'н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35748);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 39494);
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
						['о', s @ ..] => {
							super::push(out, 35749);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 41517);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 42604);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88550(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88551(s, out),
														['е', s @ ..] => super::t29::n88552(s, out),
														['т', s @ ..] => super::t29::n88553(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 41518);
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
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21025);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29674);
					match s {
						[] => true,
						['т', s @ ..] => super::t29::n64275(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29675);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35750);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['н', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'п', '\u{1}', s @ ..] => super::t29::n82951(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t29::n48697(s, out),
				['н', s @ ..] => super::t29::n48698(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фировать.
pub(crate) fn n15467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12154);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21026);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35755);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39501);
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
						['г', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39502);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 41523);
											match s {
												[] => true,
												['е', 'т', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88554(s, out),
														['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t29::n88555(s, out),
														['о', 'р', 'п', '\u{1}', s @ ..] => super::t29::n88556(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t29::n75938(s, out),
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
			super::push(out, 21027);
			match s {
				[] => true,
				['т', s @ ..] => super::t29::n48700(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t29::n30734(s, out),
		['п', s @ ..] => super::t29::n30735(s, out),
		['р', s @ ..] => super::t29::n30736(s, out),
		['ь', s @ ..] => super::t29::n30737(s, out),
		_ => false,
	}
}

/// Words ending -цировать.
pub(crate) fn n15468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12155);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21032);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29681);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35761);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29682);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 39507);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41530);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 39508);
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
				['р', s @ ..] => {
					super::push(out, 29683);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35762);
							match s {
								[] => true,
								['к', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 29684);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35763);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t29::n30739(s, out),
		['н', s @ ..] => {
			super::push(out, 21034);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29692);
					match s {
						[] => true,
						['т', 'с', 'н', s @ ..] => {
							super::push(out, 35775);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29693);
					match s {
						[] => true,
						['р', 'е', 'ф', 'ф', 'и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39534);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39535);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39536);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 21035);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29694);
					match s {
						[] => true,
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39537);
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
				['л', s @ ..] => {
					super::push(out, 29695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35776);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 35777);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39538);
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
						['о', s @ ..] => {
							super::push(out, 35778);
							match s {
								[] => true,
								['е', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 35779);
							match s {
								[] => true,
								['и', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 41562);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
		['с', s @ ..] => {
			super::push(out, 21036);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29696);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39539);
									match s {
										[] => true,
										['о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 35781);
							match s {
								[] => true,
								['и', 'м', 'ю', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 35782);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39540);
									match s {
										[] => true,
										['ю', 'л', 'ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29697);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35783);
							match s {
								[] => true,
								['г', 'о', 'к', 'е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29698);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 35784);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 21037);
			match s {
				[] => true,
				['д', s @ ..] => super::t29::n48723(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 21038);
			match s {
				[] => true,
				['и', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чировать.
pub(crate) fn n15469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12156);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 21039);
			match s {
				[] => true,
				['е', 'р', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 21040);
			match s {
				[] => true,
				['л', 'у', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35786);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39543);
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
		_ => false,
	}
}

/// Words ending -шировать.
pub(crate) fn n15470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12157);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21041);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29699);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35787);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 21042);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29700);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 29701);
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
		['и', s @ ..] => {
			super::push(out, 21043);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29702);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35788);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39544);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29703);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29704);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35790);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29705);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35791);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 39545);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 41564);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 35792);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39546);
									match s {
										[] => true,
										['к', 'и', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['к', s @ ..] => {
			super::push(out, 21044);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29706);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 39547);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 39548);
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
		['н', s @ ..] => {
			super::push(out, 21045);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29707);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35793);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 35794);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 39549);
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
		['о', s @ ..] => {
			super::push(out, 21046);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 29708);
					match s {
						[] => true,
						['л', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21047);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29709);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39550);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39551);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 39552);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41565);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39553);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39554);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 35796);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39555);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39556);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 41566);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 41567);
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21048);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29710);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35797);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t29::n48739(s, out),
				_ => false,
			}
		}
		['э', s @ ..] => super::t29::n30755(s, out),
		_ => false,
	}
}

/// Words ending -рьировать.
pub(crate) fn n30756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21050);
	match s {
		[] => true,
		['а', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35801);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 35802);
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

/// Words ending -юировать.
pub(crate) fn n15472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12159);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21051);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 29714);
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
		['ь', s @ ..] => {
			super::push(out, 21052);
			match s {
				[] => true,
				['в', 'р', 'е', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35803);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 35804);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39558);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// The word переоркестровать.
pub(crate) fn n86741(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -реаккредитовать.
pub(crate) fn n86743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42637);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word репрезентовать.
pub(crate) fn n86744(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -линтовать.
pub(crate) fn n30904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21148);
	match s {
		[] => true,
		['б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39572);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41576);
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
		_ => false,
	}
}

/// The word понтовать.
pub(crate) fn n48915(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -унтовать.
pub(crate) fn n15543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12222);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21150);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29803);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 29804);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 35864);
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
				['е', s @ ..] => {
					super::push(out, 29805);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29806);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35865);
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
				['о', s @ ..] => {
					super::push(out, 29807);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29808);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35866);
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
				['ы', s @ ..] => {
					super::push(out, 29809);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35867);
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
		['п', s @ ..] => {
			super::push(out, 21151);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35868);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35869);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 21152);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35870);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word законвертовать.
pub(crate) fn n86746(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отрапортовать.
pub(crate) fn n86747(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -пуртовать.
pub(crate) fn n30923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21166);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 29826);
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

/// The word запротестовать.
pub(crate) fn n86748(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -апустовать.
pub(crate) fn n48963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29840);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 35889);
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

/// Words ending -рустовать.
pub(crate) fn n30935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21178);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29841);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 29842);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35890);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шустовать.
pub(crate) fn n30936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21179);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word позапамятовать.
pub(crate) fn n86749(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -зовывать.
pub(crate) fn n15667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12307);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'б', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['в', 'е', 'р', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 29902);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['г', 'е', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['б', 'р', 'е', 'в', 'д', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'е', 'т', 'а', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', 'о', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', 'р', 'о', s @ ..] => {
									match s {
										[] => true,
										['н', 'е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ф', 'а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['т', 'н', 'е', 'ц', 'а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', 'т', 'а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'р', 'о', 'м', 'м', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', 'и', 'ц', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'а', 'т', 'с', 'и', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29903);
					match s {
						[] => true,
						['а', 'г', 'р', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 39599);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42638);
													s.is_empty()
												}
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', 'е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', 'т', 'к', 'а', 'р', 'а', 'х', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'к', 'е', 'л', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'е', 'п', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 21237);
			match s {
				[] => true,
				['л', 'о', 'п', 'с', 'и', '\u{1}', s @ ..] => {
					super::push(out, 29904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', 'л', 'ш', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ковывать.
pub(crate) fn n15668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12308);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21238);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29905);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41584);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'с', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'б', 'у', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'ш', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'и', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'ч', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'ш', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'п', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'ф', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', 'т', 'ш', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', 'о', 'т', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -баловывать.
pub(crate) fn n49167(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обваловывать.
pub(crate) fn n49168(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -жаловывать.
pub(crate) fn n49169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29906);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 35922);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39601);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -целовывать.
pub(crate) fn n31089(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мовывать.
pub(crate) fn n15670(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', 'з', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'о', 'ф', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', 'и', 'р', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', 'л', 'е', 'ш', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новывать.
pub(crate) fn n15671(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', 'и', 'с', 'с', 'а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21239);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39602);
									match s {
										[] => true,
										['к', 'э', s @ ..] => {
											match s {
												[] => true,
												['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['д', s @ ..] => super::t29::n88608(s, out),
														['р', 'п', '\u{1}', s @ ..] => super::t29::n88609(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', 'з', s @ ..] => {
									match s {
										[] => true,
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'а', 'м', 'а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'е', 'д', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повывать.
pub(crate) fn n15672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12310);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21240);
			s.is_empty()
		}
		['е', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 21241);
			match s {
				[] => true,
				['а', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровывать.
pub(crate) fn n15673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12311);
	match s {
		[] => true,
		['а', 'ч', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'а', 'к', s @ ..] => {
			match s {
				[] => true,
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'р', 'ф', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['о', 'н', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'н', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['б', 'м', 'о', 'л', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'а', 'р', 'г', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'н', s @ ..] => {
					match s {
						[] => true,
						['а', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'м', 'б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'н', 'о', 'р', 'б', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'м', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', 'е', 'м', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
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
						['и', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'о', 'ф', s @ ..] => super::t29::n64722(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t29::n49206(s, out),
				['п', s @ ..] => super::t29::n49207(s, out),
				['с', s @ ..] => super::t29::n49208(s, out),
				['т', s @ ..] => super::t29::n49209(s, out),
				['ш', 'р', 'а', 'ф', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'в', s @ ..] => super::t29::n31107(s, out),
		['т', s @ ..] => super::t29::n31108(s, out),
		['у', s @ ..] => super::t29::n31109(s, out),
		['ф', 'и', s @ ..] => super::t29::n31110(s, out),
		_ => false,
	}
}

/// Words ending -совывать.
pub(crate) fn n15674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12312);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21242);
			match s {
				[] => true,
				['л', 'г', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
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
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['д', 'а', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'н', 'и', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21243);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21244);
			match s {
				[] => true,
				['е', 'л', 'п', 'м', 'о', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 29907);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35923);
							match s {
								[] => true,
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 21245);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['г', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39603);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41585);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
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
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 21246);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 35924);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39604);
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
		['ю', 'л', s @ ..] => {
			match s {
				[] => true,
				['п', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товывать.
pub(crate) fn n15675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12313);
	match s {
		[] => true,
		['и', 'д', 'е', 'р', 'к', 'к', 'а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', 'а', 'н', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', 'т', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'л', 'п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'о', 'б', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', 'у', 'р', 'т', 'с', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'а', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'п', 'ш', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['п', 'ш', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', 'в', 'ш', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'в', 'н', 'о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'п', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'а', 'р', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21247);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29908);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35925);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39605);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 35926);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 29909);
					match s {
						[] => true,
						['р', 'а', s @ ..] => super::t29::n64801(s, out),
						['т', 'т', 'а', s @ ..] => super::t29::n64802(s, out),
						_ => false,
					}
				}
				['и', 'л', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'е', 'т', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => super::t29::n31128(s, out),
		['ч', 'а', 'м', 'з', 'а', 'р', '\u{1}', s @ ..] => super::t29::n31129(s, out),
		_ => false,
	}
}

/// Words ending -фовывать.
pub(crate) fn n15676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12314);
	match s {
		[] => true,
		['а', 'р', 'т', 'ш', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховывать.
pub(crate) fn n15677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12315);
	match s {
		[] => true,
		['а', 'р', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29910);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'т', 'ш', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цовывать.
pub(crate) fn n15678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12316);
	match s {
		[] => true,
		['б', 'у', 'р', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'л', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 21248);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 35927);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39606);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 35928);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39607);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 35929);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'в', 'с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'г', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 21249);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									match s {
										[] => true,
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -рстывать.
pub(crate) fn n15870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12437);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устывать.
pub(crate) fn n15871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12438);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 21394);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 30016);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстывать.
pub(crate) fn n15872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12439);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21395);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21396);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35994);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 35995);
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

