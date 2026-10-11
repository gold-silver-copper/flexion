use flexion::Path;

/// Words ending -icht.
pub(crate) fn n10661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10538);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 21332);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 21333);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 30828);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['s', 't', s @ ..] => {
									super::push(out, 41277);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 43429);
											match s {
												[] => true,
												['c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
													super::push(out, 44409);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 41278);
									s.is_empty()
								}
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
			super::push(out, 21334);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 21335);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 30829);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 41279);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 43430);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', s @ ..] => {
									super::push(out, 41280);
									s.is_empty()
								}
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
			super::push(out, 21336);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 30830);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							match s {
								[] => true,
								['d', 'l', 'a', 'w', 'r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 41281);
									s.is_empty()
								}
								['l', 'e', 'g', 'n', 'u', 'h', 'c', 's', 'd', '\u{1}', s @ ..] => {
									super::push(out, 41282);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['n', 'r', 'o', 'd', '\u{1}', s @ ..] => {
													super::push(out, 44410);
													s.is_empty()
												}
												['v', 'o', 'r', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 44411);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 43431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', s @ ..] => {
									match s {
										[] => true,
										['b', 'm', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 43432);
											s.is_empty()
										}
										['r', 'y', 'p', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 43433);
											s.is_empty()
										}
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
			super::push(out, 21337);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30831);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 30832);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 30833);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 30834);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37335);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 37336);
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 41283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 30835);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 43434);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 43435);
											match s {
												[] => true,
												['b', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 44412);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 41284);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 43436);
											s.is_empty()
										}
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
					super::push(out, 30836);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37337);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 37338);
							s.is_empty()
						}
						['i', s @ ..] => super::t15::n39487(s, out),
						_ => false,
					}
				}
				['f', s @ ..] => super::t15::n32256(s, out),
				['g', s @ ..] => super::t15::n32257(s, out),
				['h', s @ ..] => super::t15::n32258(s, out),
				['i', s @ ..] => super::t15::n32259(s, out),
				['k', s @ ..] => super::t15::n32260(s, out),
				['l', s @ ..] => super::t15::n32261(s, out),
				['m', s @ ..] => super::t15::n32262(s, out),
				['n', s @ ..] => super::t15::n32263(s, out),
				['o', s @ ..] => super::t15::n32264(s, out),
				['p', s @ ..] => super::t15::n32265(s, out),
				['r', s @ ..] => super::t15::n32266(s, out),
				['s', s @ ..] => super::t15::n32267(s, out),
				['t', s @ ..] => super::t15::n32268(s, out),
				['u', s @ ..] => super::t15::n32269(s, out),
				['z', s @ ..] => super::t15::n32270(s, out),
				['ß', s @ ..] => super::t15::n32271(s, out),
				['ü', s @ ..] => super::t15::n32272(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t15::n21955(s, out),
		['r', s @ ..] => super::t15::n21956(s, out),
		['s', s @ ..] => super::t15::n21957(s, out),
		['w', s @ ..] => super::t15::n21958(s, out),
		['z', s @ ..] => super::t15::n21959(s, out),
		_ => false,
	}
}

/// Words ending -docht.
pub(crate) fn n21960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21343);
	s.is_empty()
}

/// Words ending -rcht.
pub(crate) fn n10663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10540);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21344);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 21345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -scht.
pub(crate) fn n10664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10541);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21346);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 21347);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 30871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 21348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucht.
pub(crate) fn n10665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10542);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21349);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 21350);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 30872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 21351);
			match s {
				[] => true,
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 30873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 21352);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 30874);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 37379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21353);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 37380);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 37381);
							s.is_empty()
						}
						['l', 'l', 'a', 'f', 'n', 'u', 's', s @ ..] => {
							super::push(out, 37382);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 41343);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 41344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 37383);
							match s {
								[] => true,
								['t', 'ä', s @ ..] => {
									super::push(out, 41345);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 41346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'l', s @ ..] => {
							super::push(out, 37384);
							s.is_empty()
						}
						['u', 'z', s @ ..] => {
							super::push(out, 37385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 30875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 21354);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37386);
							match s {
								[] => true,
								['p', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 41347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', s @ ..] => {
							super::push(out, 37387);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 41348);
									s.is_empty()
								}
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
			super::push(out, 21355);
			match s {
				[] => true,
				['f', 'a', s @ ..] => {
					super::push(out, 30876);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 30877);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 37388);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 37389);
							match s {
								[] => true,
								['e', 's', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43487);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 43488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 37390);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 30878);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 30879);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 30880);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 30881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 21356);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 21357);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 30882);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 30883);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37392);
							s.is_empty()
						}
						['u', 'a', 'r', s @ ..] => {
							super::push(out, 37393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 41349);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 41350);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 37394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 30884);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 30885);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37395);
							s.is_empty()
						}
						['e', 'h', s @ ..] => {
							super::push(out, 37396);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 37397);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 37398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 37399);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 37400);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43489);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 43490);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 37401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', s @ ..] => {
					super::push(out, 30886);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 30887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ächt.
pub(crate) fn n10666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10543);
	s.is_empty()
}

/// Words ending -ücht.
pub(crate) fn n10667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10544);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 21358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -heit.
pub(crate) fn n10701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10578);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 30910);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 30911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 21394);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 30912);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37417);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 37418);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37419);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 41355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 37420);
							s.is_empty()
						}
						['z', 'l', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 37421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 21395);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30913);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 30914);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 30915);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37422);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 37423);
							match s {
								[] => true,
								['l', 'b', 's', 'b', s @ ..] => {
									super::push(out, 41356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 41357);
									s.is_empty()
								}
								['n', 'u', s @ ..] => {
									super::push(out, 41358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 30916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['f', 'a', 'r', s @ ..] => {
					super::push(out, 30917);
					s.is_empty()
				}
				['p', 'm', 'u', 'd', s @ ..] => {
					super::push(out, 30918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 30919);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 30920);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 30921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37424);
							match s {
								[] => true,
								['w', 'h', s @ ..] => {
									super::push(out, 41359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['l', 'g', 'n', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', s @ ..] => {
											super::push(out, 43491);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 43492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 41360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 41361);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 41362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 30922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 21396);
			match s {
				[] => true,
				['e', 'r', 'f', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37425);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 37426);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 37427);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 41363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 41364);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 41365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 37428);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 41366);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 41367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 41368);
									match s {
										[] => true,
										['n', 'u', 'l', s @ ..] => {
											super::push(out, 43493);
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
												['l', s @ ..] => {
													super::push(out, 44434);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 44435);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'i', 'g', s @ ..] => {
											super::push(out, 43494);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', 'n', s @ ..] => {
							super::push(out, 37429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 21397);
			match s {
				[] => true,
				['n', 'a', s @ ..] => super::t15::n32354(s, out),
				['s', s @ ..] => {
					super::push(out, 30923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30924);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 30925);
					s.is_empty()
				}
				['h', 'o', s @ ..] => {
					super::push(out, 30926);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 37431);
							s.is_empty()
						}
						['o', 't', 's', s @ ..] => {
							super::push(out, 37432);
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
				['h', 'a', s @ ..] => {
					super::push(out, 30927);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', s @ ..] => {
							super::push(out, 37433);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 37434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t15::n22024(s, out),
		['o', 'h', s @ ..] => super::t15::n22025(s, out),
		['p', 'p', 'a', 'n', 'k', s @ ..] => super::t15::n22026(s, out),
		['r', s @ ..] => super::t15::n22027(s, out),
		['s', s @ ..] => super::t15::n22028(s, out),
		['t', s @ ..] => super::t15::n22029(s, out),
		['u', s @ ..] => super::t15::n22030(s, out),
		['v', s @ ..] => super::t15::n22031(s, out),
		['ß', 'ä', s @ ..] => super::t15::n22032(s, out),
		_ => false,
	}
}

/// Words ending -keit.
pub(crate) fn n10703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10580);
	match s {
		[] => true,
		['g', 'i', s @ ..] => super::t16::n22034(s, out),
		['h', s @ ..] => {
			super::push(out, 21406);
			match s {
				[] => true,
				['c', s @ ..] => super::t16::n32411(s, out),
				_ => false,
			}
		}
		['l', 'e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 30965);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 30966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 's', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 30967);
					match s {
						[] => true,
						['e', 'i', 'l', s @ ..] => {
							super::push(out, 37510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'e', s @ ..] => {
					super::push(out, 30968);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37511);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37512);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 37513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', s @ ..] => {
					super::push(out, 30969);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 30970);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 30971);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37514);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 37515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 37516);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 37517);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 41477);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 41478);
									s.is_empty()
								}
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
			super::push(out, 21407);
			match s {
				[] => true,
				['a', 'b', s @ ..] => super::t16::n32422(s, out),
				['e', s @ ..] => super::t16::n32423(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zeit.
pub(crate) fn n10707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10584);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 21413);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 30977);
					s.is_empty()
				}
				['r', 'd', s @ ..] => {
					super::push(out, 30978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', 'g', s @ ..] => {
			super::push(out, 21414);
			s.is_empty()
		}
		['d', 'a', s @ ..] => {
			super::push(out, 21415);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 21416);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 37529);
							s.is_empty()
						}
						['n', 'e', 'w', 'h', s @ ..] => {
							super::push(out, 37530);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 37531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 30979);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					super::push(out, 30980);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 30981);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 30982);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 30983);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 30984);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 30985);
					match s {
						[] => true,
						['n', 'o', 'r', 'b', 'l', s @ ..] => {
							super::push(out, 37532);
							match s {
								[] => true,
								['e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 41512);
									s.is_empty()
								}
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
			super::push(out, 21417);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 21418);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 21419);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 37533);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 37534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', s @ ..] => {
					super::push(out, 30986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', s @ ..] => {
					super::push(out, 30987);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					super::push(out, 30988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 37535);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 37536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'g', s @ ..] => {
					super::push(out, 30989);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 30990);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37537);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 37538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 30991);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 30992);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 37539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 30993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 21420);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 30994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21421);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'e', s @ ..] => {
							super::push(out, 37540);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 37541);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									super::push(out, 41513);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 41514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'r', s @ ..] => {
							super::push(out, 37542);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 37543);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 37544);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 41515);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 41516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'n', s @ ..] => {
							super::push(out, 37545);
							s.is_empty()
						}
						['s', 'i', 'e', s @ ..] => {
							super::push(out, 37546);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['a', 'f', 'r', s @ ..] => {
									super::push(out, 41517);
									s.is_empty()
								}
								['ü', s @ ..] => {
									super::push(out, 41518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 37547);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 41519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 30995);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 30996);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 30997);
					s.is_empty()
				}
				['u', 's', s @ ..] => {
					super::push(out, 30998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 30999);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 31000);
					s.is_empty()
				}
				['o', 'd', s @ ..] => {
					super::push(out, 31001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 21422);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37548);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', '-', s @ ..] => {
					super::push(out, 31002);
					match s {
						[] => true,
						['ß', s @ ..] => {
							super::push(out, 37550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'h', s @ ..] => {
					super::push(out, 31003);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'h', s @ ..] => {
					super::push(out, 31004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'e', s @ ..] => {
					super::push(out, 31005);
					s.is_empty()
				}
				['l', 'a', 'k', '-', s @ ..] => {
					super::push(out, 31006);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 31007);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31008);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37552);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 41520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 37553);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 37554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31009);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 31010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'e', s @ ..] => {
			super::push(out, 21423);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 31011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 21424);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word skaleneffekt.
pub(crate) fn n47575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44562);
	s.is_empty()
}

/// Words ending -lungseffekt.
pub(crate) fn n47576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44563);
	s.is_empty()
}

/// Words ending -dierungseffekt.
pub(crate) fn n47577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44564);
	s.is_empty()
}

/// Words ending -perfekt.
pub(crate) fn n39800(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', 'a', 'u', 'q', 's', 'u', 'l', 'p', 'r', s @ ..] => {
			super::push(out, 41554);
			s.is_empty()
		}
		['r', 'u', s @ ..] => {
			super::push(out, 41555);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 41556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jekt.
pub(crate) fn n10845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10721);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21617);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 31124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 21618);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37597);
							s.is_empty()
						}
						['-', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 41557);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 41558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 37598);
							match s {
								[] => true,
								['t', 'i', 'd', 'n', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 41559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 37599);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 37600);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 37601);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 37602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 31125);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'u', 'e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 41560);
									s.is_empty()
								}
								['s', 't', s @ ..] => {
									match s {
										[] => true,
										['f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 43600);
											s.is_empty()
										}
										['h', 'c', 'e', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44565);
													s.is_empty()
												}
												['r', 'e', 'k', 'l', 'ö', 'v', '\u{1}', s @ ..] => {
													super::push(out, 44566);
													s.is_empty()
												}
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
		['o', s @ ..] => {
			super::push(out, 21619);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 37603);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 41561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'g', 'l', s @ ..] => {
							super::push(out, 37604);
							s.is_empty()
						}
						['m', 'o', s @ ..] => {
							super::push(out, 37605);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 41562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'l', s @ ..] => {
							super::push(out, 37606);
							s.is_empty()
						}
						['s', 'g', 'n', 'u', 'r', 'e', 'i', 's', 'i', 'n', s @ ..] => {
							super::push(out, 37607);
							s.is_empty()
						}
						['t', 'o', 'l', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 37608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lekt.
pub(crate) fn n10846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10722);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21620);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 21621);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 21622);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 31126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pekt.
pub(crate) fn n10847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10723);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 21623);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31127);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 31128);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 37609);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 41563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 31129);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 't', s @ ..] => {
									super::push(out, 41564);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 41565);
									s.is_empty()
								}
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

/// Words ending -sekt.
pub(crate) fn n10848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10724);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 21624);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 31130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21625);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31131);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 21626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tekt.
pub(crate) fn n10849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10725);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 21627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ment.
pub(crate) fn n10919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10792);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21770);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 37705);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 41632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 37706);
							match s {
								[] => true,
								['a', 'p', 'a', s @ ..] => {
									super::push(out, 41633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 31240);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 37707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31241);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 31242);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 37708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31243);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 37709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 21771);
			match s {
				[] => true,
				['b', 'm', 'a', 'j', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 31244);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 31245);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 31246);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 31247);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 41634);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 41635);
									match s {
										[] => true,
										['a', 'm', s @ ..] => super::t16::n46564(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 37710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'a', 't', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 37711);
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 37712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t16::n32733(s, out),
				['n', s @ ..] => {
					super::push(out, 31249);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31250);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 37718);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 37719);
							match s {
								[] => true,
								['x', 'e', 'r', 'e', 'i', s @ ..] => {
									super::push(out, 41646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 37720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t16::n32736(s, out),
				['t', s @ ..] => {
					super::push(out, 31252);
					s.is_empty()
				}
				['z', s @ ..] => super::t16::n32738(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t16::n22419(s, out),
		['h', s @ ..] => super::t16::n22420(s, out),
		['i', s @ ..] => super::t16::n22421(s, out),
		['m', s @ ..] => super::t16::n22422(s, out),
		['n', s @ ..] => super::t16::n22423(s, out),
		['o', s @ ..] => super::t16::n22424(s, out),
		['p', s @ ..] => super::t16::n22425(s, out),
		['r', s @ ..] => super::t16::n22426(s, out),
		['t', s @ ..] => super::t16::n22427(s, out),
		['u', s @ ..] => super::t16::n22428(s, out),
		_ => false,
	}
}

/// Words ending -atent.
pub(crate) fn n22440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21793);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 31276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntent.
pub(crate) fn n22442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21795);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 31277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stent.
pub(crate) fn n22443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21796);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31278);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 31279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttent.
pub(crate) fn n22444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21797);
	match s {
		[] => true,
		['i', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 31280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uent.
pub(crate) fn n10924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10797);
	match s {
		[] => true,
		['q', '\u{1}', s @ ..] => {
			super::push(out, 21798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vent.
pub(crate) fn n10925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10798);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21799);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 31281);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 21800);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 21801);
			match s {
				[] => true,
				['-', 'e', s @ ..] => {
					super::push(out, 31284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21802);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 21803);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 31285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 21804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zent.
pub(crate) fn n10926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10799);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 21805);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37744);
							s.is_empty()
						}
						['e', 'k', 'r', 'ä', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 37745);
							s.is_empty()
						}
						['g', 'n', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 37746);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 37747);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 37748);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37749);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 41679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 37750);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 37751);
							s.is_empty()
						}
						['z', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 37752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 21806);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31287);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31288);
					match s {
						[] => true,
						['p', 'm', s @ ..] => {
							super::push(out, 37753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 21807);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word marmeladenbrot.
pub(crate) fn n47606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44588);
	s.is_empty()
}

/// The word gnadenbrot.
pub(crate) fn n47607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44589);
	s.is_empty()
}

/// Words ending -enrot.
pub(crate) fn n22542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21889);
	s.is_empty()
}

/// Words ending -einrot.
pub(crate) fn n22543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21890);
	s.is_empty()
}

/// Words ending -rrot.
pub(crate) fn n11003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10875);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21891);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 31346);
					match s {
						[] => true,
						['o', 'n', 'n', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 37790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 31347);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trot.
pub(crate) fn n11004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10876);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 21892);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 31349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 21893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ehrwert.
pub(crate) fn n32900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31401);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 37829);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 37830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -swert.
pub(crate) fn n22625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21970);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 31402);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 37831);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 37832);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							super::push(out, 41717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'c', 'u', s @ ..] => {
			super::push(out, 31403);
			match s {
				[] => true,
				['a', 'r', 'b', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 37833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 31404);
			s.is_empty()
		}
		['n', 'e', s @ ..] => {
			super::push(out, 31405);
			s.is_empty()
		}
		['t', 'i', 'e', 'h', 't', s @ ..] => {
			super::push(out, 31406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -twert.
pub(crate) fn n22626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21971);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 31407);
			s.is_empty()
		}
		['s', 'e', 'd', s @ ..] => {
			super::push(out, 31408);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ünzwert.
pub(crate) fn n32909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31409);
	s.is_empty()
}

/// Words ending -zert.
pub(crate) fn n11055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10925);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 21973);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word abtransport.
pub(crate) fn n47608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44590);
	s.is_empty()
}

/// The word spitzensport.
pub(crate) fn n44426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41732);
	s.is_empty()
}

/// Words ending -nnsport.
pub(crate) fn n40081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37849);
	match s {
		[] => true,
		['e', 'r', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 43683);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 43684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unsport.
pub(crate) fn n40082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37850);
	s.is_empty()
}

/// Words ending -osport.
pub(crate) fn n32940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31439);
	s.is_empty()
}

/// Words ending -rsport.
pub(crate) fn n32941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31440);
	s.is_empty()
}

/// Words ending -kssport.
pub(crate) fn n32942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31441);
	s.is_empty()
}

/// Words ending -tsport.
pub(crate) fn n32943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31442);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 37851);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 37852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xsport.
pub(crate) fn n32944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31443);
	s.is_empty()
}

/// Words ending -ßsport.
pub(crate) fn n32945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31444);
	s.is_empty()
}

/// Words ending -xport.
pub(crate) fn n22655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21999);
	s.is_empty()
}

/// Words ending -rort.
pub(crate) fn n11072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10942);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sort.
pub(crate) fn n11073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10943);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 22001);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 31445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22002);
			match s {
				[] => true,
				['u', 'a', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 31446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 22003);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22004);
			match s {
				[] => true,
				['o', 'i', 't', 'k', s @ ..] => {
					super::push(out, 31447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 22005);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 22006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tort.
pub(crate) fn n11074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10944);
	s.is_empty()
}

/// Words ending -uort.
pub(crate) fn n11075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10945);
	s.is_empty()
}

/// Words ending -wort.
pub(crate) fn n11076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10946);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 22007);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 31449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 22008);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 31450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 22009);
			match s {
				[] => true,
				['n', 'u', s @ ..] => {
					super::push(out, 31451);
					match s {
						[] => true,
						['r', 'g', 'n', s @ ..] => {
							super::push(out, 37854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 22010);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31452);
					s.is_empty()
				}
				['g', 'a', 'l', 'k', s @ ..] => {
					super::push(out, 31453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 31454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'p', 'm', 'i', s @ ..] => {
			super::push(out, 22011);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37856);
							s.is_empty()
						}
						['s', 'g', 'n', 'i', 'l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 37857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 22012);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 22013);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 41733);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43685);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 41734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 37858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 31455);
					match s {
						[] => true,
						['m', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'c', 'e', 'r', s @ ..] => {
			super::push(out, 22014);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 31456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22015);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31457);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 37860);
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 41735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'd', 'u', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37861);
							s.is_empty()
						}
						['h', 'c', 'i', 'e', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37862);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 37863);
							s.is_empty()
						}
						['z', 'n', 'a', s @ ..] => {
							super::push(out, 37864);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 41736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 31458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 22016);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22017);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 37865);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 37866);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 41737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'o', s @ ..] => {
					super::push(out, 31459);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 37867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', s @ ..] => {
					super::push(out, 31460);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 37868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22018);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 37869);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 41738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 37870);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 41739);
									s.is_empty()
								}
								['n', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 41740);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 37871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 31461);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 37872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'w', s @ ..] => {
					super::push(out, 31462);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 31463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 22019);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37874);
							match s {
								[] => true,
								['i', 'h', 'c', 's', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 41741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 37875);
							match s {
								[] => true,
								['e', 'l', 'i', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 41742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 37876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 31464);
					match s {
						[] => true,
						['f', 'p', 'm', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 37877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 31465);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 41743);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 43686);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 43687);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 43688);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 43689);
											match s {
												[] => true,
												['m', 'i', 't', 's', 'e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 44591);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 43690);
											s.is_empty()
										}
										['ß', s @ ..] => {
											super::push(out, 43691);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['e', 'k', 'c', 'e', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37878);
							s.is_empty()
						}
						['o', 'i', 't', 'i', s @ ..] => {
							super::push(out, 37879);
							match s {
								[] => true,
								['s', 'o', 'p', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 41744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31466);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37880);
							match s {
								[] => true,
								['p', 'y', s @ ..] => {
									super::push(out, 41745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 37881);
							match s {
								[] => true,
								['l', 'h', 'c', 's', '\u{1}', s @ ..] => super::t16::n44442(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t16::n32974(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t16::n22676(s, out),
		['u', s @ ..] => super::t16::n22677(s, out),
		['z', s @ ..] => super::t16::n22678(s, out),
		['ß', s @ ..] => super::t16::n22679(s, out),
		_ => false,
	}
}

/// Words ending -zort.
pub(crate) fn n11077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10947);
	s.is_empty()
}

/// Words ending -ßort.
pub(crate) fn n11078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10948);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 22023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -test.
pub(crate) fn n11118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10988);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 22111);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31531);
					match s {
						[] => true,
						['n', 'o', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 37931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 31532);
					match s {
						[] => true,
						['l', 'l', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 37932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 31533);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 37933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'l', s @ ..] => {
					super::push(out, 31534);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 37934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31535);
					match s {
						[] => true,
						['u', 't', 'a', 't', 's', '-', 'l', 'a', 't', 'n', 'e', 'm', '-', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 37935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31536);
					match s {
						[] => true,
						['e', 'n', 'i', 'b', '-', 'd', 'r', 'o', 'f', 'n', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 37936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 31537);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 31538);
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 37937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 31539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 22112);
			match s {
				[] => true,
				['n', 'o', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 31540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 22113);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 22114);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 22115);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 31541);
					match s {
						[] => true,
						['g', 'r', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 37938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'h', 'a', 'n', 'f', s @ ..] => {
					super::push(out, 31542);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 37939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31543);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 31544);
					match s {
						[] => true,
						['r', 'ä', 'h', '\u{1}', s @ ..] => {
							super::push(out, 37940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 22116);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 22117);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 41755);
									s.is_empty()
								}
								['r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 41756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'p', 's', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 37941);
							s.is_empty()
						}
						['l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 37942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31545);
					match s {
						[] => true,
						['a', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 37943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22118);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 22119);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31546);
					match s {
						[] => true,
						['g', 'e', s @ ..] => {
							super::push(out, 37944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'a', 'z', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 31547);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					match s {
						[] => true,
						['n', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41757);
									s.is_empty()
								}
								['n', 'e', 'g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 41758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 37945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 31548);
					match s {
						[] => true,
						['h', 'o', 'k', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 37946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 31549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'o', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 22120);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22121);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'm', 'o', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41759);
									s.is_empty()
								}
								['m', 'o', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 41760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 37947);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 41761);
									match s {
										[] => true,
										['a', 'w', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44592);
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 44593);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 41762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41763);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 41764);
									match s {
										[] => true,
										['t', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 43693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 41765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 37948);
							match s {
								[] => true,
								['i', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 41766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 37949);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 37950);
							match s {
								[] => true,
								['s', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 41767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 37951);
							match s {
								[] => true,
								['n', 'e', 'n', 'o', 'p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41768);
									s.is_empty()
								}
								['o', 'i', 'd', 'i', '\u{1}', s @ ..] => {
									super::push(out, 41769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 31550);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 31551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22122);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 31552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22123);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31553);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 37952);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 37953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 31554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t16::n22785(s, out),
		['t', s @ ..] => {
			super::push(out, 22125);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31563);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 37961);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 41775);
									s.is_empty()
								}
								['k', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 41776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 31564);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 31565);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31566);
					s.is_empty()
				}
				['s', s @ ..] => super::t16::n33090(s, out),
				['u', s @ ..] => super::t16::n33091(s, out),
				['x', s @ ..] => super::t16::n33092(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t16::n22787(s, out),
		['ß', s @ ..] => super::t16::n22788(s, out),
		['ä', s @ ..] => super::t16::n22789(s, out),
		_ => false,
	}
}

/// Words ending -feuergeist.
pub(crate) fn n47617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44599);
	s.is_empty()
}

/// Words ending -teuergeist.
pub(crate) fn n47618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44600);
	s.is_empty()
}

/// Words ending -fmist.
pub(crate) fn n22816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22154);
	match s {
		[] => true,
		['a', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 31601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -imist.
pub(crate) fn n22818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22156);
	s.is_empty()
}

/// Words ending -kmist.
pub(crate) fn n22819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22157);
	match s {
		[] => true,
		['c', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 31602);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lmist.
pub(crate) fn n22820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22158);
	match s {
		[] => true,
		['e', 'm', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 31603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nmist.
pub(crate) fn n22821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22159);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 31604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omist.
pub(crate) fn n22822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22160);
	s.is_empty()
}

/// Words ending -rmist.
pub(crate) fn n22823(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'n', s @ ..] => {
			super::push(out, 31605);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 31606);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nist.
pub(crate) fn n11135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11005);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 22161);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 31607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 22162);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 31608);
					match s {
						[] => true,
						['e', 'l', '-', s @ ..] => {
							super::push(out, 37971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oist.
pub(crate) fn n11136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11006);
	s.is_empty()
}

/// Words ending -pist.
pub(crate) fn n11137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11007);
	s.is_empty()
}

/// Words ending -rist.
pub(crate) fn n11138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11008);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22164);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 22165);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 22166);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 22167);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 31609);
					match s {
						[] => true,
						['r', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 37972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22168);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 22169);
			match s {
				[] => true,
				['c', 'd', s @ ..] => {
					super::push(out, 31610);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 37973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 22170);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22171);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 31613);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 37975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31615);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 31616);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 37976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 22173);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 22174);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sist.
pub(crate) fn n11139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11009);
	s.is_empty()
}

/// Words ending -tist.
pub(crate) fn n11140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11010);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22175);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 31617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 22176);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					super::push(out, 31618);
					match s {
						[] => true,
						['o', 'r', 't', 't', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 37978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uist.
pub(crate) fn n11141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11011);
	s.is_empty()
}

/// Words ending -vist.
pub(crate) fn n11142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11012);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 22177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 22178);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 22179);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 31620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wist.
pub(crate) fn n11143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11013);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22180);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 22181);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 22182);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31621);
					s.is_empty()
				}
				['c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 31622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'n', 'a', 'h', 'c', 'a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 22183);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 22184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31623);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31624);
					match s {
						[] => true,
						['m', 'm', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 37980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 22185);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yist.
pub(crate) fn n11144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11014);
	s.is_empty()
}

/// Words ending -zist.
pub(crate) fn n11145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11015);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22186);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 31625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ifendienst.
pub(crate) fn n47619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44601);
	s.is_empty()
}

/// Words ending -innendienst.
pub(crate) fn n47620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44602);
	s.is_empty()
}

/// Words ending -ftendienst.
pub(crate) fn n47621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44603);
	match s {
		[] => true,
		['a', 'h', 'c', 's', 't', 'o', 'b', 'z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 44893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nachrichtendienst.
pub(crate) fn n47622(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44894);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 44895);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 44983);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 44984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 44896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sarbeitsdienst.
pub(crate) fn n47623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44604);
	s.is_empty()
}

/// Words ending -unst.
pub(crate) fn n11152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11022);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 22194);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 31635);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37989);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 41800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 37990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 31636);
					match s {
						[] => true,
						['e', 'z', s @ ..] => {
							super::push(out, 37991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31637);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 31638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 22195);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 31639);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 37993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22196);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 31640);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 37994);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 37995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 37996);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'h', s @ ..] => {
									super::push(out, 41801);
									s.is_empty()
								}
								['i', 'e', 'n', s @ ..] => {
									super::push(out, 41802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 37997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'u', s @ ..] => {
					super::push(out, 31641);
					s.is_empty()
				}
				['k', 'c', 'u', s @ ..] => {
					super::push(out, 31642);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 31643);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					super::push(out, 31644);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 41803);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 41804);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 41805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 37998);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 37999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 31645);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 38000);
							s.is_empty()
						}
						['k', 'l', s @ ..] => {
							super::push(out, 38001);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 38002);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 41806);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 41807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 38003);
							s.is_empty()
						}
						['t', 'a', s @ ..] => {
							super::push(out, 38004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 38005);
							s.is_empty()
						}
						['i', 'e', 'z', s @ ..] => {
							super::push(out, 38006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', s @ ..] => {
					super::push(out, 31646);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 31647);
					s.is_empty()
				}
				['ß', 'i', s @ ..] => {
					super::push(out, 31648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22197);
			match s {
				[] => true,
				['b', 'n', s @ ..] => {
					super::push(out, 31649);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lett.
pub(crate) fn n11200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11070);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 22298);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38055);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 41834);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 41835);
									match s {
										[] => true,
										['v', 'r', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 43718);
											s.is_empty()
										}
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
			super::push(out, 22299);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 31719);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 41836);
									s.is_empty()
								}
								['n', 'e', 'f', 'f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 41837);
									s.is_empty()
								}
								['o', 't', 'y', 'z', '\u{1}', s @ ..] => {
									super::push(out, 41838);
									s.is_empty()
								}
								['r', 'e', 'i', s @ ..] => {
									super::push(out, 41839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 31720);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 38056);
							match s {
								[] => true,
								['e', 'f', 'f', 'o', 't', 'r', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31721);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 38057);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22300);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 22301);
			match s {
				[] => true,
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 31723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22302);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31724);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31725);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41842);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 41843);
									match s {
										[] => true,
										['t', 't', 'i', 'r', 't', 'n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 43719);
											s.is_empty()
										}
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
			super::push(out, 22303);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 22304);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31726);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31727);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 38058);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 41844);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 41845);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43720);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 38059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31728);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 31729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22306);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 38060);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 38061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 31730);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 31731);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 31732);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nett.
pub(crate) fn n11202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11072);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 22307);
			match s {
				[] => true,
				['b', 'a', 'k', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 'g', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 38062);
							s.is_empty()
						}
						['n', 'e', 't', 'm', 'a', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 38063);
							s.is_empty()
						}
						['s', 'g', 'e', 'i', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 38064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 31733);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 38065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22308);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 31734);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38066);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 41846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 22309);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rett.
pub(crate) fn n11203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11073);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22310);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 31735);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38067);
							s.is_empty()
						}
						['t', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 38068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 31736);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 38069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 31737);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 31738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 22311);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 31739);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38070);
							match s {
								[] => true,
								['j', 'i', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 41847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 31740);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 38071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'n', 'e', 'd', 'e', 'g', 'n', s @ ..] => {
					super::push(out, 31741);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38072);
							match s {
								[] => true,
								['g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 41848);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'g', 'a', s @ ..] => {
							super::push(out, 38073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 38074);
							s.is_empty()
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 38075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'g', s @ ..] => {
					super::push(out, 31742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22312);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 31743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22313);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 31744);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 38076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22314);
			match s {
				[] => true,
				['b', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 31745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word falsett.
pub(crate) fn n33291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31747);
	s.is_empty()
}

/// Words ending -osett.
pub(crate) fn n22988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22317);
	s.is_empty()
}

/// The word korsett.
pub(crate) fn n22989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22318);
	s.is_empty()
}

/// Words ending -tett.
pub(crate) fn n11205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11075);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 22319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uett.
pub(crate) fn n11206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11076);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 22320);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zett.
pub(crate) fn n11207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11077);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22322);
			s.is_empty()
		}
		['s', 'e', '\u{1}', s @ ..] => {
			super::push(out, 22323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ität.
pub(crate) fn n11333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11202);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 38148);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 38149);
							s.is_empty()
						}
						['u', 'l', s @ ..] => {
							super::push(out, 38150);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 38151);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 38152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 31838);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31839);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 31840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', s @ ..] => {
			super::push(out, 22468);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 22469);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 38153);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 41893);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 41894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 38154);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 41895);
									s.is_empty()
								}
								['r', 'r', 'i', s @ ..] => {
									super::push(out, 41896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', s @ ..] => {
									super::push(out, 41897);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 41898);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 41899);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									super::push(out, 41900);
									s.is_empty()
								}
								['v', 'i', 'r', s @ ..] => {
									super::push(out, 41901);
									s.is_empty()
								}
								['z', 'e', s @ ..] => {
									super::push(out, 41902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'i', 's', s @ ..] => {
							super::push(out, 38155);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 43729);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 43730);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', 'f', s @ ..] => {
									super::push(out, 41903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t16::n40430(s, out),
						['r', s @ ..] => super::t16::n40431(s, out),
						['s', 'u', s @ ..] => {
							super::push(out, 38157);
							s.is_empty()
						}
						['t', s @ ..] => super::t16::n40433(s, out),
						['u', s @ ..] => super::t16::n40434(s, out),
						['v', s @ ..] => {
							super::push(out, 38159);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 38160);
							match s {
								[] => true,
								['o', 'l', 'h', s @ ..] => {
									super::push(out, 41918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t16::n33404(s, out),
				['o', s @ ..] => super::t16::n33405(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t16::n23148(s, out),
		['n', s @ ..] => super::t16::n23149(s, out),
		['r', s @ ..] => super::t16::n23150(s, out),
		['s', s @ ..] => super::t16::n23151(s, out),
		['t', s @ ..] => super::t16::n23152(s, out),
		['u', s @ ..] => super::t16::n23153(s, out),
		['v', s @ ..] => super::t16::n23154(s, out),
		['z', s @ ..] => super::t16::n23155(s, out),
		_ => false,
	}
}

/// Words ending -leszenz.
pub(crate) fn n40628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38332);
	match s {
		[] => true,
		['a', 'v', s @ ..] => {
			super::push(out, 41980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -neszenz.
pub(crate) fn n40629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38333);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 41981);
			s.is_empty()
		}
		['i', 'm', 'u', 'l', 'o', 'r', s @ ..] => {
			super::push(out, 41982);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ueszenz.
pub(crate) fn n40630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38334);
	s.is_empty()
}

/// Words ending -iszenz.
pub(crate) fn n33680(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 38335);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 38336);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word nazi-deutschland.
pub(crate) fn n47657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44635);
	s.is_empty()
}

/// The word hitler-deutschland.
pub(crate) fn n47658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44636);
	s.is_empty()
}

/// Words ending -sland.
pub(crate) fn n12816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12631);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 23377);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 23378);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 23379);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 23380);
			match s {
				[] => true,
				['n', 'u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 38531);
							match s {
								[] => true,
								['a', 'r', 't', 's', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 42075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'd', s @ ..] => {
							super::push(out, 38532);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 38533);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 38534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 23381);
			match s {
				[] => true,
				['c', 'i', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 32459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 23382);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 32460);
					match s {
						[] => true,
						['g', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 38535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'i', 't', 'k', 'u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 32461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 23383);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 23384);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 32462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kstand.
pub(crate) fn n24208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23425);
	match s {
		[] => true,
		['c', 'ü', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38555);
					s.is_empty()
				}
				['f', 'p', 'm', 'a', 'd', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 38556);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 38557);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 38558);
					match s {
						[] => true,
						['g', 'n', 'u', 'l', 'h', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 42083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 38559);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lstand.
pub(crate) fn n24209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23426);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 38560);
					s.is_empty()
				}
				['t', 't', s @ ..] => {
					super::push(out, 38561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 32482);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 38562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 32483);
			match s {
				[] => true,
				['i', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42084);
							s.is_empty()
						}
						['f', 'u', 'a', 'l', 's', 'i', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 42085);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 42086);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 42087);
							match s {
								[] => true,
								['k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 43799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 42088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'f', '\u{1}', s @ ..] => {
					super::push(out, 38563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nstand.
pub(crate) fn n24211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23428);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 38564);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 42089);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 44637);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['n', 'e', s @ ..] => {
													super::push(out, 44907);
													s.is_empty()
												}
												['t', 'l', 'a', 'h', 's', 'u', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 44908);
													s.is_empty()
												}
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
							super::push(out, 42090);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 38565);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 38566);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 42091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 38567);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 42092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'f', s @ ..] => {
			super::push(out, 32484);
			match s {
				[] => true,
				['g', 'n', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 38568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rstand.
pub(crate) fn n24213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23430);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'w', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 42093);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 42094);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 42095);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 42096);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 43801);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 43802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 38569);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 38570);
					s.is_empty()
				}
				['r', 'h', 'ü', 'f', '\u{1}', s @ ..] => {
					super::push(out, 38571);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 38572);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 42097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 38573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 32485);
			match s {
				[] => true,
				['v', 'n', s @ ..] => {
					super::push(out, 38574);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sstand.
pub(crate) fn n24214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23431);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 32486);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 32487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tstand.
pub(crate) fn n24215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23432);
	match s {
		[] => true,
		['s', 'h', 'c', 'ö', 'h', '\u{1}', s @ ..] => {
			super::push(out, 32488);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 32489);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 38575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ustand.
pub(crate) fn n24216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23433);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', 'e', s @ ..] => {
					super::push(out, 38576);
					s.is_empty()
				}
				['e', 'i', s @ ..] => {
					super::push(out, 38577);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 38578);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 38579);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 42099);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', 'g', s @ ..] => {
											super::push(out, 44638);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 44909);
													s.is_empty()
												}
												['r', s @ ..] => {
													match s {
														[] => true,
														['e', 'g', 'a', 'l', 'e', 'b', '\u{1}', s @ ..] => super::t16::n48052(s, out),
														['h', s @ ..] => super::t16::n48053(s, out),
														_ => false,
													}
												}
												['t', 'l', 'a', 'h', 'r', 'e', '\u{1}', s @ ..] => {
													super::push(out, 44910);
													s.is_empty()
												}
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
							super::push(out, 42100);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43803);
									s.is_empty()
								}
								['i', 'e', 's', 't', 's', 's', 'u', 'w', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 43804);
									s.is_empty()
								}
								['o', 'i', 't', 'a', 'd', 'i', 'x', 'o', '\u{1}', s @ ..] => {
									super::push(out, 43805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'r', s @ ..] => {
					super::push(out, 38580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utand.
pub(crate) fn n12849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12664);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 23434);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wäsche.
pub(crate) fn n24753(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 32756);
			s.is_empty()
		}
		['n', 'e', s @ ..] => {
			super::push(out, 32757);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 32758);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 32759);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 32760);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 38684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rstoff.
pub(crate) fn n25804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24923);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 33273);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 38981);
					s.is_empty()
				}
				['g', 'ä', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 38982);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 38983);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 38984);
					match s {
						[] => true,
						['s', 'a', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43886);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 43887);
									match s {
										[] => true,
										['o', 'j', s @ ..] => {
											super::push(out, 44654);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44913);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 43888);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 43889);
									s.is_empty()
								}
								['l', 'y', s @ ..] => {
									super::push(out, 43890);
									match s {
										[] => true,
										['h', 't', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 44655);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 43891);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43892);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 44914);
													s.is_empty()
												}
												['l', s @ ..] => {
													match s {
														[] => true,
														['e', s @ ..] => super::t16::n48054(s, out),
														['h', s @ ..] => super::t16::n48055(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'o', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 44656);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44657);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 44658);
											match s {
												[] => true,
												['l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 44915);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 43893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 38985);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 38986);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43894);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 43895);
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 43896);
									s.is_empty()
								}
								['n', 'e', 'h', 'c', 's', 'a', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 43897);
									s.is_empty()
								}
								['t', 'f', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 43898);
									s.is_empty()
								}
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
			super::push(out, 33274);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 38987);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 38988);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43899);
									s.is_empty()
								}
								['o', 'r', 'k', s @ ..] => {
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 44659);
											s.is_empty()
										}
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 44660);
											s.is_empty()
										}
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
			super::push(out, 33275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tling.
pub(crate) fn n15322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15014);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 25253);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33521);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 39132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'h', '\u{1}', s @ ..] => {
					super::push(out, 33522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 25254);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 25255);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39133);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 39134);
							match s {
								[] => true,
								['f', 'p', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 42352);
									s.is_empty()
								}
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
			super::push(out, 25256);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 25257);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33523);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 42353);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43925);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 42354);
									s.is_empty()
								}
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
			super::push(out, 25258);
			match s {
				[] => true,
				['r', 'h', s @ ..] => {
					super::push(out, 33524);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 39135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uling.
pub(crate) fn n15323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15015);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 25259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wling.
pub(crate) fn n15324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15016);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 25260);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yling.
pub(crate) fn n15325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15017);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 25261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßling.
pub(crate) fn n15327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15019);
	s.is_empty()
}

/// Words ending -genforschung.
pub(crate) fn n47692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44665);
	s.is_empty()
}

/// Words ending -renforschung.
pub(crate) fn n47693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44666);
	s.is_empty()
}

/// Words ending -mensforschung.
pub(crate) fn n47694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44667);
	s.is_empty()
}

/// Words ending -onsforschung.
pub(crate) fn n47695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44668);
	match s {
		[] => true,
		['i', 't', 'k', s @ ..] => {
			super::push(out, 44918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nftsforschung.
pub(crate) fn n47696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44669);
	s.is_empty()
}

/// Words ending -ltsforschung.
pub(crate) fn n47697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44670);
	s.is_empty()
}

/// Words ending -tforschung.
pub(crate) fn n45159(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 43941);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 43942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zforschung.
pub(crate) fn n45160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42402);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 43943);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rrschung.
pub(crate) fn n41590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39204);
	match s {
		[] => true,
		['e', 'h', 'e', 'b', 'h', s @ ..] => {
			super::push(out, 42403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uschung.
pub(crate) fn n35401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33631);
	s.is_empty()
}

/// Words ending -öschung.
pub(crate) fn n35402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33632);
	s.is_empty()
}

/// Words ending -uchung.
pub(crate) fn n26313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25376);
	s.is_empty()
}

/// Words ending -ehung.
pub(crate) fn n15434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15117);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 25377);
			s.is_empty()
		}
		['i', 'z', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33633);
					match s {
						[] => true,
						['b', '-', s @ ..] => {
							super::push(out, 39205);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 42404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', 'n', s @ ..] => {
					super::push(out, 33634);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 33635);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 42405);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 42406);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 42407);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 42408);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 42409);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 43944);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 43945);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 43946);
											s.is_empty()
										}
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
			super::push(out, 25378);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 25379);
			s.is_empty()
		}
		['t', 's', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 33636);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 33637);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 42410);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 42411);
									s.is_empty()
								}
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

/// Words ending -zeihung.
pub(crate) fn n15435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15118);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 25380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ohung.
pub(crate) fn n15436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15119);
	s.is_empty()
}

/// Words ending -ähung.
pub(crate) fn n15437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15120);
	s.is_empty()
}

/// Words ending -öhung.
pub(crate) fn n15438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15121);
	match s {
		[] => true,
		['h', 'r', 'e', 'z', s @ ..] => {
			super::push(out, 25381);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 33638);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stellung.
pub(crate) fn n26342(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 33660);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 39221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 33661);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 33662);
			s.is_empty()
		}
		['h', 'c', 'i', s @ ..] => {
			super::push(out, 33663);
			match s {
				[] => true,
				['e', 'l', 'g', 'r', s @ ..] => {
					super::push(out, 39223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 'e', 's', 'g', s @ ..] => {
			super::push(out, 33664);
			match s {
				[] => true,
				['n', 'u', s @ ..] => {
					super::push(out, 39224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 33665);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 39225);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 39226);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 42422);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43949);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 43950);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 43951);
									s.is_empty()
								}
								['l', 'h', s @ ..] => {
									super::push(out, 43952);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 44671);
											s.is_empty()
										}
										['ü', s @ ..] => {
											super::push(out, 44672);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', s @ ..] => {
									super::push(out, 43953);
									s.is_empty()
								}
								['r', 'e', 'd', s @ ..] => {
									super::push(out, 43954);
									s.is_empty()
								}
								['s', 't', s @ ..] => {
									super::push(out, 43955);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43956);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 43957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 42423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 39227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39228);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 39229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'h', s @ ..] => {
			super::push(out, 33666);
			match s {
				[] => true,
				['c', 'a', 'b', s @ ..] => {
					super::push(out, 39230);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ungsverordnung.
pub(crate) fn n47700(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 44919);
			s.is_empty()
		}
		['r', 'h', 'ü', 'f', 'h', 'c', 'r', 'u', 'd', '-', s @ ..] => {
			super::push(out, 44920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ierückgewinnung.
pub(crate) fn n45194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42435);
	s.is_empty()
}

/// Words ending -urückgewinnung.
pub(crate) fn n45195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42436);
	s.is_empty()
}

/// Words ending -lgewinnung.
pub(crate) fn n41633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39240);
	s.is_empty()
}

/// Words ending -mgewinnung.
pub(crate) fn n41634(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'r', 'b', s @ ..] => {
			super::push(out, 42437);
			s.is_empty()
		}
		['u', 'i', 'p', s @ ..] => {
			super::push(out, 42438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ngewinnung.
pub(crate) fn n41635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39241);
	s.is_empty()
}

/// Words ending -sergewinnung.
pub(crate) fn n41636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39242);
	s.is_empty()
}

/// Words ending -sgewinnung.
pub(crate) fn n41637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39243);
	s.is_empty()
}

/// Words ending -lzgewinnung.
pub(crate) fn n41638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39244);
	s.is_empty()
}

/// Words ending -rnung.
pub(crate) fn n15469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15144);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 25419);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 33691);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uschönung.
pub(crate) fn n15470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15145);
	s.is_empty()
}

/// Words ending -ierung.
pub(crate) fn n26373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25422);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 39252);
					match s {
						[] => true,
						['o', 'u', 'l', 'f', 'r', s @ ..] => {
							super::push(out, 42442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 't', s @ ..] => {
					super::push(out, 39253);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 43965);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 'f', s @ ..] => {
									super::push(out, 43966);
									s.is_empty()
								}
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
			super::push(out, 33699);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 39254);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 42443);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 43967);
									match s {
										[] => true,
										['d', 'u', s @ ..] => {
											super::push(out, 44673);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 44674);
											s.is_empty()
										}
										['r', 'u', s @ ..] => {
											super::push(out, 44675);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 44676);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 43968);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43969);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 43970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 39255);
					match s {
						[] => true,
						['f', 'ä', s @ ..] => {
							super::push(out, 42444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'c', 'o', s @ ..] => {
			super::push(out, 33700);
			match s {
				[] => true,
				['l', 'b', 'o', s @ ..] => {
					super::push(out, 39256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'y', 'm', s @ ..] => {
			super::push(out, 33701);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 33702);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 33703);
			match s {
				[] => true,
				['i', 'm', 'i', s @ ..] => {
					super::push(out, 39257);
					match s {
						[] => true,
						['r', 'k', 's', 'i', 'd', 's', s @ ..] => {
							super::push(out, 42445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'o', 'b', s @ ..] => {
			super::push(out, 33704);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 33705);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 42446);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 43971);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43972);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43973);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', 'o', 'l', 'g', 'e', s @ ..] => {
											super::push(out, 44677);
											s.is_empty()
										}
										['i', 'r', 't', 's', 'u', 'd', 'n', 'i', 'h', s @ ..] => {
											super::push(out, 44678);
											s.is_empty()
										}
										['n', 'r', s @ ..] => {
											super::push(out, 44679);
											s.is_empty()
										}
										['t', 'r', s @ ..] => {
											super::push(out, 44680);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 43974);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 44681);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 43975);
									s.is_empty()
								}
								['o', 'h', s @ ..] => {
									super::push(out, 43976);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 44682);
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
								['a', 'n', s @ ..] => {
									super::push(out, 43977);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 43978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43979);
									match s {
										[] => true,
										['b', 'r', 'u', 'r', s @ ..] => {
											super::push(out, 44683);
											s.is_empty()
										}
										['k', 'i', s @ ..] => {
											super::push(out, 44684);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'b', s @ ..] => {
									super::push(out, 43980);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 44685);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', s @ ..] => {
									super::push(out, 43981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'k', s @ ..] => {
									super::push(out, 43982);
									s.is_empty()
								}
								['e', 'd', s @ ..] => {
									super::push(out, 43983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'i', s @ ..] => {
							super::push(out, 42447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 33706);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['i', 't', 'a', s @ ..] => {
					super::push(out, 39259);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 42448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 's', 'n', 'o', 'k', 'm', s @ ..] => {
					super::push(out, 39260);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 42449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', 'i', s @ ..] => {
			super::push(out, 33707);
			match s {
				[] => true,
				['f', 'f', s @ ..] => {
					super::push(out, 39261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 33708);
			match s {
				[] => true,
				['i', 'f', 'i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 42450);
							match s {
								[] => true,
								['u', 'h', s @ ..] => {
									super::push(out, 43984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 42451);
							match s {
								[] => true,
								['r', 'e', 'p', s @ ..] => {
									super::push(out, 43985);
									s.is_empty()
								}
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

/// Words ending -ahrung.
pub(crate) fn n26378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25426);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 33714);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 33715);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 39264);
					match s {
						[] => true,
						['e', 'k', s @ ..] => {
							super::push(out, 42454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 39265);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 39266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 33716);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39267);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 39268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ährung.
pub(crate) fn n26381(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 33717);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 42455);
							s.is_empty()
						}
						['r', 'e', 't', s @ ..] => {
							super::push(out, 42456);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 42457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 33718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ührung.
pub(crate) fn n26382(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 33719);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 39269);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 42458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'i', 'r', 'k', 'e', s @ ..] => {
					super::push(out, 39270);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 39271);
					match s {
						[] => true,
						['c', 'u', 'b', 'm', s @ ..] => {
							super::push(out, 42459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'h', s @ ..] => {
					super::push(out, 39272);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 39273);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 39274);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', 'e', 'i', 'r', 'k', 'r', s @ ..] => {
							super::push(out, 42460);
							s.is_empty()
						}
						['n', 'e', 'b', s @ ..] => {
							super::push(out, 42461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 33720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -prung.
pub(crate) fn n15475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15150);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 25429);
			match s {
				[] => true,
				['i', 'e', 'r', s @ ..] => {
					super::push(out, 33721);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 39275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'k', s @ ..] => {
					super::push(out, 33722);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 33723);
					match s {
						[] => true,
						['o', 'v', 's', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 42462);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 42463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'h', s @ ..] => {
							super::push(out, 39276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 33724);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 39277);
							s.is_empty()
						}
						['i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 39278);
							s.is_empty()
						}
						['k', 'c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 39279);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ärung.
pub(crate) fn n15477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15152);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 25430);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 33725);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 33726);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 33727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 25431);
			match s {
				[] => true,
				['k', 'r', 'e', '-', s @ ..] => {
					super::push(out, 33728);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 39281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltung.
pub(crate) fn n15488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15161);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 33743);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 39304);
							s.is_empty()
						}
						['d', 'n', 'a', 'l', s @ ..] => {
							super::push(out, 39305);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 39306);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 42475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 42476);
									s.is_empty()
								}
								['i', 'e', 'r', s @ ..] => {
									super::push(out, 42477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 42478);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 42479);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 43991);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 42480);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 42481);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 42482);
									match s {
										[] => true,
										['h', 'c', 'a', s @ ..] => {
											super::push(out, 43992);
											s.is_empty()
										}
										['n', 'u', 'n', s @ ..] => {
											super::push(out, 43993);
											s.is_empty()
										}
										['r', 'a', s @ ..] => {
											super::push(out, 43994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 39307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'm', 'o', s @ ..] => {
					super::push(out, 33744);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 33745);
					s.is_empty()
				}
				['w', 'r', 'e', 'v', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 39308);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 42483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'a', s @ ..] => {
							super::push(out, 39309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stung.
pub(crate) fn n15490(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', 'e', 'b', 'z', s @ ..] => {
			super::push(out, 25442);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 33748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'f', 'n', s @ ..] => {
			super::push(out, 25443);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 33749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 'l', s @ ..] => {
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 33750);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 39312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 's', s @ ..] => {
					super::push(out, 33751);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 39313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', s @ ..] => {
					super::push(out, 33752);
					match s {
						[] => true,
						['o', 'i', 't', 'a', 's', s @ ..] => {
							super::push(out, 39314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 25444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -attung.
pub(crate) fn n26405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25445);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 33753);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orettung.
pub(crate) fn n26406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25446);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 33754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nverhüttung.
pub(crate) fn n26407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25447);
	s.is_empty()
}

/// Words ending -utung.
pub(crate) fn n15492(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25448);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['b', 'h', s @ ..] => {
					super::push(out, 33755);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', 'n', s @ ..] => {
							super::push(out, 39316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'n', s @ ..] => {
					super::push(out, 33756);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 39317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'r', 'e', 'v', 's', s @ ..] => {
			super::push(out, 25449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fangsbereich.
pub(crate) fn n47976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44921);
	s.is_empty()
}

/// Words ending -gangsbereich.
pub(crate) fn n47977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44922);
	s.is_empty()
}

/// Words ending -ldungsbereich.
pub(crate) fn n47978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44923);
	s.is_empty()
}

/// Words ending -hungsbereich.
pub(crate) fn n47979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44924);
	s.is_empty()
}

/// Words ending -rungsbereich.
pub(crate) fn n47980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44925);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 45015);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 45016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsbereich.
pub(crate) fn n45278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42507);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 44926);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 44927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 44687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 44688);
					s.is_empty()
				}
				['k', 'g', 'i', 't', 'ä', s @ ..] => {
					super::push(out, 44689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itbereich.
pub(crate) fn n45280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42509);
	s.is_empty()
}

/// Words ending -otbereich.
pub(crate) fn n45281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42510);
	s.is_empty()
}

/// Words ending -zbereich.
pub(crate) fn n41793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39382);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					super::push(out, 44012);
					s.is_empty()
				}
				['u', 'h', 'c', 's', 'n', s @ ..] => {
					super::push(out, 44013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gereich.
pub(crate) fn n35626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33831);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 39383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -greich.
pub(crate) fn n26539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25564);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 33832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kreich.
pub(crate) fn n26540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25565);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 33833);
			match s {
				[] => true,
				['r', 'a', 'm', 's', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 39384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 33834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lreich.
pub(crate) fn n26541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25566);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 33835);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 39385);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 39386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 33836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nreich.
pub(crate) fn n26542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25567);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 33837);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 39387);
					match s {
						[] => true,
						['i', 'k', 'u', 'e', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 42511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 39388);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 39389);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							super::push(out, 42512);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 42513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rreich.
pub(crate) fn n26543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25568);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 33838);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 39390);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44014);
									s.is_empty()
								}
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 44015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 39391);
					s.is_empty()
				}
				['s', 'i', s @ ..] => {
					super::push(out, 39392);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 39393);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sreich.
pub(crate) fn n26544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25569);
	s.is_empty()
}

/// Words ending -treich.
pub(crate) fn n26545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25570);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 33839);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 39394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 33840);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 39395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 39396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 33841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßreich.
pub(crate) fn n26546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25571);
	s.is_empty()
}

/// Words ending -anisch.
pub(crate) fn n26640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25656);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 33901);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 39435);
					match s {
						[] => true,
						['i', 'a', s @ ..] => {
							super::push(out, 42526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 33902);
			match s {
				[] => true,
				['o', 'r', 'l', s @ ..] => {
					super::push(out, 39436);
					match s {
						[] => true,
						['e', 's', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 42527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 33903);
			match s {
				[] => true,
				['s', 'n', 'e', 'd', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 39437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 33904);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 39438);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnisch.
pub(crate) fn n26642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25658);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 33906);
			match s {
				[] => true,
				['h', 't', 's', 'u', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 39439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 33907);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pisch.
pub(crate) fn n15736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15395);
	s.is_empty()
}

/// Words ending -risch.
pub(crate) fn n15737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15396);
	s.is_empty()
}

/// Words ending -sisch.
pub(crate) fn n15738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15397);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25659);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 25660);
			match s {
				[] => true,
				['c', 'ä', 's', 'r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 33908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tisch.
pub(crate) fn n15739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15398);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 25661);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 25662);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 33909);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 33910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 25663);
			match s {
				[] => true,
				['i', 'e', 'r', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 39440);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 39441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 25664);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 33911);
					match s {
						[] => true,
						['i', 't', '\u{1}', s @ ..] => {
							super::push(out, 39442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 33912);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 39443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 33913);
					match s {
						[] => true,
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 39444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 25665);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 25666);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33914);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 33915);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 33916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 25667);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33917);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 39445);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 39446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 39447);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 39448);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42529);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 42530);
									s.is_empty()
								}
								['d', 'n', 'a', 'l', 't', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 42531);
									s.is_empty()
								}
								['l', 'e', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 42532);
									s.is_empty()
								}
								['ö', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 42533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'a', s @ ..] => {
					super::push(out, 33918);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 33919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 25668);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33920);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 39449);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 42534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 33921);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					super::push(out, 33922);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 39450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 33923);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39451);
							s.is_empty()
						}
						['o', 'h', 'a', '\u{1}', s @ ..] => {
							super::push(out, 39452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 25669);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 39453);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 33924);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 33925);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 33926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 25670);
			match s {
				[] => true,
				['p', 'a', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 33927);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 33928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 25671);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 33929);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 39455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 25672);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33930);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 39456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', s @ ..] => {
					super::push(out, 33931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 33933);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 39458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 25673);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 25674);
			match s {
				[] => true,
				['l', 'o', 'h', 'n', s @ ..] => {
					super::push(out, 33934);
					match s {
						[] => true,
						['e', 's', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 39459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 25675);
			match s {
				[] => true,
				['j', 'd', 'ü', 's', '\u{1}', s @ ..] => {
					super::push(out, 33935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uisch.
pub(crate) fn n15740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15399);
	s.is_empty()
}

/// Words ending -visch.
pub(crate) fn n15741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15400);
	s.is_empty()
}

/// Words ending -wisch.
pub(crate) fn n15742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15401);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25676);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 25677);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 25678);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33936);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 33937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'r', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 25679);
			s.is_empty()
		}
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 25680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zisch.
pub(crate) fn n15743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15402);
	s.is_empty()
}

/// Words ending -ßisch.
pub(crate) fn n15744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15403);
	match s {
		[] => true,
		['u', 'e', 'r', 'p', 'h', s @ ..] => {
			super::push(out, 25681);
			match s {
				[] => true,
				['c', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 33938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äisch.
pub(crate) fn n15745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15404);
	s.is_empty()
}

/// The word färöisch.
pub(crate) fn n15746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15405);
	s.is_empty()
}

/// Words ending -denaustausch.
pub(crate) fn n47721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44690);
	s.is_empty()
}

/// Words ending -onenaustausch.
pub(crate) fn n47722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44691);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44995);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 44996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -renaustausch.
pub(crate) fn n47723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44692);
	s.is_empty()
}

/// Words ending -pruch.
pub(crate) fn n15815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15474);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'v', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 34046);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 34047);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 42584);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44034);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'u', 't', s @ ..] => {
											super::push(out, 44035);
											match s {
												[] => true,
												['e', 'r', 't', 'r', 'e', 'v', 'n', 'i', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
													super::push(out, 44694);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['i', 'e', 'k', s @ ..] => {
													super::push(out, 44695);
													s.is_empty()
												}
												['ä', s @ ..] => {
													super::push(out, 44696);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 42585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 39528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 34048);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 34049);
					match s {
						[] => true,
						['g', 'n', 'i', 'l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 39529);
							s.is_empty()
						}
						['k', 'e', 'k', 's', 'k', 'c', 'ü', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 39530);
							s.is_empty()
						}
						['l', 'i', 'e', 't', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 39531);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 39532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 34050);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -berungsversuch.
pub(crate) fn n47728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44697);
	s.is_empty()
}

/// Words ending -tierungsversuch.
pub(crate) fn n47729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44698);
	match s {
		[] => true,
		['u', 'r', 'k', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 44928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gsdruck.
pub(crate) fn n36099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34254);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 42626);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 44049);
							match s {
								[] => true,
								['n', 'h', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 44700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 44050);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 44701);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 44702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 44051);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 44052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nsdruck.
pub(crate) fn n36100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34255);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 39660);
			s.is_empty()
		}
		['o', 'i', 't', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 42627);
					match s {
						[] => true,
						['k', 'i', 'l', 'b', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 44053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 42628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsdruck.
pub(crate) fn n36101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34256);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 39661);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 39662);
			match s {
				[] => true,
				['t', 'i', 'm', 'r', 'o', 'f', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 42629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usdruck.
pub(crate) fn n36102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34257);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 42630);
					match s {
						[] => true,
						['a', 'd', 'b', 'm', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 44054);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 44055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 42631);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 42632);
					match s {
						[] => true,
						['e', 't', 'u', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 44056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 42633);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 42634);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tdruck.
pub(crate) fn n27175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26150);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 34258);
			match s {
				[] => true,
				['e', 'z', s @ ..] => {
					super::push(out, 39663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 34259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zdruck.
pub(crate) fn n27177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26152);
	s.is_empty()
}

/// Words ending -ßdruck.
pub(crate) fn n27178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26153);
	s.is_empty()
}

/// The word retardierungsmittel.
pub(crate) fn n47745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44713);
	s.is_empty()
}

/// The word verdienstausfall.
pub(crate) fn n45582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42786);
	s.is_empty()
}

/// Words ending -tfall.
pub(crate) fn n17273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16853);
	s.is_empty()
}

/// The word riesenzufall.
pub(crate) fn n17274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16854);
	s.is_empty()
}

/// Words ending -enprogramm.
pub(crate) fn n47106(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', 'f', 'a', 'w', s @ ..] => {
			match s {
				[] => true,
				['e', 'i', 'm', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 44934);
					s.is_empty()
				}
				['m', 'o', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 44935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 44718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -loprogramm.
pub(crate) fn n47109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44145);
	s.is_empty()
}

/// Words ending -rprogramm.
pub(crate) fn n45648(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 44146);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 44936);
							s.is_empty()
						}
						['r', 'ö', 'f', '\u{1}', s @ ..] => {
							super::push(out, 44937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'i', 'l', 'f', 'l', 'e', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 44719);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 44938);
							s.is_empty()
						}
						['u', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 44939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 't', 'k', 'n', 'u', 'j', 'n', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 44147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sprogramm.
pub(crate) fn n45649(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'g', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 44148);
			s.is_empty()
		}
		['f', 'l', 'i', 'h', '\u{1}', s @ ..] => {
			super::push(out, 44149);
			s.is_empty()
		}
		['g', 'n', s @ ..] => {
			match s {
				[] => true,
				['i', 'l', 't', 'h', 'c', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 44720);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 44940);
							s.is_empty()
						}
						['h', 'e', 'i', 'z', 'r', 'e', 'm', 'u', '\u{1}', s @ ..] => {
							super::push(out, 44941);
							s.is_empty()
						}
						['n', 'n', 'e', 'k', 'r', 'e', 'h', 'c', 'a', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 44942);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 44943);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 44997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 44998);
									s.is_empty()
								}
								['s', 'a', 'p', 'n', 'a', 'r', 'u', 't', 'k', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 44999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 45000);
									match s {
										[] => true,
										['e', 'b', 'r', 'a', 'r', 'e', 'v', 't', 'x', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 45017);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'a', 'h', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 45001);
									s.is_empty()
								}
								['s', 'a', 'l', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 45002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'o', 'i', 't', 'i', 't', 's', 'e', 'v', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 44150);
			s.is_empty()
		}
		['t', 'i', 'e', s @ ..] => {
			match s {
				[] => true,
				['b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 44721);
					s.is_empty()
				}
				['h', 'r', 'e', 'h', 'c', 'i', 's', 'o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 44722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', 'r', 'i', 'z', '\u{1}', s @ ..] => {
			super::push(out, 44151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tprogramm.
pub(crate) fn n45650(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 44152);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['h', 'a', 'f', 'm', 'u', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 44723);
					s.is_empty()
				}
				['o', 'f', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 44724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', 'm', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 44153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -aufbauprogramm.
pub(crate) fn n45651(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44154);
			s.is_empty()
		}
		['r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 44155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zprogramm.
pub(crate) fn n45652(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 's', 'd', 'n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 44725);
					s.is_empty()
				}
				['u', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'g', 'u', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 44944);
							s.is_empty()
						}
						['t', 'l', 'e', 'w', 'm', 'u', '\u{1}', s @ ..] => {
							super::push(out, 44945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 44156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trogramm.
pub(crate) fn n42566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40077);
	s.is_empty()
}

/// Words ending -togramm.
pub(crate) fn n36934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35004);
	s.is_empty()
}

/// Words ending -skriegsministerium.
pub(crate) fn n47763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44726);
	s.is_empty()
}

/// Words ending --verteidigungsministerium.
pub(crate) fn n47764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44727);
	s.is_empty()
}

/// Words ending -sverkehrsministerium.
pub(crate) fn n47765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44728);
	s.is_empty()
}

/// The word bluterbrechen.
pub(crate) fn n48004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44946);
	s.is_empty()
}

/// Words ending -fungskosten.
pub(crate) fn n47778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44740);
	s.is_empty()
}

/// Words ending -hungskosten.
pub(crate) fn n47779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44741);
	s.is_empty()
}

/// Words ending -waltungskosten.
pub(crate) fn n47780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44742);
	s.is_empty()
}

/// Words ending -hinstein.
pub(crate) fn n42902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40379);
	match s {
		[] => true,
		['t', 'n', 'a', 'x', '\u{1}', s @ ..] => {
			super::push(out, 42994);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rinstein.
pub(crate) fn n42903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40380);
	match s {
		[] => true,
		['u', '\u{1}', s @ ..] => {
			super::push(out, 42995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnstein.
pub(crate) fn n37642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35665);
	s.is_empty()
}

/// The word tonstein.
pub(crate) fn n37643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35666);
	s.is_empty()
}

/// Words ending -rnstein.
pub(crate) fn n37644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35667);
	match s {
		[] => true,
		['o', 'h', 'c', 's', 'n', s @ ..] => {
			super::push(out, 40381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unstein.
pub(crate) fn n37645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35668);
	s.is_empty()
}

/// Words ending -rstein.
pub(crate) fn n29430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28214);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 40382);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 40383);
					s.is_empty()
				}
				['t', 's', 'a', s @ ..] => {
					super::push(out, 40384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 't', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 35669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sstein.
pub(crate) fn n29431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28215);
	s.is_empty()
}

/// Words ending -tstein.
pub(crate) fn n29432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28216);
	match s {
		[] => true,
		['i', 'm', s @ ..] => {
			super::push(out, 35670);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 35671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ustein.
pub(crate) fn n29433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28217);
	match s {
		[] => true,
		['a', 'b', 's', s @ ..] => {
			super::push(out, 35672);
			match s {
				[] => true,
				['a', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 40385);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ystein.
pub(crate) fn n29434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28218);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 35673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zstein.
pub(crate) fn n29435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28219);
	match s {
		[] => true,
		['t', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 35674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utein.
pub(crate) fn n18719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18217);
	s.is_empty()
}

/// Words ending -histenschwein.
pub(crate) fn n47785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44747);
	s.is_empty()
}

/// Words ending -inistenschwein.
pub(crate) fn n47786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44748);
	s.is_empty()
}

/// Words ending -enziation.
pub(crate) fn n37894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35901);
	s.is_empty()
}

/// Words ending -kation.
pub(crate) fn n29845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28600);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35902);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 35903);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 40481);
					s.is_empty()
				}
				['f', 'i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 43037);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 43038);
							s.is_empty()
						}
						['t', 'r', s @ ..] => {
							super::push(out, 43039);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 44223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 40482);
					s.is_empty()
				}
				['n', 'u', 'm', 'm', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 43040);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 44224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 43041);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 44225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'b', 'u', s @ ..] => {
					super::push(out, 40483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 35904);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 35905);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 40484);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 43042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 35906);
			match s {
				[] => true,
				['d', 'e', 'o', s @ ..] => {
					super::push(out, 40485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lation.
pub(crate) fn n29846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28601);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 35907);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 40486);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 43043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'k', s @ ..] => {
			super::push(out, 35908);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 40487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 35909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mation.
pub(crate) fn n29847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28602);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 35910);
			match s {
				[] => true,
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 43044);
							match s {
								[] => true,
								['r', 'n', s @ ..] => {
									super::push(out, 44226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 43045);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 43046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 35911);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 40488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nation.
pub(crate) fn n29848(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 35912);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					super::push(out, 40489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 40490);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 40491);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 40492);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44227);
									s.is_empty()
								}
								['e', 'd', 'ä', s @ ..] => {
									super::push(out, 44228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 43048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 35913);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			super::push(out, 35914);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 40493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nemanzipation.
pub(crate) fn n29849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28603);
	match s {
		[] => true,
		['e', 'k', s @ ..] => {
			super::push(out, 35915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ration.
pub(crate) fn n29850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28604);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			super::push(out, 35916);
			match s {
				[] => true,
				['k', 'e', 'd', '-', s @ ..] => {
					super::push(out, 40494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 35917);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 40495);
					s.is_empty()
				}
				['n', 'e', 'g', 'e', 'd', 'l', s @ ..] => {
					super::push(out, 40496);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 43049);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 43050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 40497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 'm', 'l', s @ ..] => {
			super::push(out, 35918);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 35919);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 35920);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', 'f', 'n', s @ ..] => {
					super::push(out, 40498);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 40499);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 43052);
							match s {
								[] => true,
								['n', 'i', 'm', 'd', 'a', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 44749);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 44750);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 43053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'a', s @ ..] => {
			super::push(out, 35921);
			match s {
				[] => true,
				['t', 's', 'e', 'r', '-', s @ ..] => {
					super::push(out, 40500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sation.
pub(crate) fn n29851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28605);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43054);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 43055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 40501);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 44229);
									match s {
										[] => true,
										['r', 'o', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 44950);
													match s {
														[] => true,
														['l', s @ ..] => super::t16::n48065(s, out),
														_ => false,
													}
												}
												['s', 't', 'i', s @ ..] => {
													super::push(out, 44951);
													match s {
														[] => true,
														['e', 'h', s @ ..] => super::t16::n48066(s, out),
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
									super::push(out, 44230);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 44231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 43056);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 44232);
									s.is_empty()
								}
								['l', 'o', 'k', 'd', s @ ..] => {
									super::push(out, 44233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 40502);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 40503);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 44234);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 44235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 44236);
									s.is_empty()
								}
								['m', 'y', 'l', 'o', 'p', 'o', s @ ..] => {
									super::push(out, 44237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'i', s @ ..] => {
							super::push(out, 43057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 40504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 35922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tation.
pub(crate) fn n29852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28606);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 35923);
			match s {
				[] => true,
				['l', 'i', 'd', 'o', 'h', s @ ..] => {
					super::push(out, 40505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'g', s @ ..] => {
			super::push(out, 35924);
			match s {
				[] => true,
				['e', 'v', 'r', s @ ..] => {
					super::push(out, 40506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 35925);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 40507);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 43058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 40508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 35926);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 35927);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 40509);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 43059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 40510);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 43060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'p', 'e', 'r', s @ ..] => {
			super::push(out, 35928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uation.
pub(crate) fn n29853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28607);
	s.is_empty()
}

/// Words ending -vation.
pub(crate) fn n29854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28608);
	s.is_empty()
}

/// Words ending -ction.
pub(crate) fn n19165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18645);
	match s {
		[] => true,
		['a', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 28609);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 28610);
			match s {
				[] => true,
				['l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 35929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 28611);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43061);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 43062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 40511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'u', 'j', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 28612);
			s.is_empty()
		}
		['u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 28613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etion.
pub(crate) fn n19166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18646);
	match s {
		[] => true,
		['l', 'p', 'm', s @ ..] => {
			super::push(out, 28614);
			s.is_empty()
		}
		['r', 'k', 's', s @ ..] => {
			super::push(out, 28615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ition.
pub(crate) fn n19167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18647);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 28616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'i', 'h', 'd', s @ ..] => {
			super::push(out, 28617);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 28618);
			match s {
				[] => true,
				['e', 'p', 'x', 'e', '-', s @ ..] => {
					super::push(out, 35931);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 40512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'o', 'k', '-', 'l', s @ ..] => {
			super::push(out, 28619);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 28620);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35932);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 35933);
					match s {
						[] => true,
						['m', 't', s @ ..] => {
							super::push(out, 40513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 't', 't', s @ ..] => {
			super::push(out, 28621);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 28622);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35934);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 43063);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 44238);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', 's', s @ ..] => {
									super::push(out, 43064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 35935);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 43065);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44239);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'l', s @ ..] => {
									super::push(out, 43066);
									match s {
										[] => true,
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 44240);
											s.is_empty()
										}
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
			super::push(out, 28623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ktion.
pub(crate) fn n19168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18648);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'n', 'e', 'h', s @ ..] => {
					super::push(out, 35936);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 40514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', 's', s @ ..] => {
					super::push(out, 35937);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 40515);
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
				['f', 'r', s @ ..] => {
					super::push(out, 35938);
					s.is_empty()
				}
				['j', 'o', s @ ..] => {
					super::push(out, 35939);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 35940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 't', 's', 'n', s @ ..] => {
			super::push(out, 28624);
			match s {
				[] => true,
				['o', 'k', 'o', 'h', s @ ..] => {
					super::push(out, 35941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'u', 'f', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 35942);
					s.is_empty()
				}
				['s', 'g', 'n', 'u', 'l', s @ ..] => {
					super::push(out, 35943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'd', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 't', s @ ..] => {
					super::push(out, 35944);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 40516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 40517);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 43067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 40518);
							match s {
								[] => true,
								['a', 'g', 'o', s @ ..] => {
									super::push(out, 43068);
									s.is_empty()
								}
								['g', 'n', 'u', 't', 's', s @ ..] => {
									super::push(out, 43069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 40519);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 43070);
									s.is_empty()
								}
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

/// Words ending -vention.
pub(crate) fn n19169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18649);
	match s {
		[] => true,
		['n', 'o', s @ ..] => {
			super::push(out, 28625);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 't', 'e', s @ ..] => {
							super::push(out, 40520);
							s.is_empty()
						}
						['s', 't', s @ ..] => {
							super::push(out, 40521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otion.
pub(crate) fn n19170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18650);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 28626);
			match s {
				[] => true,
				['-', 'e', 'v', 'a', 'h', 's', 'r', 'e', 't', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 35945);
					s.is_empty()
				}
				['e', 'v', 'a', 'h', 's', 'r', 'e', 't', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 35946);
					s.is_empty()
				}
				['y', 'd', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 35947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ption.
pub(crate) fn n19171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18651);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 28627);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 35948);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 35949);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40522);
							s.is_empty()
						}
						['e', 'r', 'o', s @ ..] => {
							super::push(out, 40523);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40524);
							s.is_empty()
						}
						['o', 'i', s @ ..] => {
							super::push(out, 40525);
							s.is_empty()
						}
						['r', 'e', 'p', 't', s @ ..] => {
							super::push(out, 40526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 28628);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 28629);
			s.is_empty()
		}
		['r', 'o', 's', 'd', s @ ..] => {
			super::push(out, 28630);
			s.is_empty()
		}
		['u', 'r', 'r', 'o', 'k', 's', s @ ..] => {
			super::push(out, 28631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtion.
pub(crate) fn n19172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18652);
	s.is_empty()
}

/// Words ending -stion.
pub(crate) fn n19173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18653);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 28632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ution.
pub(crate) fn n19174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18654);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 28633);
			s.is_empty()
		}
		['l', 'o', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 35950);
					s.is_empty()
				}
				['v', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40527);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40528);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 40529);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 43071);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 40530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 28634);
			match s {
				[] => true,
				['i', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 40531);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40532);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 40533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erpapier.
pub(crate) fn n38365(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 40668);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['k', 'l', 'a', 'k', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 43117);
					s.is_empty()
				}
				['m', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 43118);
					s.is_empty()
				}
				['p', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 43119);
					s.is_empty()
				}
				['r', 't', 'l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 43120);
					s.is_empty()
				}
				['z', 'z', 'i', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 43121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'c', 'u', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 40669);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', 'm', 'i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 43122);
					s.is_empty()
				}
				['l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 43123);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -spapier.
pub(crate) fn n38366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36340);
	match s {
		[] => true,
		['a', 'l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 40670);
			s.is_empty()
		}
		['g', 'n', 'u', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 43124);
					s.is_empty()
				}
				['t', 'i', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 43125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 40671);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 40672);
			s.is_empty()
		}
		['n', 'o', 'i', 't', s @ ..] => {
			match s {
				[] => true,
				['i', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 43126);
					s.is_empty()
				}
				['p', 'o', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 43127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 40673);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 43128);
					s.is_empty()
				}
				['i', 'e', 'k', 'h', 'c', 'i', 'l', 'n', 'i', 'e', 'h', 'c', 's', 'r', 'h', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 43129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 43130);
					s.is_empty()
				}
				['m', 'k', 'c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 43131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tpapier.
pub(crate) fn n38367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36341);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 40674);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'g', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 44247);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 44248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 40675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ipier.
pub(crate) fn n20159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19599);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 29378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mpier.
pub(crate) fn n20160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19600);
	match s {
		[] => true,
		['y', s @ ..] => {
			super::push(out, 29379);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -npier.
pub(crate) fn n20161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19601);
	match s {
		[] => true,
		['o', 't', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 29380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -opier.
pub(crate) fn n20162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19602);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 29381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -upier.
pub(crate) fn n20163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19603);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 29382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zpier.
pub(crate) fn n20164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19604);
	match s {
		[] => true,
		['l', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 29383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enswasser.
pub(crate) fn n45986(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 44258);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 44259);
			match s {
				[] => true,
				['n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 44752);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rswasser.
pub(crate) fn n43310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40744);
	s.is_empty()
}

/// Words ending -sswasser.
pub(crate) fn n43311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40745);
	s.is_empty()
}

/// Words ending -tswasser.
pub(crate) fn n43312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40746);
	s.is_empty()
}

/// Words ending -twasser.
pub(crate) fn n38533(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 40747);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 40748);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 40749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uwasser.
pub(crate) fn n38534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36498);
	match s {
		[] => true,
		['a', 'n', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 40750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zwasser.
pub(crate) fn n38535(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 40751);
			match s {
				[] => true,
				['e', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 43156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 40752);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -luleiter.
pub(crate) fn n38626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36578);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 40818);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lzleiter.
pub(crate) fn n38627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36579);
	match s {
		[] => true,
		['o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 40819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reiter.
pub(crate) fn n31036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29727);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 36580);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 36581);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 40820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 36582);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 36583);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weiter.
pub(crate) fn n31037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29728);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 36584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -liter.
pub(crate) fn n20431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19865);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 29729);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 29730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulter.
pub(crate) fn n20443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19877);
	match s {
		[] => true,
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 29762);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 29763);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -älter.
pub(crate) fn n20444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19878);
	match s {
		[] => true,
		['h', 'e', 'b', 'l', s @ ..] => {
			super::push(out, 29764);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36617);
					match s {
						[] => true,
						['m', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 40837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wister.
pub(crate) fn n31142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29829);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 36668);
			match s {
				[] => true,
				['c', 's', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 43207);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 36669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lster.
pub(crate) fn n20481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19915);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 29831);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 36671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 29832);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29833);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 29834);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 36673);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 36674);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 40857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 29835);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nster.
pub(crate) fn n20483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19917);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 29836);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 29837);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 36675);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 40858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 29838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oster.
pub(crate) fn n20484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19918);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 29839);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 29840);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 36676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 29841);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 29842);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 29843);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 29844);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 29845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pster.
pub(crate) fn n20485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19919);
	s.is_empty()
}

/// Words ending -rster.
pub(crate) fn n20486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19920);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 29846);
			s.is_empty()
		}
		['u', 'w', '\u{1}', s @ ..] => {
			super::push(out, 29847);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 29848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sster.
pub(crate) fn n20487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19921);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 29849);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uster.
pub(crate) fn n20488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19922);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 29850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 29851);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 36678);
					match s {
						[] => true,
						['l', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 40859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 36679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 29852);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 29853);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29854);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 29855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öster.
pub(crate) fn n20489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19923);
	s.is_empty()
}

/// Words ending -üster.
pub(crate) fn n20490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19924);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 29856);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29857);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 36680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 29858);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 29859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utter.
pub(crate) fn n20495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19929);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 29894);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 29895);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 36712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 29896);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 36713);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 36714);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 36715);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 43209);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 43210);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 44271);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 44272);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 43211);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43212);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 40873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 36716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 29897);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 29898);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 29899);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 36717);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 40874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 36718);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 36719);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40875);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 40876);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 40877);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 40878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					super::push(out, 36720);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 40879);
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 43213);
									s.is_empty()
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
						['e', 't', 'n', s @ ..] => {
							super::push(out, 40880);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 43214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 40881);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 43215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 36721);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 40882);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 36722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 29900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zugverkehr.
pub(crate) fn n47251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44274);
	s.is_empty()
}

/// Words ending -hverkehr.
pub(crate) fn n43509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40927);
	s.is_empty()
}

/// Words ending -iverkehr.
pub(crate) fn n43510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40928);
	s.is_empty()
}

/// Words ending -kverkehr.
pub(crate) fn n43511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40929);
	s.is_empty()
}

/// Words ending -lverkehr.
pub(crate) fn n43512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40930);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 44275);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 44276);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 44754);
							s.is_empty()
						}
						['i', 'g', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 44755);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 44756);
							match s {
								[] => true,
								['i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 44952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 44277);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 44278);
					s.is_empty()
				}
				['u', 'd', 'i', 'v', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 44279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 43229);
			match s {
				[] => true,
				['d', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 44280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nverkehr.
pub(crate) fn n43513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40931);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 44281);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 44282);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					super::push(out, 44283);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 44284);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 44757);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 44758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 44285);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 44286);
					match s {
						[] => true,
						['s', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 44759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 44287);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 44288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 43230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -overkehr.
pub(crate) fn n43514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40932);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 43231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rverkehr.
pub(crate) fn n43515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40933);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 44289);
					s.is_empty()
				}
				['i', 'g', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 44290);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 44291);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 44760);
							s.is_empty()
						}
						['ü', 'g', '\u{1}', s @ ..] => {
							super::push(out, 44761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 44292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sverkehr.
pub(crate) fn n43516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40934);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 44293);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 44294);
					match s {
						[] => true,
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 44762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 43232);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 44295);
					match s {
						[] => true,
						['t', 't', 'i', 'm', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 44953);
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 44954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 44296);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 44955);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 44956);
									match s {
										[] => true,
										['g', 'e', 'g', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 45005);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 44957);
									match s {
										[] => true,
										['u', 'e', 'r', 'k', '\u{1}', s @ ..] => {
											super::push(out, 45006);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'l', 'f', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 44297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 43233);
			match s {
				[] => true,
				['e', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 44298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 43234);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 43235);
			match s {
				[] => true,
				['e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 44299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 43236);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 43237);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 43238);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 44300);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 44301);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 44302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 43239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tverkehr.
pub(crate) fn n43517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40935);
	match s {
		[] => true,
		['d', 'a', 't', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44303);
					s.is_empty()
				}
				['n', 'e', 'n', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 44304);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 44305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 43240);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 44306);
					s.is_empty()
				}
				['i', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 44307);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 44308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 43241);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 43242);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 43243);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 44309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'g', 't', 't', 'ü', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 43244);
			s.is_empty()
		}
		['x', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 43245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zverkehr.
pub(crate) fn n43518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40936);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 43246);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 43247);
			match s {
				[] => true,
				['a', 's', 'r', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44763);
							s.is_empty()
						}
						['n', 'e', 'n', 'e', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 44764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßverkehr.
pub(crate) fn n43519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40937);
	s.is_empty()
}

/// Words ending -ismus.
pub(crate) fn n21447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20848);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', 'u', 'j', 'i', s @ ..] => {
					super::push(out, 36991);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					super::push(out, 36992);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 41016);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 30432);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 36993);
					match s {
						[] => true,
						['ü', 'k', '\u{1}', s @ ..] => {
							super::push(out, 41018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 30433);
			match s {
				[] => true,
				['d', 'i', s @ ..] => {
					super::push(out, 36994);
					s.is_empty()
				}
				['i', 'o', s @ ..] => {
					super::push(out, 36995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 30434);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 36996);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 36997);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 36998);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 41019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 30435);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 30436);
			match s {
				[] => true,
				['o', 'l', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 41020);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 41021);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43277);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 43278);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 43279);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41022);
							s.is_empty()
						}
						['o', 's', 'a', 'm', 'o', s @ ..] => {
							super::push(out, 41023);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 41024);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43280);
									s.is_empty()
								}
								['i', 't', s @ ..] => {
									super::push(out, 43281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 41025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 36999);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 37000);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41026);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 41027);
							s.is_empty()
						}
						['r', 'o', 'm', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43282);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 43283);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 43284);
									s.is_empty()
								}
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
			super::push(out, 30437);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 30438);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 30439);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43285);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 43286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 41028);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 41029);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 43287);
									s.is_empty()
								}
								['r', 't', s @ ..] => {
									super::push(out, 43288);
									s.is_empty()
								}
								['z', 'n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44317);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 44318);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 44319);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 44767);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 44320);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 43289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', s @ ..] => {
							super::push(out, 41030);
							s.is_empty()
						}
						['n', s @ ..] => super::t16::n43622(s, out),
						['r', s @ ..] => super::t16::n43623(s, out),
						['t', s @ ..] => super::t16::n43624(s, out),
						['u', s @ ..] => super::t16::n43625(s, out),
						['v', s @ ..] => super::t16::n43626(s, out),
						_ => false,
					}
				}
				['e', 'l', s @ ..] => super::t16::n39086(s, out),
				['i', s @ ..] => super::t16::n39087(s, out),
				['o', s @ ..] => super::t16::n39088(s, out),
				['u', s @ ..] => super::t16::n39089(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t16::n31807(s, out),
		['n', s @ ..] => super::t16::n31808(s, out),
		['o', s @ ..] => super::t16::n31809(s, out),
		['p', s @ ..] => super::t16::n31810(s, out),
		['r', s @ ..] => super::t16::n31811(s, out),
		['s', s @ ..] => super::t16::n31812(s, out),
		['t', s @ ..] => super::t16::n31813(s, out),
		['u', 'h', s @ ..] => super::t16::n31814(s, out),
		['v', s @ ..] => super::t16::n31815(s, out),
		['x', 'e', s @ ..] => super::t16::n31816(s, out),
		['y', s @ ..] => super::t16::n31817(s, out),
		['z', s @ ..] => super::t16::n31818(s, out),
		['ä', 'p', s @ ..] => super::t16::n31819(s, out),
		_ => false,
	}
}

/// The word calciumbicarbonat.
pub(crate) fn n47817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44775);
	s.is_empty()
}

/// The word ammoniumbicarbonat.
pub(crate) fn n47818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44776);
	s.is_empty()
}

/// The word natriumbicarbonat.
pub(crate) fn n47819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44777);
	s.is_empty()
}

/// The word natriumcarbonat.
pub(crate) fn n47820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44778);
	s.is_empty()
}

/// Words ending -udampfschifffahrtsgesellschaft.
pub(crate) fn n48018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44960);
	s.is_empty()
}

/// Words ending -nschaft.
pub(crate) fn n39349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37223);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 41196);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 43374);
					match s {
						[] => true,
						['i', 'e', 'l', 't', s @ ..] => {
							super::push(out, 44362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 'e', 's', 'g', 'n', 'i', s @ ..] => {
					super::push(out, 43375);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 43376);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 44363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 43377);
					match s {
						[] => true,
						['s', 'i', 'w', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 44782);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 44783);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 44784);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'g', s @ ..] => {
											super::push(out, 44961);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 44962);
											s.is_empty()
										}
										['ä', s @ ..] => {
											super::push(out, 44963);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 45007);
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
										['g', s @ ..] => {
											super::push(out, 44964);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 44965);
											s.is_empty()
										}
										['n', 'o', 'i', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 45008);
													s.is_empty()
												}
												['t', 'a', 's', s @ ..] => {
													super::push(out, 45009);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'f', s @ ..] => {
											super::push(out, 44966);
											match s {
												[] => true,
												['a', 'h', 'c', 's', 'n', s @ ..] => {
													super::push(out, 45010);
													s.is_empty()
												}
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
				['t', 'l', s @ ..] => {
					super::push(out, 43378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 41197);
			match s {
				[] => true,
				['e', 'm', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['n', 'n', s @ ..] => {
							super::push(out, 44364);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 44785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 44365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 41198);
			match s {
				[] => true,
				['a', 'm', 'e', s @ ..] => {
					super::push(out, 43379);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 44366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rschaft.
pub(crate) fn n39350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37224);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 41199);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 43380);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 44367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 44786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 44368);
							match s {
								[] => true,
								['e', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 44787);
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
						['n', s @ ..] => {
							super::push(out, 44369);
							s.is_empty()
						}
						['ä', 'j', s @ ..] => {
							super::push(out, 44370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['h', 'o', 'w', 'n', s @ ..] => {
							super::push(out, 44371);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 44372);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 44789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'h', s @ ..] => {
					super::push(out, 43381);
					match s {
						[] => true,
						['ü', s @ ..] => {
							super::push(out, 44373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 44374);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 44375);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 44790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 44376);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 44377);
							match s {
								[] => true,
								['i', 'r', 's', s @ ..] => {
									super::push(out, 44791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 44378);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									super::push(out, 44792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 43382);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 44379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 41200);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 43383);
					match s {
						[] => true,
						['w', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 44380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 41201);
			match s {
				[] => true,
				['e', 'h', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 44381);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 44382);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 44383);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 44384);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 44385);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44793);
									s.is_empty()
								}
								['e', 'p', s @ ..] => {
									super::push(out, 44794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 44795);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 44796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									super::push(out, 44797);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 44798);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 44799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'n', 'u', s @ ..] => {
							super::push(out, 44386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tschaft.
pub(crate) fn n39352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37226);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 41203);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 41204);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 43385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'e', 'n', 'k', 'd', s @ ..] => {
			super::push(out, 41205);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 41206);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 44387);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									super::push(out, 44800);
									s.is_empty()
								}
								['g', 'u', s @ ..] => {
									super::push(out, 44801);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 44967);
											s.is_empty()
										}
										['n', 'o', 'i', 't', 'a', 'k', s @ ..] => {
											super::push(out, 44968);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 44969);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 44802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'w', 'n', 'a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 43386);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 44388);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 44389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 41207);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 41208);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 41209);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 43387);
					s.is_empty()
				}
				['i', 'w', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 44390);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 44391);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 44803);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 44804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 44392);
							s.is_empty()
						}
						['h', 'c', 'u', s @ ..] => {
							super::push(out, 44393);
							s.is_empty()
						}
						['k', 'i', s @ ..] => {
							super::push(out, 44394);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44805);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 44806);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'r', s @ ..] => {
							super::push(out, 44395);
							s.is_empty()
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 44396);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 44397);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 44807);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44970);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 45011);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 45012);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 44808);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 44809);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 44971);
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
								['i', s @ ..] => {
									super::push(out, 44810);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 44811);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 44812);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 44813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 44398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lzschaft.
pub(crate) fn n43824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41210);
	match s {
		[] => true,
		['o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 43388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -recht.
pub(crate) fn n21948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21331);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 30815);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 30816);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 30817);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 37312);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 37313);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41251);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 41252);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 43412);
									s.is_empty()
								}
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
			super::push(out, 30818);
			match s {
				[] => true,
				['d', 'e', s @ ..] => {
					super::push(out, 37314);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 37315);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 37316);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 41253);
							s.is_empty()
						}
						['r', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 41254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 30819);
			match s {
				[] => true,
				['a', 'r', 't', 's', 'n', s @ ..] => {
					super::push(out, 37317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 30820);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 37318);
					s.is_empty()
				}
				['h', 'a', 'w', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'u', s @ ..] => {
							super::push(out, 41255);
							s.is_empty()
						}
						['s', 'g', s @ ..] => {
							super::push(out, 41256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 37319);
					match s {
						[] => true,
						['v', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 41257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', s @ ..] => {
					super::push(out, 37320);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 37321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 30821);
			match s {
				[] => true,
				['m', 'i', 't', 's', 'n', 'e', 'u', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 37322);
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
						['b', s @ ..] => {
							super::push(out, 41258);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 41259);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 41260);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									super::push(out, 43413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 41261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 37323);
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
						['d', s @ ..] => {
							super::push(out, 41262);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 41263);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 41264);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 43414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 37324);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41265);
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 41266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 30822);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37325);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 37326);
					match s {
						[] => true,
						['f', 'i', 'r', 'g', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 41267);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 'k', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 43415);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 37327);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41268);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 41269);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 43417);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 43418);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 43419);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 43420);
									match s {
										[] => true,
										['m', 'i', 't', 's', 'e', 'b', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44814);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 44815);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 44816);
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
										['g', 'i', 'e', 'w', 'r', 'e', 'v', 's', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 44817);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 44818);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 44406);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 43421);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43422);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 44407);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 43423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 37328);
					match s {
						[] => true,
						['c', 'u', 'r', 'p', 's', 'r', 'e', 'd', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 41270);
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
								['b', s @ ..] => {
									super::push(out, 43424);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43425);
									match s {
										[] => true,
										['h', 'a', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 44408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 43426);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'e', 'z', 'o', 'r', 'p', 'l', 'a', 'n', 'o', 'i', 't', 'a', 'n', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 37329);
					s.is_empty()
				}
				['t', s @ ..] => super::t16::n39473(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t16::n32242(s, out),
		['u', s @ ..] => {
			super::push(out, 30824);
			s.is_empty()
		}
		['z', s @ ..] => super::t16::n32244(s, out),
		['ß', s @ ..] => super::t16::n32245(s, out),
		_ => false,
	}
}

/// Words ending -ielicht.
pub(crate) fn n39487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37339);
	s.is_empty()
}

/// Words ending -flicht.
pub(crate) fn n32256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30837);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 37340);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41285);
					match s {
						[] => true,
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 43437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 37341);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					super::push(out, 41286);
					s.is_empty()
				}
				['l', 'l', 'a', 'n', s @ ..] => {
					super::push(out, 41287);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 41288);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43438);
							match s {
								[] => true,
								['d', 'm', 'u', 'i', 'r', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 44413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 43439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 41289);
					match s {
						[] => true,
						['e', 't', 's', s @ ..] => {
							super::push(out, 43440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 43441);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 43442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 41290);
					match s {
						[] => true,
						['g', 'n', 'u', 'r', 'e', 'h', 'c', 'i', 's', 'r', s @ ..] => {
							super::push(out, 43443);
							s.is_empty()
						}
						['n', 'o', 'i', 't', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 44414);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 44415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 43444);
							s.is_empty()
						}
						['t', 'i', 'e', 'h', 'n', 'e', 's', s @ ..] => {
							super::push(out, 43445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 43446);
							s.is_empty()
						}
						['s', 'n', 'e', 'i', 'd', 's', s @ ..] => {
							super::push(out, 43447);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 43448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 37342);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -glicht.
pub(crate) fn n32257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30838);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 37343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hlicht.
pub(crate) fn n32258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30839);
	s.is_empty()
}

/// Words ending -ilicht.
pub(crate) fn n32259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30840);
	s.is_empty()
}

/// Words ending -klicht.
pub(crate) fn n32260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30841);
	s.is_empty()
}

/// Words ending -llicht.
pub(crate) fn n32261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30842);
	s.is_empty()
}

/// Words ending -mlicht.
pub(crate) fn n32262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30843);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 37344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nlicht.
pub(crate) fn n32263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30844);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 37345);
			match s {
				[] => true,
				['g', 'r', s @ ..] => {
					super::push(out, 41292);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 41293);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41294);
					match s {
						[] => true,
						['r', 'e', 't', 'a', s @ ..] => {
							super::push(out, 43449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 41295);
					match s {
						[] => true,
						['m', 'a', 'l', 'n', s @ ..] => {
							super::push(out, 43450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 41296);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 41297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 37346);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 37347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olicht.
pub(crate) fn n32264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30845);
	s.is_empty()
}

/// Words ending -plicht.
pub(crate) fn n32265(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37348);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 37349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rlicht.
pub(crate) fn n32266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30846);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 41298);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 41299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 41300);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 41301);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 41302);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 43451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 41303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 37350);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 37351);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 37352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -slicht.
pub(crate) fn n32267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30847);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 37353);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 37354);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 37355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tlicht.
pub(crate) fn n32268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30848);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 37356);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 37357);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43452);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 43453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 37358);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 41304);
					match s {
						[] => true,
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 43454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 37359);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 37360);
			match s {
				[] => true,
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 41305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulicht.
pub(crate) fn n32269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30849);
	match s {
		[] => true,
		['a', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 37361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zlicht.
pub(crate) fn n32270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30850);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 37362);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 37363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßlicht.
pub(crate) fn n32271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30851);
	s.is_empty()
}

/// Words ending -ülicht.
pub(crate) fn n32272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30852);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 37364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nicht.
pub(crate) fn n21955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21338);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 30853);
			match s {
				[] => true,
				['i', 'e', 'm', 's', 's', 'i', 'g', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 37365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 30854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -richt.
pub(crate) fn n21956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21339);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 30855);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 37366);
					match s {
						[] => true,
						['-', 'n', s @ ..] => {
							super::push(out, 41306);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41307);
							match s {
								[] => true,
								['e', 't', 't', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 43455);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 43456);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 43457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 43458);
									s.is_empty()
								}
								['t', 'ä', s @ ..] => {
									super::push(out, 43459);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 43460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 41308);
							s.is_empty()
						}
						['z', 'r', s @ ..] => {
							super::push(out, 41309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 37367);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 41310);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 41311);
							s.is_empty()
						}
						['l', 'i', s @ ..] => {
							super::push(out, 41312);
							s.is_empty()
						}
						['n', 'e', 'l', s @ ..] => {
							super::push(out, 41313);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41314);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									super::push(out, 43461);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 44416);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 41315);
							match s {
								[] => true,
								['g', 'n', 'u', 't', s @ ..] => {
									super::push(out, 43462);
									match s {
										[] => true,
										['l', 'a', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 44417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 43463);
									match s {
										[] => true,
										['i', 'e', 'b', 'r', 'a', 's', s @ ..] => {
											super::push(out, 44418);
											match s {
												[] => true,
												['e', 'd', 'n', 'u', s @ ..] => {
													super::push(out, 44819);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'l', s @ ..] => {
							super::push(out, 41316);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43464);
									s.is_empty()
								}
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
			super::push(out, 30856);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 37368);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							match s {
								[] => true,
								['h', 'e', s @ ..] => {
									super::push(out, 43465);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 43466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 37369);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 41317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 37370);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 41318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 30857);
			match s {
				[] => true,
				['e', 't', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 41319);
							s.is_empty()
						}
						['d', 'n', s @ ..] => {
							super::push(out, 41320);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 41321);
							match s {
								[] => true,
								['d', 'n', 'u', 'k', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 44419);
											s.is_empty()
										}
										['r', 'u', s @ ..] => {
											super::push(out, 44420);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 43467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 41322);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 41323);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 43468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 41324);
							match s {
								[] => true,
								['c', 's', 'i', s @ ..] => super::t16::n46340(s, out),
								_ => false,
							}
						}
						['k', 'i', s @ ..] => super::t16::n43956(s, out),
						['l', s @ ..] => super::t16::n43957(s, out),
						['m', 'm', s @ ..] => {
							super::push(out, 41325);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 'c', 'e', s @ ..] => super::t16::n46346(s, out),
								['o', s @ ..] => super::t16::n46347(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t16::n43960(s, out),
						['s', s @ ..] => super::t16::n43961(s, out),
						['t', 'r', s @ ..] => super::t16::n43962(s, out),
						['z', s @ ..] => super::t16::n43963(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicht.
pub(crate) fn n21957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21340);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 30858);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 30859);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 37371);
					match s {
						[] => true,
						['d', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 41330);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'e', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 43482);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 43483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							super::push(out, 41331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43484);
									s.is_empty()
								}
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
			super::push(out, 30860);
			match s {
				[] => true,
				['u', 'a', 'l', s @ ..] => {
					super::push(out, 37372);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 30861);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41333);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 30862);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 30863);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 37373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 30864);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 30865);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 37374);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 37375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 30866);
			match s {
				[] => true,
				['u', 'a', 'r', 'o', s @ ..] => {
					super::push(out, 37376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 30867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wicht.
pub(crate) fn n21958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21341);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30868);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 30869);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 37377);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 41335);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 43485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 41336);
							match s {
								[] => true,
								['c', 'i', 'e', 'l', 'g', s @ ..] => {
									match s {
										[] => true,
										['n', 'u', 't', s @ ..] => {
											super::push(out, 44432);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44433);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'o', s @ ..] => {
							super::push(out, 41337);
							s.is_empty()
						}
						['n', 'i', s @ ..] => {
							super::push(out, 41338);
							s.is_empty()
						}
						['r', 'e', 'w', s @ ..] => {
							super::push(out, 41339);
							match s {
								[] => true,
								['h', 'c', 's', 'b', s @ ..] => {
									super::push(out, 43486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 37378);
					match s {
						[] => true,
						['ö', s @ ..] => {
							super::push(out, 41340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zicht.
pub(crate) fn n21959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21342);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 30870);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 41341);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 41342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ankheit.
pub(crate) fn n32354(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 37430);
			s.is_empty()
		}
		['r', 'k', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 41369);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 43495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 41370);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 41371);
					s.is_empty()
				}
				['f', 'f', s @ ..] => {
					super::push(out, 41372);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 41373);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 41374);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					super::push(out, 41375);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41376);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'z', s @ ..] => {
									super::push(out, 44436);
									s.is_empty()
								}
								['s', 'o', s @ ..] => {
									super::push(out, 44437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 43496);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 44438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 41377);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 44439);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 44440);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 44441);
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
						['g', 'n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 44442);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 44443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'i', 's', 's', 'e', 'r', 'p', s @ ..] => {
							super::push(out, 43497);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 43499);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 43500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'n', s @ ..] => {
					super::push(out, 41378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nheit.
pub(crate) fn n22024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21398);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 30928);
			match s {
				[] => true,
				['b', 'e', s @ ..] => {
					super::push(out, 37435);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 37436);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44444);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44445);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'f', 'u', 'z', 'n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44446);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 44447);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 41379);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 41380);
							match s {
								[] => true,
								['u', 'b', s @ ..] => {
									match s {
										[] => true,
										['e', 'g', 't', s @ ..] => {
											super::push(out, 44448);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44449);
											s.is_empty()
										}
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
					super::push(out, 37437);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 'c', 's', 'e', s @ ..] => {
									super::push(out, 43501);
									s.is_empty()
								}
								['o', 't', s @ ..] => {
									super::push(out, 43502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'w', 'e', s @ ..] => {
							super::push(out, 41381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 37438);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41382);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 41383);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43503);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 44450);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44451);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 44452);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 43504);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44453);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44454);
											match s {
												[] => true,
												['e', 'b', s @ ..] => {
													super::push(out, 44823);
													match s {
														[] => true,
														['ü', 't', s @ ..] => super::t16::n48032(s, out),
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
							super::push(out, 41384);
							match s {
								[] => true,
								['a', 'g', 'r', 'e', 'v', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 44455);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'z', 'e', 'b', s @ ..] => {
							super::push(out, 41385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 37439);
					match s {
						[] => true,
						['c', 'o', 'r', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 43505);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', 'r', 't', 'e', 'g', s @ ..] => {
							super::push(out, 41386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 37440);
					match s {
						[] => true,
						['m', 'o', 'n', 'e', 'g', s @ ..] => {
							super::push(out, 41387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 37441);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 37442);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41388);
							match s {
								[] => true,
								['w', 'n', s @ ..] => {
									super::push(out, 43507);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 41389);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 41390);
							match s {
								[] => true,
								['a', 'l', 'e', 'g', 's', s @ ..] => {
									super::push(out, 43508);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 44457);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 44458);
											match s {
												[] => true,
												['e', 'g', 'n', s @ ..] => {
													super::push(out, 44824);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'r', s @ ..] => {
											super::push(out, 44459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 44460);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44461);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											super::push(out, 44462);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 44463);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => super::t16::n46393(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => super::t16::n39606(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t16::n32363(s, out),
		['i', s @ ..] => super::t16::n32364(s, out),
		['r', s @ ..] => super::t16::n32365(s, out),
		['ö', s @ ..] => super::t16::n32366(s, out),
		_ => false,
	}
}

/// Words ending -hoheit.
pub(crate) fn n22025(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 30933);
			s.is_empty()
		}
		['r', 'u', s @ ..] => {
			super::push(out, 30934);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 30935);
			s.is_empty()
		}
		['t', 'e', s @ ..] => {
			super::push(out, 30936);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -knappheit.
pub(crate) fn n22026(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 30937);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 30938);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 30939);
			s.is_empty()
		}
		['r', 'e', 'i', s @ ..] => {
			super::push(out, 30940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rheit.
pub(crate) fn n22027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21399);
	match s {
		[] => true,
		['a', 'l', 'k', 'n', 'e', s @ ..] => {
			super::push(out, 30941);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 30942);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 37446);
					s.is_empty()
				}
				['h', 'c', 'i', 's', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 41395);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 41396);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41397);
							s.is_empty()
						}
						['t', 'r', s @ ..] => {
							super::push(out, 41398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 30943);
			match s {
				[] => true,
				['a', 'w', 'n', 'e', s @ ..] => {
					super::push(out, 37447);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 41399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sheit.
pub(crate) fn n22028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21400);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 30944);
			s.is_empty()
		}
		['s', 'i', 'w', 'e', 'g', 's', s @ ..] => {
			super::push(out, 30945);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -theit.
pub(crate) fn n22029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21401);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 30946);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 37449);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 37450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', 'i', 'l', 'e', s @ ..] => {
					super::push(out, 37451);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 37452);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 37453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 30947);
			match s {
				[] => true,
				['n', 'a', 'w', 'e', 'g', 's', s @ ..] => {
					super::push(out, 37454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'd', s @ ..] => {
			super::push(out, 30948);
			s.is_empty()
		}
		['f', 'n', s @ ..] => {
			super::push(out, 30949);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'z', s @ ..] => {
					super::push(out, 37455);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 37456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37457);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41400);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 41401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'r', s @ ..] => {
					super::push(out, 37458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 30950);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 37459);
					s.is_empty()
				}
				['c', 'ü', s @ ..] => {
					super::push(out, 37460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 30951);
			match s {
				[] => true,
				['m', 'i', 't', 's', 'e', 'b', 't', s @ ..] => {
					super::push(out, 37461);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 37462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 30952);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 30953);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 41402);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 41403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', 'r', s @ ..] => {
							super::push(out, 41404);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41405);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 41406);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'i', 'r', 'o', 's', s @ ..] => {
									super::push(out, 43513);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'i', 't', 'o', s @ ..] => {
							super::push(out, 41407);
							s.is_empty()
						}
						['z', 'n', s @ ..] => {
							super::push(out, 41408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 43515);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 43516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 41409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 30954);
			match s {
				[] => true,
				['i', 'e', 'r', s @ ..] => {
					super::push(out, 37463);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41410);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 41411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 37464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 30955);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 37465);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 37466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 30956);
			match s {
				[] => true,
				['t', 'i', 'p', s @ ..] => {
					super::push(out, 37467);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uheit.
pub(crate) fn n22030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21402);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 30957);
			s.is_empty()
		}
		['e', 'n', s @ ..] => {
			super::push(out, 30958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vheit.
pub(crate) fn n22031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21403);
	s.is_empty()
}

/// Words ending -äßheit.
pub(crate) fn n22032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21404);
	s.is_empty()
}

