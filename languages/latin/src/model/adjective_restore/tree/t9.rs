use flexion::Path;

/// Words ending -viatus.
pub(crate) fn n4516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3763);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7162);
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
		['e', s @ ..] => {
			super::push(out, 7163);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10996);
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
					super::push(out, 10997);
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

/// Words ending -ilatus.
pub(crate) fn n4524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3769);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7181);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11004);
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
				['u', s @ ..] => {
					super::push(out, 11005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7182);
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
		['d', s @ ..] => {
			super::push(out, 7183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7184);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7185);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14405);
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
		['h', s @ ..] => {
			super::push(out, 7186);
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
		['m', s @ ..] => {
			super::push(out, 7187);
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
		['p', s @ ..] => {
			super::push(out, 7188);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11006);
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
		['t', s @ ..] => {
			super::push(out, 7189);
			match s {
				[] => true,
				['n', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11007);
					match s {
						[] => true,
						['u', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 14406);
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
		_ => false,
	}
}

/// Words ending -llatus.
pub(crate) fn n4525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3770);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11008);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 11009);
					match s {
						[] => true,
						['c', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 11010);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7191);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14407);
							match s {
								[] => true,
								['l', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14408);
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
				['c', s @ ..] => {
					super::push(out, 11012);
					match s {
						[] => true,
						['i', 'd', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', 'e', 'r', 'b', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 11013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 14409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11015);
					match s {
						[] => true,
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 11016);
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
				['t', s @ ..] => {
					super::push(out, 11017);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14410);
							s.is_empty()
						}
						['u', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 14411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 11018);
					match s {
						[] => true,
						['a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 14412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 11019);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14413);
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
					super::push(out, 11020);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 16613);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 17669);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 's', 'n', s @ ..] => {
									super::push(out, 16614);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 17670);
											s.is_empty()
										}
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
					super::push(out, 11021);
					match s {
						[] => true,
						['g', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14414);
							match s {
								[] => true,
								['s', s @ ..] => {
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
							super::push(out, 14415);
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
				['m', s @ ..] => {
					super::push(out, 11022);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11023);
					match s {
						[] => true,
						['a', s @ ..] => {
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
										['i', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11024);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14416);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14417);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16615);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => super::t12::n30885(s, out),
								['n', s @ ..] => super::t12::n30886(s, out),
								['r', s @ ..] => super::t12::n30887(s, out),
								['x', s @ ..] => super::t12::n30888(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'a', 'c', '\u{1}', s @ ..] => super::t12::n16924(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t12::n9834(s, out),
		['u', s @ ..] => super::t12::n9835(s, out),
		_ => false,
	}
}

/// Words ending -olatus.
pub(crate) fn n4528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3771);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11026);
					match s {
						[] => true,
						['r', 'g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11027);
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
			super::push(out, 7196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11028);
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
		['e', s @ ..] => {
			super::push(out, 7197);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11029);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 14420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7198);
			match s {
				[] => true,
				['r', s @ ..] => {
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
						['b', s @ ..] => {
							super::push(out, 14421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'm', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7199);
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
			super::push(out, 7200);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11030);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11031);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11032);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 11033);
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
			super::push(out, 7201);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 7202);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11034);
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
					super::push(out, 11035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14422);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14423);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14424);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14425);
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
					super::push(out, 11036);
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
					super::push(out, 11037);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11038);
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
					super::push(out, 11039);
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

/// Words ending -ulatus.
pub(crate) fn n4532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3775);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11040);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 14426);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16619);
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
							super::push(out, 14427);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14428);
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
					super::push(out, 11041);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16620);
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
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11042);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
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
				['u', s @ ..] => {
					super::push(out, 11043);
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
		['c', s @ ..] => {
			super::push(out, 7204);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11044);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16621);
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
							super::push(out, 14430);
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
										['c', 'u', 'a', 'p', '\u{1}', s @ ..] => {
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
								['o', s @ ..] => {
									match s {
										[] => true,
										['b', 'l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'g', 'i', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 17671);
											match s {
												[] => true,
												['a', 'l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 18007);
													s.is_empty()
												}
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
						['n', s @ ..] => {
							super::push(out, 14431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t12::n16962(s, out),
				['i', s @ ..] => super::t12::n16963(s, out),
				['l', 'a', 'c', '\u{1}', s @ ..] => super::t12::n16964(s, out),
				['o', s @ ..] => super::t12::n16965(s, out),
				['r', s @ ..] => super::t12::n16966(s, out),
				['s', s @ ..] => super::t12::n16967(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t12::n9853(s, out),
		['g', s @ ..] => super::t12::n9854(s, out),
		['i', s @ ..] => super::t12::n9855(s, out),
		['l', s @ ..] => super::t12::n9856(s, out),
		['m', s @ ..] => super::t12::n9857(s, out),
		['n', s @ ..] => super::t12::n9858(s, out),
		['p', s @ ..] => super::t12::n9859(s, out),
		['r', 'r', 'e', 's', s @ ..] => super::t12::n9860(s, out),
		['s', s @ ..] => super::t12::n9861(s, out),
		['t', s @ ..] => super::t12::n9862(s, out),
		['v', s @ ..] => super::t12::n9863(s, out),
		['x', 'e', '\u{1}', s @ ..] => super::t12::n9864(s, out),
		_ => false,
	}
}

/// Words ending -crimatus.
pub(crate) fn n17012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11076);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -simatus.
pub(crate) fn n9881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7231);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11077);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -timatus.
pub(crate) fn n9882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7232);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11079);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11080);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14454);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14455);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ximatus.
pub(crate) fn n9883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7233);
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

/// Words ending -lmatus.
pub(crate) fn n4537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3779);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmatus.
pub(crate) fn n4538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3780);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7234);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14456);
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
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7235);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7236);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11081);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11082);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -omatus.
pub(crate) fn n4539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3781);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'u', s @ ..] => {
			super::push(out, 7237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmatus.
pub(crate) fn n4540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3782);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7239);
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
							super::push(out, 14457);
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
							super::push(out, 14458);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16631);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16632);
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
			super::push(out, 7240);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14459);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16633);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16634);
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
							super::push(out, 14460);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14461);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16635);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16636);
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
							super::push(out, 14462);
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
					super::push(out, 11083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -smatus.
pub(crate) fn n4541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3783);
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

/// Words ending -umatus.
pub(crate) fn n4542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3784);
	match s {
		[] => true,
		['c', 'e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11084);
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
			super::push(out, 7242);
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
		['l', s @ ..] => {
			super::push(out, 7243);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7244);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14463);
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
		['r', 'r', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gnatus.
pub(crate) fn n4546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3787);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7254);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7255);
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
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
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
							super::push(out, 14469);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16639);
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
						['i', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14470);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16640);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16641);
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
				['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7256);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11094);
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
			super::push(out, 7257);
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
							super::push(out, 14471);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16642);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16643);
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
							super::push(out, 14472);
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
		_ => false,
	}
}

/// Words ending -inatus.
pub(crate) fn n4547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3788);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7258);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7259);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11096);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 11097);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11098);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14473);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14474);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 14475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11099);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14476);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11100);
					match s {
						[] => true,
						['i', 't', 'a', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11101);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16644);
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
				['s', s @ ..] => {
					super::push(out, 11102);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11103);
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
			super::push(out, 7260);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11104);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14478);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11105);
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
								['n', 'i', '\u{1}', s @ ..] => {
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
			super::push(out, 7261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11106);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14480);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14481);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 14483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11107);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 16645);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 16646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11108);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14484);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17673);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 17674);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 18008);
													match s {
														[] => true,
														['l', 'a', '\u{1}', s @ ..] => super::t12::n36903(s, out),
														_ => false,
													}
												}
												['e', s @ ..] => {
													super::push(out, 18009);
													s.is_empty()
												}
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
							super::push(out, 14485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7262);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14486);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7263);
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
							super::push(out, 14488);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16647);
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
								['r', s @ ..] => {
									super::push(out, 16648);
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
							super::push(out, 14489);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => super::t12::n31032(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t12::n17082(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t12::n9928(s, out),
		['p', s @ ..] => super::t12::n9929(s, out),
		['r', s @ ..] => super::t12::n9930(s, out),
		['t', s @ ..] => super::t12::n9931(s, out),
		['u', s @ ..] => super::t12::n9932(s, out),
		['v', s @ ..] => super::t12::n9933(s, out),
		_ => false,
	}
}

/// Words ending -innatus.
pub(crate) fn n9936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7271);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => {
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

/// Words ending -onatus.
pub(crate) fn n4550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3791);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7272);
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
			super::push(out, 7273);
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
		['e', s @ ..] => {
			super::push(out, 7274);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 11130);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7275);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11131);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11132);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14507);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14508);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14509);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7276);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7277);
			match s {
				[] => true,
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7278);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11133);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11134);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11135);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11136);
					match s {
						[] => true,
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
		['s', s @ ..] => {
			super::push(out, 7279);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11137);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11138);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnatus.
pub(crate) fn n4551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3792);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7281);
			match s {
				[] => true,
				['c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7282);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11140);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14511);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14512);
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
					super::push(out, 11141);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 14513);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11142);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14515);
							match s {
								[] => true,
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
		['o', s @ ..] => {
			super::push(out, 7283);
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
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11143);
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
		['u', s @ ..] => {
			super::push(out, 7284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unatus.
pub(crate) fn n4552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3793);
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
		['l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7285);
			match s {
				[] => true,
				['r', 'o', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14516);
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

/// Words ending -obratus.
pub(crate) fn n9973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7297);
	match s {
		[] => true,
		['r', 'p', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 14524);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 14525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ubratus.
pub(crate) fn n9974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7298);
	match s {
		[] => true,
		['c', 'u', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14526);
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

/// Words ending -cratus.
pub(crate) fn n4563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3804);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7299);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14527);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14528);
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
			super::push(out, 7300);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 11155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7301);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7302);
			match s {
				[] => true,
				['l', s @ ..] => {
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

/// Words ending -eratus.
pub(crate) fn n4565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3806);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7303);
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
		['b', s @ ..] => {
			super::push(out, 7304);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11156);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16658);
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
					super::push(out, 11157);
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
									super::push(out, 16659);
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
					super::push(out, 11158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14529);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 14530);
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
			super::push(out, 7305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11159);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14531);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 14532);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 14533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 16660);
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
							super::push(out, 14534);
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
					super::push(out, 11161);
					match s {
						[] => true,
						['u', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7306);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11163);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16661);
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
					super::push(out, 11164);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16662);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16663);
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
				['n', s @ ..] => {
					super::push(out, 11165);
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
					super::push(out, 11166);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 14535);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14536);
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
			super::push(out, 7307);
			match s {
				[] => true,
				['c', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7308);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11168);
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
					super::push(out, 11169);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14537);
							s.is_empty()
						}
						['m', s @ ..] => super::t12::n24770(s, out),
						['r', s @ ..] => super::t12::n24771(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n9986(s, out),
		['l', s @ ..] => super::t12::n9987(s, out),
		['m', s @ ..] => super::t12::n9988(s, out),
		['n', s @ ..] => super::t12::n9989(s, out),
		['p', s @ ..] => super::t12::n9990(s, out),
		['s', s @ ..] => super::t12::n9991(s, out),
		['t', s @ ..] => super::t12::n9992(s, out),
		['u', s @ ..] => super::t12::n9993(s, out),
		['v', s @ ..] => super::t12::n9994(s, out),
		_ => false,
	}
}

/// Words ending -gratus.
pub(crate) fn n4566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3807);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7318);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 11183);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16670);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16671);
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
					super::push(out, 11184);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14550);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 14551);
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
			super::push(out, 7319);
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
			super::push(out, 7320);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 16672);
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
							super::push(out, 14553);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14554);
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
					super::push(out, 11186);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11187);
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
			super::push(out, 7321);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7322);
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

/// Words ending -hratus.
pub(crate) fn n4567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3808);
	s.is_empty()
}

/// Words ending -iratus.
pub(crate) fn n4568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3809);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11188);
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
			super::push(out, 7324);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
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
							super::push(out, 14555);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16673);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16674);
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
			super::push(out, 7325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 7326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11189);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11190);
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

/// Words ending -oratus.
pub(crate) fn n4569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3810);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 7327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11191);
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
									super::push(out, 16675);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 16676);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11192);
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
			super::push(out, 7328);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11193);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16677);
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
					super::push(out, 11194);
					match s {
						[] => true,
						['u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11195);
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
			super::push(out, 7329);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7330);
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
			super::push(out, 7331);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7332);
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
			super::push(out, 7333);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11196);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16678);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				['p', s @ ..] => {
					super::push(out, 11197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14556);
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
			super::push(out, 7334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11198);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14558);
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
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11199);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 14559);
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
				['r', 'a', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7335);
			match s {
				[] => true,
				['g', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => super::t12::n24857(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t12::n24858(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t12::n10018(s, out),
		['r', s @ ..] => super::t12::n10019(s, out),
		['t', s @ ..] => super::t12::n10020(s, out),
		['v', s @ ..] => super::t12::n10021(s, out),
		['x', s @ ..] => super::t12::n10022(s, out),
		_ => false,
	}
}

/// Words ending -pratus.
pub(crate) fn n4570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3811);
	match s {
		[] => true,
		['u', 't', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rratus.
pub(crate) fn n4571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3812);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7341);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14560);
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
			super::push(out, 7342);
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
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7343);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 11206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7344);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14561);
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

/// Words ending -tratus.
pub(crate) fn n4572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3813);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7345);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11207);
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
					super::push(out, 11208);
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
			super::push(out, 7346);
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
			super::push(out, 7347);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14562);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14563);
							match s {
								[] => true,
								['a', 'c', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 7348);
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
			super::push(out, 7349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11209);
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
					super::push(out, 11210);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 11211);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11212);
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
					super::push(out, 11213);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14566);
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
													super::push(out, 18010);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n36907(s, out),
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
					super::push(out, 11214);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11215);
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

/// Words ending -uratus.
pub(crate) fn n4573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3814);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11216);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16680);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16681);
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
		['b', s @ ..] => {
			super::push(out, 7351);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 7352);
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11217);
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
			super::push(out, 7353);
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
			super::push(out, 7354);
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
			super::push(out, 7355);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11218);
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
									super::push(out, 16682);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 16683);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16684);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 16685);
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
					super::push(out, 11219);
					match s {
						[] => true,
						['u', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11220);
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
			super::push(out, 7356);
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
					super::push(out, 11221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14567);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14569);
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
				['n', s @ ..] => {
					super::push(out, 11222);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11223);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11224);
					match s {
						[] => true,
						['u', 'm', s @ ..] => {
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
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7358);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11225);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11226);
					match s {
						[] => true,
						['u', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7359);
			match s {
				[] => true,
				['n', 'e', 'm', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 7360);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11227);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14570);
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
							super::push(out, 14571);
							s.is_empty()
						}
						['s', s @ ..] => super::t12::n24910(s, out),
						_ => false,
					}
				}
				['b', s @ ..] => super::t12::n17337(s, out),
				['c', s @ ..] => {
					super::push(out, 11229);
					s.is_empty()
				}
				['i', s @ ..] => super::t12::n17339(s, out),
				['p', s @ ..] => super::t12::n17340(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yratus.
pub(crate) fn n4574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3815);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7361);
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
		['l', '\u{1}', s @ ..] => {
			super::push(out, 7362);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word desponsatus.
pub(crate) fn n36493(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ssatus.
pub(crate) fn n4583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3824);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7368);
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
		['e', 'c', 'a', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7369);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11239);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11240);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7370);
			match s {
				[] => true,
				['c', 'c', s @ ..] => {
					super::push(out, 11242);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 14578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14579);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ausatus.
pub(crate) fn n10061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7371);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cusatus.
pub(crate) fn n10062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7372);
	match s {
		[] => true,
		['c', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
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

/// The word obtusatus.
pub(crate) fn n10063(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ectatus.
pub(crate) fn n10067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7376);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11251);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14582);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14583);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11252);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14585);
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
					super::push(out, 14586);
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
					super::push(out, 14587);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11253);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16689);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 16690);
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
			super::push(out, 11254);
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
							super::push(out, 16691);
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
							super::push(out, 16692);
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
						['x', s @ ..] => {
							super::push(out, 16693);
							match s {
								[] => true,
								['e', s @ ..] => {
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
				['x', 'e', s @ ..] => {
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
			super::push(out, 11255);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 14589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11256);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14590);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16695);
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
			super::push(out, 11257);
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

/// Words ending -nctatus.
pub(crate) fn n10069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7378);
	match s {
		[] => true,
		['u', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
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
						['m', s @ ..] => {
							super::push(out, 16698);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 17683);
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
						['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 16699);
							match s {
								[] => true,
								['a', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 17684);
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
				['o', 'r', 'g', 'i', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rctatus.
pub(crate) fn n10071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7380);
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
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uctatus.
pub(crate) fn n10072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7381);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11261);
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
			super::push(out, 11262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16700);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16701);
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
			super::push(out, 11263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14593);
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

/// Words ending -etatus.
pub(crate) fn n4587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3828);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7382);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 7383);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7384);
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
			super::push(out, 7385);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7386);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11264);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11265);
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
			super::push(out, 7387);
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
			super::push(out, 7388);
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
			super::push(out, 7389);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11266);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 11267);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7390);
			s.is_empty()
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 7391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itatus.
pub(crate) fn n4588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3829);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7392);
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
		['c', s @ ..] => {
			super::push(out, 7393);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11269);
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
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'x', 'e', s @ ..] => {
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
					super::push(out, 11270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14594);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14595);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14596);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16702);
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
							super::push(out, 14598);
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
			super::push(out, 7394);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11271);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11272);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14599);
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
							super::push(out, 14600);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14601);
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
					super::push(out, 11273);
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
			super::push(out, 7395);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14602);
							match s {
								[] => true,
								['f', s @ ..] => super::t12::n31319(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t12::n25023(s, out),
						['x', s @ ..] => super::t12::n25024(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t12::n17425(s, out),
				['o', s @ ..] => super::t12::n17426(s, out),
				['r', s @ ..] => {
					super::push(out, 11277);
					s.is_empty()
				}
				['u', s @ ..] => super::t12::n17428(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t12::n10088(s, out),
		['m', s @ ..] => super::t12::n10089(s, out),
		['n', s @ ..] => super::t12::n10090(s, out),
		['p', s @ ..] => super::t12::n10091(s, out),
		['r', s @ ..] => super::t12::n10092(s, out),
		['s', s @ ..] => super::t12::n10093(s, out),
		['t', s @ ..] => super::t12::n10094(s, out),
		['u', s @ ..] => super::t12::n10095(s, out),
		['v', s @ ..] => super::t12::n10096(s, out),
		_ => false,
	}
}

/// Words ending -ltatus.
pub(crate) fn n4589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3830);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7405);
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
			super::push(out, 7406);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11308);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14615);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntatus.
pub(crate) fn n4590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3831);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7407);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11309);
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
									super::push(out, 16707);
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
					super::push(out, 11310);
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
									super::push(out, 16708);
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
			super::push(out, 7408);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16709);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											super::push(out, 17687);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 17688);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 18011);
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['l', 'u', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 17689);
											match s {
												[] => true,
												['c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 18012);
													s.is_empty()
												}
												_ => false,
											}
										}
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
				['g', s @ ..] => {
					super::push(out, 11312);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 11313);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11314);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14617);
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
							super::push(out, 14618);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14619);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14620);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14621);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14622);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 11315);
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
					super::push(out, 11316);
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
					super::push(out, 11317);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otatus.
pub(crate) fn n4591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3832);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7409);
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
		['m', s @ ..] => {
			super::push(out, 7410);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7411);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14624);
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
						_ => false,
					}
				}
				['i', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14625);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14626);
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
			super::push(out, 7413);
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

/// Words ending -ptatus.
pub(crate) fn n4592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3833);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7414);
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
			super::push(out, 7415);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11319);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11320);
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
				['n', 'i', '\u{1}', s @ ..] => {
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
			super::push(out, 7416);
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

/// Words ending -rtatus.
pub(crate) fn n4593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3834);
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
			super::push(out, 7417);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11321);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 11322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14627);
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
			super::push(out, 7418);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11323);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 11324);
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
					super::push(out, 11325);
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
							super::push(out, 14628);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16711);
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
							super::push(out, 14629);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16712);
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

/// Words ending -status.
pub(crate) fn n4594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3835);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7419);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7420);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11326);
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
				['v', s @ ..] => {
					super::push(out, 11327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14630);
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
			super::push(out, 7421);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 11328);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14631);
							match s {
								[] => true,
								['n', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14632);
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
					super::push(out, 11329);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14633);
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
							super::push(out, 14634);
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
					super::push(out, 11330);
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
							super::push(out, 14635);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14636);
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
						['o', s @ ..] => {
							super::push(out, 14637);
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
			super::push(out, 7422);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14638);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14639);
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
			super::push(out, 7423);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7424);
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
							super::push(out, 14640);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 16713);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 16714);
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
								['i', '\u{1}', s @ ..] => {
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

/// Words ending -ttatus.
pub(crate) fn n4595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3836);
	match s {
		[] => true,
		['i', 'v', s @ ..] => {
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
						['n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'v', s @ ..] => {
					super::push(out, 11331);
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 14641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utatus.
pub(crate) fn n4596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3837);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7425);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7426);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11332);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7427);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7428);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 7429);
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
							super::push(out, 14642);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16715);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16716);
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
		['m', s @ ..] => {
			super::push(out, 7430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11334);
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
					super::push(out, 11335);
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
					super::push(out, 11336);
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
			super::push(out, 7431);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11337);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14644);
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
					super::push(out, 11338);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11339);
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
					super::push(out, 11340);
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
					super::push(out, 11341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14645);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14646);
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
			super::push(out, 7433);
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
			super::push(out, 7434);
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

/// Words ending -aestuatus.
pub(crate) fn n17573(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -utuatus.
pub(crate) fn n10149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7448);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14653);
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

/// Words ending -vefactus.
pub(crate) fn n17626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11379);
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

/// Words ending -ifactus.
pub(crate) fn n10187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7474);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 11380);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11381);
			match s {
				[] => true,
				['c', 'u', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lfactus.
pub(crate) fn n10188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7475);
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

/// The word confactus.
pub(crate) fn n17631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11382);
	s.is_empty()
}

/// The word satisfactus.
pub(crate) fn n10190(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word manufactus.
pub(crate) fn n10191(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nlectus.
pub(crate) fn n10230(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			super::push(out, 11406);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mectus.
pub(crate) fn n4634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3874);
	match s {
		[] => true,
		['u', s @ ..] => {
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

/// Words ending -nectus.
pub(crate) fn n4635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3875);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11407);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11408);
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

/// Words ending -pectus.
pub(crate) fn n4636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3876);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7498);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7499);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11409);
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
					super::push(out, 11410);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14686);
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
					super::push(out, 11411);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11412);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11413);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14687);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14688);
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
					super::push(out, 11414);
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
					super::push(out, 11415);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11416);
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
		_ => false,
	}
}

/// Words ending -rectus.
pub(crate) fn n4637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3877);
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
			super::push(out, 7501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11417);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 14689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7502);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7503);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11418);
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
				['o', s @ ..] => {
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
					super::push(out, 11419);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16730);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16731);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 16732);
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
							super::push(out, 14690);
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

/// Words ending -sectus.
pub(crate) fn n4638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3878);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7504);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11421);
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
			super::push(out, 7505);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11423);
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
			super::push(out, 7506);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7507);
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

/// Words ending -tectus.
pub(crate) fn n4639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3879);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7508);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11425);
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
			super::push(out, 7509);
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
			super::push(out, 7510);
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

/// Words ending -vectus.
pub(crate) fn n4640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3880);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7511);
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
			super::push(out, 7512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11426);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11427);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 11428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11429);
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
			super::push(out, 7513);
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
			super::push(out, 7514);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7515);
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
			super::push(out, 7516);
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

/// Words ending -trictus.
pub(crate) fn n10295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7531);
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
					super::push(out, 14694);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 16733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16734);
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
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14695);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16736);
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

/// Words ending -evictus.
pub(crate) fn n10297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11439);
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

/// Words ending -nvictus.
pub(crate) fn n10298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7533);
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

/// Words ending -ovictus.
pub(crate) fn n10299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7534);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rvictus.
pub(crate) fn n10300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7535);
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

/// The word seiunctus.
pub(crate) fn n25372(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -iiunctus.
pub(crate) fn n17791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11450);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 14700);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -niunctus.
pub(crate) fn n17792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11451);
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

/// The word interiunctus.
pub(crate) fn n17793(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word disiunctus.
pub(crate) fn n17794(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -munctus.
pub(crate) fn n10316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7545);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word inunctus.
pub(crate) fn n10317(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -punctus.
pub(crate) fn n10318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7546);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'b', 'l', 'a', '\u{1}', s @ ..] => {
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

/// The word perunctus.
pub(crate) fn n10319(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word constructus.
pub(crate) fn n35086(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word superstructus.
pub(crate) fn n25395(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word exstructus.
pub(crate) fn n25396(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -xtructus.
pub(crate) fn n17833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11462);
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

/// Words ending -suctus.
pub(crate) fn n4658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3897);
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

/// The word permonitus.
pub(crate) fn n17985(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ronitus.
pub(crate) fn n10474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7642);
	s.is_empty()
}

/// Words ending -tonitus.
pub(crate) fn n10475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7643);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11508);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unitus.
pub(crate) fn n4724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3951);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14720);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14721);
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
				['m', s @ ..] => {
					super::push(out, 11511);
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
				['n', 'i', '\u{1}', s @ ..] => {
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
			super::push(out, 7645);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14722);
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

/// Words ending -guritus.
pub(crate) fn n10508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7666);
	match s {
		[] => true,
		['i', 'l', s @ ..] => {
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

/// The word decompositus.
pub(crate) fn n36505(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word recompositus.
pub(crate) fn n36506(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -opositus.
pub(crate) fn n18045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11526);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ppositus.
pub(crate) fn n18046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11527);
	match s {
		[] => true,
		['a', s @ ..] => {
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
		['o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 's', s @ ..] => {
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

/// Words ending -erpositus.
pub(crate) fn n18047(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -spositus.
pub(crate) fn n18048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11528);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 14735);
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
		['n', s @ ..] => {
			super::push(out, 14736);
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

/// The word expositus.
pub(crate) fn n18050(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rsitus.
pub(crate) fn n4742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3967);
	match s {
		[] => true,
		['e', 'c', 'r', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssitus.
pub(crate) fn n4743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3968);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7675);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
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
								_ => false,
							}
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
					super::push(out, 11529);
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

/// Words ending -nvestitus.
pub(crate) fn n25576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14742);
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

/// The word supervestitus.
pub(crate) fn n25577(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ostitus.
pub(crate) fn n10536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7685);
	s.is_empty()
}

/// The word iudicativus.
pub(crate) fn n36507(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word abiudicativus.
pub(crate) fn n36508(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -iativus.
pub(crate) fn n10913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7810);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 11652);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lativus.
pub(crate) fn n10914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7811);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14792);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11655);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11656);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mativus.
pub(crate) fn n10915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7812);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14793);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nativus.
pub(crate) fn n10916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7813);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11658);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14795);
					match s {
						[] => true,
						['o', 'i', 't', 'a', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 14796);
					match s {
						[] => true,
						['a', 'm', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14797);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 16764);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16765);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 18013);
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
							super::push(out, 16766);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 11659);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rativus.
pub(crate) fn n10918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7815);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11660);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14798);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 11661);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11662);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14800);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 14801);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14802);
					match s {
						[] => true,
						['o', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11663);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11664);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16767);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sativus.
pub(crate) fn n10919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11666);
			match s {
				[] => true,
				['e', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11667);
			match s {
				[] => true,
				['c', 'c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tativus.
pub(crate) fn n10920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7817);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11668);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11669);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11670);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11671);
			match s {
				[] => true,
				['l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 14803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vativus.
pub(crate) fn n10921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7818);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11672);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16769);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 16770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ctivus.
pub(crate) fn n4987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4116);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7819);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11674);
					s.is_empty()
				}
				['f', 'e', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11675);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11676);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'b', 'a', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7820);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11677);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14804);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 16771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11678);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11679);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11680);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14805);
							s.is_empty()
						}
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 14806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7821);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7822);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7823);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11681);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14807);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 14808);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14809);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14810);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etivus.
pub(crate) fn n4988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4117);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7824);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itivus.
pub(crate) fn n4989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4118);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7826);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7827);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7828);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7829);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11683);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16772);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16773);
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
		['p', s @ ..] => {
			super::push(out, 7830);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7831);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11684);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14812);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
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
							super::push(out, 14813);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11685);
					match s {
						[] => true,
						['p', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 7832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntivus.
pub(crate) fn n4990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4119);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11686);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11687);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otivus.
pub(crate) fn n4991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4120);
	s.is_empty()
}

/// Words ending -ptivus.
pub(crate) fn n4992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4121);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7833);
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 11688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7834);
			s.is_empty()
		}
		['o', s @ ..] => {
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

/// Words ending -rtivus.
pub(crate) fn n4993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4122);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7835);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stivus.
pub(crate) fn n4994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4123);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11689);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11690);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11691);
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
		['u', s @ ..] => {
			super::push(out, 7837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utivus.
pub(crate) fn n4995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4124);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7838);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 7839);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7840);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11692);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7841);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word permeabilis.
pub(crate) fn n18681(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -fabilis.
pub(crate) fn n5501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4489);
	match s {
		[] => true,
		['f', 'e', s @ ..] => {
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

/// Words ending -gabilis.
pub(crate) fn n5502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4490);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8059);
			match s {
				[] => true,
				['r', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14849);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8060);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8061);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 11812);
					match s {
						[] => true,
						['s', 'e', 'v', 'n', 'i', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 11813);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
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
			super::push(out, 8062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -habilis.
pub(crate) fn n5503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4491);
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

/// Words ending -iabilis.
pub(crate) fn n5504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4492);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8063);
			match s {
				[] => true,
				['o', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14851);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16785);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16786);
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
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8064);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8065);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11814);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16787);
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
				['i', s @ ..] => {
					super::push(out, 11815);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -labilis.
pub(crate) fn n5505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4493);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'g', 'n', 'o', 'c', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 8066);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11817);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11818);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'm', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'c', 'a', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mabilis.
pub(crate) fn n5506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4494);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8067);
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
		['e', s @ ..] => {
			super::push(out, 8068);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8069);
			match s {
				[] => true,
				['t', 's', 'e', 'a', s @ ..] => {
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
			super::push(out, 8070);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11819);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nabilis.
pub(crate) fn n5507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4495);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8071);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14852);
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
		['g', s @ ..] => {
			super::push(out, 8072);
			match s {
				[] => true,
				['u', 'p', 'x', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14853);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14854);
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
		['i', s @ ..] => {
			super::push(out, 8073);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11821);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11822);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16788);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11823);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14855);
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
						['r', s @ ..] => {
							super::push(out, 14856);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 17698);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 17699);
											s.is_empty()
										}
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
					super::push(out, 11824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8074);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8075);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8076);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11825);
					match s {
						[] => true,
						['t', 'a', 'r', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 11826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pabilis.
pub(crate) fn n5508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4496);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 8078);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'c', s @ ..] => {
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

/// Words ending -rabilis.
pub(crate) fn n5509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4497);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14857);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 16790);
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
								['s', s @ ..] => {
									super::push(out, 16791);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 17700);
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
						['m', s @ ..] => {
							super::push(out, 14858);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8080);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8081);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8082);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11828);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14859);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 17701);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 17702);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11829);
					match s {
						[] => true,
						['o', 't', s @ ..] => {
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
					super::push(out, 11830);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
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
					super::push(out, 11831);
					match s {
						[] => true,
						['l', 'u', 'v', s @ ..] => {
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
					super::push(out, 11832);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14861);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14862);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 17703);
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
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 11833);
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
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
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8083);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8084);
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
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8085);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11834);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11835);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 11836);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 11837);
					match s {
						[] => true,
						['e', s @ ..] => {
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
			super::push(out, 8086);
			match s {
				[] => true,
				['a', 'n', s @ ..] => super::t12::n18727(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t12::n11391(s, out),
		['u', s @ ..] => super::t12::n11392(s, out),
		_ => false,
	}
}

/// Words ending -sabilis.
pub(crate) fn n5510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4498);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 8089);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 11845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8090);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8091);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8092);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11846);
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
				['c', s @ ..] => {
					super::push(out, 11847);
					match s {
						[] => true,
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
		_ => false,
	}
}

/// Words ending -tabilis.
pub(crate) fn n5511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4499);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8093);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11848);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
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
					super::push(out, 11849);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14864);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14865);
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
					super::push(out, 11850);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
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
			super::push(out, 8094);
			match s {
				[] => true,
				['r', 'p', 'r', 'e', 't', 'n', 'i', s @ ..] => {
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
			super::push(out, 8095);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11851);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 11852);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11853);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14866);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14867);
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
										['x', 'e', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11854);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11855);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11856);
					match s {
						[] => true,
						['q', 'e', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 11857);
					match s {
						[] => true,
						['e', s @ ..] => {
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
			super::push(out, 8096);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11858);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14868);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17704);
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
						['s', s @ ..] => {
							super::push(out, 14869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8097);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11859);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11860);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['o', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 8099);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11862);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11863);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16793);
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
					super::push(out, 11864);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11865);
					match s {
						[] => true,
						['g', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 8101);
			match s {
				[] => true,
				['m', s @ ..] => super::t12::n18760(s, out),
				['r', s @ ..] => {
					super::push(out, 11867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uabilis.
pub(crate) fn n5512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4500);
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

/// Words ending -vabilis.
pub(crate) fn n5513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4501);
	match s {
		[] => true,
		['r', 'e', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 11868);
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
				['n', s @ ..] => {
					super::push(out, 11869);
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

/// Words ending -xabilis.
pub(crate) fn n5514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4502);
	s.is_empty()
}

/// Words ending -eronensis.
pub(crate) fn n18986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12024);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 14919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oronensis.
pub(crate) fn n18987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12025);
	s.is_empty()
}

/// Words ending -sonensis.
pub(crate) fn n11651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8271);
	s.is_empty()
}

/// Words ending -zonensis.
pub(crate) fn n11652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8272);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unensis.
pub(crate) fn n5710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4646);
	s.is_empty()
}

/// The word obrigescens.
pub(crate) fn n26306(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -frigescens.
pub(crate) fn n19688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12202);
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

/// Words ending -iescens.
pub(crate) fn n6778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5169);
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

/// Words ending -lescens.
pub(crate) fn n6779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5170);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 8633);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14962);
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
		['l', s @ ..] => {
			super::push(out, 8634);
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
				['o', 'm', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 8635);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12203);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14963);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8636);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14964);
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

/// Words ending -mescens.
pub(crate) fn n6780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5171);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12204);
					s.is_empty()
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

/// Words ending -nescens.
pub(crate) fn n6781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5172);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8637);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12205);
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
					super::push(out, 12206);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 12207);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14965);
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
				['y', s @ ..] => {
					super::push(out, 12208);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 14966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8638);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14968);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14969);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16810);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16811);
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
							super::push(out, 14970);
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
				['v', s @ ..] => {
					super::push(out, 12211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8639);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8640);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8641);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pescens.
pub(crate) fn n6782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5173);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 8643);
			s.is_empty()
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8644);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8645);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
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

/// Words ending -rescens.
pub(crate) fn n6783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5174);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'c', s @ ..] => {
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
				['m', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8647);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 14971);
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
							super::push(out, 14972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8648);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12212);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16812);
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
					super::push(out, 12213);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12214);
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
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8649);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12215);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 12216);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 12217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8650);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8651);
			match s {
				[] => true,
				['v', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 8652);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12218);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12219);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16813);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8653);
			match s {
				[] => true,
				['o', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8654);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12220);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12221);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 8655);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12222);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 12223);
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
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sescens.
pub(crate) fn n6784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5175);
	s.is_empty()
}

/// Words ending -tescens.
pub(crate) fn n6785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5176);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8656);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8657);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8658);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8659);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12225);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12226);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8660);
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
		['u', s @ ..] => {
			super::push(out, 8661);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12228);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12229);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14973);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 14974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uescens.
pub(crate) fn n6786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5177);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8662);
			match s {
				[] => true,
				['n', 'a', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16814);
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
		['q', s @ ..] => {
			super::push(out, 8663);
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
			super::push(out, 8664);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 12230);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12231);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14975);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14977);
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

/// Words ending -vescens.
pub(crate) fn n6787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5178);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8665);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12232);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12233);
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
		['i', s @ ..] => {
			super::push(out, 8666);
			s.is_empty()
		}
		['l', 'u', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8667);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14978);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -piciens.
pub(crate) fn n7070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5332);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12301);
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
					super::push(out, 12302);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15014);
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
					super::push(out, 12303);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12304);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12305);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15015);
							match s {
								[] => true,
								['r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15017);
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
					super::push(out, 12306);
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
					super::push(out, 12307);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12308);
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
		_ => false,
	}
}

/// Words ending -icandus.
pub(crate) fn n7940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5806);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9224);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 9225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12479);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16833);
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
					super::push(out, 12480);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12481);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15067);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 16834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16835);
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
							super::push(out, 15068);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 15069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 15070);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16836);
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
				['n', s @ ..] => {
					super::push(out, 12482);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15071);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17706);
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
							super::push(out, 15072);
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
					super::push(out, 12483);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15073);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15074);
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
					super::push(out, 12484);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15075);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15076);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 16837);
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
									super::push(out, 16838);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 17707);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 17708);
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
									super::push(out, 16839);
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
							super::push(out, 15077);
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
		['f', s @ ..] => super::t12::n14176(s, out),
		['l', s @ ..] => {
			super::push(out, 9227);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12486);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12487);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12488);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12489);
					s.is_empty()
				}
				['p', s @ ..] => super::t12::n20639(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9228);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12491);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15092);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9229);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12492);
					s.is_empty()
				}
				['r', s @ ..] => super::t12::n20643(s, out),
				['u', s @ ..] => super::t12::n20644(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t12::n14180(s, out),
		['r', s @ ..] => super::t12::n14181(s, out),
		['s', s @ ..] => super::t12::n14182(s, out),
		['t', s @ ..] => super::t12::n14183(s, out),
		['v', s @ ..] => super::t12::n14184(s, out),
		_ => false,
	}
}

/// Words ending -ncandus.
pub(crate) fn n7942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5808);
	match s {
		[] => true,
		['u', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12508);
					match s {
						[] => true,
						['e', 'v', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12509);
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

/// Words ending -ocandus.
pub(crate) fn n7943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5809);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 9235);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9236);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12511);
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
								['e', 'r', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9238);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 9239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 12513);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15107);
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
					super::push(out, 12515);
					match s {
						[] => true,
						['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12516);
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
					super::push(out, 12517);
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

/// Words ending -rcandus.
pub(crate) fn n7944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5810);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9240);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15108);
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
			super::push(out, 9241);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15109);
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

/// Words ending -scandus.
pub(crate) fn n7945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5811);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9242);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9243);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12518);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12519);
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
			super::push(out, 9244);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15110);
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

/// Words ending -ducandus.
pub(crate) fn n14198(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12520);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12521);
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

/// Words ending -undandus.
pub(crate) fn n14216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9260);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'a', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 12524);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15119);
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
					super::push(out, 15120);
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
			super::push(out, 12525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15121);
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
			super::push(out, 12526);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15122);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15123);
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

/// Words ending -odandus.
pub(crate) fn n7953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5818);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 9261);
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
			super::push(out, 9262);
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
					super::push(out, 12527);
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

/// Words ending -rdandus.
pub(crate) fn n7954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5819);
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

/// Words ending -udandus.
pub(crate) fn n7955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5820);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9263);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12528);
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
					super::push(out, 12529);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16876);
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
			super::push(out, 9264);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9265);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12530);
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
			super::push(out, 9266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12531);
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

/// Words ending -lligandus.
pub(crate) fn n20766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12546);
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
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nligandus.
pub(crate) fn n20768(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -migandus.
pub(crate) fn n14249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9278);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12547);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12548);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rigandus.
pub(crate) fn n14250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9279);
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
			super::push(out, 12549);
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

/// Words ending -tigandus.
pub(crate) fn n14251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9280);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12550);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16877);
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
			super::push(out, 12551);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 15133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 12552);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12553);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15134);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15135);
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
					super::push(out, 15136);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 15137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vigandus.
pub(crate) fn n14252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9281);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12554);
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
						['e', s @ ..] => {
							super::push(out, 16878);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16879);
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
			super::push(out, 12555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgandus.
pub(crate) fn n7971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5830);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 9282);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15138);
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
			super::push(out, 9283);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12556);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 12557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15139);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15140);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15141);
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

/// Words ending -ngandus.
pub(crate) fn n7972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5831);
	match s {
		[] => true,
		['o', 'l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12558);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12559);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ogandus.
pub(crate) fn n7973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5832);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12560);
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
					super::push(out, 12561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15142);
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
					super::push(out, 12562);
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
					super::push(out, 12563);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12564);
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

/// Words ending -rgandus.
pub(crate) fn n7974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5833);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12565);
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
							super::push(out, 15143);
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

/// Words ending -ugandus.
pub(crate) fn n7975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5834);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 9284);
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
			super::push(out, 9285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12566);
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
					super::push(out, 12567);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12568);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12569);
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
			super::push(out, 9286);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9287);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word appropiandus.
pub(crate) fn n20829(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word expiandus.
pub(crate) fn n14286(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -riandus.
pub(crate) fn n7984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5843);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9306);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 9307);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9308);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15148);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 9309);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9310);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9311);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12580);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12581);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12582);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 12583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siandus.
pub(crate) fn n7985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5844);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9312);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiandus.
pub(crate) fn n7986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5845);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9314);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9315);
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
			super::push(out, 9316);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12584);
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
			super::push(out, 9317);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12585);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12586);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12587);
					match s {
						[] => true,
						['n', s @ ..] => {
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
									super::push(out, 16882);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17715);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 17716);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 17717);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 17718);
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
								['n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16883);
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
			super::push(out, 9318);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9319);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viandus.
pub(crate) fn n7987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5846);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9321);
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
		['e', s @ ..] => {
			super::push(out, 9322);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12588);
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
					super::push(out, 12589);
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

/// Words ending -nihilandus.
pub(crate) fn n20871(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -milandus.
pub(crate) fn n14327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9339);
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
				['n', s @ ..] => {
					super::push(out, 15153);
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

/// Words ending -pilandus.
pub(crate) fn n14328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9340);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12595);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12596);
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

/// Words ending -tilandus.
pub(crate) fn n14329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9341);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 12597);
			match s {
				[] => true,
				['u', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 15154);
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

/// Words ending -llandus.
pub(crate) fn n7994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5853);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9342);
			match s {
				[] => true,
				['v', s @ ..] => {
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
			super::push(out, 9343);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15155);
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
				['f', s @ ..] => {
					super::push(out, 12599);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9344);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12601);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15156);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 12602);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12603);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 15158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12604);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15159);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15160);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15161);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16885);
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
									super::push(out, 16886);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16887);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 16888);
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
					super::push(out, 12605);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 12606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9345);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olandus.
pub(crate) fn n7995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5854);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9347);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 9348);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12607);
					match s {
						[] => true,
						['r', 'g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12608);
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
			super::push(out, 9349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12609);
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
			super::push(out, 9350);
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
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9351);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 9352);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12610);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12611);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12612);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12613);
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
			super::push(out, 9354);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 9355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
					super::push(out, 12615);
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
				['n', s @ ..] => {
					super::push(out, 12616);
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
					super::push(out, 12617);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12618);
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
					super::push(out, 12619);
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

