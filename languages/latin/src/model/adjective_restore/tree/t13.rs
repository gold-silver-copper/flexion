use flexion::Path;

/// Words ending -rtaturus.
pub(crate) fn n15850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10456);
	match s {
		[] => true,
		['a', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 13921);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16273);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 16274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17536);
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
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13922);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 16275);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 16276);
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
				['p', s @ ..] => {
					super::push(out, 16277);
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
							super::push(out, 17537);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17985);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17986);
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
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 17538);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17987);
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

/// Words ending -staturus.
pub(crate) fn n15851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10457);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 13923);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17539);
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
		['b', s @ ..] => {
			super::push(out, 13924);
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
			super::push(out, 13925);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 16278);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17540);
							match s {
								[] => true,
								['n', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17541);
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
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16279);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 17542);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
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
						['u', s @ ..] => {
							super::push(out, 17543);
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
				['t', s @ ..] => {
					super::push(out, 16280);
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
							super::push(out, 17544);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17545);
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
						['o', s @ ..] => {
							super::push(out, 17546);
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
						_ => false,
					}
				}
				['u', 'q', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13926);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16283);
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
		['n', s @ ..] => {
			super::push(out, 13927);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16285);
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
		['u', s @ ..] => {
			super::push(out, 13928);
			match s {
				[] => true,
				['b', 'r', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17547);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17988);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17989);
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
						['n', 'a', s @ ..] => {
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
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
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
						['f', '\u{1}', s @ ..] => {
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

/// Words ending -utaturus.
pub(crate) fn n15852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10458);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 13929);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16286);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13930);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13931);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13932);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17548);
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
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13933);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16288);
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
					super::push(out, 16289);
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16290);
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
			super::push(out, 13934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16291);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17550);
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
					super::push(out, 16292);
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
				['s', s @ ..] => {
					super::push(out, 16293);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17551);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17552);
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
			super::push(out, 13936);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 13937);
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

/// The word mutuaturus.
pub(crate) fn n34330(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -omutuaturus.
pub(crate) fn n34331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17558);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -specturus.
pub(crate) fn n23371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13996);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 16351);
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
			super::push(out, 16352);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 17587);
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
			super::push(out, 16353);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 16354);
			match s {
				[] => true,
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16355);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17589);
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
			super::push(out, 16356);
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
			super::push(out, 16357);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 16358);
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

/// Words ending -recturus.
pub(crate) fn n15891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10496);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13997);
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
			super::push(out, 13998);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13999);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 16359);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
					super::push(out, 16360);
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
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17997);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 18085);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 18086);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -secturus.
pub(crate) fn n15892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10497);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 14000);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16362);
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
			super::push(out, 14001);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16364);
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
			super::push(out, 14002);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14003);
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

/// The word tecturus.
pub(crate) fn n23385(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word obtecturus.
pub(crate) fn n23386(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -etecturus.
pub(crate) fn n23387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14004);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 16365);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16366);
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

/// The word circumtecturus.
pub(crate) fn n23388(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ntecturus.
pub(crate) fn n23389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14005);
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

/// Words ending -otecturus.
pub(crate) fn n23390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14006);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vecturus.
pub(crate) fn n15894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10499);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 14007);
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
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 14008);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16367);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16368);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 16369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16370);
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
			super::push(out, 14009);
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
			super::push(out, 14010);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14011);
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
			super::push(out, 14012);
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

/// Words ending -instricturus.
pub(crate) fn n36375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17999);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onstricturus.
pub(crate) fn n36376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18000);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -victurus.
pub(crate) fn n15902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10507);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 14026);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 16377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16378);
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
			super::push(out, 14027);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14028);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14029);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nstructurus.
pub(crate) fn n34507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17599);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 18003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 18004);
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

/// Words ending -xtructurus.
pub(crate) fn n30108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16398);
	match s {
		[] => true,
		['e', s @ ..] => {
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

/// Words ending -sucturus.
pub(crate) fn n15912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10517);
	s.is_empty()
}

/// Words ending -triturus.
pub(crate) fn n15987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10576);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 14132);
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
		['d', s @ ..] => {
			super::push(out, 14133);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14134);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 14135);
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
			super::push(out, 14136);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 14137);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 14138);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17619);
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

/// Words ending -liguriturus.
pub(crate) fn n30311(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rsiturus.
pub(crate) fn n15996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10583);
	match s {
		[] => true,
		['e', 'c', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ssiturus.
pub(crate) fn n15997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10584);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14147);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word consideratissimus.
pub(crate) fn n36975(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inconsideratissimus.
pub(crate) fn n36976(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word consceleratissimus.
pub(crate) fn n36977(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -electissimus.
pub(crate) fn n35660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17821);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ilectissimus.
pub(crate) fn n35661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17822);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rectissimus.
pub(crate) fn n33286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17217);
	s.is_empty()
}

/// Words ending -ictissimus.
pub(crate) fn n28730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15783);
	s.is_empty()
}

/// Words ending -nctissimus.
pub(crate) fn n28731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15784);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17218);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17219);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 17220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -octissimus.
pub(crate) fn n28732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15785);
	s.is_empty()
}

/// Words ending -rctissimus.
pub(crate) fn n28733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15786);
	s.is_empty()
}

/// Words ending -uctissimus.
pub(crate) fn n28734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15787);
	s.is_empty()
}

/// Words ending -etissimus.
pub(crate) fn n22500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13455);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itissimus.
pub(crate) fn n22501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13456);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 15789);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 15790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17823);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 17824);
							match s {
								[] => true,
								['m', 'i', '\u{1}', s @ ..] => {
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
					super::push(out, 17222);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17223);
					match s {
						[] => true,
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15791);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 15792);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 17224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15793);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
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
			super::push(out, 15794);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17225);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 15795);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17227);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltissimus.
pub(crate) fn n22502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13457);
	match s {
		[] => true,
		['u', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ntissimus.
pub(crate) fn n22503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13458);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15796);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 17229);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 17230);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17825);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17231);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17232);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 18038);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 18039);
									match s {
										[] => true,
										['m', 'e', 't', s @ ..] => {
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
					super::push(out, 17233);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17827);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17828);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 18040);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 18041);
									s.is_empty()
								}
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
			super::push(out, 15797);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17234);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17829);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 18042);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 18043);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 18093);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 18094);
													s.is_empty()
												}
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
							super::push(out, 17830);
							match s {
								[] => true,
								['n', s @ ..] => {
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
				['d', s @ ..] => {
					super::push(out, 17235);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17831);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 18044);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 18045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 17832);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 18046);
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
								['r', s @ ..] => {
									super::push(out, 18047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 17236);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'g', 'e', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 17834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17237);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17835);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 18048);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 18049);
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
								['a', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17238);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17836);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17837);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17838);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 18050);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 18051);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 18052);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 18053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17239);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17839);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17840);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 18054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17240);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17841);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17241);
					match s {
						[] => true,
						['o', 'p', s @ ..] => super::t13::n35688(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17242);
					match s {
						[] => true,
						['q', s @ ..] => super::t13::n35689(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otissimus.
pub(crate) fn n22504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13459);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 15798);
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
		['p', s @ ..] => {
			super::push(out, 15799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptissimus.
pub(crate) fn n22505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13460);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 15800);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17243);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtissimus.
pub(crate) fn n22506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13461);
	match s {
		[] => true,
		['e', s @ ..] => {
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
				['f', s @ ..] => {
					super::push(out, 17245);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17843);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17246);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17845);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stissimus.
pub(crate) fn n22507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13462);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15801);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17247);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 17248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15802);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 17249);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17847);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17250);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17251);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 17849);
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
						['u', s @ ..] => {
							super::push(out, 17850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 15803);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 15804);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17252);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 17253);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17254);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utissimus.
pub(crate) fn n22508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13463);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15805);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 15806);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15807);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17255);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17256);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 18058);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15808);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 15809);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 15810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ficaturus.
pub(crate) fn n22847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13596);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17290);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17857);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 17291);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17859);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 17292);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17861);
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
							super::push(out, 17862);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17293);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17294);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17864);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17865);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17866);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 17867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 17295);
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
				['p', s @ ..] => {
					super::push(out, 17296);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17297);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17868);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 17869);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17870);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17871);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17872);
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
						['r', s @ ..] => {
							super::push(out, 17873);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17298);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17299);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17875);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 17876);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 18064);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 18065);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 18066);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 18088);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 18089);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 17877);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17878);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17879);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 18067);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 18068);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 18069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 17300);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17880);
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
							super::push(out, 17881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -plicaturus.
pub(crate) fn n29050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15898);
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
			super::push(out, 17301);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 17882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17883);
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
			super::push(out, 17302);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17884);
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
			super::push(out, 17303);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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
			super::push(out, 17304);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17885);
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

/// Words ending -rnicaturus.
pub(crate) fn n29055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15903);
	s.is_empty()
}

/// Words ending -unicaturus.
pub(crate) fn n29056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15904);
	s.is_empty()
}

/// Words ending -picaturus.
pub(crate) fn n22851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13600);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17307);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ricaturus.
pub(crate) fn n22852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13601);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15905);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17309);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17886);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 15906);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17310);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17311);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17888);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 15907);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17312);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17891);
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
					super::push(out, 17313);
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
					super::push(out, 17314);
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
			super::push(out, 15908);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 15909);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 15910);
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
			super::push(out, 15911);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17315);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicaturus.
pub(crate) fn n22853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13602);
	s.is_empty()
}

/// Words ending -ticaturus.
pub(crate) fn n22854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13603);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 15912);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 15913);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 15914);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17317);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vicaturus.
pub(crate) fn n22855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13604);
	s.is_empty()
}

/// The word abrenuntiaturus.
pub(crate) fn n36982(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -simulaturus.
pub(crate) fn n29364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 17378);
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

/// Words ending -cumulaturus.
pub(crate) fn n29365(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -pulaturus.
pub(crate) fn n23034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13734);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16052);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16053);
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
			super::push(out, 16054);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17379);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 17380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17914);
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

/// Words ending -sulaturus.
pub(crate) fn n23035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13735);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 16055);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 16056);
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

/// Words ending -tulaturus.
pub(crate) fn n23036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13736);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16057);
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
			super::push(out, 16058);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 16059);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 17381);
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
							super::push(out, 17915);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 18074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 17916);
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

/// The word adtaminaturus.
pub(crate) fn n36000(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word contaminaturus.
pub(crate) fn n36001(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word attaminaturus.
pub(crate) fn n36002(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word examinaturus.
pub(crate) fn n33859(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -eminaturus.
pub(crate) fn n29443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16091);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17936);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 17937);
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
		['g', s @ ..] => {
			super::push(out, 17416);
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
			super::push(out, 17417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17938);
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

/// Words ending -iminaturus.
pub(crate) fn n29444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16092);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 17418);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17419);
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

/// Words ending -lminaturus.
pub(crate) fn n29445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16093);
	match s {
		[] => true,
		['u', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mminaturus.
pub(crate) fn n29446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16094);
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ominaturus.
pub(crate) fn n29447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16095);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 17420);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 17421);
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
			super::push(out, 17422);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17939);
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

/// Words ending -rminaturus.
pub(crate) fn n29448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16096);
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
							super::push(out, 18075);
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

/// Words ending -uminaturus.
pub(crate) fn n29449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16097);
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

/// Words ending -pinaturus.
pub(crate) fn n23093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13782);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16098);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 16099);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 16100);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16101);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17423);
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
			super::push(out, 16102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rinaturus.
pub(crate) fn n23094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13783);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 16103);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 16104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tinaturus.
pub(crate) fn n23095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13784);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16105);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 16106);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 16107);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17424);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 17940);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17425);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18076);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 18077);
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
							super::push(out, 17943);
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
			super::push(out, 16108);
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

/// Words ending -uinaturus.
pub(crate) fn n23096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13785);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17426);
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
					super::push(out, 17427);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vinaturus.
pub(crate) fn n23097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13786);
	s.is_empty()
}

/// Words ending -nsceleraturus.
pub(crate) fn n36076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17949);
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -meraturus.
pub(crate) fn n23146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13820);
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
					super::push(out, 17460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17461);
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

/// Words ending -neraturus.
pub(crate) fn n23147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13821);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16142);
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
							super::push(out, 17950);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 18079);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 18080);
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
			super::push(out, 16143);
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
			super::push(out, 16144);
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

/// Words ending -peraturus.
pub(crate) fn n23148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13822);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 16145);
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
			super::push(out, 16146);
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
			super::push(out, 16147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17462);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 17463);
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
					super::push(out, 17464);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17465);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17466);
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
			super::push(out, 16148);
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

/// Words ending -seraturus.
pub(crate) fn n23149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13823);
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

/// Words ending -teraturus.
pub(crate) fn n23150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13824);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16149);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17951);
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

/// Words ending -ueraturus.
pub(crate) fn n23151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13825);
	s.is_empty()
}

/// Words ending -veraturus.
pub(crate) fn n23152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13826);
	s.is_empty()
}

/// Words ending -roraturus.
pub(crate) fn n23173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13845);
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

/// Words ending -toraturus.
pub(crate) fn n23174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13846);
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

/// Words ending -voraturus.
pub(crate) fn n23175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13847);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16166);
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

/// The word exoraturus.
pub(crate) fn n29600(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word demonstraturus.
pub(crate) fn n36985(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word exagitaturus.
pub(crate) fn n36243(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -igitaturus.
pub(crate) fn n29734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16228);
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

/// Words ending -ogitaturus.
pub(crate) fn n29735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16229);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 17517);
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
			super::push(out, 17518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rgitaturus.
pub(crate) fn n29736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16230);
	s.is_empty()
}

/// Words ending -ugitaturus.
pub(crate) fn n29737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16231);
	s.is_empty()
}

/// Words ending -litaturus.
pub(crate) fn n23236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13900);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 16232);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16233);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16234);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17519);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17979);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 17980);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17981);
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
					super::push(out, 17520);
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
			super::push(out, 16235);
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

/// Words ending -mitaturus.
pub(crate) fn n23237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13901);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16236);
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
									super::push(out, 18083);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 18084);
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
			super::push(out, 16237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17521);
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
			super::push(out, 16238);
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
			super::push(out, 16239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nitaturus.
pub(crate) fn n23238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13902);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16240);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16241);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 16242);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pitaturus.
pub(crate) fn n23239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13903);
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

/// Words ending -ritaturus.
pub(crate) fn n23240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13904);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16243);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16244);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 16245);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16246);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17522);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17523);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sitaturus.
pub(crate) fn n23241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13905);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16247);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16248);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 16249);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16250);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 16251);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 16252);
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

/// Words ending -titaturus.
pub(crate) fn n23242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13906);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 16253);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17982);
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
					super::push(out, 17525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16254);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uitaturus.
pub(crate) fn n23243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13907);
	match s {
		[] => true,
		['q', 'e', s @ ..] => {
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vitaturus.
pub(crate) fn n23244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13908);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 16256);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16257);
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
			super::push(out, 16258);
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

/// Words ending -potentissimus.
pub(crate) fn n35688(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 18055);
			match s {
				[] => true,
				['n', 'm', 'o', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -quentissimus.
pub(crate) fn n35689(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 18056);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18057);
			s.is_empty()
		}
		_ => false,
	}
}

