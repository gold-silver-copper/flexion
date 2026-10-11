use flexion::Path;

/// Words ending -удишь.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1714);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1715);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1716);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5219);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5220);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5221);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11354);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 11355);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11356);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5222);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1717);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5224);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5225);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1718);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5227);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5228);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5229);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5230);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1719);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1720);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5232);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5234);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5235);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5236);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11362);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24504);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24505);
											s.is_empty()
										}
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
					super::push(out, 5237);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18307);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18308);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 24506);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18309);
									s.is_empty()
								}
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
			super::push(out, 1722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5238);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5239);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11364);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11365);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18310);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 18311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 11366);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11367);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18313);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24507);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5241);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5242);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5243);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11372);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5245);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1723);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1724);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5246);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5247);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5248);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5249);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5250);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1725);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1726);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5252);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 11379);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18314);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5253);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t6::n5493(s, out),
				['т', s @ ..] => super::t6::n5494(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t6::n1771(s, out),
		['т', s @ ..] => super::t6::n1772(s, out),
		['х', s @ ..] => {
			super::push(out, 1730);
			s.is_empty()
		}
		['ч', s @ ..] => super::t6::n1774(s, out),
		['ы', s @ ..] => super::t6::n1775(s, out),
		_ => false,
	}
}

/// Words ending -ыдишь.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 1733);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11398);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11399);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдишь.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядишь.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1735);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11401);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11402);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11403);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11404);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18320);
									s.is_empty()
								}
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
			super::push(out, 1736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5268);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5269);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11405);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18321);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24512);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11406);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18322);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24513);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5270);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5271);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11409);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 24514);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29156);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 24515);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 29157);
													s.is_empty()
												}
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
					super::push(out, 5274);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5275);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5276);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бежишь.
pub(crate) fn n1795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1751);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5302);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5303);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5304);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5305);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5306);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5307);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5308);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вежишь.
pub(crate) fn n1796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1752);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 5309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11431);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -адежишь.
pub(crate) fn n5550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5310);
	s.is_empty()
}

/// Words ending -лежишь.
pub(crate) fn n1798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1754);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5311);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5312);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5313);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5314);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5315);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11436);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5316);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5317);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5318);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11440);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11441);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11442);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5319);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11445);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11446);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5321);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11447);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -межишь.
pub(crate) fn n1799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1755);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5322);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5323);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5324);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нежишь.
pub(crate) fn n1800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1756);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5326);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5327);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18332);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5328);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11449);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5329);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5331);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11451);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11452);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -режишь.
pub(crate) fn n1801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1757);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5332);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18336);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5333);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18338);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъежишь.
pub(crate) fn n1802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1758);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 5334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жжишь.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11454);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11455);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зжишь.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5335);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11457);
							match s {
								[] => true,
								['р', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24519);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24520);
											s.is_empty()
										}
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
					super::push(out, 5336);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18340);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18341);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24521);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18342);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24522);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 11459);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18343);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24523);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24524);
											s.is_empty()
										}
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
			super::push(out, 1761);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11460);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11461);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1762);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5337);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11463);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11464);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11465);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11466);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18344);
									s.is_empty()
								}
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
			super::push(out, 1763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5338);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5339);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18345);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18346);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18347);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18348);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24525);
											s.is_empty()
										}
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

/// Words ending -ижишь.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1764);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11467);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1765);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 1766);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лжишь.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1767);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11469);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11470);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18349);
									s.is_empty()
								}
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

/// Words ending -мжишь.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 321);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5341);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжишь.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1769);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					super::push(out, 5342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1770);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5343);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожишь.
pub(crate) fn n324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1771);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5344);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1772);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5346);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18350);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18351);
									s.is_empty()
								}
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
			super::push(out, 1773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5347);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5348);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5349);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5350);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11471);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 18352);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24526);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 11472);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18353);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24527);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 24528);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 29158);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5351);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1774);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5353);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18355);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11474);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11475);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11477);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11478);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5356);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5357);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18357);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18358);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 18359);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18360);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18361);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24529);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24530);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24531);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 11482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18362);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11483);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24532);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24533);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29159);
													s.is_empty()
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 24534);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29160);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 24535);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 24536);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29161);
													s.is_empty()
												}
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
					super::push(out, 5358);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11484);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18364);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 18365);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18366);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18367);
									s.is_empty()
								}
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
			super::push(out, 1777);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5359);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t6::n12132(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t6::n5605(s, out),
				['ы', s @ ..] => super::t6::n5606(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржишь.
pub(crate) fn n325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 324);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11486);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11487);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11488);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11489);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11490);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18371);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18372);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 18373);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 18374);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11491);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11492);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11493);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18376);
									s.is_empty()
								}
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
			super::push(out, 1779);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5363);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужишь.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 325);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5364);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18377);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18378);
									s.is_empty()
								}
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
			super::push(out, 1781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5365);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11496);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11497);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11498);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11499);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18379);
									s.is_empty()
								}
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
			super::push(out, 1782);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5367);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11501);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11502);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 11503);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11504);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11505);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11506);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11507);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11509);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11510);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 11511);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11512);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18383);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11513);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18384);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24537);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 18385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11514);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 11515);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5370);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11516);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18388);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 18389);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 24538);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5371);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5372);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11518);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18390);
									s.is_empty()
								}
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
			super::push(out, 1783);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5373);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5374);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11519);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 11520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5375);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5376);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжишь.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5378);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5379);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5380);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5382);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5383);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11524);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11525);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 18392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 11526);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24539);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 24540);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 24541);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5384);
					match s {
						[] => true,
						['м', 'о', s @ ..] => {
							super::push(out, 11528);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5386);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5387);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южишь.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1787);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1788);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5388);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5389);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5390);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5391);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1789);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11532);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11533);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11534);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11535);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18394);
									s.is_empty()
								}
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
			super::push(out, 1790);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11536);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжишь.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11539);
							match s {
								[] => true,
								['б', 'о', s @ ..] => {
									super::push(out, 18396);
									s.is_empty()
								}
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
			super::push(out, 1792);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1793);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11540);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 1794);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1795);
			match s {
				[] => true,
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11542);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бразишь.
pub(crate) fn n5665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5415);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18406);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18407);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18408);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24543);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18409);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24545);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29163);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31912);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24546);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18410);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24548);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 24549);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29164);
									match s {
										[] => true,
										['л', 'б', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 33032);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33033);
													s.is_empty()
												}
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
							super::push(out, 24550);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29165);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31914);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 29166);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											super::push(out, 31915);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 29167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18411);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зразишь.
pub(crate) fn n5666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5416);
	s.is_empty()
}

/// Words ending -оразишь.
pub(crate) fn n5667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5417);
	s.is_empty()
}

/// Words ending -сразишь.
pub(crate) fn n5668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5418);
	s.is_empty()
}

/// Words ending -тразишь.
pub(crate) fn n5669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5419);
	s.is_empty()
}

/// Words ending -ыразишь.
pub(crate) fn n5670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5420);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уазишь.
pub(crate) fn n1846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1801);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 5421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бзишь.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1802);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1803);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5423);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езишь.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5424);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18412);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18413);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18414);
									s.is_empty()
								}
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
			super::push(out, 1805);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5425);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 11559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11560);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11561);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18415);
									s.is_empty()
								}
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
			super::push(out, 1806);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5427);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11562);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изишь.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1807);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1808);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11564);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11565);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5430);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1809);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11568);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 11569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1811);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йзишь.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1812);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5433);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мзишь.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 1813);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11570);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзишь.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озишь.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1815);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5434);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18417);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18418);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24553);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 18419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5435);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5436);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5437);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5438);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5439);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11574);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5440);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 5441);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18420);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18421);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5442);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							super::push(out, 11577);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 11578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5443);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5444);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5445);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1816);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5446);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11581);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11582);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1817);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5448);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1818);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11585);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 11586);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11587);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 11588);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 5450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1819);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5451);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18424);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5452);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18426);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5453);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1820);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1821);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11590);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11591);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11592);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5456);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 11594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18428);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18429);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 18430);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 24554);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29168);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 18431);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18432);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24555);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 24556);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18433);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18434);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18435);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24557);
											s.is_empty()
										}
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
			super::push(out, 1822);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5457);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 11595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1823);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5458);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t6::n5712(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рзишь.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1824);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5460);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11598);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 18437);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24558);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 11599);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11601);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11602);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11603);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11605);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узишь.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1825);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1826);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5463);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11607);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11608);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5465);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5466);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5467);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5468);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18438);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 18439);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18440);
									s.is_empty()
								}
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
			super::push(out, 1829);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5470);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11611);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11612);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11613);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11614);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 11615);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5471);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5472);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5473);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5474);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5475);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11618);
							match s {
								[] => true,
								['в', 'т', s @ ..] => {
									super::push(out, 18443);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 5476);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5477);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5478);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5479);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11623);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 11624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18444);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18445);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 24560);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 18446);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24561);
											s.is_empty()
										}
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
			super::push(out, 1833);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5480);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18447);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 18448);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24562);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18449);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18450);
									s.is_empty()
								}
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
			super::push(out, 1834);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5481);
					match s {
						[] => true,
						['а', 'р', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18451);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18452);
									s.is_empty()
								}
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

/// Words ending -ьзишь.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['о', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11625);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11626);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзишь.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5484);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18453);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 18454);
									s.is_empty()
								}
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

/// Words ending -язишь.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5485);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5486);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5487);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ибалишь.
pub(crate) fn n5743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5489);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11632);
			match s {
				[] => true,
				['н', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24563);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбалишь.
pub(crate) fn n5744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5490);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11633);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18458);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -валишь.
pub(crate) fn n1886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1841);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5491);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5492);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11634);
					s.is_empty()
				}
				['н', 'р', s @ ..] => {
					super::push(out, 11635);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5493);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5494);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5495);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11636);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11637);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5496);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11638);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11639);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5498);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11640);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11641);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11642);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11643);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11644);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11645);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5500);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5501);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 5502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11646);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					super::push(out, 11647);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11648);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11649);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11650);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5503);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11651);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -галишь.
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1842);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5504);
			match s {
				[] => true,
				['а', 'п', 'ш', 'о', s @ ..] => {
					super::push(out, 11652);
					s.is_empty()
				}
				['у', 'ф', 'а', s @ ..] => {
					super::push(out, 11653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -далишь.
pub(crate) fn n1888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1843);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 5505);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18468);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 18469);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29169);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29170);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29171);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24568);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24569);
									s.is_empty()
								}
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
			super::push(out, 5506);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жалишь.
pub(crate) fn n1889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1844);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5508);
			s.is_empty()
		}
		['у', 'р', 'к', '\u{1}', s @ ..] => {
			super::push(out, 5509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -калишь.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1845);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5510);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5511);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11655);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18471);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11656);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18472);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18473);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18474);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24572);
									s.is_empty()
								}
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
			super::push(out, 5512);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5513);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5514);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11660);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 11661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5515);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5516);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11662);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 18475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11663);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11664);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11665);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 18477);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11666);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18478);
							match s {
								[] => true,
								['у', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29172);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29173);
											s.is_empty()
										}
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
					super::push(out, 11667);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5518);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5519);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11669);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -малишь.
pub(crate) fn n1891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1846);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5520);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11671);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 18480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5521);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5522);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 5523);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11672);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24574);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24575);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24576);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24577);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29174);
											s.is_empty()
										}
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
		['ш', s @ ..] => {
			super::push(out, 5524);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -налишь.
pub(crate) fn n1892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1847);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11674);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11675);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5526);
			match s {
				[] => true,
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18482);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5527);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -палишь.
pub(crate) fn n1893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1848);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5528);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5529);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11676);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5530);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5531);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5532);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5533);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11678);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11679);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18484);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 18485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5534);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11680);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5535);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5536);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ралишь.
pub(crate) fn n1894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1849);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5537);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11683);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5538);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11684);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11685);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5539);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5540);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11687);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11688);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11689);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11690);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -салишь.
pub(crate) fn n1895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1850);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5541);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5542);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5543);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5544);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5545);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5546);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11692);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -талишь.
pub(crate) fn n1896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1851);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5547);
			match s {
				[] => true,
				['п', 'а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 11693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5548);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11694);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чалишь.
pub(crate) fn n1897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1852);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5549);
			s.is_empty()
		}
		['а', 'н', s @ ..] => {
			super::push(out, 5550);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5551);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18491);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5552);
			s.is_empty()
		}
		['о', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11696);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5553);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5554);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5555);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11698);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шалишь.
pub(crate) fn n1898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1853);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5556);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5557);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5558);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -блишь.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5562);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18493);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 18494);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18495);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24578);
											s.is_empty()
										}
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

/// Words ending -овлишь.
pub(crate) fn n1902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1857);
	s.is_empty()
}

/// Words ending -глишь.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1858);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1859);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5564);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5565);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5566);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18496);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 18497);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18498);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 18499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5568);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5569);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5570);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 11704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18500);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18501);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18502);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24579);
											s.is_empty()
										}
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

/// Words ending -длишь.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1861);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1862);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11705);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11706);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11707);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11708);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18503);
									s.is_empty()
								}
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
			super::push(out, 1863);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 5572);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5573);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5574);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елишь.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 347);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5576);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5577);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5578);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5579);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5580);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5581);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5582);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11712);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 11713);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 11714);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5583);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5584);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5585);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5586);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5587);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18505);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18506);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 18507);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5588);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5589);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5590);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5591);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5592);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18509);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24580);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 18510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5593);
					match s {
						[] => true,
						['а', 'р', 'д', s @ ..] => {
							super::push(out, 11721);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5594);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11722);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 11723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5595);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11725);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 11726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5597);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1868);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5598);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 11728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5599);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 11729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1869);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1870);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5601);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5602);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5603);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5604);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11732);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11733);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5606);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5607);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11734);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 11735);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 11736);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 5608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11737);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5609);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1871);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5610);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18514);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18515);
									s.is_empty()
								}
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
			super::push(out, 1872);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11741);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11742);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18516);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18517);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24581);
											s.is_empty()
										}
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
		['с', s @ ..] => super::t6::n1918(s, out),
		['т', s @ ..] => super::t6::n1919(s, out),
		['ц', s @ ..] => super::t6::n1920(s, out),
		['ч', s @ ..] => super::t6::n1921(s, out),
		['ш', s @ ..] => super::t6::n1922(s, out),
		['щ', s @ ..] => {
			super::push(out, 1878);
			match s {
				[] => true,
				['с', s @ ..] => super::t6::n5890(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -злишь.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 348);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1879);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1880);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11756);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11757);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18526);
									s.is_empty()
								}
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

/// Words ending -илишь.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1883);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5639);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 11758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5640);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5641);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1885);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5644);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 11761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5646);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5647);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5648);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5649);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5650);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5651);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11762);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18527);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18528);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24587);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29175);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 29176);
													s.is_empty()
												}
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
					super::push(out, 5652);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5653);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5654);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 5655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11763);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11764);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5656);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5657);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5658);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5659);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5660);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18529);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5663);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йлишь.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1891);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11767);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11768);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5665);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 5666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1892);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5667);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5668);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 11770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5669);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клишь.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1893);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11772);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11773);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5671);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 11775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 5672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11776);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11777);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18532);
									s.is_empty()
								}
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

/// Words ending -ллишь.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 5673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5674);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5675);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 11778);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18533);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18534);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18535);
									s.is_empty()
								}
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
			super::push(out, 1898);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11779);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млишь.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11781);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олишь.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5679);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5680);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11783);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 11784);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18536);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											super::push(out, 24588);
											s.is_empty()
										}
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
					super::push(out, 5681);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5682);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5683);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5684);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11788);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11789);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
									super::push(out, 18537);
									s.is_empty()
								}
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
			super::push(out, 1902);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5685);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5686);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5687);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 11791);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18539);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11792);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11793);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5688);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11794);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18542);
									match s {
										[] => true,
										['л', 'б', 'о', s @ ..] => {
											super::push(out, 24589);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 29177);
													s.is_empty()
												}
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
							super::push(out, 11795);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 11796);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11797);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5689);
					match s {
						[] => true,
						['и', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18545);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5691);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5692);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11798);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 11799);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 18547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5694);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5695);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5696);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18548);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18549);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5697);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11801);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 18550);
									s.is_empty()
								}
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
			super::push(out, 1905);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5698);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5699);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5700);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5701);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5702);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 11803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5703);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5704);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5705);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 11806);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 5706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11808);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11809);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11810);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11811);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t6::n1952(s, out),
		['р', s @ ..] => {
			super::push(out, 1908);
			s.is_empty()
		}
		['с', s @ ..] => super::t6::n1954(s, out),
		['т', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 5724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t6::n5980(s, out),
				['и', s @ ..] => super::t6::n5981(s, out),
				['с', s @ ..] => super::t6::n5982(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t6::n1957(s, out),
		_ => false,
	}
}

/// Words ending -плишь.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1914);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5732);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11833);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11834);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11835);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11836);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5734);
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 11838);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							super::push(out, 11839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлишь.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5735);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18560);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18561);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24593);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29178);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18562);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18563);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24594);
											s.is_empty()
										}
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
					super::push(out, 5736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1918);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11841);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слишь.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1919);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11843);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11844);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11845);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18564);
									s.is_empty()
								}
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
			super::push(out, 1920);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1921);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11846);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11847);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 11848);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 11849);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11850);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 5740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11851);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11852);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 11853);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11854);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 11855);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11856);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18569);
									s.is_empty()
								}
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
			super::push(out, 1922);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5741);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1923);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11858);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11859);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11860);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18570);
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
						['\u{1}', s @ ..] => {
							super::push(out, 11861);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11863);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11864);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11865);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 18571);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 18572);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 24595);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 29179);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 24596);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29180);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 18573);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 18574);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11866);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18576);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18577);
									s.is_empty()
								}
								['с', 'е', 'б', '\u{1}', s @ ..] => {
									super::push(out, 18578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 11867);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11868);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18579);
									s.is_empty()
								}
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

/// Words ending -тлишь.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1925);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1926);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 5743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1927);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5744);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18580);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18581);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18582);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24597);
											s.is_empty()
										}
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
					super::push(out, 5745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1928);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1929);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5746);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улишь.
pub(crate) fn n360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 359);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1930);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5748);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18583);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18584);
									s.is_empty()
								}
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
			super::push(out, 1931);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 1932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5749);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5750);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5751);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1933);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5753);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 11870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11871);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11872);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1934);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1935);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5756);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5758);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5759);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5760);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5762);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5763);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5764);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5765);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1938);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5766);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5767);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1939);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5769);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5770);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5771);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11884);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18585);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18586);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5772);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5773);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5774);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 1941);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5776);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 11886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флишь.
pub(crate) fn n361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1942);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5777);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11887);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11888);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18588);
									s.is_empty()
								}
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
			super::push(out, 1943);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11889);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11890);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18589);
									s.is_empty()
								}
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

/// Words ending -хлишь.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1945);
			match s {
				[] => true,
				['р', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11891);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1946);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 5780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 5781);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11893);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11894);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 18590);
									s.is_empty()
								}
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
			super::push(out, 1948);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1949);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11895);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлишь.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1950);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1951);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылишь.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 1952);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5785);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5786);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5787);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5788);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5789);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5790);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5791);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5792);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5793);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11898);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 18591);
									s.is_empty()
								}
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
			super::push(out, 1954);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11899);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11900);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18592);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 18593);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18594);
									s.is_empty()
								}
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
			super::push(out, 1955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5794);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5796);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5797);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юлишь.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1956);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1957);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1958);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1959);
			s.is_empty()
		}
		['ъ', s @ ..] => {
			super::push(out, 1960);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ялишь.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1961);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5798);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5799);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5800);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5801);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1962);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5802);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5803);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5804);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5805);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рорекламишь.
pub(crate) fn n30955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29181);
	s.is_empty()
}

/// Words ending -продинамишь.
pub(crate) fn n30956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29182);
	s.is_empty()
}

/// Words ending -каймишь.
pub(crate) fn n6092(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11920);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11921);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еймишь.
pub(crate) fn n2023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1978);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5829);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18601);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18602);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18603);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18604);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24602);
									s.is_empty()
								}
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
			super::push(out, 5830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лмишь.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1979);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5831);
					s.is_empty()
				}
				['х', 'с', s @ ..] => {
					super::push(out, 5832);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омишь.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5833);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5834);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5835);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11925);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18605);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18606);
									s.is_empty()
								}
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
			super::push(out, 1981);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5836);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 11926);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18607);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18608);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24603);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11927);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18610);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24604);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 24605);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24606);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29183);
													s.is_empty()
												}
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
					super::push(out, 5837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5838);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5839);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5840);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5841);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5842);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5843);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11928);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 11929);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5844);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5845);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18612);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18613);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 24607);
											s.is_empty()
										}
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
					super::push(out, 5846);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5847);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5848);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5849);
					match s {
						[] => true,
						['к', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18614);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18615);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18616);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24608);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18617);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11931);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 18619);
									s.is_empty()
								}
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
			super::push(out, 1984);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11932);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11933);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5851);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 11935);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18620);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24609);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24610);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5852);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5853);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							super::push(out, 11937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5854);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5855);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5856);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5857);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмишь.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1986);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 5858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1987);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11939);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11940);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11941);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11942);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11943);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5860);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11944);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 18622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 11945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18623);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18624);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5861);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1988);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11947);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11948);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 11949);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18627);
									s.is_empty()
								}
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

/// Words ending -тмишь.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1989);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5863);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умишь.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5864);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5865);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 18628);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24611);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 18629);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18630);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 24612);
											s.is_empty()
										}
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
					super::push(out, 5866);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1993);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5870);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5871);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хмишь.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1995);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11951);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ымишь.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5873);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5874);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5876);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18632);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18633);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24613);
											s.is_empty()
										}
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

/// Words ending -ьмишь.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 1998);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11953);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11954);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 11955);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11956);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11957);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18634);
									s.is_empty()
								}
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
			super::push(out, 2000);
			match s {
				[] => true,
				['е', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11958);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмишь.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2001);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5877);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5878);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямишь.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11961);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11962);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11963);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18635);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24614);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 11964);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18636);
									s.is_empty()
								}
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
			super::push(out, 2003);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ванишь.
pub(crate) fn n2050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2005);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5885);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5886);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5887);
			s.is_empty()
		}
		['у', 'д', 'о', s @ ..] => {
			super::push(out, 5888);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ганишь.
pub(crate) fn n2051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2006);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5889);
			match s {
				[] => true,
				['л', 'а', 'б', 'о', s @ ..] => {
					super::push(out, 11971);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5890);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5891);
			match s {
				[] => true,
				['л', 'у', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18640);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5892);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 11974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5893);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18642);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5894);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 11976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18645);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18646);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18647);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24619);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 24620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18648);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24621);
									s.is_empty()
								}
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

/// Words ending -занишь.
pub(crate) fn n2053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2008);
	match s {
		[] => true,
		['и', 'т', 'р', s @ ..] => {
			match s {
				[] => true,
				['а', 'п', 'а', s @ ..] => {
					super::push(out, 11977);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ианишь.
pub(crate) fn n2054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2009);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -канишь.
pub(crate) fn n2055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2010);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5898);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18650);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18651);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18652);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24622);
									s.is_empty()
								}
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
			super::push(out, 5899);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 11979);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5900);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					super::push(out, 11980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11981);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5902);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11983);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ланишь.
pub(crate) fn n2056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2011);
	match s {
		[] => true,
		['б', 'е', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11984);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5903);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5904);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5905);
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18655);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -манишь.
pub(crate) fn n2057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2012);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5906);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5907);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11988);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11989);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11990);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 11991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18658);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18659);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5908);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5909);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5910);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5911);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5912);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11995);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11996);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5913);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11998);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 11999);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24623);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24624);
									s.is_empty()
								}
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
			super::push(out, 5914);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5915);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12001);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18661);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18662);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -панишь.
pub(crate) fn n2058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2013);
	match s {
		[] => true,
		['о', 'л', 'р', 'о', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12004);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранишь.
pub(crate) fn n2059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5918);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5919);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12006);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12007);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18666);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18667);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18668);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24625);
									s.is_empty()
								}
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
			super::push(out, 5920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12009);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12010);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12011);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12012);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12013);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5921);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12014);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12015);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12016);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5922);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12017);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12018);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5923);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5924);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5925);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18673);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18674);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5926);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5927);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12024);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12025);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санишь.
pub(crate) fn n2060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2015);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -танишь.
pub(crate) fn n2061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2016);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5930);
			match s {
				[] => true,
				['л', 'р', 'а', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18676);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18677);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5931);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фанишь.
pub(crate) fn n2062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2017);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5933);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5934);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щанишь.
pub(crate) fn n2063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2018);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бнишь.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2019);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12027);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12028);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12029);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -внишь.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5936);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 12031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2021);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12032);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12033);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5938);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12034);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12035);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18680);
									s.is_empty()
								}
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

/// Words ending -гнишь.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2023);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12036);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -днишь.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12038);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5940);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5941);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', '\u{1}', s @ ..] => {
					super::push(out, 5942);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'а', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5944);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5946);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12042);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12043);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енишь.
pub(crate) fn n386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12045);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12046);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12047);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12048);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 12049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18681);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18682);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 12050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5950);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18684);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18685);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 12052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5951);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12054);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12055);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12056);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5953);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18687);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5954);
					match s {
						[] => true,
						['е', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18689);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18690);
									s.is_empty()
								}
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
			super::push(out, 2031);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5955);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18691);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18692);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 18693);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18694);
									s.is_empty()
								}
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
			super::push(out, 2032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5956);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5957);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5958);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 12058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2034);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5961);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12059);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18696);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18697);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 18698);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18699);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18700);
									s.is_empty()
								}
								['о', 'п', s @ ..] => {
									super::push(out, 18701);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18702);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24626);
											s.is_empty()
										}
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
					super::push(out, 5962);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5963);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12060);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12061);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12062);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12063);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12064);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12065);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t6::n2082(s, out),
		['п', s @ ..] => {
			super::push(out, 2037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5974);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5975);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5976);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12076);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12077);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 12078);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18716);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5977);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5978);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2084(s, out),
		['с', s @ ..] => {
			super::push(out, 2039);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5982);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t6::n2086(s, out),
		['ф', s @ ..] => super::t6::n2087(s, out),
		['ц', s @ ..] => super::t6::n2088(s, out),
		['ч', s @ ..] => super::t6::n2089(s, out),
		['щ', s @ ..] => super::t6::n2090(s, out),
		_ => false,
	}
}

/// Words ending -жнишь.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2045);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5998);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5999);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12094);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24633);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 24634);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29186);
													s.is_empty()
												}
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

/// Words ending -знишь.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12095);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6001);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12098);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12099);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18723);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18724);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24635);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 18725);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18726);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18727);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24636);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 24637);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29187);
													s.is_empty()
												}
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
			super::push(out, 2048);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6003);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 6004);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2049);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 6005);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2050);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12103);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6007);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18728);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18729);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18730);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24638);
											s.is_empty()
										}
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

/// Words ending -инишь.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6008);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6009);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6010);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6011);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6012);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12108);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 12109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2053);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12110);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2054);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6014);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 18731);
									s.is_empty()
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24639);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24640);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 24641);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24642);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 24643);
											s.is_empty()
										}
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
		['з', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6015);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18732);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18733);
									s.is_empty()
								}
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
			super::push(out, 2056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6016);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6017);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12114);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12115);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 12116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12117);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12118);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12119);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12120);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6019);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6020);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6021);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18740);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18741);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18742);
									s.is_empty()
								}
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
			super::push(out, 2059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6022);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6023);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6024);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12123);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18743);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 24644);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 18744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6025);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6026);
					match s {
						[] => true,
						['щ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18745);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6027);
					match s {
						[] => true,
						['а', 'р', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18747);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2061);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2062);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6030);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t6::n2109(s, out),
		['ш', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6042);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6043);
					match s {
						[] => true,
						['е', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t6::n19814(s, out),
								['а', s @ ..] => super::t6::n19815(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => super::t6::n2111(s, out),
		_ => false,
	}
}

/// Words ending -йнишь.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2066);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6045);
					match s {
						[] => true,
						['и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18757);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 18758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12138);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6048);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнишь.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6049);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12140);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18759);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 18760);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24648);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29190);
													s.is_empty()
												}
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
							super::push(out, 12141);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12142);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12143);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18761);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12144);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18763);
									s.is_empty()
								}
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

/// Words ending -мнишь.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2070);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2071);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2072);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2073);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6051);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6052);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12147);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12148);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12149);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12150);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ннишь.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онишь.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2075);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6054);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12152);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									super::push(out, 18764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6055);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2076);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12154);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12155);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12156);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12157);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24649);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29191);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 24650);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29192);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 29193);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 29194);
													s.is_empty()
												}
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
							super::push(out, 12158);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12159);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18765);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18766);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24651);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12160);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12161);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12162);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18767);
									s.is_empty()
								}
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
			super::push(out, 2077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6057);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6058);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6059);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6060);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2078);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6062);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6063);
					match s {
						[] => true,
						['о', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18768);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6064);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18770);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18771);
									s.is_empty()
								}
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
			super::push(out, 2079);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6065);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2080);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6066);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18772);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6067);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18774);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18775);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18776);
									s.is_empty()
								}
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
			super::push(out, 2082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6068);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6069);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6070);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 12168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2083);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6071);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 12169);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18777);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24652);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t6::n6347(s, out),
				['о', s @ ..] => {
					super::push(out, 6073);
					match s {
						[] => true,
						['к', s @ ..] => super::t6::n12885(s, out),
						['п', s @ ..] => super::t6::n12886(s, out),
						['с', s @ ..] => super::t6::n12887(s, out),
						['х', s @ ..] => {
							super::push(out, 12182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t6::n6349(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t6::n2130(s, out),
		['н', s @ ..] => super::t6::n2131(s, out),
		['п', s @ ..] => super::t6::n2132(s, out),
		['р', s @ ..] => super::t6::n2133(s, out),
		['с', s @ ..] => super::t6::n2134(s, out),
		['т', s @ ..] => super::t6::n2135(s, out),
		['ф', s @ ..] => super::t6::n2136(s, out),
		['ь', s @ ..] => super::t6::n2137(s, out),
		_ => false,
	}
}

/// Words ending -пнишь.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2092);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12208);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнишь.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2094);
			match s {
				[] => true,
				['в', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12210);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', '\u{1}', s @ ..] => {
					super::push(out, 6102);
					s.is_empty()
				}
				['з', '\u{1}', s @ ..] => {
					super::push(out, 6103);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6104);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12212);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 18811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12213);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12214);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12215);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12216);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18812);
									s.is_empty()
								}
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

/// Words ending -снишь.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2095);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6106);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2096);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6107);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12218);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18813);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12219);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6108);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12220);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12221);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12222);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12223);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12224);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18816);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24658);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29199);
													s.is_empty()
												}
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
			super::push(out, 2097);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6110);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 12225);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18817);
									s.is_empty()
								}
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
			super::push(out, 2098);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2099);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6111);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 6112);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6113);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнишь.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2100);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12227);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2101);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2102);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унишь.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2103);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2104);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2105);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2106);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6118);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18818);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18819);
									s.is_empty()
								}
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
			super::push(out, 2107);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12229);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12230);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12231);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18821);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 18822);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18823);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24659);
											s.is_empty()
										}
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
			super::push(out, 2108);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6119);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6120);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнишь.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шнишь.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2110);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2111);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6122);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6123);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12235);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 18824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12236);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18825);
									s.is_empty()
								}
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
			super::push(out, 2112);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынишь.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2113);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12237);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юнишь.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2115);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2116);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6126);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6127);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2117);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6128);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янишь.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2118);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2119);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6129);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18826);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18827);
									s.is_empty()
								}
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
			super::push(out, 2120);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6130);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							super::push(out, 12241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2121);
			match s {
				[] => true,
				['б', 'у', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12242);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2122);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6132);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18828);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18829);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18830);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18831);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18832);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24660);
											s.is_empty()
										}
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
			super::push(out, 2123);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6133);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18833);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18834);
									s.is_empty()
								}
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
			super::push(out, 2124);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6134);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2125);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12245);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12246);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2126);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6136);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6137);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 18835);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12249);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6139);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12251);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12252);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 18838);
									s.is_empty()
								}
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

/// Words ending -выкипишь.
pub(crate) fn n13015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12303);
	s.is_empty()
}

/// Words ending -рипишь.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2164);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 6219);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18848);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18849);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12304);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сипишь.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2165);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6220);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шипишь.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2166);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6222);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6223);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щипишь.
pub(crate) fn n2213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2167);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6225);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йпишь.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2168);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6226);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6227);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 6228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12307);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12308);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12309);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18852);
									s.is_empty()
								}
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
			super::push(out, 2169);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 6229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лпишь.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мпишь.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2171);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12310);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12311);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18853);
									s.is_empty()
								}
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

/// Words ending -опишь.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2172);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6230);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6231);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2173);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6233);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6234);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6235);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6236);
					match s {
						[] => true,
						['т', 'о', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 12312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6238);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6239);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6240);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12313);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2177);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6243);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18854);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 18855);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12316);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12317);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12318);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6245);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12320);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18857);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6246);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12321);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6247);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6248);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6249);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12323);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 12324);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12325);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 18860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24663);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24664);
											s.is_empty()
										}
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
					super::push(out, 6250);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2179);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6252);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6253);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6254);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6255);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6256);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6257);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6258);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2180);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6259);
					match s {
						[] => true,
						['т', 'о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18861);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 18862);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18863);
									s.is_empty()
								}
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

/// Words ending -ппишь.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпишь.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2182);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2183);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12328);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12329);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12330);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12331);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12332);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12333);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18864);
									s.is_empty()
								}
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
			super::push(out, 2184);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12334);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12335);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12336);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12337);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18865);
									s.is_empty()
								}
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

/// Words ending -спишь.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2185);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2186);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2187);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2188);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упишь.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 2189);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2190);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6265);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6266);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12339);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6267);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6268);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6269);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6270);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6271);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 6272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12342);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12343);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6273);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6274);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6275);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12345);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 18867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6276);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12347);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 18868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6277);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6278);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6279);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2192);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6280);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2193);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6282);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 12349);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18869);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18870);
									s.is_empty()
								}
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
			super::push(out, 2194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6283);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6284);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6285);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6286);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6287);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6288);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12351);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6289);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12353);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12354);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18872);
									s.is_empty()
								}
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

/// Words ending -ыпишь.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2196);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2197);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япишь.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2198);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12355);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12356);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18873);
									s.is_empty()
								}
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

/// Words ending -едваришь.
pub(crate) fn n13073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12359);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одваришь.
pub(crate) fn n13074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12360);
	s.is_empty()
}

/// Words ending -еваришь.
pub(crate) fn n6586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6297);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12361);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 12362);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зваришь.
pub(crate) fn n6587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6298);
	s.is_empty()
}

/// Words ending -кваришь.
pub(crate) fn n6588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6299);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 12363);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18877);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18878);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оваришь.
pub(crate) fn n6589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6300);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12364);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 12365);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18879);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12366);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12367);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18880);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сваришь.
pub(crate) fn n6590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6301);
	s.is_empty()
}

/// Words ending -тваришь.
pub(crate) fn n6591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6302);
	s.is_empty()
}

/// Words ending -ываришь.
pub(crate) fn n6592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6303);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12369);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гаришь.
pub(crate) fn n2247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2201);
	match s {
		[] => true,
		['е', 'ч', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12370);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12372);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -даришь.
pub(crate) fn n2248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2202);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6304);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6305);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12374);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6306);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6307);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6308);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6309);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6310);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12379);
					match s {
						[] => true,
						['и', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24667);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24668);
									s.is_empty()
								}
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
			super::push(out, 6311);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12380);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6312);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12381);
					match s {
						[] => true,
						['а', 'л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24669);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24670);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 24671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6313);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6314);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12384);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12385);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жаришь.
pub(crate) fn n2249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2203);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6315);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6316);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6317);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заришь.
pub(crate) fn n2250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2204);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6318);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6319);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18884);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иаришь.
pub(crate) fn n2251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2205);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 6321);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каришь.
pub(crate) fn n2252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2206);
	s.is_empty()
}

/// Words ending -маришь.
pub(crate) fn n2253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2207);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6322);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18886);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6323);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6324);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6325);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 6326);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наришь.
pub(crate) fn n2254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2208);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6327);
			match s {
				[] => true,
				['л', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18889);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -паришь.
pub(crate) fn n2255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2209);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6329);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6330);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6331);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6332);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6333);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6334);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6335);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12393);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 12394);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12395);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6336);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6337);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 6338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12396);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12397);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12398);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12399);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6339);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -саришь.
pub(crate) fn n2256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2210);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6340);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12401);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6341);
			match s {
				[] => true,
				['и', 'м', 'о', 'к', 'о', s @ ..] => {
					super::push(out, 12403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6342);
			match s {
				[] => true,
				['г', 'о', s @ ..] => {
					super::push(out, 12404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таришь.
pub(crate) fn n2257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2211);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6343);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18893);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 18894);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24672);
									s.is_empty()
								}
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
			super::push(out, 6344);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6345);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6346);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12407);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6347);
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					super::push(out, 12409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 6348);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6349);
			match s {
				[] => true,
				['м', '\u{1}', s @ ..] => {
					super::push(out, 12411);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уаришь.
pub(crate) fn n2258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2212);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 6350);
			match s {
				[] => true,
				['а', 'с', 'к', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18895);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фаришь.
pub(crate) fn n2259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2213);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 6351);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12412);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -харишь.
pub(crate) fn n2260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2214);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6352);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6353);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12413);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18898);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18899);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24673);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 24674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24675);
									s.is_empty()
								}
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
			super::push(out, 6354);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6355);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12417);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 12418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -царишь.
pub(crate) fn n2261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2215);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6356);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чаришь.
pub(crate) fn n2262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2216);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шаришь.
pub(crate) fn n2263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2217);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6358);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6359);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6360);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6361);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6362);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бришь.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6363);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 12421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 6364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6365);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12423);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12424);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2219);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6366);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18904);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18905);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18906);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24676);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 12426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2220);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12427);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2221);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6368);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12429);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18907);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24677);
											s.is_empty()
										}
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
			super::push(out, 2222);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12430);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12431);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12432);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12433);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12434);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12435);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12436);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', 'т', '\u{1}', s @ ..] => {
			super::push(out, 2223);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2224);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вришь.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2225);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12437);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гришь.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2226);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6371);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6372);
					s.is_empty()
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12439);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12440);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12441);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12442);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2227);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12444);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дришь.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2228);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12446);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2229);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6375);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6376);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6377);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2230);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2231);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6379);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18910);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18911);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18912);
									s.is_empty()
								}
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
			super::push(out, 2232);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12450);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2233);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12452);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12453);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12454);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12455);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12456);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12457);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еришь.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2234);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6383);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 12459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6385);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6386);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12460);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 12461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6387);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6388);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12462);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12463);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6389);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6391);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6392);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6393);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12465);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24678);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24679);
											s.is_empty()
										}
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
					super::push(out, 6394);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6395);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12466);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18915);
									s.is_empty()
								}
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
			super::push(out, 2237);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6396);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2238);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6397);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12468);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24680);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 24681);
											s.is_empty()
										}
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
					super::push(out, 6398);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2239);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6399);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 12470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6400);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12471);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18916);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 18917);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 6401);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 12473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2241);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6402);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 6404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12476);
							match s {
								[] => true,
								['п', 'с', '\u{1}', s @ ..] => {
									super::push(out, 18918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6405);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12477);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24683);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24684);
											s.is_empty()
										}
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
			super::push(out, 2243);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6406);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6407);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6408);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6409);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12478);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12479);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 12480);
							match s {
								[] => true,
								['и', 'л', 'о', s @ ..] => {
									super::push(out, 18920);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 12481);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 18921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6410);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6411);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6412);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12482);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18922);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24686);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24687);
											s.is_empty()
										}
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
					super::push(out, 6413);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12483);
							match s {
								[] => true,
								['а', s @ ..] => super::t6::n19991(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t6::n6708(s, out),
				['у', s @ ..] => super::t6::n6709(s, out),
				['ы', s @ ..] => super::t6::n6710(s, out),
				['ь', s @ ..] => super::t6::n6711(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t6::n2290(s, out),
		['п', s @ ..] => super::t6::n2291(s, out),
		['с', s @ ..] => super::t6::n2292(s, out),
		['т', s @ ..] => super::t6::n2293(s, out),
		['ф', s @ ..] => super::t6::n2294(s, out),
		['х', s @ ..] => super::t6::n2295(s, out),
		['ч', s @ ..] => super::t6::n2296(s, out),
		['ш', s @ ..] => super::t6::n2297(s, out),
		['щ', s @ ..] => super::t6::n2298(s, out),
		['ь', s @ ..] => super::t6::n2299(s, out),
		_ => false,
	}
}

/// Words ending -зришь.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2254);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2255);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2256);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6457);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 6458);
					match s {
						[] => true,
						['и', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18950);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18951);
									s.is_empty()
								}
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
			super::push(out, 2257);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6460);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 12513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2259);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6461);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6462);
					match s {
						[] => true,
						['о', 'п', 'а', s @ ..] => {
							super::push(out, 12514);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6463);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2260);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6465);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иришь.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6466);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6467);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6468);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2262);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6469);
					match s {
						[] => true,
						['а', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18953);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18954);
									s.is_empty()
								}
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
			super::push(out, 2263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6470);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6471);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6472);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12519);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 12520);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6474);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6475);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 12522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6476);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6477);
					match s {
						[] => true,
						['б', 'е', 'д', '\u{1}', s @ ..] => {
							super::push(out, 12523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кришь.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12524);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оришь.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 436);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2268);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 6479);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12527);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12528);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12529);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18957);
									s.is_empty()
								}
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
			super::push(out, 2269);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12530);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12531);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6482);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18959);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18960);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18961);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 24697);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29201);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18962);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18963);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12533);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18964);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24699);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24700);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24701);
											s.is_empty()
										}
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
					super::push(out, 6483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12534);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12535);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12537);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12538);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12539);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12540);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18965);
									match s {
										[] => true,
										['и', 'ж', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29202);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 29203);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 18966);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24702);
											match s {
												[] => true,
												['л', 'б', 'у', s @ ..] => {
													super::push(out, 29204);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 24703);
											match s {
												[] => true,
												['б', 'о', s @ ..] => {
													super::push(out, 29205);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 18967);
									match s {
										[] => true,
										['о', 'л', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29206);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 29207);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 24704);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 29208);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 18968);
									match s {
										[] => true,
										['и', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29209);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 29210);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18969);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 18970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12541);
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 18971);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24705);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12542);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t6::n2316(s, out),
		['д', s @ ..] => super::t6::n2317(s, out),
		['ж', s @ ..] => super::t6::n2318(s, out),
		['з', s @ ..] => super::t6::n2319(s, out),
		['к', s @ ..] => super::t6::n2320(s, out),
		['м', s @ ..] => super::t6::n2321(s, out),
		['н', s @ ..] => super::t6::n2322(s, out),
		['п', s @ ..] => super::t6::n2323(s, out),
		['с', s @ ..] => super::t6::n2324(s, out),
		['т', s @ ..] => super::t6::n2325(s, out),
		['ф', s @ ..] => super::t6::n2326(s, out),
		['х', s @ ..] => super::t6::n2327(s, out),
		['ш', s @ ..] => {
			super::push(out, 2282);
			match s {
				[] => true,
				['а', s @ ..] => super::t6::n6841(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

