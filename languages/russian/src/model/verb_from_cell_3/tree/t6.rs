use flexion::Path;

/// Words ending -товишь.
pub(crate) fn n1686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1643);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 5030);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18163);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18164);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 18165);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18166);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18167);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5031);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18169);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18170);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18171);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5032);
			match s {
				[] => true,
				['а', 'в', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18173);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5033);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11156);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поруководишь.
pub(crate) fn n33822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31902);
	s.is_empty()
}

/// Words ending -ороводишь.
pub(crate) fn n19281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18238);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 24474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29140);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 29141);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -проводишь.
pub(crate) fn n19282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18239);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24475);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24476);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24477);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24478);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товодишь.
pub(crate) fn n11931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11294);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18240);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 24479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховодишь.
pub(crate) fn n11932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11295);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18241);
			match s {
				[] => true,
				['е', 'в', 'а', s @ ..] => {
					super::push(out, 24480);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -есводишь.
pub(crate) fn n11933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11296);
	s.is_empty()
}

/// Words ending -тводишь.
pub(crate) fn n5416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5177);
	s.is_empty()
}

/// Words ending -уводишь.
pub(crate) fn n5417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5178);
	s.is_empty()
}

/// Words ending -ыводишь.
pub(crate) fn n5418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5179);
	match s {
		[] => true,
		['в', 'о', s @ ..] => {
			super::push(out, 11297);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18242);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -годишь.
pub(crate) fn n1748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1705);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5180);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5181);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5182);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 11298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кодишь.
pub(crate) fn n1749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1706);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5184);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5185);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 5186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11299);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11300);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лодишь.
pub(crate) fn n1750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1707);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5187);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18245);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18246);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18248);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18249);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18250);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18251);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18252);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18253);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24481);
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
							super::push(out, 18254);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18255);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24482);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 24483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18256);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18257);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24484);
									s.is_empty()
								}
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
			super::push(out, 5188);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11303);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11304);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18258);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18259);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11306);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -родишь.
pub(crate) fn n1751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1708);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5189);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11307);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11308);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18261);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24486);
									s.is_empty()
								}
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
			super::push(out, 5191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11309);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11310);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 18262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11311);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11312);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18264);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 18265);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24487);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31904);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 31905);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 31906);
													s.is_empty()
												}
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
							super::push(out, 18266);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18268);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18269);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 24488);
									match s {
										[] => true,
										['у', 'с', 'а', s @ ..] => {
											super::push(out, 29145);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 31907);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11314);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11315);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5192);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11316);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18271);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29146);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29147);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 29148);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31908);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 29149);
											s.is_empty()
										}
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
					super::push(out, 11317);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5193);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5194);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5195);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11320);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18273);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 11321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18274);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18275);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24490);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 24491);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29150);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31909);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 31910);
													s.is_empty()
												}
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
							super::push(out, 18276);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18277);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18278);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18279);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11322);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18280);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11323);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18281);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5196);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5197);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5198);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11326);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ходишь.
pub(crate) fn n1752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1709);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5199);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11327);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5200);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5201);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5202);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5203);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5204);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11331);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11332);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 18284);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24495);
									s.is_empty()
								}
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
			super::push(out, 5205);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11333);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11334);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11335);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24496);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 24497);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29151);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5206);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11336);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11337);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18286);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 18287);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24498);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11338);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18288);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24500);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24501);
									s.is_empty()
								}
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
			super::push(out, 5207);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5209);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 11340);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прудишь.
pub(crate) fn n5493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5254);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11381);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11382);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 11383);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11384);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11385);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11386);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11387);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трудишь.
pub(crate) fn n5494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11388);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11389);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11390);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -судишь.
pub(crate) fn n1771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1728);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5256);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5257);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5258);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5259);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5260);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11392);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24508);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29153);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24509);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 29154);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 31911);
													match s {
														[] => true,
														['л', 'б', 'а', 'з', '\u{1}', s @ ..] => super::t8::n35015(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 29155);
											s.is_empty()
										}
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
					super::push(out, 11393);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24510);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24511);
									s.is_empty()
								}
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
			super::push(out, 5261);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тудишь.
pub(crate) fn n1772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1729);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 5262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11395);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11396);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11397);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чудишь.
pub(crate) fn n1774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1731);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5263);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5264);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыудишь.
pub(crate) fn n1775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1732);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5266);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word стожишь.
pub(crate) fn n12132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11485);
	s.is_empty()
}

/// Words ending -чтожишь.
pub(crate) fn n5605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5361);
	match s {
		[] => true,
		['и', 'н', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18368);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18369);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 18370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытожишь.
pub(crate) fn n5606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5362);
	s.is_empty()
}

/// Words ending -лхозишь.
pub(crate) fn n5712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5459);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11597);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 18436);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -селишь.
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1873);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5613);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5614);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5615);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5616);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5617);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18518);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5618);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11745);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5619);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11746);
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 18521);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24582);
									s.is_empty()
								}
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
			super::push(out, 5620);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11747);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24583);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24584);
									s.is_empty()
								}
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
			super::push(out, 5621);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5622);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5623);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11748);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телишь.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1874);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5624);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5625);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18523);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18524);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5626);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11749);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24585);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24586);
									s.is_empty()
								}
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
			super::push(out, 5627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -целишь.
pub(crate) fn n1920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1875);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5628);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5629);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5630);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5631);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5632);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11753);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -челишь.
pub(crate) fn n1921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1876);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 5633);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шелишь.
pub(crate) fn n1922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1877);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5634);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11755);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сщелишь.
pub(crate) fn n5890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5635);
	s.is_empty()
}

/// Words ending -молишь.
pub(crate) fn n1952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1907);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5707);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5708);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5709);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5710);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5711);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5712);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11815);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 18554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11816);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11817);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11818);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11819);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18555);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11820);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5714);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5715);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5716);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -солишь.
pub(crate) fn n1954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1909);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5717);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5718);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5719);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5720);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5721);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11826);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5722);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 11829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5723);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11830);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word фолишь.
pub(crate) fn n5980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5725);
	s.is_empty()
}

/// Words ending -ифолишь.
pub(crate) fn n5981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5726);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11831);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24591);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сфолишь.
pub(crate) fn n5982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5727);
	s.is_empty()
}

/// Words ending -холишь.
pub(crate) fn n1957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1912);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5728);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5729);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5730);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -менишь.
pub(crate) fn n2082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2036);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5964);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12066);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12067);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5965);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5966);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5967);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5968);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12070);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24627);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 18707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24629);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24630);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29184);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29185);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31916);
													s.is_empty()
												}
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
							super::push(out, 18708);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5969);
			match s {
				[] => true,
				['и', 'о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18712);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5970);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5971);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12073);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5972);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5973);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12075);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренишь.
pub(crate) fn n2084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2038);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 5979);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12080);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12081);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12082);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5980);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5981);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12084);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12085);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенишь.
pub(crate) fn n2086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2040);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5984);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5985);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5986);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12087);
					match s {
						[] => true,
						['л', 'ю', 'б', '\u{1}', s @ ..] => {
							super::push(out, 18721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5987);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5988);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5989);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5990);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фенишь.
pub(crate) fn n2087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2041);
	s.is_empty()
}

/// Words ending -ценишь.
pub(crate) fn n2088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2042);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5991);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5992);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5993);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5994);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12091);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12092);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ченишь.
pub(crate) fn n2089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2043);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щенишь.
pub(crate) fn n2090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2044);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чинишь.
pub(crate) fn n2109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2063);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6031);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6032);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12126);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6033);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6034);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6035);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18749);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6036);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6037);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6038);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12131);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12132);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12133);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6039);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6040);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12136);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24645);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24646);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29188);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24647);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29189);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6041);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word вершинишь.
pub(crate) fn n19814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18753);
	s.is_empty()
}

/// Words ending -авершинишь.
pub(crate) fn n19815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18754);
	s.is_empty()
}

/// Words ending -щинишь.
pub(crate) fn n2111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2065);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6044);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18755);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клонишь.
pub(crate) fn n6347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6072);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12170);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12171);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12172);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12173);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12174);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 18781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12175);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 12176);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 12177);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -колонишь.
pub(crate) fn n12885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12179);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18783);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29196);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полонишь.
pub(crate) fn n12886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12180);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -солонишь.
pub(crate) fn n12887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12181);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18785);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слонишь.
pub(crate) fn n6349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6074);
	s.is_empty()
}

/// Words ending -монишь.
pub(crate) fn n2130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2084);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6075);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6076);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12184);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нонишь.
pub(crate) fn n2131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2085);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6077);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -понишь.
pub(crate) fn n2132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2086);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 6078);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					super::push(out, 12186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронишь.
pub(crate) fn n2133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2087);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6079);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6080);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12187);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 12188);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18789);
							match s {
								[] => true,
								['а', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29197);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29198);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6081);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12189);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6082);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 12191);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6083);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18792);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18793);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18794);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18795);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18796);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18797);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18798);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18799);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12194);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12195);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24656);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 24657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18802);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18803);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18804);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6084);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6085);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12197);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6086);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 6087);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12198);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6088);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сонишь.
pub(crate) fn n2134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2088);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6089);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 12200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6090);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12201);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонишь.
pub(crate) fn n2135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2089);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6091);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6092);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6093);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6094);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12204);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 12205);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фонишь.
pub(crate) fn n2136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2090);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6096);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6097);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6098);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6099);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 6100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьонишь.
pub(crate) fn n2137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2091);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6101);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аломеришь.
pub(crate) fn n19991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18923);
	s.is_empty()
}

/// Words ending -тмеришь.
pub(crate) fn n6708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6414);
	s.is_empty()
}

/// Words ending -умеришь.
pub(crate) fn n6709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6415);
	s.is_empty()
}

/// Words ending -ымеришь.
pub(crate) fn n6710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6416);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьмеришь.
pub(crate) fn n6711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6417);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 12485);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18924);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неришь.
pub(crate) fn n2290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2244);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6418);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6419);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18925);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 12487);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 6420);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6421);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18927);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6422);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -перишь.
pub(crate) fn n2291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2245);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6423);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6424);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6425);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 6426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12489);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6427);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 6428);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12490);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 18930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -серишь.
pub(crate) fn n2292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2246);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6429);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6430);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 6431);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12491);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12492);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теришь.
pub(crate) fn n2293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2247);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6433);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18931);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 18932);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 18933);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18934);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24690);
									s.is_empty()
								}
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
			super::push(out, 6434);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12495);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6435);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12496);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18936);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 18937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6436);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12497);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6437);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12498);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 18939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12499);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24692);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 24693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18940);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12501);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24694);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 24695);
									s.is_empty()
								}
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
			super::push(out, 6438);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12502);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6439);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12503);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 18943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -феришь.
pub(crate) fn n2294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2248);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6441);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18944);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18945);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херишь.
pub(crate) fn n2295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2249);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6442);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6443);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6444);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6445);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 12506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18946);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черишь.
pub(crate) fn n2296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2250);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6446);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6447);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6448);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12509);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеришь.
pub(crate) fn n2297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2251);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6449);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6450);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 12510);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щеришь.
pub(crate) fn n2298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2252);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6451);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6452);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьеришь.
pub(crate) fn n2299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2253);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 6454);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12511);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 18948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6455);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12512);
					match s {
						[] => true,
						['к', 'а', 'р', 'б', 'о', s @ ..] => {
							super::push(out, 18949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -горишь.
pub(crate) fn n2316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2270);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6485);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6486);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 6487);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6488);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6489);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12543);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12544);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 12545);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6490);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6491);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12546);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6492);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6493);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6494);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6495);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -доришь.
pub(crate) fn n2317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2271);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6496);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12549);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6497);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18977);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18978);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24706);
									s.is_empty()
								}
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
			super::push(out, 6498);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жоришь.
pub(crate) fn n2318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2272);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6500);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12553);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зоришь.
pub(crate) fn n2319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2273);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6501);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6502);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12555);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6503);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12557);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коришь.
pub(crate) fn n2320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6504);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6505);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6506);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6507);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 12560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6508);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 6509);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12561);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12562);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12563);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моришь.
pub(crate) fn n2321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2275);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6511);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6512);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6513);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6514);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 6515);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12564);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6516);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6517);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6518);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6519);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 6520);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6521);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18983);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18984);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24707);
									s.is_empty()
								}
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
			super::push(out, 6522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12567);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 12568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -норишь.
pub(crate) fn n2322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2276);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6523);
			match s {
				[] => true,
				['и', 'о', s @ ..] => {
					super::push(out, 12569);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 18985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поришь.
pub(crate) fn n2323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2277);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6524);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12570);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18986);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24708);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24709);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 29211);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24710);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											super::push(out, 29212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 18987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12571);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12572);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12573);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18988);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12574);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12575);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6526);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18990);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18991);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 24712);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 6527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12577);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12578);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12579);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соришь.
pub(crate) fn n2324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2278);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6528);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6529);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6530);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6531);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6532);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12582);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12583);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12584);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6533);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18996);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торишь.
pub(crate) fn n2325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2279);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6534);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6535);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12587);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12588);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24713);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24714);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24715);
									s.is_empty()
								}
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
			super::push(out, 6536);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12589);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12590);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6537);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12591);
					match s {
						[] => true,
						['о', 'м', '\u{1}', s @ ..] => {
							super::push(out, 19000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6538);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6539);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12593);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 12594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19001);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6540);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 6541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -форишь.
pub(crate) fn n2326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2280);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6542);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12595);
					match s {
						[] => true,
						['е', 'с', '\u{1}', s @ ..] => {
							super::push(out, 19003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6543);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хоришь.
pub(crate) fn n2327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2281);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6544);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ашоришь.
pub(crate) fn n6841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6545);
	s.is_empty()
}

/// Words ending -тгламуришь.
pub(crate) fn n26188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24723);
	s.is_empty()
}

/// Words ending -жмуришь.
pub(crate) fn n6887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6591);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12646);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хмуришь.
pub(crate) fn n6888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6592);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12648);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12649);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12650);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нуришь.
pub(crate) fn n2341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2295);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6593);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 6594);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6595);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пуришь.
pub(crate) fn n2342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2296);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6596);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 12654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туришь.
pub(crate) fn n2343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2297);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6597);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6598);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12655);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 19037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12656);
					match s {
						[] => true,
						['и', 'р', 'а', 'к', '\u{1}', s @ ..] => {
							super::push(out, 19038);
							s.is_empty()
						}
						['у', 'т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24724);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24725);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24726);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29215);
											s.is_empty()
										}
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
					super::push(out, 12657);
					match s {
						[] => true,
						['е', 'п', 'м', 'е', 'т', '\u{1}', s @ ..] => {
							super::push(out, 19039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12658);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12659);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 19040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6600);
			match s {
				[] => true,
				['а', 'х', '\u{1}', s @ ..] => {
					super::push(out, 12660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6601);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12661);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12662);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6602);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12663);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12664);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12665);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6604);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуришь.
pub(crate) fn n2344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2298);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6605);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щуришь.
pub(crate) fn n2345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2299);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6606);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6607);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одоносишь.
pub(crate) fn n20164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19088);
	match s {
		[] => true,
		['л', 'п', 'т', s @ ..] => {
			super::push(out, 24746);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поносишь.
pub(crate) fn n13491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12755);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19089);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19090);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24748);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роносишь.
pub(crate) fn n13492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12756);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 19092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сносишь.
pub(crate) fn n7007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6708);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12757);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тносишь.
pub(crate) fn n7008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6709);
	s.is_empty()
}

/// Words ending -уносишь.
pub(crate) fn n7009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6710);
	s.is_empty()
}

/// Words ending -ыносишь.
pub(crate) fn n7010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6711);
	match s {
		[] => true,
		['в', 'о', s @ ..] => {
			super::push(out, 12759);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19093);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -росишь.
pub(crate) fn n2401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2355);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6712);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12760);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12761);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12762);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19094);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 12763);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24750);
									s.is_empty()
								}
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
			super::push(out, 6713);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19096);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12765);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19098);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19099);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19100);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 19101);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24752);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29225);
											s.is_empty()
										}
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
					super::push(out, 12767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19102);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12768);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12769);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12770);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19104);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 19105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12771);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19106);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19107);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19108);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12772);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12773);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6715);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12774);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24754);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сосишь.
pub(crate) fn n2402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2356);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6716);
			match s {
				[] => true,
				['л', 'ы', 'п', 'о', s @ ..] => {
					super::push(out, 12775);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'х', 'т', s @ ..] => {
					super::push(out, 12776);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уетишь.
pub(crate) fn n2438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2392);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 6815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фетишь.
pub(crate) fn n2439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6816);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12888);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шетишь.
pub(crate) fn n2440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2394);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6817);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19171);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19172);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 19173);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 19174);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 19175);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24787);
									s.is_empty()
								}
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
			super::push(out, 6818);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12889);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24788);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24789);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 24790);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротишь.
pub(crate) fn n2464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2418);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12943);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24801);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24802);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29236);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6855);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19216);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6856);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12945);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19218);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29237);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29238);
											s.is_empty()
										}
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
					super::push(out, 12946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6857);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19219);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19220);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 19221);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19222);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19223);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19224);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19225);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 19226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хсотишь.
pub(crate) fn n7165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6859);
	match s {
		[] => true,
		['у', 'в', 'д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19227);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19228);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 24804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотишь.
pub(crate) fn n2466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2420);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12950);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12951);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12952);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 12953);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12954);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19230);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24805);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шотишь.
pub(crate) fn n2467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2421);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 6861);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12956);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24807);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12957);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19232);
							match s {
								[] => true,
								['к', 'с', '\u{1}', s @ ..] => {
									super::push(out, 24809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -крестишь.
pub(crate) fn n13749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13001);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19259);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19260);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19261);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19262);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 19263);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 19264);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отестишь.
pub(crate) fn n13751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13003);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 19265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -честишь.
pub(crate) fn n7190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6883);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13004);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 13005);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19266);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29242);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -истишь.
pub(crate) fn n2476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2430);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6884);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19267);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19268);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19269);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19270);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24827);
									s.is_empty()
								}
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
			super::push(out, 6885);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13007);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 19271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6886);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13008);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24828);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24829);
									s.is_empty()
								}
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
			super::push(out, 6887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13009);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13010);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 13011);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13012);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13013);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13014);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13015);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лстишь.
pub(crate) fn n2478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2432);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6888);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19273);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 19274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мстишь.
pub(crate) fn n2479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6889);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -остишь.
pub(crate) fn n2480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2434);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6891);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 13016);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19275);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29244);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29245);
											s.is_empty()
										}
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
						['и', s @ ..] => {
							super::push(out, 19276);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29247);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29248);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 29249);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19277);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 24830);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19278);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24831);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29251);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29252);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 31918);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6892);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13017);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13018);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 13019);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13020);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6893);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13021);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13022);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24832);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24833);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6894);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13024);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24835);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24836);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29253);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13025);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13026);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19281);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 24837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13027);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13028);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13029);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13030);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13031);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							super::push(out, 19283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6897);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13032);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13033);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рстишь.
pub(crate) fn n2481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2435);
	match s {
		[] => true,
		['е', 'ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13034);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13035);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6898);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 13037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устишь.
pub(crate) fn n2482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2436);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6899);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13039);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13041);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6902);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6903);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6904);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13044);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13045);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13046);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13047);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6905);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19288);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19289);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19291);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьстишь.
pub(crate) fn n2483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2437);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6906);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13050);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13051);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -забаламутишь.
pub(crate) fn n33840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31919);
	s.is_empty()
}

/// Words ending -набаламутишь.
pub(crate) fn n33841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31920);
	s.is_empty()
}

/// Words ending -рочишь.
pub(crate) fn n2572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2525);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7075);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13228);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13229);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7076);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19370);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19371);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19373);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13233);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19375);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24858);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29258);
											s.is_empty()
										}
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
					super::push(out, 13234);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13235);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19376);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24859);
									s.is_empty()
								}
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
			super::push(out, 7077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13236);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13237);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 13238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7078);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 7079);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13239);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19377);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19378);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19379);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19380);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 19381);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24863);
									s.is_empty()
								}
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
			super::push(out, 7080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13240);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 13241);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13242);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19382);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одсочишь.
pub(crate) fn n14003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13243);
	s.is_empty()
}

/// Words ending -есочишь.
pub(crate) fn n7409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7082);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 13244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19384);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19385);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысочишь.
pub(crate) fn n7410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7083);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точишь.
pub(crate) fn n2574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2527);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7084);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7085);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13246);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13247);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7086);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7087);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7088);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7089);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7090);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7091);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13253);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 13254);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 13255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19388);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19389);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 24864);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13256);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13257);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13258);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19391);
							match s {
								[] => true,
								['м', 'а', s @ ..] => {
									super::push(out, 24866);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29259);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 19392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7092);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13259);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13260);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 19395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7093);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7094);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7095);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13263);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 19396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24868);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 24869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 13264);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езручишь.
pub(crate) fn n14083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13323);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иручишь.
pub(crate) fn n7490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7158);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 13324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оручишь.
pub(crate) fn n7491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7159);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 13325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыручишь.
pub(crate) fn n7492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7160);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сучишь.
pub(crate) fn n2599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2552);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7161);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7162);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 7163);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7164);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7165);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13327);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7166);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7167);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тучишь.
pub(crate) fn n2600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2553);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7168);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7169);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7170);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7171);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13331);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13332);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13333);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13334);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 7172);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13335);
					match s {
						[] => true,
						['н', 'у', 'м', '\u{1}', s @ ..] => {
							super::push(out, 19413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щучишь.
pub(crate) fn n2601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2554);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7173);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7174);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыучишь.
pub(crate) fn n2602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2555);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яучишь.
pub(crate) fn n2603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2556);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аспотрошишь.
pub(crate) fn n31038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29263);
	s.is_empty()
}

/// Words ending -ыпотрошишь.
pub(crate) fn n26358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24885);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тошишь.
pub(crate) fn n2654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2607);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7267);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7268);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19469);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7269);
			match s {
				[] => true,
				['у', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19471);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аешься.
pub(crate) fn n2702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2655);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7355);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13504);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 19513);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13505);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19515);
							match s {
								[] => true,
								['х', '\u{1}', s @ ..] => {
									super::push(out, 24895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13506);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19517);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 19518);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 24896);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29266);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 24897);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 29267);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31921);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13507);
					match s {
						[] => true,
						['о', 'д', '\u{1}', s @ ..] => {
							super::push(out, 19519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13508);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24898);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29268);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24899);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29269);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31922);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13509);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19520);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19521);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29270);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29271);
											s.is_empty()
										}
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
					super::push(out, 13510);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19522);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19523);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24900);
									s.is_empty()
								}
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29272);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29273);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t8::n7700(s, out),
		['г', s @ ..] => super::t8::n7701(s, out),
		['д', s @ ..] => super::t8::n7702(s, out),
		['ж', s @ ..] => {
			super::push(out, 7359);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13543);
					match s {
						[] => true,
						['л', 'б', 'о', s @ ..] => {
							super::push(out, 19637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19638);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25128);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25129);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 25130);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'л', s @ ..] => {
					super::push(out, 13544);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19640);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 19641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 13545);
					match s {
						[] => true,
						['о', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25132);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 25133);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13546);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19643);
							match s {
								[] => true,
								['о', 'о', s @ ..] => {
									super::push(out, 25135);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29516);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'у', s @ ..] => {
							super::push(out, 19644);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25136);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'о', 'д', 'а', s @ ..] => super::t8::n14321(s, out),
				['о', s @ ..] => super::t8::n14322(s, out),
				['у', s @ ..] => super::t8::n14323(s, out),
				['я', s @ ..] => super::t8::n14324(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t8::n7704(s, out),
		['к', s @ ..] => super::t8::n7705(s, out),
		['л', s @ ..] => super::t8::n7706(s, out),
		['м', s @ ..] => super::t8::n7707(s, out),
		['н', s @ ..] => super::t8::n7708(s, out),
		['п', s @ ..] => super::t8::n7709(s, out),
		['р', s @ ..] => super::t8::n7710(s, out),
		['с', s @ ..] => super::t8::n7711(s, out),
		['т', s @ ..] => super::t8::n7712(s, out),
		['х', s @ ..] => super::t8::n7713(s, out),
		['ц', s @ ..] => super::t8::n7714(s, out),
		['ч', s @ ..] => super::t8::n7715(s, out),
		['ш', s @ ..] => super::t8::n7716(s, out),
		['щ', s @ ..] => super::t8::n7717(s, out),
		_ => false,
	}
}

/// Words ending -дешься.
pub(crate) fn n2705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2658);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7381);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13703);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25629);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25630);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29848);
											s.is_empty()
										}
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
					super::push(out, 13704);
					s.is_empty()
				}
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19972);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19973);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 19974);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 19975);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19976);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19977);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25631);
									s.is_empty()
								}
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
			super::push(out, 7382);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19978);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19979);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19980);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19981);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19982);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13706);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13707);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 13708);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19984);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19985);
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 25633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 7383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13709);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13710);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 19987);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25634);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 19988);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 19989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13711);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13712);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7384);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 7385);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7386);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 7387);
			match s {
				[] => true,
				['л', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19991);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7388);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13715);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19993);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25635);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25636);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25637);
									match s {
										[] => true,
										['в', s @ ..] => {
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
				['с', s @ ..] => {
					super::push(out, 13716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еешься.
pub(crate) fn n2706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2659);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7389);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13718);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13719);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13720);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7391);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13721);
					s.is_empty()
				}
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19995);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13722);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19997);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19998);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19999);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25638);
									s.is_empty()
								}
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
			super::push(out, 7392);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 7393);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20000);
							s.is_empty()
						}
						['ж', '\u{1}', s @ ..] => {
							super::push(out, 20001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13724);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25639);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', '\u{1}', s @ ..] => {
							super::push(out, 20002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13725);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25641);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25642);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29851);
											s.is_empty()
										}
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
					super::push(out, 13726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20003);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20004);
							s.is_empty()
						}
						['е', 'в', 'с', '\u{1}', s @ ..] => {
							super::push(out, 20005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7394);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20006);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20007);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20009);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20010);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20011);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20012);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13729);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20013);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29852);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 29853);
											s.is_empty()
										}
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
					super::push(out, 13730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7395);
			match s {
				[] => true,
				['д', 'и', 'в', '\u{1}', s @ ..] => {
					super::push(out, 13731);
					s.is_empty()
				}
				['е', 'л', 'е', 'з', '\u{1}', s @ ..] => {
					super::push(out, 13732);
					s.is_empty()
				}
				['и', 'с', '\u{1}', s @ ..] => {
					super::push(out, 13733);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13734);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13735);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25644);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 25645);
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
						['а', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25646);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 20014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7396);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7397);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13736);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20016);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20017);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20018);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 13737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20019);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13738);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20021);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 25649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13739);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20022);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20023);
							match s {
								[] => true,
								['е', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29854);
											s.is_empty()
										}
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t8::n14526(s, out),
				['о', s @ ..] => super::t8::n14527(s, out),
				['ы', s @ ..] => super::t8::n14528(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t8::n7743(s, out),
		_ => false,
	}
}

/// Words ending -жешься.
pub(crate) fn n2707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2660);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7400);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20025);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20026);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20027);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20028);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25651);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25652);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29856);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20029);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20030);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20031);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13746);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20032);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20033);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20034);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20035);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20036);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25654);
									s.is_empty()
								}
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
			super::push(out, 7401);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20037);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20038);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20039);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20040);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20041);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 25655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29857);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 25656);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29858);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 25657);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32199);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32200);
													s.is_empty()
												}
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
							super::push(out, 20042);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 20043);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20044);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20045);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20046);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20047);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20048);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20049);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 7402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13749);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7403);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20050);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20051);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20052);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20053);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20054);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25659);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13752);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13753);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20055);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29859);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29860);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 29861);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32201);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7404);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7405);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13755);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20056);
							match s {
								[] => true,
								['н', 'а', s @ ..] => {
									super::push(out, 25660);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29862);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7406);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13756);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7407);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13758);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t8::n14545(s, out),
				['л', s @ ..] => {
					super::push(out, 13760);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t8::n14547(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иешься.
pub(crate) fn n2709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2662);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7412);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7413);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7414);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кешься.
pub(crate) fn n2710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2663);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 7415);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13769);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13770);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13771);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лешься.
pub(crate) fn n2711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2664);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7416);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13772);
					match s {
						[] => true,
						['л', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25665);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25666);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25667);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13773);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7417);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20071);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20072);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13775);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25670);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25671);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25672);
									s.is_empty()
								}
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
			super::push(out, 7418);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 7419);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13776);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20073);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29864);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 20074);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25673);
									s.is_empty()
								}
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
			super::push(out, 7420);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13777);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20075);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20076);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20077);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20078);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20079);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20080);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20081);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20082);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20083);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25676);
									s.is_empty()
								}
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
			super::push(out, 7421);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13780);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20084);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29866);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29867);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29868);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 29869);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32202);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13781);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 20085);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25677);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13782);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25679);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25680);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 25681);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 25682);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25683);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29870);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 29871);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32203);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 25684);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25685);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 25686);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25687);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29872);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7422);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13783);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13784);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мешься.
pub(crate) fn n2712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2665);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 7423);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13785);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13786);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13787);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13788);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13789);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13790);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 7425);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13792);
					s.is_empty()
				}
				['о', 'д', s @ ..] => {
					super::push(out, 13793);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 13794);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7426);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 7427);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нешься.
pub(crate) fn n2713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2666);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7428);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13796);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 13797);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 13798);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13799);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 13800);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 13801);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13802);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20090);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13803);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13804);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20092);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20093);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20094);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25689);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 29873);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20095);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 20096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7429);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13806);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20097);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 20098);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20099);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13807);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 13808);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7430);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7431);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13809);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13810);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13811);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20101);
							match s {
								[] => true,
								['с', 'ы', s @ ..] => {
									super::push(out, 25692);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29874);
											s.is_empty()
										}
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
					super::push(out, 13812);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20102);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25693);
									match s {
										[] => true,
										['о', 'п', 'с', s @ ..] => {
											super::push(out, 29875);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32204);
													s.is_empty()
												}
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
							super::push(out, 20103);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20104);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20105);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13813);
					match s {
						[] => true,
						['р', 'д', s @ ..] => {
							super::push(out, 20106);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25696);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29876);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13814);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20107);
							match s {
								[] => true,
								['в', 'т', s @ ..] => {
									super::push(out, 25698);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 29877);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20108);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 25699);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29878);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29879);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32205);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32206);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 32207);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32208);
													match s {
														[] => true,
														['в', s @ ..] => super::t8::n35092(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 29880);
											s.is_empty()
										}
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
					super::push(out, 13815);
					s.is_empty()
				}
				['ы', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20109);
							s.is_empty()
						}
						['р', 'д', s @ ..] => {
							super::push(out, 20110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', 'р', s @ ..] => {
					super::push(out, 13816);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7432);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13817);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13818);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t8::n7777(s, out),
		['ж', s @ ..] => {
			super::push(out, 7434);
			s.is_empty()
		}
		['з', s @ ..] => super::t8::n7779(s, out),
		['и', s @ ..] => super::t8::n7780(s, out),
		['к', s @ ..] => super::t8::n7781(s, out),
		['м', s @ ..] => super::t8::n7782(s, out),
		['о', s @ ..] => super::t8::n7783(s, out),
		['п', s @ ..] => super::t8::n7784(s, out),
		['р', s @ ..] => super::t8::n7785(s, out),
		['с', s @ ..] => super::t8::n7786(s, out),
		['т', s @ ..] => super::t8::n7787(s, out),
		['у', s @ ..] => super::t8::n7788(s, out),
		['ф', s @ ..] => super::t8::n7789(s, out),
		['х', s @ ..] => super::t8::n7790(s, out),
		['ц', s @ ..] => super::t8::n7791(s, out),
		['ч', s @ ..] => super::t8::n7792(s, out),
		['щ', s @ ..] => super::t8::n7793(s, out),
		['ы', s @ ..] => super::t8::n7794(s, out),
		['ь', s @ ..] => super::t8::n7795(s, out),
		['ю', s @ ..] => super::t8::n7796(s, out),
		['я', s @ ..] => super::t8::n7797(s, out),
		_ => false,
	}
}

/// Words ending -оешься.
pub(crate) fn n2714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2667);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7454);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7455);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 7456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13897);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13898);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13899);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13900);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13901);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13902);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13903);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13904);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13905);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 13906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20232);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'д', s @ ..] => {
					super::push(out, 13907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13908);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13909);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7459);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -решься.
pub(crate) fn n2715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2668);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13911);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7461);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20236);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20237);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20238);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20239);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 25791);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29926);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20240);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20241);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20242);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20243);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20244);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20245);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25793);
									s.is_empty()
								}
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
			super::push(out, 7462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13913);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7463);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13915);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13916);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 13917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20247);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20248);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20249);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20250);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20253);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20254);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20255);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20256);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20257);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25795);
									s.is_empty()
								}
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
			super::push(out, 7464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13921);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13922);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13923);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13924);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13925);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13926);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13927);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 13928);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13929);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13930);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13931);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13932);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13933);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сешься.
pub(crate) fn n2716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2669);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7467);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20261);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7468);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20263);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20264);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20265);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20266);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20267);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25796);
									s.is_empty()
								}
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
			super::push(out, 7469);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20268);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20269);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20270);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20271);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25797);
									s.is_empty()
								}
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
			super::push(out, 7470);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 13934);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7471);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20273);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20274);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20275);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тешься.
pub(crate) fn n2717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2670);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7472);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13935);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20276);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20277);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25800);
									s.is_empty()
								}
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
			super::push(out, 7473);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13936);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 20278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20279);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20280);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20281);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20282);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20283);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20284);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20285);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20286);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20287);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20288);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13938);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7474);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 13940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 7475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13941);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13942);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13943);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7476);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уешься.
pub(crate) fn n2718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7477);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 7478);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13945);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20291);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29927);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29928);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29929);
											s.is_empty()
										}
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
					super::push(out, 13946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20292);
							match s {
								[] => true,
								['р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 25803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13947);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20293);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20294);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29930);
											s.is_empty()
										}
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
					super::push(out, 13948);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20295);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29931);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29932);
											s.is_empty()
										}
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
					super::push(out, 13949);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20296);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25806);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25807);
									s.is_empty()
								}
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
			super::push(out, 7479);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13950);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25808);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25809);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13951);
					match s {
						[] => true,
						['р', 'е', 'ж', s @ ..] => super::t8::n21444(s, out),
						['с', s @ ..] => super::t8::n21445(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t8::n7824(s, out),
		['д', s @ ..] => super::t8::n7825(s, out),
		['ж', s @ ..] => super::t8::n7826(s, out),
		['з', s @ ..] => super::t8::n7827(s, out),
		['к', s @ ..] => super::t8::n7828(s, out),
		['л', s @ ..] => super::t8::n7829(s, out),
		['м', s @ ..] => super::t8::n7830(s, out),
		['н', s @ ..] => super::t8::n7831(s, out),
		['п', s @ ..] => super::t8::n7832(s, out),
		['р', s @ ..] => super::t8::n7833(s, out),
		['с', s @ ..] => super::t9::n7834(s, out),
		['т', s @ ..] => super::t9::n7835(s, out),
		['ф', s @ ..] => super::t9::n7836(s, out),
		['х', s @ ..] => super::t9::n7837(s, out),
		['ц', s @ ..] => super::t9::n7838(s, out),
		['ч', s @ ..] => super::t9::n7839(s, out),
		['ш', s @ ..] => super::t9::n7840(s, out),
		_ => false,
	}
}

/// Words ending -чешься.
pub(crate) fn n2719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2672);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7497);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14078);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14079);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26147);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26148);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26149);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26150);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30277);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7498);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14080);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20490);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 26151);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14081);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26152);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 26153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20491);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20492);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20494);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20495);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20496);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20497);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20498);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20499);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20500);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20501);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20502);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20503);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20504);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 20505);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20506);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26159);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26160);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30278);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7499);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14087);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26161);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26162);
									s.is_empty()
								}
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
			super::push(out, 7500);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14088);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20507);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26163);
									s.is_empty()
								}
								['щ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30279);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30280);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20508);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14089);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20509);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30281);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 30282);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32499);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14090);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20510);
							match s {
								[] => true,
								['о', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30283);
											s.is_empty()
										}
										['а', s @ ..] => {
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
				['п', s @ ..] => {
					super::push(out, 14091);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20511);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26165);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 26166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t9::n14898(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t9::n7845(s, out),
		['ы', s @ ..] => super::t9::n7846(s, out),
		['я', s @ ..] => super::t9::n7847(s, out),
		_ => false,
	}
}

/// Words ending -шешься.
pub(crate) fn n2720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2673);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7504);
			match s {
				[] => true,
				['п', 'ы', s @ ..] => {
					super::push(out, 14099);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7505);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14100);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 20519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20520);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20521);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20522);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20523);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20524);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20525);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20526);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26176);
									s.is_empty()
								}
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
			super::push(out, 7506);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20527);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20528);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20529);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20530);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20531);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26177);
									s.is_empty()
								}
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
			super::push(out, 7507);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14103);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7508);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14104);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26178);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26179);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26180);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26181);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26182);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30285);
											s.is_empty()
										}
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
					super::push(out, 14105);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щешься.
pub(crate) fn n2721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2674);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7509);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26183);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26184);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26185);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26186);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30286);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 20534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14106);
					match s {
						[] => true,
						['е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 20535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14107);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 14108);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14109);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7511);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14110);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14111);
					match s {
						[] => true,
						['о', 'п', 'ы', s @ ..] => {
							super::push(out, 20537);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26187);
									s.is_empty()
								}
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
			super::push(out, 7512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъешься.
pub(crate) fn n2722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2675);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 7513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыешься.
pub(crate) fn n2723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2676);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7515);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьешься.
pub(crate) fn n2724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2677);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14113);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14114);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14115);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14116);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14117);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20541);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14118);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20542);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 20543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26189);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26190);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20544);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20545);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20546);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20547);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14119);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14120);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26194);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26195);
									s.is_empty()
								}
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
			super::push(out, 7518);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14121);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14122);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14123);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14124);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20552);
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							super::push(out, 20553);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20554);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14126);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14127);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14128);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14129);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14130);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14131);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14132);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14133);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20559);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20560);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 20561);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20562);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20563);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20564);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14134);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14135);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26198);
									s.is_empty()
								}
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
			super::push(out, 7520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14136);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14137);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14138);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14139);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20568);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20569);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20570);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 20571);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20572);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26199);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 20573);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20574);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20575);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14141);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14142);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t9::n7865(s, out),
		_ => false,
	}
}

/// Words ending -юешься.
pub(crate) fn n2725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2678);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7522);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14148);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20586);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20587);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26205);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26206);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 26207);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 26208);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26209);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 20588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 14149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20589);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20590);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14150);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20591);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20592);
							match s {
								[] => true,
								['м', 'е', 'т', 'ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30290);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30291);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30292);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20593);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 20594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14151);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20596);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20597);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26211);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 26212);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 26213);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 30293);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32500);
													s.is_empty()
												}
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
							super::push(out, 20598);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20599);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26214);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20600);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 26216);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 30294);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32501);
													s.is_empty()
												}
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
							super::push(out, 20601);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20602);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20603);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20604);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20605);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20606);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20607);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20608);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7523);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7524);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7525);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яешься.
pub(crate) fn n2726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2679);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7526);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 14158);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20612);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20613);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20614);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20615);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26219);
									s.is_empty()
								}
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
			super::push(out, 7527);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14161);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7528);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14163);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20616);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20617);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26220);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26221);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 26222);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26223);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 26224);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26225);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26226);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 26227);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 26228);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 26229);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26230);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30295);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 20618);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 20619);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20620);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 14164);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 20622);
							match s {
								[] => true,
								['л', 'в', 'о', s @ ..] => {
									super::push(out, 26231);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30296);
											s.is_empty()
										}
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
					super::push(out, 14165);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20623);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 26232);
									match s {
										[] => true,
										['у', s @ ..] => super::t9::n32153(s, out),
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 26233);
									match s {
										[] => true,
										['п', 'с', s @ ..] => super::t9::n32154(s, out),
										_ => false,
									}
								}
								['р', s @ ..] => super::t9::n27811(s, out),
								['т', 'с', 'ы', s @ ..] => super::t9::n27812(s, out),
								_ => false,
							}
						}
						['и', s @ ..] => super::t9::n21831(s, out),
						['о', s @ ..] => super::t9::n21832(s, out),
						['т', s @ ..] => super::t9::n21833(s, out),
						['я', s @ ..] => super::t9::n21834(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t9::n14974(s, out),
				['е', s @ ..] => super::t9::n14975(s, out),
				['и', s @ ..] => super::t9::n14976(s, out),
				['м', s @ ..] => super::t9::n14977(s, out),
				['о', 'г', s @ ..] => super::t9::n14978(s, out),
				['п', s @ ..] => super::t9::n14979(s, out),
				['с', s @ ..] => super::t9::n14980(s, out),
				['т', s @ ..] => super::t9::n14981(s, out),
				['у', s @ ..] => super::t9::n14982(s, out),
				['ф', s @ ..] => super::t9::n14983(s, out),
				['х', s @ ..] => super::t9::n14984(s, out),
				['ш', s @ ..] => super::t9::n14985(s, out),
				['ы', s @ ..] => super::t9::n14986(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t9::n7873(s, out),
		['о', s @ ..] => super::t9::n7874(s, out),
		['р', s @ ..] => super::t9::n7875(s, out),
		['с', s @ ..] => super::t9::n7876(s, out),
		['у', s @ ..] => super::t9::n7877(s, out),
		_ => false,
	}
}

/// Words ending -ебишься.
pub(crate) fn n7881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7537);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14211);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20678);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20679);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20680);
					match s {
						[] => true,
						['а', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30345);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30346);
									s.is_empty()
								}
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
			super::push(out, 14212);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20681);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30347);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30348);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30349);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32525);
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
								['\u{1}', s @ ..] => {
									super::push(out, 30350);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30351);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30352);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30353);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32526);
											s.is_empty()
										}
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
					super::push(out, 20682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лбишься.
pub(crate) fn n7882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7538);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14213);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26299);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26300);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 26301);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 26302);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26303);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30354);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26304);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26305);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20684);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбишься.
pub(crate) fn n7883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7539);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14214);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26307);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26308);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обишься.
pub(crate) fn n7884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7540);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14215);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20685);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26310);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30358);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30359);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 32527);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30360);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 32528);
											s.is_empty()
										}
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
					super::push(out, 20686);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14216);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14217);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20689);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 20690);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26313);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14218);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26315);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26316);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14219);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26318);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26319);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26321);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20695);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30362);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30363);
									s.is_empty()
								}
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
			super::push(out, 14220);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26324);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбишься.
pub(crate) fn n7885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7541);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14221);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 20697);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26326);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26327);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26328);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26329);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30364);
									s.is_empty()
								}
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
			super::push(out, 14222);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26330);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 26331);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26332);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26333);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20699);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убишься.
pub(crate) fn n7886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7542);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 14223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20700);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20701);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20702);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20703);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20704);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20705);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26338);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									super::push(out, 30366);
									s.is_empty()
								}
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
			super::push(out, 14224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20706);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20707);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20708);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20709);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14225);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14226);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20711);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 20712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26340);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26341);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26342);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20713);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26344);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30368);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32529);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14227);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20714);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20715);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20717);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 20718);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20719);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20720);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20721);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20722);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20723);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20724);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26347);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26348);
							s.is_empty()
						}
						['о', 'р', s @ ..] => {
							super::push(out, 26349);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20726);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20727);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26351);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбишься.
pub(crate) fn n7887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7543);
	match s {
		[] => true,
		['д', 'з', s @ ..] => {
			super::push(out, 14228);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14229);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юбишься.
pub(crate) fn n7888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7544);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20729);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20730);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябишься.
pub(crate) fn n7889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7545);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14232);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20732);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20733);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вишься.
pub(crate) fn n2729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2682);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7546);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20735);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20736);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 26352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14235);
					match s {
						[] => true,
						['о', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26353);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26354);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26355);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30370);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 26356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20738);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20739);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26357);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 26358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 20740);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20741);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20742);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20743);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14237);
					match s {
						[] => true,
						['е', 'р', 'т', 'с', '\u{1}', s @ ..] => {
							super::push(out, 20744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14238);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20745);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26360);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26361);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26362);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26363);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26364);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30371);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26365);
									s.is_empty()
								}
								['е', 'щ', 'т', '\u{1}', s @ ..] => {
									super::push(out, 26366);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26367);
									s.is_empty()
								}
								['с', 'е', 'б', '\u{1}', s @ ..] => {
									super::push(out, 26368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14239);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20747);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26369);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26370);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26371);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30372);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 20748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26372);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26373);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26374);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26375);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26376);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30374);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26377);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26378);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26379);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20750);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30376);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30377);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t9::n32238(s, out),
										['а', s @ ..] => super::t9::n32239(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t9::n15051(s, out),
				['т', s @ ..] => super::t9::n15052(s, out),
				['ч', s @ ..] => super::t9::n15053(s, out),
				['ш', s @ ..] => super::t9::n15054(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t9::n7891(s, out),
		['з', s @ ..] => super::t9::n7892(s, out),
		['и', s @ ..] => super::t9::n7893(s, out),
		['л', s @ ..] => super::t9::n7894(s, out),
		['о', s @ ..] => super::t9::n7895(s, out),
		['р', s @ ..] => super::t9::n7896(s, out),
		['т', s @ ..] => super::t9::n7897(s, out),
		['я', s @ ..] => super::t9::n7898(s, out),
		_ => false,
	}
}

/// Words ending -гишься.
pub(crate) fn n2730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2683);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7555);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20807);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дишься.
pub(crate) fn n2731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2684);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7556);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14277);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20809);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26449);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26450);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30404);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 14278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14279);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20812);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20813);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 20814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26451);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26452);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26453);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30405);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20815);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 20816);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20817);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26454);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30406);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 26455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 20818);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14281);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20819);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30407);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30408);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 26458);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 30410);
											s.is_empty()
										}
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
					super::push(out, 14282);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20820);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 20821);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20822);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 26460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14283);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20823);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20824);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20825);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20826);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14284);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 14285);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 14286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20828);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20829);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26462);
									s.is_empty()
								}
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
			super::push(out, 7557);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7558);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14288);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14289);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 20831);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14290);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14291);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20833);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20834);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26464);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30411);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => super::t9::n22062(s, out),
						['з', s @ ..] => {
							super::push(out, 20836);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20837);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20838);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20839);
							s.is_empty()
						}
						['ч', s @ ..] => super::t9::n22067(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t9::n15103(s, out),
				['ц', s @ ..] => super::t9::n15104(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t9::n7903(s, out),
		['и', s @ ..] => super::t9::n7904(s, out),
		['й', s @ ..] => super::t9::n7905(s, out),
		['н', s @ ..] => super::t9::n7906(s, out),
		['о', s @ ..] => super::t9::n7907(s, out),
		['р', s @ ..] => super::t9::n7908(s, out),
		['у', s @ ..] => super::t9::n7909(s, out),
		['ы', s @ ..] => super::t9::n7910(s, out),
		['ю', s @ ..] => super::t9::n7911(s, out),
		['я', s @ ..] => super::t9::n7912(s, out),
		_ => false,
	}
}

/// Words ending -еишься.
pub(crate) fn n2732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2685);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7569);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20956);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20957);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20958);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26580);
									s.is_empty()
								}
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
			super::push(out, 7570);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20959);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20960);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 7571);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 14334);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жишься.
pub(crate) fn n2733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2686);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7572);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14335);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14336);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20962);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26581);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30479);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 26582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 20963);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 26583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20964);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14337);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 20965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14338);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14339);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20967);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20968);
							match s {
								[] => true,
								['д', 'у', 'б', '\u{1}', s @ ..] => {
									super::push(out, 26586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20969);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 26587);
									s.is_empty()
								}
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
			super::push(out, 7573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14340);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14341);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 14342);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20971);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20972);
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 26588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14343);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14344);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14345);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20975);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 20976);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26589);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20977);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20978);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20979);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 20980);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26590);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 20981);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14346);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20983);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20984);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26592);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 26593);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30482);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30483);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 26594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20985);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20986);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20987);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26595);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14348);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14349);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20989);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30485);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30486);
											s.is_empty()
										}
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
					super::push(out, 14350);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 7574);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7575);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14352);
					match s {
						[] => true,
						['р', s @ ..] => super::t9::n22222(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t9::n15164(s, out),
				['о', s @ ..] => super::t9::n15165(s, out),
				['ю', s @ ..] => super::t9::n15166(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t9::n7920(s, out),
		['л', s @ ..] => super::t9::n7921(s, out),
		['н', s @ ..] => super::t9::n7922(s, out),
		['о', s @ ..] => super::t9::n7923(s, out),
		['р', s @ ..] => super::t9::n7924(s, out),
		['у', s @ ..] => super::t9::n7925(s, out),
		['ы', s @ ..] => super::t9::n7926(s, out),
		['ю', s @ ..] => super::t9::n7927(s, out),
		['я', s @ ..] => super::t9::n7928(s, out),
		_ => false,
	}
}

/// Words ending -зишься.
pub(crate) fn n2734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2687);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7585);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14383);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14384);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14385);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21038);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 21039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21040);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21041);
							match s {
								[] => true,
								['з', 'з', s @ ..] => {
									super::push(out, 26650);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 21042);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30518);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 30519);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32581);
													match s {
														[] => true,
														['б', s @ ..] => super::t9::n35309(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 32582);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30520);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32583);
													match s {
														[] => true,
														['з', s @ ..] => super::t9::n35310(s, out),
														_ => false,
													}
												}
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
							super::push(out, 21043);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21044);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21045);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21046);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21047);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14387);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 21048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7586);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7587);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14389);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21049);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 26652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26653);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26654);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14390);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26656);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26657);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26658);
									s.is_empty()
								}
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
			super::push(out, 7588);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14391);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26659);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26660);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30521);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 26661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21052);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14392);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26662);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26663);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30522);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 26664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14394);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7589);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 14395);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t9::n7935(s, out),
		['р', s @ ..] => {
			super::push(out, 7592);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14403);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14404);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 21078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n7937(s, out),
		['ю', s @ ..] => {
			super::push(out, 7594);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7595);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лишься.
pub(crate) fn n2735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2688);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7596);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14417);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21095);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30539);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30540);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30541);
											s.is_empty()
										}
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
					super::push(out, 14418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21096);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21097);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21098);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21099);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21100);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21101);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21102);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21103);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21104);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21105);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 21106);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26695);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26696);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30542);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32585);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 32586);
													s.is_empty()
												}
												['з', s @ ..] => {
													super::push(out, 32587);
													match s {
														[] => true,
														['а', s @ ..] => super::t9::n35311(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 32588);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 30543);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 30544);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 26697);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 30545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26698);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 30546);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 30547);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 26699);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30548);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30549);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 26700);
									match s {
										[] => true,
										['в', s @ ..] => super::t9::n32413(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => super::t9::n22345(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t9::n15230(s, out),
				['д', s @ ..] => super::t9::n15231(s, out),
				['ж', s @ ..] => super::t9::n15232(s, out),
				['и', s @ ..] => super::t9::n15233(s, out),
				['к', s @ ..] => super::t9::n15234(s, out),
				['м', s @ ..] => super::t9::n15235(s, out),
				['п', s @ ..] => super::t9::n15236(s, out),
				['р', s @ ..] => super::t9::n15237(s, out),
				['с', s @ ..] => super::t9::n15238(s, out),
				['ч', s @ ..] => super::t9::n15239(s, out),
				['ш', s @ ..] => super::t9::n15240(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t9::n7941(s, out),
		['г', s @ ..] => super::t9::n7942(s, out),
		['д', s @ ..] => super::t9::n7943(s, out),
		['е', s @ ..] => super::t9::n7944(s, out),
		['з', s @ ..] => super::t9::n7945(s, out),
		['и', s @ ..] => super::t9::n7946(s, out),
		['к', s @ ..] => super::t9::n7947(s, out),
		['м', s @ ..] => super::t9::n7948(s, out),
		['о', s @ ..] => super::t9::n7949(s, out),
		['п', s @ ..] => super::t9::n7950(s, out),
		['р', s @ ..] => super::t9::n7951(s, out),
		['с', s @ ..] => super::t9::n7952(s, out),
		['т', s @ ..] => super::t9::n7953(s, out),
		['у', s @ ..] => super::t9::n7954(s, out),
		['х', s @ ..] => super::t9::n7955(s, out),
		['ш', s @ ..] => super::t9::n7956(s, out),
		['ы', s @ ..] => super::t9::n7957(s, out),
		['э', s @ ..] => super::t9::n7958(s, out),
		['ю', s @ ..] => super::t9::n7959(s, out),
		['я', s @ ..] => super::t9::n7960(s, out),
		_ => false,
	}
}

