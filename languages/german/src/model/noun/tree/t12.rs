use flexion::Path;

/// Words ending -lut.
pub(crate) fn n3126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3121);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11132);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22398);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 11133);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 22400);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 22401);
					s.is_empty()
				}
				['h', 'u', s @ ..] => {
					super::push(out, 22402);
					s.is_empty()
				}
				['l', 'l', s @ ..] => {
					super::push(out, 22403);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22404);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', 'f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 38123);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 41883);
											s.is_empty()
										}
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 41884);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 38124);
									s.is_empty()
								}
								['i', 'r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 38125);
									s.is_empty()
								}
								['s', 'h', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 38126);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 38127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22405);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22406);
					match s {
						[] => true,
						['n', 'o', 'i', 't', 'a', 'u', 'r', 't', 's', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 31805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 22407);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 11134);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 22408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11135);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 22409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11136);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 22410);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mut.
pub(crate) fn n3127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3122);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11137);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 11138);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11139);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 22412);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11140);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11141);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22414);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11142);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22416);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 22417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11143);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11144);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11145);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11146);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22420);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22421);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31807);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 31808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38128);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 38129);
									s.is_empty()
								}
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
			super::push(out, 11147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22422);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22423);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 22424);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31809);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 31810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 22425);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22426);
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 31811);
							s.is_empty()
						}
						['o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 31812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11148);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 11149);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 22427);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nut.
pub(crate) fn n3128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3123);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11150);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11151);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11152);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -out.
pub(crate) fn n3129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3124);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11154);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 11155);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22428);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22429);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 22430);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 38130);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 38131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 31813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 22431);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22432);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 22433);
					match s {
						[] => true,
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 31814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 22434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 11156);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 11157);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11158);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11159);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 22435);
					match s {
						[] => true,
						['i', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 31815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 22436);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11160);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22438);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22439);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22440);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 11161);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11162);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 22442);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 31816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11163);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31817);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31818);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 31819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11164);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11165);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22445);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 31820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11166);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 11167);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22448);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 22449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -put.
pub(crate) fn n3130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3125);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11168);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 11169);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11170);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11171);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11172);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22450);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 22451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rut.
pub(crate) fn n3131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3126);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11174);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 11175);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11176);
			match s {
				[] => true,
				['c', 's', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 22452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 11177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tut.
pub(crate) fn n3132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3127);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11178);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 22453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11179);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 22454);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 31822);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 38132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 38133);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 41885);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 38134);
									match s {
										[] => true,
										['i', 'l', s @ ..] => {
											super::push(out, 41886);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 43726);
													s.is_empty()
												}
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
				['v', s @ ..] => {
					super::push(out, 22455);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 31823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wut.
pub(crate) fn n3133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3128);
	s.is_empty()
}

/// Words ending -äut.
pub(crate) fn n3134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3129);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 11180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ndfunkgerät.
pub(crate) fn n46655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43727);
	s.is_empty()
}

/// The word bordfunkgerät.
pub(crate) fn n46656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43728);
	s.is_empty()
}

/// Words ending -tät.
pub(crate) fn n3149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3144);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11201);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 31836);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t15::n11333(s, out),
		['r', s @ ..] => {
			super::push(out, 11203);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -abbau.
pub(crate) fn n23159(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31870);
			s.is_empty()
		}
		['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 31871);
			s.is_empty()
		}
		['e', 'i', 't', 'a', 'r', 'k', 'o', 'r', 'ü', 'b', '\u{1}', s @ ..] => {
			super::push(out, 31872);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['f', 'o', 't', 's', 'd', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 38192);
					s.is_empty()
				}
				['r', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 38193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 31873);
			match s {
				[] => true,
				['a', 'i', 'l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 38194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 'i', 'h', 't', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 31874);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 31875);
			match s {
				[] => true,
				['e', 'l', 'l', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 38195);
					s.is_empty()
				}
				['h', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 38196);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 38197);
					s.is_empty()
				}
				['o', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 38198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31876);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 38199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 31877);
			match s {
				[] => true,
				['g', 'n', 'u', 'n', 'n', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 38200);
					s.is_empty()
				}
				['i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 38201);
					s.is_empty()
				}
				['r', 'e', 't', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 38202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 31878);
			match s {
				[] => true,
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 38203);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 38204);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dbau.
pub(crate) fn n11343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11212);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 22479);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 31879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22480);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 31880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31881);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 31882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ebau.
pub(crate) fn n11344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11213);
	match s {
		[] => true,
		['b', 'l', 'ö', 'w', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 22481);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 22482);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 31883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 22483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 22484);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 's', 's', 'o', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 38205);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 38206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 22485);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 22486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fbau.
pub(crate) fn n11345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11214);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 22487);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 31885);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 31886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31887);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 31888);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38207);
							s.is_empty()
						}
						['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 38208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31889);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 31890);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38209);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 38210);
							s.is_empty()
						}
						['u', 'm', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 38211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gbau.
pub(crate) fn n11346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11215);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22488);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 22489);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', 'e', 's', 'f', 'e', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 38212);
							s.is_empty()
						}
						['m', 'u', 'a', 'r', 't', 'l', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 38213);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 41950);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 43768);
											s.is_empty()
										}
										['t', 'n', 'a', 'm', 'a', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 43769);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 38214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22490);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hbau.
pub(crate) fn n11347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11216);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 38215);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 38216);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 38217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 31891);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 31892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22491);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 31893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lbau.
pub(crate) fn n11349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11218);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22493);
			match s {
				[] => true,
				['n', 'i', 'd', 'u', 't', 'i', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 31894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41951);
									s.is_empty()
								}
								['m', 'h', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 41952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 38218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 38219);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 41953);
									s.is_empty()
								}
								['u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31895);
					match s {
						[] => true,
						['t', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 38220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'a', s @ ..] => {
			match s {
				[] => true,
				['f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 31896);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 31897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 22494);
			s.is_empty()
		}
		['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 22495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mbau.
pub(crate) fn n11350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11219);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 22496);
			match s {
				[] => true,
				['r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 31898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31899);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nbau.
pub(crate) fn n11351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11220);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22498);
			match s {
				[] => true,
				['e', 's', 'r', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 31900);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 31901);
					s.is_empty()
				}
				['l', 'l', 'o', 'w', 'm', 'u', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 31902);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 38221);
							s.is_empty()
						}
						['n', 'h', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 38222);
							s.is_empty()
						}
						['z', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 38223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31903);
					match s {
						[] => true,
						['a', 'k', 'i', 'r', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 38224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 31904);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 31905);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 41955);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 41956);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 41957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 31906);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 31907);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 31908);
					match s {
						[] => true,
						['i', 'h', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 38225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31909);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 31910);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 38226);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 38227);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 41958);
									s.is_empty()
								}
								['ü', s @ ..] => {
									super::push(out, 41959);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 43770);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 31911);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 22499);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 22500);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 38229);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 38230);
							match s {
								[] => true,
								['s', 'k', 'l', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 38231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22501);
			match s {
				[] => true,
				['g', 'g', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 31912);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 31913);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rbau.
pub(crate) fn n11352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11221);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 31914);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 38232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 31915);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 38233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'ä', 'g', 'r', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 31916);
					s.is_empty()
				}
				['i', 'm', 'm', 'o', 'n', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 31917);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 31918);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 31919);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 31920);
					match s {
						[] => true,
						['r', 'ö', 'k', '\u{1}', s @ ..] => {
							super::push(out, 38234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31921);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 31922);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 38235);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 38236);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 38237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 31923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22502);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38238);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 38239);
							match s {
								[] => true,
								['a', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 41961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 31924);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sbau.
pub(crate) fn n11353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11222);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 22503);
			match s {
				[] => true,
				['n', 'u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 38240);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 38241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 22504);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 31925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22505);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 22506);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38242);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 38243);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 38244);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 38245);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 38246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tbau.
pub(crate) fn n11354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11223);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 22507);
			match s {
				[] => true,
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 31926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'b', s @ ..] => {
			super::push(out, 22508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zbau.
pub(crate) fn n11358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11227);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 22509);
			s.is_empty()
		}
		['t', 'a', s @ ..] => {
			super::push(out, 22510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dau.
pub(crate) fn n3157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3152);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11228);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 11229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eau.
pub(crate) fn n3158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3153);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11230);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11231);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11232);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22511);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31927);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11233);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11234);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11235);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 22512);
					match s {
						[] => true,
						['p', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 31929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 11236);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 31930);
							match s {
								[] => true,
								['h', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 38247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', s @ ..] => {
							super::push(out, 31931);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38248);
									s.is_empty()
								}
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

/// Words ending -fau.
pub(crate) fn n3159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3154);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 11237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gau.
pub(crate) fn n3160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3155);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11238);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 11239);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11240);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11241);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11242);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22513);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 22514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11243);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hau.
pub(crate) fn n3161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3156);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11245);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 11246);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 31932);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31933);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38249);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 38250);
									s.is_empty()
								}
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
			super::push(out, 11247);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22515);
					match s {
						[] => true,
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 31934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11248);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 11249);
			match s {
				[] => true,
				['l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 22517);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iau.
pub(crate) fn n3162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3157);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 11250);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jau.
pub(crate) fn n3163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3158);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 11252);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lau.
pub(crate) fn n3164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3159);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11253);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 22518);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38251);
									s.is_empty()
								}
								['k', 'n', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 38252);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 38253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 31935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22519);
					s.is_empty()
				}
				['r', 'u', 'z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 22520);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22521);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31936);
							match s {
								[] => true,
								['i', 'n', 'ö', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31937);
							s.is_empty()
						}
						['t', 'h', 'c', 'a', 'n', 'r', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 31938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 22522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11254);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22523);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22524);
					match s {
						[] => true,
						['e', 'r', 'ä', 'b', s @ ..] => {
							match s {
								[] => true,
								['-', 'n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 38255);
									s.is_empty()
								}
								['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 38256);
									s.is_empty()
								}
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

/// Words ending -mau.
pub(crate) fn n3165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3160);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 11256);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nau.
pub(crate) fn n3166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3161);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pau.
pub(crate) fn n3167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3162);
	s.is_empty()
}

/// Words ending -rau.
pub(crate) fn n3168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3163);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 11258);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', 'l', s @ ..] => {
							super::push(out, 31939);
							match s {
								[] => true,
								['o', 'f', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 38257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							super::push(out, 31940);
							match s {
								[] => true,
								['u', 'a', 'r', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 38258);
									s.is_empty()
								}
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
			super::push(out, 11259);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 22525);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22526);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22527);
					match s {
						[] => true,
						['e', 'g', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 31941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11260);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22529);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sau.
pub(crate) fn n3169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3164);
	s.is_empty()
}

/// Words ending -tau.
pub(crate) fn n3170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3165);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11261);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 11262);
			match s {
				[] => true,
				['i', 'n', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 22530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11263);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22531);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 31942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 22532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11264);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11265);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11266);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 22534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11267);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 22535);
					match s {
						[] => true,
						['f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 31943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 22536);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 22537);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22538);
					match s {
						[] => true,
						['o', 'm', 'r', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 31944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22539);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31945);
							match s {
								[] => true,
								['n', 'u', 't', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 41962);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 41963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 31946);
							match s {
								[] => true,
								['o', 'i', 't', 'i', 't', 's', 'e', 'v', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 38259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 31947);
							match s {
								[] => true,
								['h', 'e', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 38260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 22540);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31948);
							s.is_empty()
						}
						['u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11268);
			match s {
				[] => true,
				['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 22541);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uau.
pub(crate) fn n3171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3166);
	s.is_empty()
}

/// Words ending -vau.
pub(crate) fn n3172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3167);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wau.
pub(crate) fn n3173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3168);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11270);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tanz.
pub(crate) fn n11721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11582);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22722);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 22723);
			s.is_empty()
		}
		['e', 'r', 'o', 'l', 'k', 'l', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 22724);
			s.is_empty()
		}
		['g', 'n', 'i', 'r', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 22725);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 22726);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 22727);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 32051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 32052);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 32053);
					s.is_empty()
				}
				['s', 'e', 't', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 32054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22728);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22729);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 32055);
					match s {
						[] => true,
						['ä', 'r', 'p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 38309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'l', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 22730);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 22731);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 32056);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 32057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22732);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					super::push(out, 32058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 22733);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 32059);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 32060);
					match s {
						[] => true,
						['e', 'i', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 38310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 32061);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 38311);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 41971);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 43775);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 38312);
							match s {
								[] => true,
								['r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 32062);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 32063);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 38313);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 38314);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 41973);
									match s {
										[] => true,
										['t', 'a', 'm', 'r', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 43776);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 41974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 32064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 22734);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 38315);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 38316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22735);
			s.is_empty()
		}
		['z', 't', 'i', 'n', 'h', 'c', 's', 'l', 'e', 'f', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 22736);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enz.
pub(crate) fn n3465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3459);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11587);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11588);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22740);
					match s {
						[] => true,
						['k', 'e', s @ ..] => {
							super::push(out, 32066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 22741);
					match s {
						[] => true,
						['z', 'e', s @ ..] => {
							super::push(out, 32067);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 38318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 22742);
					s.is_empty()
				}
				['n', 'e', 't', s @ ..] => {
					super::push(out, 22743);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 22744);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11589);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22745);
					s.is_empty()
				}
				['i', 'l', 'l', 'e', 't', 'n', 'i', 'l', s @ ..] => {
					super::push(out, 22746);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22747);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 22748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11590);
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['i', 'f', 'f', 'e', 'e', s @ ..] => {
							super::push(out, 32069);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11591);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22749);
					s.is_empty()
				}
				['a', 'v', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32071);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 38319);
									s.is_empty()
								}
								['t', 'l', s @ ..] => {
									super::push(out, 38320);
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
						['d', 'n', 'i', s @ ..] => {
							super::push(out, 32072);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 32073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					super::push(out, 22750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', 'h', s @ ..] => {
			super::push(out, 11592);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11593);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22752);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 32074);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38321);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 38322);
									s.is_empty()
								}
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
			super::push(out, 11594);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 32075);
							match s {
								[] => true,
								['n', 'o', 'k', 'r', s @ ..] => {
									super::push(out, 38323);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 41976);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 32076);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 32077);
							match s {
								[] => true,
								['e', 'r', 'r', s @ ..] => {
									super::push(out, 38324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 22753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 22754);
					s.is_empty()
				}
				['ä', 'r', 'p', 'l', s @ ..] => {
					super::push(out, 22755);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11595);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22756);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 32079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 22757);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 22758);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22759);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22760);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38325);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 38326);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 38327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32080);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'n', s @ ..] => {
											super::push(out, 41977);
											s.is_empty()
										}
										['n', 'i', s @ ..] => {
											super::push(out, 41978);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', 'o', s @ ..] => {
									super::push(out, 38328);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 41979);
											s.is_empty()
										}
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
			super::push(out, 11596);
			match s {
				[] => true,
				['q', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32081);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 32082);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 32083);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22761);
					match s {
						[] => true,
						['g', 'n', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 38330);
									s.is_empty()
								}
								['n', 'i', 's', s @ ..] => {
									super::push(out, 38331);
									s.is_empty()
								}
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
			super::push(out, 11597);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 11598);
			match s {
				[] => true,
				['a', 'j', 'b', s @ ..] => {
					super::push(out, 22762);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 22763);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22764);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => super::t15::n40628(s, out),
								['n', s @ ..] => super::t15::n40629(s, out),
								['u', s @ ..] => super::t15::n40630(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t15::n33680(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sprinz.
pub(crate) fn n23486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22766);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vinz.
pub(crate) fn n11742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11602);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 22767);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lunz.
pub(crate) fn n3467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3461);
	match s {
		[] => true,
		['h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 11603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ünz.
pub(crate) fn n3468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3462);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 11604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urz.
pub(crate) fn n3473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3467);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 11619);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 22790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 11620);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11621);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 32092);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 32093);
							match s {
								[] => true,
								['u', 'r', s @ ..] => {
									super::push(out, 38350);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', s @ ..] => {
							super::push(out, 32094);
							match s {
								[] => true,
								['t', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 38351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['ö', s @ ..] => {
									super::push(out, 38352);
									s.is_empty()
								}
								['ü', s @ ..] => {
									super::push(out, 38353);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											super::push(out, 41988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 38354);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38355);
									match s {
										[] => true,
										['u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 41989);
											s.is_empty()
										}
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
		['w', s @ ..] => {
			super::push(out, 11622);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22792);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 22793);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32095);
							s.is_empty()
						}
						['i', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22794);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'l', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 38356);
									s.is_empty()
								}
								['k', 'l', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 38357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 32097);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 32098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22795);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32099);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 32100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22796);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22797);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 32101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -terwortschatz.
pub(crate) fn n46715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43777);
	s.is_empty()
}

/// Words ending -satz.
pub(crate) fn n11773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11633);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 22816);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 22817);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 22818);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 32108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22819);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 22820);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 32109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22821);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 32110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22822);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 38362);
							match s {
								[] => true,
								['i', 'g', s @ ..] => {
									super::push(out, 41995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 38363);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 41996);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'b', s @ ..] => {
					super::push(out, 32111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 22823);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 's', s @ ..] => {
							super::push(out, 38364);
							match s {
								[] => true,
								['r', 'ö', 'b', '\u{1}', s @ ..] => {
									super::push(out, 41997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 38365);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 41998);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 41999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 38366);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 38367);
							match s {
								[] => true,
								['m', 'a', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 42000);
									s.is_empty()
								}
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
			super::push(out, 22824);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 38368);
							match s {
								[] => true,
								['ö', s @ ..] => {
									super::push(out, 42001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 38369);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 42002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 32112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22825);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 32113);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 38370);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 38371);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 38372);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 38373);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 38374);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 38375);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 38376);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 38377);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 38378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 32114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 22826);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 32115);
					match s {
						[] => true,
						['n', 'u', 't', s @ ..] => {
							super::push(out, 38379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 32116);
					match s {
						[] => true,
						['i', 'z', 'z', s @ ..] => {
							super::push(out, 38380);
							match s {
								[] => true,
								['n', 'e', 'r', 'e', 'f', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 42003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 32117);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 32118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 22827);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 32119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22828);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 22829);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etz.
pub(crate) fn n3478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3472);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 11635);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11636);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 22832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11637);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 22833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11638);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 22834);
					match s {
						[] => true,
						['i', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 32121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11639);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22835);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 32122);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 38381);
									s.is_empty()
								}
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
			super::push(out, 11640);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32123);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'd', 'n', 'u', 'k', s @ ..] => {
							super::push(out, 32124);
							s.is_empty()
						}
						['i', 'e', 'l', s @ ..] => {
							super::push(out, 32125);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 32126);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 32127);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 38383);
									s.is_empty()
								}
								['s', 'e', s @ ..] => {
									super::push(out, 38384);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42004);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 42005);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 32128);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32129);
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 43778);
													s.is_empty()
												}
												['i', 'n', 'u', 'e', 'l', 'h', 'c', 's', 'e', 'b', 's', 'g', s @ ..] => {
													super::push(out, 43779);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', 'e', s @ ..] => {
											super::push(out, 42006);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 42007);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 42008);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42009);
											match s {
												[] => true,
												['e', 'h', s @ ..] => {
													super::push(out, 43780);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 42010);
											s.is_empty()
										}
										['z', 'n', s @ ..] => {
											super::push(out, 42011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 38385);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 42012);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42013);
											s.is_empty()
										}
										['o', 'i', 't', 'a', 'k', s @ ..] => {
											super::push(out, 42014);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 38386);
									match s {
										[] => true,
										['f', 'a', 'h', 'c', 's', 'r', 'e', 'n', s @ ..] => {
											super::push(out, 42015);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42016);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 't', s @ ..] => {
									super::push(out, 38387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 32130);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 32131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11641);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22836);
					match s {
						[] => true,
						['p', 'a', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 32132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itz.
pub(crate) fn n3479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3473);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11642);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11643);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 11644);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11645);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 22838);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 32133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11646);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 22839);
					match s {
						[] => true,
						['ü', 't', '\u{1}', s @ ..] => {
							super::push(out, 32134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11647);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 22840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11648);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 22841);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 22842);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 22843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11649);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22844);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22845);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'u', 'r', 'g', 'ß', s @ ..] => {
									super::push(out, 38388);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42017);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 42018);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'i', s @ ..] => {
									super::push(out, 38389);
									s.is_empty()
								}
								['t', 'n', s @ ..] => {
									super::push(out, 38390);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 38391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 32136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22846);
					match s {
						[] => true,
						['e', 'm', 'a', s @ ..] => {
							super::push(out, 32137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22847);
					match s {
						[] => true,
						['e', 'd', 'i', s @ ..] => {
							super::push(out, 32138);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22848);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 32139);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 38393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 22849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11650);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 22850);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 22851);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32140);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 32141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22852);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 38394);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 38395);
									s.is_empty()
								}
								['t', 't', s @ ..] => {
									super::push(out, 38396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 32142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtz.
pub(crate) fn n3483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3477);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11660);
			s.is_empty()
		}
		['u', 'w', 's', 'u', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 11661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utz.
pub(crate) fn n3484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3478);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11662);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11663);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11664);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32143);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 32144);
							match s {
								[] => true,
								['n', 'u', s @ ..] => {
									super::push(out, 38397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38398);
									s.is_empty()
								}
								['p', 'o', s @ ..] => {
									super::push(out, 38399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'u', 'a', s @ ..] => {
							super::push(out, 32145);
							s.is_empty()
						}
						['l', 'l', 'o', s @ ..] => {
							super::push(out, 32146);
							s.is_empty()
						}
						['m', 'm', s @ ..] => {
							super::push(out, 32147);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', s @ ..] => {
											super::push(out, 42019);
											s.is_empty()
										}
										['h', 'c', 'ä', s @ ..] => {
											super::push(out, 42020);
											s.is_empty()
										}
										['n', 'n', s @ ..] => {
											super::push(out, 42021);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42022);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['n', 'a', s @ ..] => {
													super::push(out, 43781);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 43782);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 38400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 't', 'i', s @ ..] => {
									super::push(out, 38401);
									s.is_empty()
								}
								['ö', s @ ..] => {
									super::push(out, 38402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'r', s @ ..] => {
									super::push(out, 38403);
									s.is_empty()
								}
								['n', 'o', 'i', 't', 'a', 'd', s @ ..] => {
									super::push(out, 38404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'h', s @ ..] => {
							super::push(out, 32148);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 32149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11665);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 's', s @ ..] => {
			super::push(out, 11666);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11667);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 22859);
					s.is_empty()
				}
				['r', 'e', 't', 's', s @ ..] => {
					super::push(out, 22860);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22861);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 22862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11668);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 22863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11669);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22864);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22865);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ätz.
pub(crate) fn n3485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3479);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 11670);
			match s {
				[] => true,
				['h', 'c', 's', 'e', 'g', 'r', s @ ..] => {
					super::push(out, 22866);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ötz.
pub(crate) fn n3486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3480);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11671);
			s.is_empty()
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 11672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ütz.
pub(crate) fn n3487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3481);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 11673);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32151);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 32152);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 32153);
							match s {
								[] => true,
								['e', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 38405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 32154);
							match s {
								[] => true,
								['u', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 38406);
									s.is_empty()
								}
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
			super::push(out, 11674);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 22867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11675);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11676);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 22868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -coniumchlorid.
pub(crate) fn n46722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43784);
	s.is_empty()
}

/// The word aluminiumfluorid.
pub(crate) fn n46723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43785);
	s.is_empty()
}

/// Words ending -peroxid.
pub(crate) fn n33963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32332);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38481);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 38482);
			match s {
				[] => true,
				['f', 'o', 't', 's', 'r', 'e', 's', 's', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 42054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 38483);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 38484);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43786);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 43787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 38485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -troxid.
pub(crate) fn n24006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23239);
	match s {
		[] => true,
		['e', 't', '\u{1}', s @ ..] => {
			super::push(out, 32333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntoxid.
pub(crate) fn n12700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12521);
	match s {
		[] => true,
		['e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 23240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pband.
pub(crate) fn n12787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12602);
	s.is_empty()
}

/// Words ending -rband.
pub(crate) fn n12788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12603);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 23336);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 32413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 23337);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 32414);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 38510);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 42061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 32415);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 38511);
							match s {
								[] => true,
								['ö', 'f', '\u{1}', s @ ..] => {
									super::push(out, 42062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 32416);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 38512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 32417);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 38513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 32418);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 32419);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 38514);
							match s {
								[] => true,
								['c', 'a', 'h', s @ ..] => {
									super::push(out, 42063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 38515);
							match s {
								[] => true,
								['e', 'k', s @ ..] => {
									super::push(out, 42064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 38516);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									super::push(out, 42065);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 42066);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 43788);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'm', s @ ..] => {
							super::push(out, 38517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 32420);
					match s {
						[] => true,
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 38518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 23338);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 32421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 23339);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 32422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 23340);
			match s {
				[] => true,
				['e', 'p', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 32423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 23341);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 32424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 23342);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 32425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 23343);
			match s {
				[] => true,
				['n', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 32426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sband.
pub(crate) fn n12789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12604);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 23344);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 38520);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 42067);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 42068);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 42069);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 42070);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 42071);
									s.is_empty()
								}
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
			super::push(out, 23345);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 23346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tband.
pub(crate) fn n12790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12605);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 23347);
			s.is_empty()
		}
		['h', 'c', 'i', 'd', 'e', s @ ..] => {
			super::push(out, 23348);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 32427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 23349);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 38521);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 38522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			super::push(out, 23350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yband.
pub(crate) fn n12791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12606);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 23351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zband.
pub(crate) fn n12792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12607);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 23352);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 32428);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 32429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 23353);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 23354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßband.
pub(crate) fn n12793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12608);
	s.is_empty()
}

/// Words ending -fand.
pub(crate) fn n4499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4478);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 12609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gand.
pub(crate) fn n4500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4479);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 12610);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23355);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 32430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 23356);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hand.
pub(crate) fn n4501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4480);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 12611);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			super::push(out, 12612);
			match s {
				[] => true,
				['b', 'o', s @ ..] => {
					super::push(out, 23357);
					s.is_empty()
				}
				['d', 'u', s @ ..] => {
					super::push(out, 23358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', s @ ..] => {
			super::push(out, 12613);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iand.
pub(crate) fn n4502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4481);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 12615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kand.
pub(crate) fn n4503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4482);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 12616);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -land.
pub(crate) fn n4504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4483);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12617);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 12618);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 12619);
			match s {
				[] => true,
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 23359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 12620);
			match s {
				[] => true,
				['ö', s @ ..] => {
					super::push(out, 23360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 12621);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 23361);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 23362);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32431);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 32432);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 38523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 23363);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 32433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 12622);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 23364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 12623);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 23365);
					match s {
						[] => true,
						['e', 's', 'g', 'e', 'i', 'r', 'k', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 32434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 12624);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32435);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38524);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 32436);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 32437);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32438);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 38526);
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42072);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 38527);
									match s {
										[] => true,
										['u', 'e', 'd', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													match s {
														[] => true,
														['i', 'z', 'a', 'n', '\u{1}', s @ ..] => super::t15::n47657(s, out),
														['r', 'e', 'l', 't', 'i', 'h', '\u{1}', s @ ..] => super::t15::n47658(s, out),
														_ => false,
													}
												}
												['i', 'z', 'a', 'n', '\u{1}', s @ ..] => {
													super::push(out, 43789);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 43790);
													s.is_empty()
												}
												['r', 'e', 'l', 't', 'i', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43791);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 43792);
													s.is_empty()
												}
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
		['i', s @ ..] => {
			super::push(out, 12625);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23366);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 32439);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 32440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 12626);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23367);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 32441);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 32442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 12627);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32443);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 32444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 23368);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 23369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 12628);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23370);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 32445);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 32446);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 32447);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 32448);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 32449);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 32450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 23371);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 23372);
					s.is_empty()
				}
				['r', 'e', 'u', s @ ..] => {
					super::push(out, 23373);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 23374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12629);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12630);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23375);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32451);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38528);
									match s {
										[] => true,
										['i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 42073);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 32452);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 32453);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 32454);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38529);
									match s {
										[] => true,
										['i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 42074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 32455);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 32456);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 32457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 23376);
					match s {
						[] => true,
						['v', 'n', s @ ..] => {
							super::push(out, 32458);
							match s {
								[] => true,
								['e', 'p', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 38530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t15::n12816(s, out),
		['t', s @ ..] => {
			super::push(out, 12632);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12633);
			match s {
				[] => true,
				['a', 'b', 'b', s @ ..] => {
					super::push(out, 23385);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 23386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 't', s @ ..] => {
			super::push(out, 12634);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mand.
pub(crate) fn n4505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4484);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12635);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 23387);
					s.is_empty()
				}
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 23388);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 23389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 12636);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 23390);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 23391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 12637);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 12638);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 23392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12639);
			match s {
				[] => true,
				['i', 'f', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 23393);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 23394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rand.
pub(crate) fn n4506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4485);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 12640);
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					super::push(out, 23395);
					s.is_empty()
				}
				['e', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 23396);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 23397);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 32463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32464);
							match s {
								[] => true,
								['i', 'r', 'f', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 38536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 32465);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 38537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 32466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'g', s @ ..] => {
					super::push(out, 23398);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 23399);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 32468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 12641);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12642);
			match s {
				[] => true,
				['b', 'e', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 23400);
					s.is_empty()
				}
				['p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 23401);
					s.is_empty()
				}
				['s', 'ä', 'k', '\u{1}', s @ ..] => {
					super::push(out, 23402);
					s.is_empty()
				}
				['t', 's', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 23403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 12643);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 12644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23404);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 23405);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 32469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 23406);
					s.is_empty()
				}
				['o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 23407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 12645);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 12646);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 12647);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 12648);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12649);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'h', s @ ..] => {
							super::push(out, 32470);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 32471);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 32472);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 23408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12650);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12651);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					super::push(out, 23409);
					match s {
						[] => true,
						['u', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 32473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12652);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 12653);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 23410);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 23411);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32474);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 38539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'w', s @ ..] => {
							super::push(out, 32475);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 32476);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38541);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 38542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 23412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12654);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 23413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 12655);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 23414);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sand.
pub(crate) fn n4507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4486);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 12656);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 12657);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12658);
			match s {
				[] => true,
				['e', 't', 's', 'ü', s @ ..] => {
					super::push(out, 23415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12659);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 23416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 12660);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 12661);
			match s {
				[] => true,
				['l', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 23417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tand.
pub(crate) fn n4508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4487);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 12662);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 23418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12663);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 23419);
					match s {
						[] => true,
						['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 32477);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 32478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 23420);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38543);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 38544);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 38545);
									match s {
										[] => true,
										['g', 'n', 'u', 'd', 'n', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42076);
											s.is_empty()
										}
										['t', 'i', 'e', 'h', 'r', 'e', 'h', 'c', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 42077);
											s.is_empty()
										}
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
					super::push(out, 23421);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 23422);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 32479);
							match s {
								[] => true,
								['n', 'e', 'm', s @ ..] => {
									super::push(out, 38546);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 38547);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 42078);
											match s {
												[] => true,
												['o', 'f', s @ ..] => {
													super::push(out, 43793);
													s.is_empty()
												}
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
							super::push(out, 32480);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 32481);
							match s {
								[] => true,
								['u', 'r', 's', s @ ..] => {
									super::push(out, 38548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 23423);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 38549);
									match s {
										[] => true,
										['a', 'b', 'h', 'c', 'o', 'k', '-', 'r', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42079);
											s.is_empty()
										}
										['g', s @ ..] => {
											match s {
												[] => true,
												['e', 'r', 'a', 'u', 't', '\u{1}', s @ ..] => {
													super::push(out, 43794);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 43795);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 38550);
									match s {
										[] => true,
										['e', 's', s @ ..] => {
											super::push(out, 42080);
											match s {
												[] => true,
												['e', 'i', 'r', '\u{1}', s @ ..] => {
													super::push(out, 43796);
													s.is_empty()
												}
												['s', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 43797);
													s.is_empty()
												}
												['u', 'r', 'd', '\u{1}', s @ ..] => {
													super::push(out, 43798);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 't', s @ ..] => {
									super::push(out, 38551);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 38552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 23424);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38553);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 42081);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 38554);
									match s {
										[] => true,
										['h', 'n', s @ ..] => {
											super::push(out, 42082);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t15::n24208(s, out),
				['l', s @ ..] => super::t15::n24209(s, out),
				['m', s @ ..] => {
					super::push(out, 23427);
					s.is_empty()
				}
				['n', s @ ..] => super::t15::n24211(s, out),
				['o', 'c', s @ ..] => {
					super::push(out, 23429);
					s.is_empty()
				}
				['r', s @ ..] => super::t15::n24213(s, out),
				['s', s @ ..] => super::t15::n24214(s, out),
				['t', s @ ..] => super::t15::n24215(s, out),
				['u', s @ ..] => super::t15::n24216(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n12849(s, out),
		_ => false,
	}
}

/// Words ending -wand.
pub(crate) fn n4509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4488);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 12665);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12666);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 23435);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 32490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 23436);
					match s {
						[] => true,
						['r', 'e', 'ß', s @ ..] => {
							super::push(out, 32491);
							match s {
								[] => true,
								['ü', 'b', '\u{1}', s @ ..] => {
									super::push(out, 38581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 23437);
					match s {
						[] => true,
						['i', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 32492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 12667);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 23438);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38582);
									match s {
										[] => true,
										['e', 'd', 'r', s @ ..] => {
											super::push(out, 42101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 38583);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42102);
											match s {
												[] => true,
												['e', 'i', 'r', 't', 'e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 43806);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'i', 'e', 'b', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42103);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 38584);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 42104);
											s.is_empty()
										}
										['s', 'e', 't', s @ ..] => {
											super::push(out, 42105);
											s.is_empty()
										}
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
			super::push(out, 12668);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 12669);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12670);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23439);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 23440);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38585);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 38586);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 42106);
											match s {
												[] => true,
												['u', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43807);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 42107);
											s.is_empty()
										}
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
			super::push(out, 12671);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23441);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 23442);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 32493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12672);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 12673);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 12674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnende.
pub(crate) fn n24445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23645);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 32603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pende.
pub(crate) fn n13167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12975);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 23646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rende.
pub(crate) fn n13168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12976);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 23647);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 23648);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 32604);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 32605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 23649);
			s.is_empty()
		}
		['ä', 'b', 'e', 'g', 't', 'ä', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 23650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sende.
pub(crate) fn n13169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12977);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 32606);
					s.is_empty()
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 32607);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 32608);
					s.is_empty()
				}
				['w', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 32609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 23651);
			s.is_empty()
		}
		['h', 'c', 'a', 'w', 'n', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 23652);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 23653);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 32610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 23654);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 23655);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 23656);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 23657);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 32611);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tende.
pub(crate) fn n13170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12978);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 23658);
			match s {
				[] => true,
				['e', 'b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 32612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 23659);
			s.is_empty()
		}
		['s', 'i', s @ ..] => {
			super::push(out, 23660);
			match s {
				[] => true,
				['e', 'l', 't', 's', 'n', 'e', 'i', 'd', 'l', 'i', 'v', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 32613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 23661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wende.
pub(crate) fn n13172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12980);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 23662);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 32614);
					match s {
						[] => true,
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 38621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 32615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 23663);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 23664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zende.
pub(crate) fn n13173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12981);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 23665);
			match s {
				[] => true,
				['a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 32616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 23666);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 32617);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 38622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 32618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßende.
pub(crate) fn n13174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12982);
	s.is_empty()
}

/// Words ending -inde.
pub(crate) fn n4721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4697);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 12983);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23667);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 32619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 23668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 12984);
			match s {
				[] => true,
				['m', 'e', 'g', 's', s @ ..] => {
					super::push(out, 23669);
					match s {
						[] => true,
						['t', 'f', s @ ..] => {
							super::push(out, 32620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 12985);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 23670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12986);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 23671);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 23672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12987);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 12988);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23674);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 23675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onde.
pub(crate) fn n4722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4698);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 12989);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 23676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12990);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnde.
pub(crate) fn n4723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4699);
	match s {
		[] => true,
		['e', 'g', s @ ..] => {
			super::push(out, 12991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unde.
pub(crate) fn n4724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4700);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 12992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23677);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 23678);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 23679);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 32621);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 32622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 23680);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 32623);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 38623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 32624);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 32625);
							match s {
								[] => true,
								['o', 't', s @ ..] => {
									super::push(out, 38625);
									match s {
										[] => true,
										['m', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42111);
											s.is_empty()
										}
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
					super::push(out, 23681);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 23682);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 23683);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 32626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 23684);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 23685);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 32627);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 32628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 23686);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'i', s @ ..] => {
									super::push(out, 38626);
									match s {
										[] => true,
										['e', 'z', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42112);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 38627);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42113);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 32629);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 38628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 23687);
					match s {
						[] => true,
						['r', 'ü', 'b', 'e', 's', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 32630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 23688);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32631);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 32632);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 38629);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 38630);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42114);
											s.is_empty()
										}
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
					super::push(out, 23689);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32633);
							match s {
								[] => true,
								['n', 'a', 't', 's', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 38631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', 'i', 'l', 'b', s @ ..] => {
							super::push(out, 32634);
							match s {
								[] => true,
								['e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 38632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 32635);
							match s {
								[] => true,
								['a', 'h', 'f', 'u', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 23690);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 38634);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 38635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 32636);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 32637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 23691);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32638);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 38636);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 23692);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 32639);
							match s {
								[] => true,
								['r', 'e', 'f', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 38637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 23693);
					match s {
						[] => true,
						['o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 32640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12993);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 12994);
			match s {
				[] => true,
				['s', 'r', 'e', s @ ..] => {
					super::push(out, 23694);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 32641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 12995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ände.
pub(crate) fn n4725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4701);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 12996);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 12997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23695);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 23696);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 23697);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 32642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 12998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ünde.
pub(crate) fn n4726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4702);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 12999);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 23698);
					match s {
						[] => true,
						['z', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13000);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 23699);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word karpfenartige.
pub(crate) fn n46749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43808);
	s.is_empty()
}

/// The word schlupfwespenartige.
pub(crate) fn n46750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43809);
	s.is_empty()
}

/// Words ending -glerartige.
pub(crate) fn n46751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43810);
	s.is_empty()
}

/// Words ending -tlerartige.
pub(crate) fn n46752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43811);
	s.is_empty()
}

/// Words ending -prache.
pub(crate) fn n24691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23882);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38658);
					match s {
						[] => true,
						['a', 'z', s @ ..] => {
							super::push(out, 42120);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43812);
									s.is_empty()
								}
								['e', 'r', s @ ..] => {
									super::push(out, 43813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', s @ ..] => {
							super::push(out, 42121);
							s.is_empty()
						}
						['l', 'u', s @ ..] => {
							super::push(out, 42122);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 42123);
							s.is_empty()
						}
						['o', 'v', s @ ..] => {
							super::push(out, 42124);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42125);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 42126);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 42127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', 'g', s @ ..] => {
					super::push(out, 38659);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 38660);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'e', 'w', 'e', s @ ..] => {
							super::push(out, 42128);
							s.is_empty()
						}
						['i', 'r', s @ ..] => {
							super::push(out, 42129);
							s.is_empty()
						}
						['s', 'n', s @ ..] => {
							super::push(out, 42130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 38661);
					s.is_empty()
				}
				['i', 'w', s @ ..] => {
					super::push(out, 38662);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 43814);
									s.is_empty()
								}
								['u', 'x', s @ ..] => {
									super::push(out, 43815);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'f', s @ ..] => {
							super::push(out, 42131);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 43816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', s @ ..] => {
							super::push(out, 42132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 42133);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 42134);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 43817);
									s.is_empty()
								}
								['i', 'l', 'a', s @ ..] => {
									super::push(out, 43818);
									s.is_empty()
								}
								['k', 'r', s @ ..] => {
									super::push(out, 43819);
									s.is_empty()
								}
								['m', 'u', s @ ..] => {
									super::push(out, 43820);
									s.is_empty()
								}
								['n', 'e', 'i', s @ ..] => {
									super::push(out, 43821);
									s.is_empty()
								}
								['r', 'u', s @ ..] => {
									super::push(out, 43822);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 44639);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 44640);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44641);
											s.is_empty()
										}
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
						['e', s @ ..] => {
							super::push(out, 42135);
							match s {
								[] => true,
								['d', 'n', 'e', 'g', s @ ..] => {
									super::push(out, 43823);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 43824);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 43825);
									s.is_empty()
								}
								['k', 'm', s @ ..] => {
									super::push(out, 43826);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['ä', s @ ..] => {
											super::push(out, 44642);
											s.is_empty()
										}
										['ü', s @ ..] => {
											super::push(out, 44643);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'k', s @ ..] => {
									super::push(out, 43827);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 43828);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43829);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 43830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 42136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 38663);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43831);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'n', 'a', 'm', 'd', s @ ..] => {
							super::push(out, 42137);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['f', 'ä', s @ ..] => {
									super::push(out, 43833);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43834);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44644);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'a', 'n', s @ ..] => {
							super::push(out, 42138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							super::push(out, 42139);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 38664);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iche.
pub(crate) fn n4932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4904);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13281);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 23889);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 23890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32721);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 32722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 23891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13282);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23892);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 32724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 23893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13283);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 23894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13284);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 23895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13285);
			match s {
				[] => true,
				['q', '\u{1}', s @ ..] => {
					super::push(out, 23896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 13286);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23897);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lche.
pub(crate) fn n4933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4905);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'b', s @ ..] => {
					super::push(out, 23898);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 23899);
					match s {
						[] => true,
						['n', 'ä', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 32725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nche.
pub(crate) fn n4934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4906);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 23900);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 32726);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 38665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 23901);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38666);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 38667);
									s.is_empty()
								}
								['k', 'i', s @ ..] => {
									super::push(out, 38668);
									s.is_empty()
								}
								['l', 'i', 't', s @ ..] => {
									super::push(out, 38669);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 38670);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 42141);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 38671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 23902);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 32727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oche.
pub(crate) fn n4935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4907);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 13287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23903);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 32728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13288);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 23904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13289);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23905);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 23906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13290);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 23907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13291);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13292);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 13293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rche.
pub(crate) fn n4936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4908);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 13294);
			s.is_empty()
		}
		['e', 'w', s @ ..] => {
			super::push(out, 13295);
			s.is_empty()
		}
		['i', 'k', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 23909);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 23910);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 32729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sche.
pub(crate) fn n4937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4909);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13296);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 23911);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 23912);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32731);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 38672);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42143);
											s.is_empty()
										}
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
					super::push(out, 23913);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 32732);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 38673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 23914);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 32733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13297);
			match s {
				[] => true,
				['r', 'd', s @ ..] => {
					super::push(out, 23915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 13298);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 23916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13299);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23917);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 23918);
					match s {
						[] => true,
						['n', 'ä', 't', 's', 'f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 32734);
							s.is_empty()
						}
						['r', 'i', 'r', 'e', 'ß', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 32735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 23919);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 23920);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 23921);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 32737);
							s.is_empty()
						}
						['i', 'e', 'h', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 32738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 23922);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32739);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 38674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 32740);
							match s {
								[] => true,
								['m', 'r', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 38675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 23923);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 32741);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 32742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13300);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23924);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 23925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13301);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13302);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 23927);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 32743);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 32744);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 23928);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'k', 'c', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 38677);
									s.is_empty()
								}
								['r', 'e', 'n', 't', 'n', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 38678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 23929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13303);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23930);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 32745);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 38679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 23931);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 23932);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 23933);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 32746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 23934);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 23935);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32748);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'a', 'l', s @ ..] => {
											match s {
												[] => true,
												['s', 's', 'u', 'r', '\u{1}', s @ ..] => {
													super::push(out, 43835);
													s.is_empty()
												}
												['ß', s @ ..] => {
													super::push(out, 43836);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 42144);
											s.is_empty()
										}
										['o', 'i', s @ ..] => {
											super::push(out, 42145);
											s.is_empty()
										}
										['s', 't', s @ ..] => {
											super::push(out, 42146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 32749);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 32750);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 32751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 23936);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 32752);
							match s {
								[] => true,
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 38681);
									s.is_empty()
								}
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
			super::push(out, 13304);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 23937);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 23938);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 32753);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 38682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 23939);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 23940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 23941);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 23942);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32755);
							match s {
								[] => true,
								['e', 'g', 'm', 'r', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 38683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 13305);
			match s {
				[] => true,
				['w', s @ ..] => super::t15::n24753(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uche.
pub(crate) fn n4938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4910);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13306);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 23943);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 32761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 13307);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13308);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13309);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 23944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13310);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 23945);
					match s {
						[] => true,
						['-', 'e', 's', 'u', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 32762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 23946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13311);
			match s {
				[] => true,
				['b', 'o', s @ ..] => {
					super::push(out, 23947);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 23948);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 23949);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 23950);
					s.is_empty()
				}
				['s', 'r', s @ ..] => {
					super::push(out, 23951);
					s.is_empty()
				}
				['t', 'x', s @ ..] => {
					super::push(out, 23952);
					s.is_empty()
				}
				['z', 't', 'a', 'l', s @ ..] => {
					super::push(out, 23953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iwche.
pub(crate) fn n13518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13312);
	s.is_empty()
}

/// Words ending -yche.
pub(crate) fn n4940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4912);
	match s {
		[] => true,
		['s', 'p', 'o', s @ ..] => {
			super::push(out, 13313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äche.
pub(crate) fn n4941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4913);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 13314);
			match s {
				[] => true,
				['f', 'r', s @ ..] => {
					super::push(out, 23954);
					match s {
						[] => true,
						['e', 'b', 'o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38685);
									s.is_empty()
								}
								['s', 'r', s @ ..] => {
									super::push(out, 38686);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42147);
											s.is_empty()
										}
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
			super::push(out, 13315);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 23955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'h', 'c', 's', 'n', s @ ..] => {
			super::push(out, 13316);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 23956);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üche.
pub(crate) fn n4942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4914);
	match s {
		[] => true,
		['k', '-', s @ ..] => {
			super::push(out, 13317);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 23957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ologie.
pub(crate) fn n24804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23989);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['o', 'h', s @ ..] => {
					super::push(out, 38704);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 42160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'e', s @ ..] => {
					super::push(out, 38705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 32788);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 38706);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['-', 'e', s @ ..] => {
									super::push(out, 43837);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 43838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 38707);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 38708);
					s.is_empty()
				}
				['s', 'a', s @ ..] => {
					super::push(out, 38709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 32789);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'y', 's', 'p', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 42161);
							s.is_empty()
						}
						['n', 'e', 'f', s @ ..] => {
							super::push(out, 42162);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42163);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 42164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 38710);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 38711);
					match s {
						[] => true,
						['a', 'p', 'h', s @ ..] => {
							super::push(out, 42165);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 42166);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 42167);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 42168);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 42169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 32790);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 38712);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 38713);
					match s {
						[] => true,
						['o', 'ä', s @ ..] => {
							super::push(out, 42170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 38714);
					match s {
						[] => true,
						['t', 'o', s @ ..] => {
							super::push(out, 42171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 38715);
					match s {
						[] => true,
						['y', 'h', 'p', 'o', 'r', s @ ..] => {
							super::push(out, 42172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 38716);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 38717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 32791);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 38718);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 38719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 32792);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 38720);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 38721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 32793);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 38722);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 38723);
					s.is_empty()
				}
				['o', 'h', s @ ..] => {
					super::push(out, 38724);
					s.is_empty()
				}
				['s', 'o', s @ ..] => {
					super::push(out, 38725);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 38726);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 42173);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 42174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 32794);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 38727);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 38728);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 38729);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 42175);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 42176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'r', s @ ..] => {
							super::push(out, 42177);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 38730);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 42179);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 42180);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42181);
							match s {
								[] => true,
								['h', 'c', 'o', 't', s @ ..] => {
									super::push(out, 43840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 38731);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 38732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 32795);
			s.is_empty()
		}
		['p', 'o', 't', s @ ..] => {
			super::push(out, 32796);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 32797);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 38733);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					super::push(out, 38734);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 38735);
					s.is_empty()
				}
				['u', 't', s @ ..] => {
					super::push(out, 38736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', s @ ..] => {
			super::push(out, 32798);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 38737);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 42182);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 43841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 42183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'o', s @ ..] => {
					super::push(out, 38738);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 38739);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 42184);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 42185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 38740);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 38741);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 38742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 32799);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 32800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ergie.
pub(crate) fn n13576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13364);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 23992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', 'a', 'l', 'e', 'k', s @ ..] => {
			super::push(out, 23993);
			s.is_empty()
		}
		['n', 'e', s @ ..] => {
			super::push(out, 23994);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 32802);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 32803);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 38743);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 38744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', s @ ..] => {
					super::push(out, 32804);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 32805);
					match s {
						[] => true,
						['g', 'n', 'u', 't', s @ ..] => {
							super::push(out, 38745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urgie.
pub(crate) fn n13578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13366);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 23995);
			s.is_empty()
		}
		['r', 'i', 'h', 'c', 'o', s @ ..] => {
			super::push(out, 23996);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 23997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ogeographie.
pub(crate) fn n46785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43842);
	s.is_empty()
}

/// Words ending -oreographie.
pub(crate) fn n46786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43843);
	s.is_empty()
}

/// Words ending -nlithographie.
pub(crate) fn n46787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43844);
	s.is_empty()
}

/// Words ending -rthographie.
pub(crate) fn n46788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43845);
	s.is_empty()
}

/// Words ending -biographie.
pub(crate) fn n46789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43846);
	s.is_empty()
}

/// Words ending -roradiographie.
pub(crate) fn n46790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43847);
	s.is_empty()
}

/// Words ending -nangiographie.
pub(crate) fn n46791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43848);
	s.is_empty()
}

/// Words ending -lographie.
pub(crate) fn n44921(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 43849);
			s.is_empty()
		}
		['e', 'p', s @ ..] => {
			super::push(out, 43850);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 43851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nographie.
pub(crate) fn n44923(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 43852);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 43853);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 44645);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 44646);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 44647);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 44648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 43854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tographie.
pub(crate) fn n44926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42191);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 44911);
							s.is_empty()
						}
						['o', 'r', 'h', 'c', 's', 'a', s @ ..] => {
							super::push(out, 44912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 44649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'e', s @ ..] => {
			super::push(out, 43855);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 43856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äographie.
pub(crate) fn n44927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42192);
	s.is_empty()
}

/// Words ending -ophie.
pub(crate) fn n13583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13370);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 24000);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							super::push(out, 38748);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 38749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 32808);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 38750);
							s.is_empty()
						}
						['l', 'i', 'h', 'p', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 42193);
									s.is_empty()
								}
								['n', 'e', 'h', s @ ..] => {
									super::push(out, 42194);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 42195);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'b', s @ ..] => {
											super::push(out, 43857);
											s.is_empty()
										}
										['t', 'h', 'c', 'e', s @ ..] => {
											super::push(out, 43858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 38751);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 38752);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 38753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -shie.
pub(crate) fn n4996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4965);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13371);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 24001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -thie.
pub(crate) fn n4997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4966);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13373);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 24002);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32809);
							s.is_empty()
						}
						['m', 'y', 's', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38754);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 38755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', s @ ..] => {
									super::push(out, 38756);
									s.is_empty()
								}
								['m', 'r', s @ ..] => {
									super::push(out, 38757);
									s.is_empty()
								}
								['n', 'i', s @ ..] => {
									super::push(out, 38758);
									s.is_empty()
								}
								['y', 'm', 'o', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 42196);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 42197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ö', s @ ..] => {
									super::push(out, 38759);
									s.is_empty()
								}
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
			super::push(out, 13374);
			match s {
				[] => true,
				['y', 's', '\u{1}', s @ ..] => {
					super::push(out, 24003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13375);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 24004);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -aktophilie.
pub(crate) fn n46803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43859);
	s.is_empty()
}

/// Words ending -ektophilie.
pub(crate) fn n46804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43860);
	s.is_empty()
}

/// Words ending -tionstheorie.
pub(crate) fn n34544(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 42214);
					s.is_empty()
				}
				['m', 'r', 'o', 'f', 'ä', s @ ..] => {
					super::push(out, 42215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 38791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -titätstheorie.
pub(crate) fn n34545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32860);
	s.is_empty()
}

/// Words ending -ttheorie.
pub(crate) fn n24919(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 32861);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 32862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nztheorie.
pub(crate) fn n24920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24088);
	s.is_empty()
}

/// Words ending -gorie.
pub(crate) fn n13689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13467);
	s.is_empty()
}

/// Words ending -horie.
pub(crate) fn n13690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13468);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 24089);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 32863);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 32864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 24090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -morie.
pub(crate) fn n13691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13469);
	s.is_empty()
}

/// Words ending -porie.
pub(crate) fn n13692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13470);
	s.is_empty()
}

/// Words ending -historie.
pub(crate) fn n13693(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 24091);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 24092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rrie.
pub(crate) fn n5046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5011);
	match s {
		[] => true,
		['o', 'j', 't', '\u{1}', s @ ..] => {
			super::push(out, 13471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trie.
pub(crate) fn n5047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5012);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 24093);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32865);
							s.is_empty()
						}
						['h', 'c', 'y', 's', 'p', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38792);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 32866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'd', s @ ..] => {
					super::push(out, 24094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13472);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 32867);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38794);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 38795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38796);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 42216);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'h', s @ ..] => {
									super::push(out, 38797);
									s.is_empty()
								}
								['n', 'o', 'x', s @ ..] => {
									super::push(out, 38798);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38799);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42217);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 43861);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'k', 'e', 'p', 's', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 43862);
													s.is_empty()
												}
												['s', 'n', 'o', 'i', 't', 'p', s @ ..] => {
													super::push(out, 43863);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 38800);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 42218);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 42219);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 42220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 32868);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 32869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13473);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13474);
			match s {
				[] => true,
				['u', 'd', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32870);
							s.is_empty()
						}
						['k', 'a', s @ ..] => {
							super::push(out, 32871);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 32872);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 32873);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'd', s @ ..] => {
							super::push(out, 32874);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 32875);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 32876);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urie.
pub(crate) fn n5048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5013);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 13475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 'b', s @ ..] => {
			super::push(out, 13476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrie.
pub(crate) fn n5049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5014);
	match s {
		[] => true,
		['g', 'y', s @ ..] => {
			super::push(out, 13477);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 13478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ärie.
pub(crate) fn n5050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5015);
	s.is_empty()
}

/// Words ending -wangsgedanke.
pub(crate) fn n46808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43864);
	match s {
		[] => true,
		['z', '\u{1}', s @ ..] => {
			super::push(out, 44650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ingsgedanke.
pub(crate) fn n46809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43865);
	match s {
		[] => true,
		['l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 44651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ille.
pub(crate) fn n5232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13712);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 24261);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 32954);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 38824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 24262);
					match s {
						[] => true,
						['e', 'm', 's', s @ ..] => {
							super::push(out, 32955);
							match s {
								[] => true,
								['t', 'i', 'e', 'h', s @ ..] => {
									super::push(out, 38825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'r', s @ ..] => {
					super::push(out, 24263);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 24264);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 32956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13713);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13714);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 24265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24266);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 24267);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 24268);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13716);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 24269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13717);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 13718);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 24270);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 24271);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 32958);
							match s {
								[] => true,
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 38826);
									s.is_empty()
								}
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
			super::push(out, 13719);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32959);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 32960);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 32961);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 32962);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 32963);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 24272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13720);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24273);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32965);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 32966);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 32967);
							match s {
								[] => true,
								['a', 't', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 38827);
									s.is_empty()
								}
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
			super::push(out, 13721);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 24274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 13722);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 24275);
					match s {
						[] => true,
						['r', 'o', 'f', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 32968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 24276);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 24277);
					match s {
						[] => true,
						['g', 'n', 'u', 't', s @ ..] => {
							super::push(out, 32969);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38828);
									match s {
										[] => true,
										['a', 't', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 42225);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'i', 'e', 'b', s @ ..] => {
							super::push(out, 32970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 24278);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 24279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kontrolle.
pub(crate) fn n34664(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 38829);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 42226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 38830);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 42227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['r', 'h', 'e', 'k', 'r', 'e', 'v', 'g', s @ ..] => {
					super::push(out, 42228);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					super::push(out, 42229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'k', s @ ..] => {
			super::push(out, 38831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulle.
pub(crate) fn n5234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5197);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 13728);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 24285);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 24286);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 32971);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 32972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13729);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 13730);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -älle.
pub(crate) fn n5235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5198);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 13732);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 24287);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 24288);
					match s {
						[] => true,
						['g', 'n', s @ ..] => {
							super::push(out, 32973);
							match s {
								[] => true,
								['h', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 38832);
									s.is_empty()
								}
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
			super::push(out, 13733);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ölle.
pub(crate) fn n5236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5199);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 13734);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 13735);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 13736);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24289);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ülle.
pub(crate) fn n5237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5200);
	match s {
		[] => true,
		['f', 's', s @ ..] => {
			super::push(out, 13737);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13738);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 13739);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 24291);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13740);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 24292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word australopithezine.
pub(crate) fn n25251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24405);
	s.is_empty()
}

/// Words ending -kzine.
pub(crate) fn n14170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13924);
	s.is_empty()
}

/// Words ending -nzine.
pub(crate) fn n14171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13925);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 24406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nsäure.
pub(crate) fn n25399(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 33078);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 38871);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					super::push(out, 38872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 38873);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 38874);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 42237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 42238);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 42239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 38875);
					s.is_empty()
				}
				['s', 'r', s @ ..] => {
					super::push(out, 38876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 38877);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 38878);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 42240);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43866);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 43867);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 43868);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 43869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 42241);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 42242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 38879);
					s.is_empty()
				}
				['h', 'p', s @ ..] => {
					super::push(out, 38880);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 38881);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					super::push(out, 38882);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 42243);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 42244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 38883);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 38884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', 'r', 'a', 'k', s @ ..] => {
					super::push(out, 38885);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 38886);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 38887);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 38888);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 38889);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 38890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ersäure.
pub(crate) fn n34787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33080);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 38891);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 42245);
					s.is_empty()
				}
				['t', 'u', 'b', 'o', 'n', s @ ..] => {
					super::push(out, 42246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsäure.
pub(crate) fn n25402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24544);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 33082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zsäure.
pub(crate) fn n25404(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'o', s @ ..] => {
			super::push(out, 33083);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 33084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dpresse.
pub(crate) fn n34857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33144);
	s.is_empty()
}

/// Words ending -chpresse.
pub(crate) fn n34858(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 38913);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 38914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ipresse.
pub(crate) fn n34859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33145);
	s.is_empty()
}

/// Words ending -lpresse.
pub(crate) fn n34860(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 38915);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 38916);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mpresse.
pub(crate) fn n34861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33146);
	s.is_empty()
}

/// Words ending -genpresse.
pub(crate) fn n34862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33147);
	s.is_empty()
}

/// Words ending -rpresse.
pub(crate) fn n34863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33148);
	s.is_empty()
}

/// Words ending -spresse.
pub(crate) fn n34864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33149);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 38917);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tpresse.
pub(crate) fn n34865(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 38918);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 38919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ypresse.
pub(crate) fn n34866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33150);
	s.is_empty()
}

/// Words ending -tresse.
pub(crate) fn n25516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24649);
	s.is_empty()
}

/// Words ending -tesse.
pub(crate) fn n14562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14303);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 24650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -isse.
pub(crate) fn n5620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5574);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14304);
			match s {
				[] => true,
				['b', 'a', 'l', s @ ..] => {
					super::push(out, 24651);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 24652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14305);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 24653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 14306);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 14307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24654);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 24655);
					s.is_empty()
				}
				['o', 'k', 'a', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 24656);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 24657);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 24658);
					match s {
						[] => true,
						['l', 'ä', 'h', 'r', 'e', 'v', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 38920);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 38921);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 38922);
									s.is_empty()
								}
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
			super::push(out, 14308);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 14309);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osse.
pub(crate) fn n5621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5575);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14310);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 14311);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 24659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 14312);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14313);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 14314);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 14315);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14316);
			match s {
				[] => true,
				['a', 'k', 'n', s @ ..] => {
					super::push(out, 24661);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 24662);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 24663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 14317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usse.
pub(crate) fn n5622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5576);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14318);
			s.is_empty()
		}
		['b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24664);
					s.is_empty()
				}
				['n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 24665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 14319);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 14320);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14321);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 14322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ässe.
pub(crate) fn n5623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5577);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14323);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eengeschichte.
pub(crate) fn n46814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43870);
	s.is_empty()
}

/// Words ending -chengeschichte.
pub(crate) fn n46815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43871);
	s.is_empty()
}

/// Words ending -zingeschichte.
pub(crate) fn n46816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43872);
	s.is_empty()
}

/// Words ending -dlungsgeschichte.
pub(crate) fn n46817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43873);
	s.is_empty()
}

/// Words ending -itsgeschichte.
pub(crate) fn n46818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43874);
	s.is_empty()
}

/// Words ending -lätte.
pub(crate) fn n14810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14544);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 24810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ütte.
pub(crate) fn n5757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5709);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 14546);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stoff.
pub(crate) fn n15003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14728);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 24914);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 33261);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 33262);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42280);
									s.is_empty()
								}
								['l', 'e', 't', 't', 'i', 'm', 's', 'n', 'e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 42281);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['h', 't', 'n', 'a', 'x', '\u{1}', s @ ..] => {
													super::push(out, 44652);
													s.is_empty()
												}
												['p', 'ü', 'k', '\u{1}', s @ ..] => {
													super::push(out, 44653);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'n', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 43875);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['g', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 43876);
											s.is_empty()
										}
										['z', 'a', '\u{1}', s @ ..] => {
											super::push(out, 43877);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 42282);
									s.is_empty()
								}
								['s', 'n', 'o', 'i', 's', 'r', 'e', 'p', 's', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 42283);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['t', 'a', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 43878);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 43879);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'n', 'e', 'z', 's', 'e', 'r', 'o', 'u', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 42284);
									s.is_empty()
								}
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
			super::push(out, 24915);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 33263);
					match s {
						[] => true,
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 38974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 38975);
							s.is_empty()
						}
						['ü', 'z', '\u{1}', s @ ..] => {
							super::push(out, 38976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 24916);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 24917);
			match s {
				[] => true,
				['p', 'm', s @ ..] => {
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 38977);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 42285);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 42286);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43880);
											s.is_empty()
										}
										['i', 'e', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 43881);
											s.is_empty()
										}
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
			super::push(out, 24918);
			match s {
				[] => true,
				['n', 'e', 'r', 'p', 's', 'k', s @ ..] => {
					super::push(out, 33264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 24919);
			match s {
				[] => true,
				['c', 'i', 'r', s @ ..] => {
					super::push(out, 33265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 24920);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 33266);
					match s {
						[] => true,
						['i', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42287);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 42288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 33267);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42289);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 42290);
									s.is_empty()
								}
								['n', 'e', 's', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 42291);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 42292);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'u', 't', 's', 'i', 'e', 'l', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 43882);
											s.is_empty()
										}
										['n', 'o', 'i', 't', 'k', 'u', 'r', 't', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 43883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 38978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 24921);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 33268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 24922);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 33269);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 38979);
							match s {
								[] => true,
								['o', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43884);
											s.is_empty()
										}
										['r', 'o', 'l', 'h', 'c', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 43885);
											s.is_empty()
										}
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
					super::push(out, 33270);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 33271);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 33272);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 38980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t15::n25804(s, out),
		['s', s @ ..] => {
			super::push(out, 24924);
			match s {
				[] => true,
				['h', 'c', 'u', 'r', s @ ..] => {
					super::push(out, 33276);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 38989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 24925);
			match s {
				[] => true,
				['f', 'a', s @ ..] => {
					super::push(out, 33277);
					match s {
						[] => true,
						['r', 'k', 'o', s @ ..] => {
							super::push(out, 38990);
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 42293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'n', 'u', 'k', 'n', s @ ..] => {
					super::push(out, 33278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 't', 'a', 's', 'u', 'z', 'n', s @ ..] => {
			super::push(out, 24926);
			s.is_empty()
		}
		['ß', 'i', s @ ..] => {
			super::push(out, 24927);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word fleischeintopf.
pub(crate) fn n46848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43901);
	s.is_empty()
}

/// The word rindfleischeintopf.
pub(crate) fn n46849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43902);
	s.is_empty()
}

/// The word westgotenkönig.
pub(crate) fn n47690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44663);
	s.is_empty()
}

/// The word ostgotenkönig.
pub(crate) fn n47691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44664);
	s.is_empty()
}

/// Words ending -rückgang.
pub(crate) fn n26042(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33437);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 33438);
			s.is_empty()
		}
		['n', 'e', 't', 'r', 'u', 'b', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 33439);
			s.is_empty()
		}
		['r', 'u', 't', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'e', 'p', 'm', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 39078);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 39079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', 'f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 42336);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 43917);
									s.is_empty()
								}
								['r', 'e', 'k', 'l', 'ö', 'v', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 43918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 39080);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 39081);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 39082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 't', 'a', 's', 'm', 'u', '\u{1}', s @ ..] => {
			super::push(out, 33440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hlgang.
pub(crate) fn n26044(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 33442);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 39083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 33443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mgang.
pub(crate) fn n15222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14916);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 33444);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 33445);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngang.
pub(crate) fn n15223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14917);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25131);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 33446);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 39084);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 39085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 33447);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 39086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 25132);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 42337);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 43919);
											s.is_empty()
										}
										['t', 'ü', s @ ..] => {
											super::push(out, 43920);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'a', s @ ..] => {
							super::push(out, 39087);
							s.is_empty()
						}
						['s', 'g', 'a', s @ ..] => {
							super::push(out, 39088);
							match s {
								[] => true,
								['r', 't', 'f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 42338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 42339);
									s.is_empty()
								}
								['s', 'e', 'w', 'd', s @ ..] => {
									super::push(out, 42340);
									s.is_empty()
								}
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

/// Words ending -rgang.
pub(crate) fn n15224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14918);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25133);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 33448);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 33449);
					s.is_empty()
				}
				['i', 'z', 'a', 'p', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39089);
							match s {
								[] => true,
								['i', 's', 'a', 't', 'n', 'a', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 42341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'a', 't', 't', 'i', 'm', 'h', 'c', 'a', 'n', 'g', s @ ..] => {
							super::push(out, 39090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 25134);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39091);
							match s {
								[] => true,
								['d', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 42342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'n', 'u', s @ ..] => {
							match s {
								[] => true,
								['n', 'n', 'e', 'r', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 42343);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 42344);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43921);
											s.is_empty()
										}
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

/// Words ending -sgang.
pub(crate) fn n15225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14919);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 25135);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 25136);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 25137);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 25138);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 25139);
			match s {
				[] => true,
				['a', 't', 'p', s @ ..] => {
					super::push(out, 33451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tgang.
pub(crate) fn n15226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14920);
	s.is_empty()
}

/// Words ending -ugang.
pub(crate) fn n15227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14921);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33452);
					s.is_empty()
				}
				['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 33453);
					s.is_empty()
				}
				['e', 'n', 'i', 'l', 'n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 33454);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 33455);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39092);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 39093);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 39094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 33456);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zgang.
pub(crate) fn n15228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14922);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 25140);
			s.is_empty()
		}
		['u', 'e', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 25141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hang.
pub(crate) fn n6103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6042);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14924);
			match s {
				[] => true,
				['b', 'd', s @ ..] => {
					super::push(out, 25143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 14925);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14926);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 14927);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 25144);
					s.is_empty()
				}
				['u', 'l', s @ ..] => {
					super::push(out, 25145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 14928);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', 'p', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 33457);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'm', 'a', 's', 'u', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39095);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 39096);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 'h', 'c', 's', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 39097);
									s.is_empty()
								}
								['t', 'm', 'a', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 39098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'w', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 33459);
							s.is_empty()
						}
						['s', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 33460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 25146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14929);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25147);
					match s {
						[] => true,
						['b', 'ü', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39099);
									s.is_empty()
								}
								['e', 'g', 'a', 'r', 'f', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 39100);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', 'r', 'o', '\u{1}', s @ ..] => {
											super::push(out, 42345);
											s.is_empty()
										}
										['u', 'e', 't', 's', 'r', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 42346);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 't', 'o', 'b', 'e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 25148);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 39102);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39103);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 39104);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39105);
									s.is_empty()
								}
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
			super::push(out, 14930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iang.
pub(crate) fn n6104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6043);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 14931);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 14932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jang.
pub(crate) fn n6105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6044);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kang.
pub(crate) fn n6106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6045);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14934);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lang.
pub(crate) fn n6107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6046);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14935);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14936);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 14937);
			match s {
				[] => true,
				['-', 'g', 'n', 'a', 'l', 'i', '\u{1}', s @ ..] => {
					super::push(out, 25151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 14938);
			match s {
				[] => true,
				['i', 'e', 'r', s @ ..] => {
					super::push(out, 25152);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 33461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 14940);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 25154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 14941);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 25155);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mang.
pub(crate) fn n6108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6047);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rang.
pub(crate) fn n6109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6048);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14943);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 25156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 14944);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 25157);
					s.is_empty()
				}
				['s', 'g', s @ ..] => {
					super::push(out, 25158);
					match s {
						[] => true,
						['n', 'u', 't', s @ ..] => {
							super::push(out, 33462);
							match s {
								[] => true,
								['l', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 39106);
									s.is_empty()
								}
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
			super::push(out, 14945);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 25159);
					match s {
						[] => true,
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 33463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 14946);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 14947);
			match s {
				[] => true,
				['ö', s @ ..] => {
					super::push(out, 25160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14948);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25161);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14949);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 14950);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 25163);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 25164);
					match s {
						[] => true,
						['s', 'b', 'e', 'w', s @ ..] => {
							super::push(out, 33464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sang.
pub(crate) fn n6110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6049);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14951);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 14952);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 25165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'k', s @ ..] => {
			super::push(out, 14953);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tang.
pub(crate) fn n6111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6050);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14955);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 25166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14956);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 14957);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14958);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 14959);
			match s {
				[] => true,
				['e', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 25167);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14960);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 25169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 14961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uang.
pub(crate) fn n6112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6051);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 14962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wang.
pub(crate) fn n6113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6052);
	match s {
		[] => true,
		['h', 'c', 's', 'r', s @ ..] => {
			super::push(out, 14963);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 14964);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 25170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 14965);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 25171);
					match s {
						[] => true,
						['u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 33465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 25172);
					match s {
						[] => true,
						['h', 'u', s @ ..] => {
							super::push(out, 33466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 25173);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 25174);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 39107);
									s.is_empty()
								}
								['p', 'p', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42347);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 42348);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33467);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33468);
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 39109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25175);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 33470);
							match s {
								[] => true,
								['e', 'm', 'a', 'n', 'r', 'a', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 39110);
									s.is_empty()
								}
								['o', 'i', 't', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42349);
											s.is_empty()
										}
										['k', 'a', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42350);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 33471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 25176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yang.
pub(crate) fn n6114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6053);
	s.is_empty()
}

/// Words ending -thing.
pub(crate) fn n15296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14988);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25214);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 25215);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 33487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 25216);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33488);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 33489);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -king.
pub(crate) fn n6125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6064);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14989);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 14990);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 33490);
							match s {
								[] => true,
								['r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 39114);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 33491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25217);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 25218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 14991);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 14992);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 25219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 14993);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 25220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 14994);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25221);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33492);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33493);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 25222);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 33495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14995);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 25223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14996);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25224);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14997);
			match s {
				[] => true,
				['a', 't', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 25226);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ling.
pub(crate) fn n6126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6065);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14998);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 25227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 14999);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 25228);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 33497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', 'l', 's', 'g', s @ ..] => {
					super::push(out, 25229);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 25230);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 15000);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 25231);
					match s {
						[] => true,
						['c', 'p', s @ ..] => {
							super::push(out, 33499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 15001);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 25232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15002);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 25233);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 25234);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 33500);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 33501);
							match s {
								[] => true,
								['r', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 39115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 15003);
			match s {
				[] => true,
				['p', 'ö', s @ ..] => {
					super::push(out, 25236);
					match s {
						[] => true,
						['r', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 25237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 15004);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 33503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', 'j', '\u{1}', s @ ..] => {
							super::push(out, 33504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15005);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 33505);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 39117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 33506);
							match s {
								[] => true,
								['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 39118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 25238);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 25239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 15006);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25240);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 33507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 25241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 15007);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33508);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 39119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 33509);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 33510);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 39120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 25242);
					match s {
						[] => true,
						['n', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 33511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15008);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25243);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 25244);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 33512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33513);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 25245);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 33515);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 39122);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 39123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 33516);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 39124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 25246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15009);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 15010);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 15011);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 25247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15012);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25248);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 33517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25249);
					match s {
						[] => true,
						['b', 'i', s @ ..] => {
							super::push(out, 33518);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 33519);
							match s {
								[] => true,
								['c', 'ä', s @ ..] => {
									super::push(out, 39125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39126);
									s.is_empty()
								}
								['t', 'e', s @ ..] => {
									super::push(out, 39127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 25250);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39128);
									s.is_empty()
								}
								['r', 'e', 'b', s @ ..] => {
									super::push(out, 39129);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 42351);
											match s {
												[] => true,
												['a', 'z', '\u{1}', s @ ..] => {
													super::push(out, 43922);
													s.is_empty()
												}
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
					super::push(out, 25251);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 33520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 15013);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 25252);
					match s {
						[] => true,
						['ö', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 39130);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39131);
									match s {
										[] => true,
										['p', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43923);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 43924);
													s.is_empty()
												}
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
		['t', s @ ..] => super::t15::n15322(s, out),
		['u', s @ ..] => super::t15::n15323(s, out),
		['w', s @ ..] => super::t15::n15324(s, out),
		['y', s @ ..] => super::t15::n15325(s, out),
		['z', s @ ..] => {
			super::push(out, 15018);
			s.is_empty()
		}
		['ß', s @ ..] => super::t15::n15327(s, out),
		_ => false,
	}
}

/// Words ending -ming.
pub(crate) fn n6127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6066);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15020);
			match s {
				[] => true,
				['e', 'r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 33526);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', '-', 'e', 'n', 'i', 'l', 'n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 25262);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 25263);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['-', 'y', 'd', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39136);
									s.is_empty()
								}
								['y', 'd', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39137);
									s.is_empty()
								}
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
			super::push(out, 15021);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 15022);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 25265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15023);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25266);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25267);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 33528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15024);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 33529);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ning.
pub(crate) fn n6128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6067);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15025);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 25268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15026);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 15027);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 33531);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33532);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39138);
									match s {
										[] => true,
										['e', 'i', s @ ..] => {
											super::push(out, 42355);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 't', s @ ..] => {
									super::push(out, 39139);
									match s {
										[] => true,
										['i', 'e', 'k', s @ ..] => {
											super::push(out, 42356);
											match s {
												[] => true,
												['h', 'c', 'i', 'l', 'g', 'e', 'w', 'e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 43926);
													s.is_empty()
												}
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
					super::push(out, 25269);
					match s {
						[] => true,
						['-', 'a', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 33533);
							s.is_empty()
						}
						['a', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 33534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15028);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25270);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 33535);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 25271);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 33537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15029);
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 25272);
					s.is_empty()
				}
				['o', 'c', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 25273);
					s.is_empty()
				}
				['y', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 25274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15030);
			match s {
				[] => true,
				['a', 'e', 'l', 'e', s @ ..] => {
					super::push(out, 25275);
					match s {
						[] => true,
						['l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 33538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 25276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 15031);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 25277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ping.
pub(crate) fn n6129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6068);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 15032);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15033);
			match s {
				[] => true,
				['u', 'j', 'e', 'e', s @ ..] => {
					super::push(out, 25279);
					match s {
						[] => true,
						['g', 'n', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 33540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15034);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 25280);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25281);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 33541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 15035);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25282);
					match s {
						[] => true,
						['n', 'd', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 33542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25283);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 33543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 25284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', '-', s @ ..] => {
			super::push(out, 15036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ring.
pub(crate) fn n6130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6069);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15037);
			s.is_empty()
		}
		['-', s @ ..] => {
			match s {
				[] => true,
				['l', 'w', 'o', 'b', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 25285);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 25286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 15038);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25287);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 33544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 25288);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									match s {
										[] => true,
										['e', 'k', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42357);
											s.is_empty()
										}
										['r', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 42358);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'k', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39140);
									s.is_empty()
								}
								['r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 39141);
									s.is_empty()
								}
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
			super::push(out, 15039);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 15040);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 25289);
					s.is_empty()
				}
				['l', 'o', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33545);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 33546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 25290);
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 25291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15041);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 25292);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 25293);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 39142);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25294);
					match s {
						[] => true,
						['f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 33547);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33548);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 39144);
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 42359);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 39145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 33549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'n', 'o', 'i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 25295);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 25296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33550);
							s.is_empty()
						}
						['k', 'c', 'r', 'a', 'm', 's', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 33551);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33552);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39146);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 39147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 25297);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33553);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 15042);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 25298);
					match s {
						[] => true,
						['ü', 'f', '\u{1}', s @ ..] => {
							super::push(out, 33555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 15043);
			s.is_empty()
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 25299);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 25300);
					s.is_empty()
				}
				['u', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 25301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', 'm', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 15044);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 15045);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15046);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					super::push(out, 25302);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['m', 'i', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 25304);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 25305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15047);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 33556);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33557);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33558);
							s.is_empty()
						}
						['x', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 33559);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 33560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15048);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 25306);
					s.is_empty()
				}
				['n', 'r', 'o', 'p', 'r', 'e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 25307);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25308);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 25309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15049);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25310);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 33561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25311);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 33562);
							s.is_empty()
						}
						['l', 'd', s @ ..] => {
							super::push(out, 33563);
							s.is_empty()
						}
						['u', 'e', s @ ..] => {
							super::push(out, 33564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 15050);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 25312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15051);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 25313);
					s.is_empty()
				}
				['f', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 25314);
					s.is_empty()
				}
				['h', 'c', 'i', 'd', 'n', 'e', 'l', 'l', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 25315);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 25316);
					match s {
						[] => true,
						['a', 'l', 'l', 'i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 33565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25317);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 33566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15052);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 25318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 15053);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 15054);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 15055);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 25319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sing.
pub(crate) fn n6131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6070);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15056);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25320);
					s.is_empty()
				}
				['u', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 25321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15057);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 15058);
			match s {
				[] => true,
				['a', 'v', s @ ..] => {
					super::push(out, 25322);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 33567);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 33568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ting.
pub(crate) fn n6132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6071);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15059);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 15060);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['-', 'e', s @ ..] => {
							super::push(out, 33569);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 33570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15061);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25324);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 25325);
					match s {
						[] => true,
						['r', 'a', 'm', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39148);
									s.is_empty()
								}
								['s', 'h', 'c', 'u', 'r', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 39149);
									s.is_empty()
								}
								['t', 'd', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 39150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'a', s @ ..] => {
			super::push(out, 15062);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 15063);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 15064);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15065);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 15066);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 25326);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							match s {
								[] => true,
								['-', 'n', 'o', 'i', 't', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39151);
									s.is_empty()
								}
								['n', 'o', 'i', 't', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15067);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 15068);
			match s {
				[] => true,
				['a', 'c', 'd', s @ ..] => {
					super::push(out, 25328);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25329);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25330);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25331);
					match s {
						[] => true,
						['h', 'g', s @ ..] => {
							super::push(out, 33572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39153);
									s.is_empty()
								}
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
			super::push(out, 15069);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25332);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25333);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 33573);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 33574);
							match s {
								[] => true,
								['p', 's', 'n', 'e', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 42360);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 42361);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 42362);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33575);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39154);
									match s {
										[] => true,
										['c', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42363);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 39155);
									s.is_empty()
								}
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
			super::push(out, 15070);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 15071);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 25334);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 33576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ving.
pub(crate) fn n6133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6072);
	match s {
		[] => true,
		['a', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 15072);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 25335);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 25336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15073);
			match s {
				[] => true,
				['o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 25337);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xing.
pub(crate) fn n6135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6074);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25338);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33577);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 25339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 15076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ying.
pub(crate) fn n6136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6075);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 15077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zing.
pub(crate) fn n6137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6076);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 15078);
			s.is_empty()
		}
		['n', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 15079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ldung.
pub(crate) fn n15416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15102);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25348);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 33585);
					match s {
						[] => true,
						['f', 'ü', 'b', '\u{1}', s @ ..] => {
							super::push(out, 39160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 33586);
					match s {
						[] => true,
						['o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 39161);
							s.is_empty()
						}
						['ü', 'l', 'f', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 39162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 33587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 25349);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 39163);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 42371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 39164);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 42372);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 42373);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 42374);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 42375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 39165);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 39166);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'd', s @ ..] => {
											super::push(out, 43927);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 43928);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 43929);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'e', 'm', s @ ..] => {
									super::push(out, 42376);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 42377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 39167);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 42378);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 43930);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', s @ ..] => {
									super::push(out, 42379);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 42380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', s @ ..] => {
							super::push(out, 39168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 25350);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33588);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 39169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 25351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -endung.
pub(crate) fn n26285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25353);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 33589);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 33590);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 39170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'r', s @ ..] => {
			super::push(out, 33591);
			match s {
				[] => true,
				['e', 'v', 'r', s @ ..] => {
					super::push(out, 39171);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 42381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -indung.
pub(crate) fn n26286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25354);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 39172);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 42382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 39173);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 39174);
					match s {
						[] => true,
						['i', 'e', 'r', 'p', 'h', s @ ..] => {
							super::push(out, 42383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 't', s @ ..] => {
			super::push(out, 33592);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sundung.
pub(crate) fn n26287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25355);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 33593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ündung.
pub(crate) fn n26288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25356);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 33594);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39176);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 39177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 33595);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 33596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rdung.
pub(crate) fn n15419(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'w', 'k', s @ ..] => {
			super::push(out, 25357);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 25358);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 33597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fung.
pub(crate) fn n6149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6088);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 15106);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 33598);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33599);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 33600);
							match s {
								[] => true,
								['d', 's', 's', s @ ..] => {
									super::push(out, 39179);
									s.is_empty()
								}
								['k', 'e', 'b', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'a', 'r', 'b', 'd', s @ ..] => {
											super::push(out, 42384);
											s.is_empty()
										}
										['n', 'e', 'g', s @ ..] => {
											super::push(out, 42385);
											s.is_empty()
										}
										['s', 'n', 'o', 'i', 't', 'p', s @ ..] => {
											super::push(out, 42386);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 25359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			super::push(out, 15107);
			match s {
				[] => true,
				['w', 'r', 'e', 'v', '-', s @ ..] => {
					super::push(out, 25360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 15108);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gung.
pub(crate) fn n6150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6089);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15109);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 15110);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 25361);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 25362);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 39180);
									s.is_empty()
								}
								['d', 'r', s @ ..] => {
									super::push(out, 39181);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 42387);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 'g', s @ ..] => {
									super::push(out, 39182);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 39183);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42388);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 39184);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39185);
									match s {
										[] => true,
										['e', 'z', s @ ..] => {
											super::push(out, 42389);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'g', 'n', 'u', 'k', s @ ..] => {
									super::push(out, 39186);
									s.is_empty()
								}
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
			super::push(out, 15111);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 25363);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33601);
							match s {
								[] => true,
								['i', 'r', 'f', 'e', 'b', 't', s @ ..] => {
									super::push(out, 39187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33602);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42390);
											s.is_empty()
										}
										['t', 'r', 'e', 'v', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 43931);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 43932);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'ä', 't', 's', 'r', 'e', 'v', 'r', s @ ..] => {
							super::push(out, 33603);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 33604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 's', s @ ..] => {
					super::push(out, 25364);
					match s {
						[] => true,
						['g', 'n', 'u', 't', 'u', s @ ..] => {
							super::push(out, 33605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', 'i', 'b', s @ ..] => {
					super::push(out, 25365);
					s.is_empty()
				}
				['n', 'i', 'e', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 33606);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 39188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 33607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25366);
					match s {
						[] => true,
						['s', 'ü', 'l', 'f', 'r', 'e', 'v', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39189);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 25367);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 33608);
							match s {
								[] => true,
								['ä', 'h', 'c', 's', 'e', 'b', 'r', s @ ..] => {
									super::push(out, 39191);
									match s {
										[] => true,
										['e', 't', 'n', s @ ..] => {
											super::push(out, 42391);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'c', 'e', s @ ..] => {
							super::push(out, 33609);
							match s {
								[] => true,
								['r', 'e', 'b', 'h', s @ ..] => {
									super::push(out, 39192);
									s.is_empty()
								}
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
			super::push(out, 15112);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25368);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 33610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25369);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 33611);
							s.is_empty()
						}
						['r', 'b', 'r', 'e', 't', s @ ..] => {
							super::push(out, 33612);
							match s {
								[] => true,
								['n', 'u', 's', s @ ..] => {
									super::push(out, 39193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'z', s @ ..] => {
							super::push(out, 33613);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 39194);
									s.is_empty()
								}
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
			super::push(out, 15113);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33614);
							s.is_empty()
						}
						['w', 'z', 'r', 'e', 'v', 'l', s @ ..] => {
							super::push(out, 33615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', 'r', 'e', 'v', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', s @ ..] => {
							super::push(out, 33616);
							s.is_empty()
						}
						['f', 'f', 'o', 't', 's', 'f', s @ ..] => {
							super::push(out, 33617);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 33618);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 't', 'i', 'm', 'i', s @ ..] => {
									super::push(out, 39195);
									s.is_empty()
								}
								['l', 'o', s @ ..] => {
									super::push(out, 39196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 33619);
							match s {
								[] => true,
								['o', 'r', 't', 's', 'n', s @ ..] => {
									super::push(out, 39197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', s @ ..] => {
							super::push(out, 33620);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							super::push(out, 33621);
							s.is_empty()
						}
						['t', 's', 'b', s @ ..] => {
							super::push(out, 33622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15114);
			match s {
				[] => true,
				['e', 'z', 'r', 'e', s @ ..] => {
					super::push(out, 25370);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 33623);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 39198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', 'w', 'b', 'a', 'r', s @ ..] => {
			super::push(out, 15115);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 15116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hung.
pub(crate) fn n6151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6090);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25372);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 33625);
							match s {
								[] => true,
								['r', 'e', 'b', 'ü', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42392);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 43933);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 42393);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 43934);
													s.is_empty()
												}
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
					super::push(out, 25373);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25374);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'g', 't', s @ ..] => {
									super::push(out, 39199);
									s.is_empty()
								}
								['r', 'r', 'e', s @ ..] => {
									super::push(out, 39200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'r', s @ ..] => {
									super::push(out, 39201);
									s.is_empty()
								}
								['r', 'r', 'e', 'h', 'r', 'e', 'v', 'd', s @ ..] => {
									super::push(out, 39202);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25375);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33626);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 33627);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 33628);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33629);
							match s {
								[] => true,
								['o', 'f', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', s @ ..] => {
											super::push(out, 42394);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 42395);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 42396);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 43935);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 42397);
											s.is_empty()
										}
										['l', 'e', 'b', 'a', s @ ..] => {
											super::push(out, 42398);
											s.is_empty()
										}
										['m', 'u', 'l', s @ ..] => {
											super::push(out, 42399);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42400);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['g', s @ ..] => super::t15::n47692(s, out),
														['r', s @ ..] => super::t15::n47693(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 42401);
											match s {
												[] => true,
												['e', 'h', s @ ..] => {
													super::push(out, 43936);
													s.is_empty()
												}
												['h', s @ ..] => {
													super::push(out, 43937);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 43938);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 43939);
													s.is_empty()
												}
												['i', 'n', 't', s @ ..] => {
													super::push(out, 43940);
													s.is_empty()
												}
												['n', s @ ..] => {
													match s {
														[] => true,
														['e', 'm', s @ ..] => super::t15::n47694(s, out),
														['o', s @ ..] => super::t15::n47695(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													match s {
														[] => true,
														['f', 'n', s @ ..] => super::t15::n47696(s, out),
														['l', s @ ..] => super::t15::n47697(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => super::t15::n45159(s, out),
										['z', s @ ..] => super::t15::n45160(s, out),
										_ => false,
									}
								}
								['r', s @ ..] => super::t15::n41590(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 33630);
							s.is_empty()
						}
						['u', s @ ..] => super::t15::n35401(s, out),
						['ö', s @ ..] => super::t15::n35402(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t15::n26313(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t15::n15434(s, out),
		['i', 'e', 'z', s @ ..] => super::t15::n15435(s, out),
		['o', s @ ..] => super::t15::n15436(s, out),
		['ä', s @ ..] => super::t15::n15437(s, out),
		['ö', s @ ..] => super::t15::n15438(s, out),
		_ => false,
	}
}

/// Words ending -jung.
pub(crate) fn n6152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6091);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kung.
pub(crate) fn n6153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6092);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25382);
					s.is_empty()
				}
				['e', 'd', 't', s @ ..] => {
					super::push(out, 25383);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 33639);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 33640);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39206);
									s.is_empty()
								}
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
			super::push(out, 15123);
			match s {
				[] => true,
				['a', 'k', 't', 'n', 'e', 'n', s @ ..] => {
					super::push(out, 25384);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'k', 'r', 'e', 'n', 'e', s @ ..] => {
					super::push(out, 25386);
					match s {
						[] => true,
						['g', 'n', 'u', 'l', 'b', s @ ..] => {
							super::push(out, 33641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33642);
							match s {
								[] => true,
								['a', 'g', s @ ..] => {
									super::push(out, 39207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 33643);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 39208);
									s.is_empty()
								}
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
				['i', 'w', 'r', s @ ..] => {
					super::push(out, 25387);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 33644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 25388);
					match s {
						[] => true,
						['t', 's', 'r', 'e', 'v', 'n', 'u', s @ ..] => {
							super::push(out, 33645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lung.
pub(crate) fn n6154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6093);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 15124);
			match s {
				[] => true,
				['e', 'i', 's', 't', s @ ..] => {
					super::push(out, 25389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15125);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25390);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33646);
							s.is_empty()
						}
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 33647);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 33648);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 25391);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39210);
									s.is_empty()
								}
								['r', 'r', s @ ..] => {
									super::push(out, 39211);
									match s {
										[] => true,
										['e', 'l', s @ ..] => {
											super::push(out, 42412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 33649);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 39212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'i', s @ ..] => {
					super::push(out, 25392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'n', s @ ..] => {
			super::push(out, 15126);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15127);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 33650);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 42413);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 42414);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 42415);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 43947);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 42416);
											s.is_empty()
										}
										['n', 'e', 'n', 'n', s @ ..] => {
											super::push(out, 42417);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 43948);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 42418);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 33651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25394);
					s.is_empty()
				}
				['ä', 'z', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 33652);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 33653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 25395);
					match s {
						[] => true,
						['f', 'm', s @ ..] => {
							super::push(out, 33654);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 39213);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42419);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 39214);
									s.is_empty()
								}
								['s', 'n', 'o', 'i', 't', s @ ..] => {
									super::push(out, 39215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25396);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'd', s @ ..] => {
									super::push(out, 39216);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 42420);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 39217);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 42421);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 33656);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39218);
									s.is_empty()
								}
								['i', 'e', 's', 'n', s @ ..] => {
									super::push(out, 39219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', 's', 'i', s @ ..] => {
							super::push(out, 33657);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'c', 'i', 'w', 't', 'n', 'e', s @ ..] => {
			match s {
				[] => true,
				['l', 'u', s @ ..] => {
					super::push(out, 25397);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 33658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 't', s @ ..] => {
					super::push(out, 25398);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 33659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15128);
			match s {
				[] => true,
				['e', 't', 's', s @ ..] => super::t15::n26342(s, out),
				['ä', 'f', 'm', s @ ..] => {
					super::push(out, 25399);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 25400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15129);
			match s {
				[] => true,
				['m', 'a', 's', 'r', 'e', 'v', 'l', s @ ..] => {
					super::push(out, 25401);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15130);
			match s {
				[] => true,
				['h', 'n', s @ ..] => {
					super::push(out, 25402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mung.
pub(crate) fn n6155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6094);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25403);
					s.is_empty()
				}
				['ä', 'l', 'r', s @ ..] => {
					super::push(out, 25404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15132);
			match s {
				[] => true,
				['i', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 39231);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									super::push(out, 39232);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'o', s @ ..] => {
							super::push(out, 33668);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 33669);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'i', s @ ..] => {
							super::push(out, 33670);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 33671);
							s.is_empty()
						}
						['s', 'g', 'n', 'a', 'g', 'r', 'e', 't', 'n', 'u', 't', s @ ..] => {
							super::push(out, 33672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'd', 'l', s @ ..] => {
					super::push(out, 25405);
					match s {
						[] => true,
						['l', 'a', 'h', 'c', 's', 't', s @ ..] => {
							super::push(out, 33673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15133);
			match s {
				[] => true,
				['r', 't', 's', 'r', 'e', 'v', 'e', s @ ..] => {
					super::push(out, 25406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 25407);
					s.is_empty()
				}
				['ä', 'w', 'r', 'e', 'a', s @ ..] => {
					super::push(out, 25408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15134);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33674);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 39234);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 39235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 33675);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 15135);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nung.
pub(crate) fn n6156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6095);
	match s {
		[] => true,
		['a', 'l', 'p', 'n', 'e', 'i', s @ ..] => {
			super::push(out, 15136);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 15137);
			match s {
				[] => true,
				['r', 'o', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 33677);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 33678);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 33679);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33680);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'u', 'r', 'g', '-', s @ ..] => {
											super::push(out, 42424);
											s.is_empty()
										}
										['n', 'e', 'm', s @ ..] => {
											super::push(out, 42425);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42426);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 43958);
													match s {
														[] => true,
														['n', 'u', s @ ..] => super::t15::n47700(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 43959);
													s.is_empty()
												}
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
							super::push(out, 33681);
							s.is_empty()
						}
						['u', 'a', 'b', 'r', s @ ..] => {
							super::push(out, 33682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'i', 'd', 'e', 'b', 't', s @ ..] => {
			super::push(out, 15138);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 25409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 15139);
			match s {
				[] => true,
				['f', 'a', s @ ..] => {
					super::push(out, 25410);
					match s {
						[] => true,
						['w', 'e', 'b', 't', s @ ..] => {
							super::push(out, 33683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', 'e', 'r', 'h', s @ ..] => {
					super::push(out, 25411);
					s.is_empty()
				}
				['u', 'e', 'l', 's', s @ ..] => {
					super::push(out, 25412);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 33684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15140);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33685);
							match s {
								[] => true,
								['r', 'l', s @ ..] => {
									super::push(out, 39236);
									match s {
										[] => true,
										['a', 'm', s @ ..] => {
											super::push(out, 42427);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 25413);
					s.is_empty()
				}
				['ö', 's', s @ ..] => {
					super::push(out, 25414);
					match s {
						[] => true,
						['s', 'u', 'a', 'l', s @ ..] => {
							super::push(out, 33687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 15141);
			match s {
				[] => true,
				['e', 'm', 'm', s @ ..] => {
					super::push(out, 25415);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 15142);
			match s {
				[] => true,
				['c', 'o', 'r', 't', 'r', s @ ..] => {
					super::push(out, 25416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15143);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25417);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 33689);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 42428);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42429);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 43960);
													s.is_empty()
												}
												['h', s @ ..] => {
													super::push(out, 43961);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 42430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 33690);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', 'r', 'e', 'v', 'l', 'l', 'ü', s @ ..] => {
									super::push(out, 39237);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 42431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42432);
											match s {
												[] => true,
												['l', 'ü', s @ ..] => {
													super::push(out, 43962);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'e', 't', s @ ..] => {
											super::push(out, 42433);
											s.is_empty()
										}
										['r', 'e', 't', 'ü', s @ ..] => {
											super::push(out, 42434);
											s.is_empty()
										}
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
					super::push(out, 25418);
					match s {
						[] => true,
						['w', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39238);
									s.is_empty()
								}
								['f', 'f', 'o', 't', 's', 'r', s @ ..] => {
									super::push(out, 39239);
									s.is_empty()
								}
								['k', 'c', 'ü', 'r', s @ ..] => {
									match s {
										[] => true,
										['e', 'i', s @ ..] => super::t15::n45194(s, out),
										['u', s @ ..] => super::t15::n45195(s, out),
										_ => false,
									}
								}
								['l', s @ ..] => super::t15::n41633(s, out),
								['m', s @ ..] => super::t15::n41634(s, out),
								['n', s @ ..] => super::t15::n41635(s, out),
								['r', 'e', 's', s @ ..] => super::t15::n41636(s, out),
								['s', s @ ..] => super::t15::n41637(s, out),
								['z', 'l', s @ ..] => super::t15::n41638(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t15::n15469(s, out),
		['ö', 'h', 'c', 's', 'u', s @ ..] => super::t15::n15470(s, out),
		_ => false,
	}
}

/// Words ending -eppung.
pub(crate) fn n6157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6096);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 15146);
			s.is_empty()
		}
		_ => false,
	}
}

