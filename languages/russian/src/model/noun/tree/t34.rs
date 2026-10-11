use flexion::Path;

/// Words ending -сушки.
pub(crate) fn n28592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28398);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 55808);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 83851);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушки.
pub(crate) fn n28593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28399);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 55809);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 55810);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 83853);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106218);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 83854);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 106219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 83855);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 106220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эшки.
pub(crate) fn n9508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9481);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 28400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юшки.
pub(crate) fn n9509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9482);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 28401);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 55811);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 83856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яшки.
pub(crate) fn n9510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9483);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 28402);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 55812);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83857);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 106221);
									s.is_empty()
								}
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
			super::push(out, 28403);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55813);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 83858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 28404);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 55814);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 83859);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 106222);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 121435);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пошевеньки.
pub(crate) fn n132106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130749);
	s.is_empty()
}

/// Words ending -етвереньки.
pub(crate) fn n132107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130750);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 136197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окономияки.
pub(crate) fn n132108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130751);
	s.is_empty()
}

/// Words ending -наркорубли.
pub(crate) fn n132110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130753);
	s.is_empty()
}

/// Words ending -тальятелли.
pub(crate) fn n132111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130754);
	s.is_empty()
}

/// The word рокабилли.
pub(crate) fn n132112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130755);
	s.is_empty()
}

/// Words ending -ндзмараули.
pub(crate) fn n132113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130756);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 136199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рамбамбули.
pub(crate) fn n132114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130757);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ксандреули.
pub(crate) fn n132115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130758);
	s.is_empty()
}

/// Words ending -тернет-сми.
pub(crate) fn n132119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130762);
	s.is_empty()
}

/// Words ending -тортеллини.
pub(crate) fn n132124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130767);
	s.is_empty()
}

/// Words ending -каннеллони.
pub(crate) fn n132125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130768);
	s.is_empty()
}

/// Words ending -окьюментари.
pub(crate) fn n137614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136203);
	s.is_empty()
}

/// Words ending -бьянконери.
pub(crate) fn n132130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130773);
	s.is_empty()
}

/// Words ending -гуштингири.
pub(crate) fn n132131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130774);
	s.is_empty()
}

/// Words ending -хикикомори.
pub(crate) fn n132132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130775);
	s.is_empty()
}

/// Words ending --энд-керри.
pub(crate) fn n132133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130776);
	s.is_empty()
}

/// Words ending -осс-кантри.
pub(crate) fn n132134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130777);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 136204);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 139385);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -карнопутри.
pub(crate) fn n132135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130778);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 136205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нью-джерси.
pub(crate) fn n132137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130780);
	s.is_empty()
}

/// Words ending -таллахасси.
pub(crate) fn n132138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130781);
	s.is_empty()
}

/// Words ending -йтроб-сити.
pub(crate) fn n132139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130782);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136207);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-области.
pub(crate) fn n132140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130783);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еленовости.
pub(crate) fn n132141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130784);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 136209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -икиновости.
pub(crate) fn n132142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130785);
	s.is_empty()
}

/// Words ending -ол-емкости.
pub(crate) fn n132143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130786);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139387);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ности.
pub(crate) fn n29113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28919);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 56223);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84145);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 106385);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121515);
									s.is_empty()
								}
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
			super::push(out, 56224);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 84146);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 106386);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121516);
									s.is_empty()
								}
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
			super::push(out, 56225);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84147);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 106387);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121517);
									s.is_empty()
								}
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
			super::push(out, 56226);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84148);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106388);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121518);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 130787);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136211);
													match s {
														[] => true,
														['в', s @ ..] => super::t46::n140828(s, out),
														_ => false,
													}
												}
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
							super::push(out, 106389);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121519);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130788);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 136212);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 56227);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 84149);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106390);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усти.
pub(crate) fn n9910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9883);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 28920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -альфа-лучи.
pub(crate) fn n132148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130791);
	s.is_empty()
}

/// Words ending -ыба-попугай.
pub(crate) fn n137626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136215);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ди-дабудай.
pub(crate) fn n132151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130794);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 136216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кекусинкай.
pub(crate) fn n132152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130795);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136217);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word джансарай.
pub(crate) fn n132153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130796);
	s.is_empty()
}

/// Words ending -аздношатай.
pub(crate) fn n132154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130797);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 136218);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тай-болтай.
pub(crate) fn n132155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130798);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136219);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -антиполицай.
pub(crate) fn n132156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130799);
	s.is_empty()
}

/// Words ending -ната-музей.
pub(crate) fn n132158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130801);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136221);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139391);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 141230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхив-музей.
pub(crate) fn n132159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -парк-музей.
pub(crate) fn n132160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130803);
	s.is_empty()
}

/// Words ending -удно-музей.
pub(crate) fn n132161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130804);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136223);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орец-музей.
pub(crate) fn n132162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130805);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 136224);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 139392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word бич-волей.
pub(crate) fn n132163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130806);
	s.is_empty()
}

/// Words ending --плей.
pub(crate) fn n29412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29218);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 56450);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 84295);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106477);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 121560);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130807);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аплей.
pub(crate) fn n29413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29219);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 56451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84296);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сплей.
pub(crate) fn n29414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29220);
	s.is_empty()
}

/// Words ending -рлей.
pub(crate) fn n10154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10126);
	match s {
		[] => true,
		['а', 'т', s @ ..] => {
			super::push(out, 29221);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 56452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слей.
pub(crate) fn n10155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10127);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29222);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56453);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улей.
pub(crate) fn n10156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10128);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29223);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -элей.
pub(crate) fn n10157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10129);
	s.is_empty()
}

/// The word сельдерей.
pub(crate) fn n132165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130808);
	s.is_empty()
}

/// Words ending -кола-лицей.
pub(crate) fn n132166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130809);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 136225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139393);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -колченогий.
pub(crate) fn n132167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130810);
	s.is_empty()
}

/// Words ending -унбиквадий.
pub(crate) fn n132168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130811);
	s.is_empty()
}

/// Words ending -унунквадий.
pub(crate) fn n132169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130812);
	s.is_empty()
}

/// Words ending -моспоридий.
pub(crate) fn n132170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130813);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136226);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 139394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -балантидий.
pub(crate) fn n132171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130814);
	s.is_empty()
}

/// The word кустодий.
pub(crate) fn n122845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121571);
	s.is_empty()
}

/// Words ending -фодий.
pub(crate) fn n29549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29353);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56582);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56583);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 84367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 106510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдий.
pub(crate) fn n10249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29354);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 56584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29355);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 56585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эдий.
pub(crate) fn n10250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10222);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кривобокий.
pub(crate) fn n132172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130815);
	s.is_empty()
}

/// Words ending -ский.
pub(crate) fn n10266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10238);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29377);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56603);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 84380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 29378);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56604);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 84381);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106515);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121574);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 130816);
													s.is_empty()
												}
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
							super::push(out, 84382);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 106516);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121575);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 130817);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n137638(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84383);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 106517);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121576);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 130818);
													s.is_empty()
												}
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
							super::push(out, 84384);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 106518);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121577);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130819);
													s.is_empty()
												}
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
							super::push(out, 84385);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106519);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121578);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130820);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n137639(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 56605);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 84386);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 106520);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121579);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130821);
													match s {
														[] => true,
														['б', s @ ..] => super::t46::n137640(s, out),
														_ => false,
													}
												}
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
							super::push(out, 84387);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 84388);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 106521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 106522);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 121580);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130822);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n137641(s, out),
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
									super::push(out, 106523);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121581);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 84389);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 84390);
							match s {
								[] => true,
								['д', s @ ..] => super::t46::n107658(s, out),
								['т', s @ ..] => super::t46::n107659(s, out),
								['х', s @ ..] => {
									super::push(out, 106526);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121585);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t46::n85284(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t46::n29575(s, out),
		['е', s @ ..] => super::t46::n29576(s, out),
		['ж', s @ ..] => super::t46::n29577(s, out),
		['з', s @ ..] => super::t46::n29578(s, out),
		['й', s @ ..] => super::t46::n29579(s, out),
		['к', s @ ..] => super::t46::n29580(s, out),
		['м', s @ ..] => super::t46::n29581(s, out),
		['н', s @ ..] => super::t46::n29582(s, out),
		['п', s @ ..] => super::t46::n29583(s, out),
		['р', s @ ..] => super::t46::n29584(s, out),
		['с', s @ ..] => super::t46::n29585(s, out),
		['т', s @ ..] => super::t46::n29586(s, out),
		['у', s @ ..] => super::t46::n29587(s, out),
		['х', s @ ..] => super::t46::n29588(s, out),
		['ч', s @ ..] => super::t46::n29589(s, out),
		['ш', s @ ..] => super::t46::n29590(s, out),
		['ь', s @ ..] => super::t46::n29591(s, out),
		_ => false,
	}
}

/// Words ending --кузнецкий.
pub(crate) fn n132230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130873);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136257);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 139408);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мельницкий.
pub(crate) fn n132231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130874);
	s.is_empty()
}

/// Words ending -яцкий.
pub(crate) fn n29594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29398);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 56656);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 84494);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 106624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ький.
pub(crate) fn n10268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10240);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 29399);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 56657);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 84495);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106625);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121660);
											s.is_empty()
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
								['и', s @ ..] => {
									super::push(out, 106626);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 121661);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130875);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 106627);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 121662);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 130876);
													match s {
														[] => true,
														['е', s @ ..] => super::t46::n137669(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', '\u{1}', s @ ..] => {
											super::push(out, 121663);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 84496);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 106628);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 84497);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 106629);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121665);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 84498);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 106630);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121666);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 106631);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 121667);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 130877);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29400);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56658);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 84499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -толий.
pub(crate) fn n29624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29428);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56685);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 84512);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121670);
									s.is_empty()
								}
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
			super::push(out, 56686);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 84513);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фолий.
pub(crate) fn n29625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29429);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 56687);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 84514);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улий.
pub(crate) fn n10276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10248);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 29430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юлий.
pub(crate) fn n10277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10249);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ногородний.
pub(crate) fn n132235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130878);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 136259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нний.
pub(crate) fn n10297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10269);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29465);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 56726);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 84536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29466);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 56727);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84537);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 106652);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121673);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130879);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 29467);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 56728);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 84538);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 106653);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 121674);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оний.
pub(crate) fn n10298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10270);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29468);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29469);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 29470);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 29471);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56730);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84539);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106654);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121675);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130880);
													match s {
														[] => true,
														['е', s @ ..] => super::t46::n137671(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29472);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 56731);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84540);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121676);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'е', s @ ..] => {
					super::push(out, 56732);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 84541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106656);
									s.is_empty()
								}
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
			super::push(out, 29473);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 56733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84542);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 56734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 29474);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 29475);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 56735);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рний.
pub(crate) fn n10299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10271);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29476);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 56736);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тний.
pub(crate) fn n10300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10272);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29477);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 56737);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84544);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 106658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уний.
pub(crate) fn n10301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10273);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29478);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 56738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29479);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 56739);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84545);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106659);
									s.is_empty()
								}
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
			super::push(out, 29480);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 56740);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84546);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106660);
									s.is_empty()
								}
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
			super::push(out, 29481);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 56741);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84547);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 106661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фний.
pub(crate) fn n10302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10274);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29482);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 56742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шний.
pub(crate) fn n10303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10275);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29483);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56743);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84548);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 106662);
									s.is_empty()
								}
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
			super::push(out, 29484);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 56744);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 84549);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 106663);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121677);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29485);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 56745);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84550);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 106664);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121678);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -виночерпий.
pub(crate) fn n132238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130881);
	s.is_empty()
}

/// Words ending -бендарий.
pub(crate) fn n107808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106674);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 121682);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 130882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зарий.
pub(crate) fn n29699(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 56764);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 84568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иарий.
pub(crate) fn n29700(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 56765);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 56766);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 84569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 106675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 56767);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 84570);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106676);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121683);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130883);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 56768);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84571);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'б', '\u{1}', s @ ..] => {
					super::push(out, 84572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 56769);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 84573);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 106678);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 106679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 84574);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -карий.
pub(crate) fn n29701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56770);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 84575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56771);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 56772);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84576);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 106681);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 106682);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121685);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 130884);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 56773);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 56774);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84577);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 106683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 56775);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -марий.
pub(crate) fn n29703(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 56776);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 56777);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 84578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нарий.
pub(crate) fn n29704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29502);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84579);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106684);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 84580);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 106685);
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
				['л', s @ ..] => {
					super::push(out, 84581);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106686);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121687);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130885);
											s.is_empty()
										}
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
					super::push(out, 84582);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 106687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 56778);
			match s {
				[] => true,
				['л', 'л', 'о', 'п', 'а', s @ ..] => {
					super::push(out, 84583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 106688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 84584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 56779);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84585);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106689);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121688);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130886);
											s.is_empty()
										}
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
					super::push(out, 84586);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 106690);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121689);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130887);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 106691);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121690);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130888);
											s.is_empty()
										}
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
					super::push(out, 84587);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 84588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оарий.
pub(crate) fn n29705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29503);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 56780);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 84589);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 106692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рарий.
pub(crate) fn n29706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29504);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 56781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 56782);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 84591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56783);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 84592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 56784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сарий.
pub(crate) fn n29707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29505);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 56785);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тарий.
pub(crate) fn n29708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29506);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56786);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 84593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106693);
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
				['л', s @ ..] => {
					super::push(out, 84594);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106694);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121691);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130889);
											s.is_empty()
										}
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
					super::push(out, 84595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106695);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121692);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130890);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 56787);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 84596);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 106696);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121693);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 130891);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 106697);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121694);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130892);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 136261);
													match s {
														[] => true,
														['-', s @ ..] => super::t46::n140851(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 136262);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 84597);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 84598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 56788);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 56789);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106700);
							s.is_empty()
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121695);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130893);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136263);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n140852(s, out),
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
									super::push(out, 121696);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130894);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136264);
													match s {
														[] => true,
														['м', s @ ..] => super::t46::n140853(s, out),
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
									super::push(out, 121697);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130895);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136265);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 121698);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130896);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 136266);
													match s {
														[] => true,
														['с', s @ ..] => super::t46::n140854(s, out),
														_ => false,
													}
												}
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
							super::push(out, 106701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 56790);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 84600);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 84601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 56791);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84602);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 106702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уарий.
pub(crate) fn n29709(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 56792);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 84603);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 56793);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 84604);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106704);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 106705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 84605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фарий.
pub(crate) fn n29710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29507);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56794);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 84606);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 106706);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -харий.
pub(crate) fn n29711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29508);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56795);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 84607);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -брий.
pub(crate) fn n10311(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 29509);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 56796);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 84608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29510);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 56797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дрий.
pub(crate) fn n10312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10283);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 29511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерий.
pub(crate) fn n10313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10284);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29512);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 29513);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 56799);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84610);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106707);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121700);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130897);
													s.is_empty()
												}
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
							super::push(out, 84611);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106708);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121701);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56800);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 84612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29515);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56801);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 84613);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 56802);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 84614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29516);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 56803);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 84615);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 106710);
									s.is_empty()
								}
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
			super::push(out, 29517);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 56804);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 84616);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106711);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121702);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130898);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29518);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56805);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 56806);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 84617);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106712);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 121703);
											s.is_empty()
										}
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
					super::push(out, 56807);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 84618);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 84619);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 106714);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121704);
											s.is_empty()
										}
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
					super::push(out, 56808);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84620);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 106715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 84621);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 106716);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121705);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130899);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 56809);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84623);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 106717);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 121706);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 84624);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 106718);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121707);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 84625);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 106719);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121708);
											s.is_empty()
										}
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
					super::push(out, 56810);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 84626);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 84627);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 84628);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 84629);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106720);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121709);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130900);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'н', s @ ..] => {
							super::push(out, 84630);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106721);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121710);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 130901);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t46::n57392(s, out),
				['с', s @ ..] => super::t46::n57393(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t46::n29722(s, out),
		['ч', s @ ..] => super::t46::n29723(s, out),
		_ => false,
	}
}

/// Words ending -ирий.
pub(crate) fn n10314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10285);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29521);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 56817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29522);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56818);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 84638);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106730);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121713);
											s.is_empty()
										}
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
					super::push(out, 56819);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 84639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 29523);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 56820);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84640);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орий.
pub(crate) fn n10315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10286);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29524);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 29525);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 56821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 84641);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 106732);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121714);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29526);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 29527);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 56822);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 84642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106733);
									s.is_empty()
								}
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
			super::push(out, 29528);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56823);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 84643);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 106734);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121715);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130903);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 56824);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29529);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 29530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трий.
pub(crate) fn n10316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10287);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 56825);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 84645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 56826);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 84646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29531);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29532);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 56828);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84648);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 56829);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 84649);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 84650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29533);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 56830);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84651);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106736);
									s.is_empty()
								}
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
			super::push(out, 29534);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 56831);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урий.
pub(crate) fn n10317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10288);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 56832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 29536);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 29537);
			match s {
				[] => true,
				['р', 'е', 'м', '\u{1}', s @ ..] => {
					super::push(out, 56833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29538);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 29539);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 56834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрий.
pub(crate) fn n10318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10289);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 29540);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 56835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрий.
pub(crate) fn n10319(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29541);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 29542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярий.
pub(crate) fn n10320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10290);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29543);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56836);
					s.is_empty()
				}
				['у', 'к', s @ ..] => {
					super::push(out, 56837);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 84652);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106737);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121716);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 130904);
													match s {
														[] => true,
														['к', s @ ..] => super::t46::n137678(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унбипентий.
pub(crate) fn n132262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130905);
	s.is_empty()
}

/// Words ending -унунпентий.
pub(crate) fn n132263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130906);
	s.is_empty()
}

/// Words ending -констанций.
pub(crate) fn n132264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130907);
	s.is_empty()
}

/// Words ending -городничий.
pub(crate) fn n132265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130908);
	s.is_empty()
}

/// Words ending -ясельничий.
pub(crate) fn n132266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130909);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -страдавший.
pub(crate) fn n132267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130910);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136269);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word сослужащий.
pub(crate) fn n132268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130911);
	s.is_empty()
}

/// Words ending -вездесущий.
pub(crate) fn n132269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130912);
	s.is_empty()
}

/// Words ending -ведывающий.
pub(crate) fn n132270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130913);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136270);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 139415);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -восстающий.
pub(crate) fn n132271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130914);
	s.is_empty()
}

/// The word удушающий.
pub(crate) fn n132272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130915);
	s.is_empty()
}

/// Words ending -вующий.
pub(crate) fn n57494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56912);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 84702);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 106772);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 121737);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 130916);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 121738);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 130917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 121739);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 130918);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136271);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 139416);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n142688(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дующий.
pub(crate) fn n57495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56913);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 84703);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 106773);
					match s {
						[] => true,
						['м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130919);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 130920);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136272);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139417);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кующий.
pub(crate) fn n57496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56914);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 84704);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 106774);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рующий.
pub(crate) fn n57497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56915);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 84705);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 106775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word танцующий.
pub(crate) fn n57498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56916);
	s.is_empty()
}

/// Words ending -яющий.
pub(crate) fn n29819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29612);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 56917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84706);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106776);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящий.
pub(crate) fn n10374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10344);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56918);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 84707);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 106777);
									s.is_empty()
								}
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
			super::push(out, 29614);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 29615);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56919);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84708);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106778);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 121742);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 130921);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29616);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29617);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56920);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 56921);
					match s {
						[] => true,
						['о', 'м', 'с', 'д', 'е', 'р', 'е', 'п', 'в', '\u{1}', s @ ..] => {
							super::push(out, 84709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рана-изгой.
pub(crate) fn n132279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130922);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 136273);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 139418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ство-изгой.
pub(crate) fn n132280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130923);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 136274);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остромской.
pub(crate) fn n132281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130924);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -четвертной.
pub(crate) fn n132282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130925);
	s.is_empty()
}

/// Words ending -ород-герой.
pub(crate) fn n132283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130926);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 136276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онер-герой.
pub(crate) fn n132284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130927);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 136277);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 139420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ость-герой.
pub(crate) fn n132285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130928);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136278);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139421);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урачниковый.
pub(crate) fn n137692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136281);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 139423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пролесковый.
pub(crate) fn n137693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136282);
	s.is_empty()
}

/// Words ending -лопальтовый.
pub(crate) fn n137694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136283);
	s.is_empty()
}

/// Words ending -кабельтовый.
pub(crate) fn n137695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136284);
	s.is_empty()
}

/// Words ending -дюливаемый.
pub(crate) fn n132295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130938);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 136285);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -испытуемый.
pub(crate) fn n132296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130939);
	s.is_empty()
}

/// Words ending -равославный.
pub(crate) fn n132297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130940);
	s.is_empty()
}

/// The word праведный.
pub(crate) fn n132298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130941);
	s.is_empty()
}

/// Words ending -лгопрудный.
pub(crate) fn n132299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130942);
	s.is_empty()
}

/// Words ending -южный.
pub(crate) fn n30034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29827);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 57109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжный.
pub(crate) fn n30035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29828);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 57110);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 84849);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зный.
pub(crate) fn n10520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10490);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29829);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 29830);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 29831);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29832);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57111);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 84850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йный.
pub(crate) fn n10521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10491);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29833);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 57112);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84851);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106859);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121785);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 130943);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мный.
pub(crate) fn n10522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10492);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29835);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нный.
pub(crate) fn n10523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10493);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29836);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 57113);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84852);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 106860);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 106861);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121786);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130944);
													match s {
														[] => true,
														['с', s @ ..] => super::t46::n137697(s, out),
														_ => false,
													}
												}
												['ц', s @ ..] => {
													super::push(out, 130945);
													match s {
														[] => true,
														['и', s @ ..] => super::t46::n137698(s, out),
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
									super::push(out, 106862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 57114);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 84853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 57115);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 57116);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 57117);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 84854);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106863);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121787);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130946);
													match s {
														[] => true,
														['г', s @ ..] => super::t46::n137699(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29837);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 57119);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 84855);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 106864);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 121788);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130947);
													match s {
														[] => true,
														['в', s @ ..] => super::t46::n137700(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57120);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 84856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 57121);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84857);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 57122);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 84858);
							match s {
								[] => true,
								['а', 'л', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 106866);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 106867);
									match s {
										[] => true,
										['л', s @ ..] => super::t46::n123064(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => super::t46::n85758(s, out),
						['г', s @ ..] => super::t46::n85759(s, out),
						['е', s @ ..] => super::t46::n85760(s, out),
						['о', s @ ..] => super::t46::n85761(s, out),
						['п', s @ ..] => super::t46::n85762(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 57123);
					s.is_empty()
				}
				['о', s @ ..] => super::t46::n57707(s, out),
				['р', s @ ..] => super::t46::n57708(s, out),
				['т', s @ ..] => super::t46::n57709(s, out),
				['ч', s @ ..] => super::t46::n57710(s, out),
				['ш', s @ ..] => super::t46::n57711(s, out),
				['щ', s @ ..] => super::t46::n57712(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t46::n30045(s, out),
		['о', s @ ..] => super::t46::n30046(s, out),
		['ы', s @ ..] => super::t46::n30047(s, out),
		['я', s @ ..] => super::t46::n30048(s, out),
		_ => false,
	}
}

/// Words ending -рный.
pub(crate) fn n10524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10494);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29842);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 57134);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84877);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 106885);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121803);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 130959);
													match s {
														[] => true,
														['б', s @ ..] => super::t46::n137709(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57135);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84878);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106886);
									s.is_empty()
								}
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
			super::push(out, 29843);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57136);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84879);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 106887);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121804);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 130960);
													match s {
														[] => true,
														['о', s @ ..] => super::t46::n137710(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57137);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84880);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 57138);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84881);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106889);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121805);
											s.is_empty()
										}
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
					super::push(out, 57139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29844);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 57140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29845);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57141);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 84883);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106890);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121806);
											s.is_empty()
										}
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
					super::push(out, 57142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84884);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106891);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121807);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130961);
													match s {
														[] => true,
														['ы', s @ ..] => super::t46::n137711(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57143);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 84885);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 106892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 57144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 57145);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 84887);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 106893);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121808);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29846);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 29847);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 57146);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84888);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106894);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121809);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130962);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сный.
pub(crate) fn n10525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10495);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29848);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 57147);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106895);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121810);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121811);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 130963);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 121812);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130964);
													match s {
														[] => true,
														['п', s @ ..] => super::t46::n137712(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57148);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 84889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29849);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тный.
pub(crate) fn n10526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10496);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29850);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 57149);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 84890);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106896);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 121813);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29851);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 57150);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 84891);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106897);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121814);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29852);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 57151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29853);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29854);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 29855);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					super::push(out, 57154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84892);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 106898);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121815);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130965);
													match s {
														[] => true,
														['б', s @ ..] => super::t46::n137713(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чный.
pub(crate) fn n10527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10497);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29856);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57155);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 84893);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106899);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121816);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29857);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 57156);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84894);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 106900);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121817);
											s.is_empty()
										}
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
					super::push(out, 57157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84895);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 106901);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 84896);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 106902);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121819);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29858);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 57158);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 84897);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106903);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 121820);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 84898);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106904);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121821);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 130966);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t46::n137714(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'б', 'а', 'ф', '\u{1}', s @ ..] => {
					super::push(out, 57159);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 57160);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 84899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29859);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 57161);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84900);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 106905);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121822);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130967);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 57162);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84901);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106906);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121823);
											s.is_empty()
										}
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
					super::push(out, 57163);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84902);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 106907);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121824);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 84903);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 106908);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 121825);
											s.is_empty()
										}
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
					super::push(out, 57164);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84904);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106909);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121826);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 106910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 57165);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84905);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 106911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 84906);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106912);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 106913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 84907);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106914);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121827);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130969);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29860);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57166);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 84908);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шный.
pub(crate) fn n10528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10498);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 29861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьный.
pub(crate) fn n10529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10499);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29862);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57167);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 84909);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 84910);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106916);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121828);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130970);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							super::push(out, 84911);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 106917);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121829);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 130971);
													match s {
														[] => true,
														['-', s @ ..] => super::t46::n137715(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57168);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 84912);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106918);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121830);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 130972);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 57169);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 84913);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 106919);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 121831);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130973);
													match s {
														[] => true,
														['е', s @ ..] => super::t46::n137716(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57170);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 57171);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 84914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яный.
pub(crate) fn n10530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10500);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 29863);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 57172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шестисотый.
pub(crate) fn n132331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130974);
	s.is_empty()
}

/// Words ending -тье-пиджак.
pub(crate) fn n132332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130975);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136305);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139436);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каракалпак.
pub(crate) fn n132333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130976);
	s.is_empty()
}

/// Words ending -получерпак.
pub(crate) fn n132334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130977);
	s.is_empty()
}

/// Words ending -ов-призрак.
pub(crate) fn n132335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130978);
	s.is_empty()
}

/// Words ending -од-призрак.
pub(crate) fn n132336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130979);
	s.is_empty()
}

/// Words ending -полусумрак.
pub(crate) fn n132337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130980);
	s.is_empty()
}

/// Words ending -е-фрак.
pub(crate) fn n57935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57351);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 85006);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 106960);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121848);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 130981);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136306);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нс-человек.
pub(crate) fn n132340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130983);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136307);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139437);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -редчеловек.
pub(crate) fn n132341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130984);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136308);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -всечеловек.
pub(crate) fn n132342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130985);
	s.is_empty()
}

/// Words ending -рвочеловек.
pub(crate) fn n132343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130986);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136309);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -едочеловек.
pub(crate) fn n132344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130987);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яночеловек.
pub(crate) fn n132345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130988);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 136311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олучеловек.
pub(crate) fn n132346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130989);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -коростелек.
pub(crate) fn n132347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130990);
	s.is_empty()
}

/// Words ending -тралопитек.
pub(crate) fn n132348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130991);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136313);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 139438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тревел-чек.
pub(crate) fn n132349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130992);
	s.is_empty()
}

/// The word мотылечек.
pub(crate) fn n132350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130993);
	s.is_empty()
}

/// The word паренечек.
pub(crate) fn n132351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130994);
	s.is_empty()
}

/// Words ending -ичек.
pub(crate) fn n10856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10824);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30332);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57557);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85118);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107008);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 121868);
											s.is_empty()
										}
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
					super::push(out, 57558);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85119);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107009);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121869);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130995);
													match s {
														[] => true,
														['д', s @ ..] => super::t46::n137726(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30333);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 30334);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57559);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 57560);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 107010);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 121870);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 130996);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t46::n137727(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => super::t46::n108148(s, out),
								['н', s @ ..] => super::t46::n108149(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t46::n58145(s, out),
				['е', s @ ..] => super::t46::n58146(s, out),
				['ж', 'о', s @ ..] => super::t46::n58147(s, out),
				['з', s @ ..] => {
					super::push(out, 57564);
					s.is_empty()
				}
				['й', s @ ..] => super::t46::n58149(s, out),
				['м', s @ ..] => {
					super::push(out, 57566);
					s.is_empty()
				}
				['н', s @ ..] => super::t46::n58151(s, out),
				['п', s @ ..] => super::t46::n58152(s, out),
				['р', s @ ..] => super::t46::n58153(s, out),
				['с', s @ ..] => super::t46::n58154(s, out),
				['т', s @ ..] => super::t46::n58155(s, out),
				['ч', s @ ..] => super::t46::n58156(s, out),
				['ш', s @ ..] => super::t46::n58157(s, out),
				['щ', s @ ..] => super::t46::n58158(s, out),
				['ь', s @ ..] => super::t46::n58159(s, out),
				['я', s @ ..] => super::t46::n58160(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очек.
pub(crate) fn n10857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10825);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30335);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57577);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 57578);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85145);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107041);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 107042);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 121903);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131026);
													s.is_empty()
												}
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
							super::push(out, 85146);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 107043);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121904);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131027);
													match s {
														[] => true,
														['п', s @ ..] => super::t46::n137746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30336);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57579);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 57580);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 57581);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107044);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 107045);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121905);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 107046);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121906);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30337);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57582);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 85149);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107047);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121907);
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
									super::push(out, 107048);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 107049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 57583);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85151);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107050);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 121908);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131028);
													match s {
														[] => true,
														['н', s @ ..] => super::t46::n137747(s, out),
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
									super::push(out, 107051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 57584);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 85152);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 85153);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 85154);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107053);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121909);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131029);
													s.is_empty()
												}
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
							super::push(out, 85155);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107054);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121910);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 131030);
													match s {
														[] => true,
														['о', s @ ..] => super::t46::n137748(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['г', s @ ..] => {
											super::push(out, 121911);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 121912);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131031);
													match s {
														[] => true,
														['с', s @ ..] => super::t46::n137749(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 85156);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 85157);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107055);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 121913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'с', s @ ..] => super::t46::n86064(s, out),
						_ => false,
					}
				}
				['я', 'р', 'о', s @ ..] => super::t46::n58171(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t46::n30549(s, out),
		['з', s @ ..] => super::t46::n30550(s, out),
		['л', s @ ..] => super::t46::n30551(s, out),
		['м', s @ ..] => super::t46::n30552(s, out),
		['н', s @ ..] => super::t46::n30553(s, out),
		['п', s @ ..] => super::t46::n30554(s, out),
		['р', s @ ..] => super::t46::n30555(s, out),
		['с', s @ ..] => super::t46::n30556(s, out),
		['т', s @ ..] => super::t46::n30557(s, out),
		['ц', s @ ..] => super::t46::n30558(s, out),
		['ч', s @ ..] => super::t46::n30559(s, out),
		['ш', s @ ..] => super::t46::n30560(s, out),
		_ => false,
	}
}

/// Words ending -робушек.
pub(crate) fn n86170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85264);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 107144);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 121972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вушек.
pub(crate) fn n30579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30368);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 57651);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85265);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107145);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -душек.
pub(crate) fn n30580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30369);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57652);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мушек.
pub(crate) fn n30581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30370);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57653);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 85267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 57654);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нушек.
pub(crate) fn n30582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30371);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 57655);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пушек.
pub(crate) fn n30583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30372);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 57656);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85269);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рушек.
pub(crate) fn n30584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30373);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 57657);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышек.
pub(crate) fn n10866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10834);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30374);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57658);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85271);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107147);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121974);
											s.is_empty()
										}
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
					super::push(out, 57659);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85272);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107148);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121975);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30375);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 30376);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57660);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 85273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30377);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57661);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 85274);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107149);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121976);
											s.is_empty()
										}
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
					super::push(out, 57662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30378);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 30379);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30380);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word эшек.
pub(crate) fn n30592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30381);
	s.is_empty()
}

/// Words ending -ехрублевик.
pub(crate) fn n132417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131060);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 136352);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 139450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улевик.
pub(crate) fn n58282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57692);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 85294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -невик.
pub(crate) fn n30621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30407);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 57693);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85295);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 57694);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 57695);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85297);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 57696);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57697);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85299);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 57698);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85300);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107162);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 121981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 85301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оевик.
pub(crate) fn n30622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30408);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 57699);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85302);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107164);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121982);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131061);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 57700);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85303);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ревик.
pub(crate) fn n30623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30409);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57701);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 85304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57702);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 85305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тевик.
pub(crate) fn n30624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30410);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57703);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цевик.
pub(crate) fn n30625(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 57704);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85307);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 57705);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чевик.
pub(crate) fn n30626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30411);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57706);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 57707);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 57708);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 57709);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шевик.
pub(crate) fn n30627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30412);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 57710);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 57711);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85310);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107168);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121983);
									s.is_empty()
								}
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
			super::push(out, 57712);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85311);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 85312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щевик.
pub(crate) fn n30628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30413);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57713);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 85313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57714);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57715);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 57716);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85316);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивик.
pub(crate) fn n10886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10854);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 30414);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57717);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85317);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овик.
pub(crate) fn n10887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10855);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30415);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57718);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85318);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 57719);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85319);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 107174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 57720);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 85320);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 107175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 57721);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 85321);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107176);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121984);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 30417);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57723);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85322);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 57724);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85323);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 107178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 85324);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107179);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121985);
											s.is_empty()
										}
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
						['л', s @ ..] => {
							super::push(out, 85325);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107180);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 57725);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85328);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107183);
									s.is_empty()
								}
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
			super::push(out, 30418);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57726);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85329);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 85330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85331);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107185);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121987);
											s.is_empty()
										}
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
					super::push(out, 57727);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85332);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 107186);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121988);
											s.is_empty()
										}
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
					super::push(out, 57728);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 57729);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 57730);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 85334);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 85335);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85336);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 57731);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107189);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 107190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 57732);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 85338);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85339);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 107192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 57733);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107193);
									s.is_empty()
								}
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
			super::push(out, 30419);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 30420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57735);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 85342);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 85343);
							s.is_empty()
						}
						['р', s @ ..] => super::t46::n86250(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t46::n58327(s, out),
				['и', s @ ..] => {
					super::push(out, 57737);
					match s {
						[] => true,
						['н', s @ ..] => super::t46::n86252(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t46::n58329(s, out),
				['ю', s @ ..] => super::t46::n58330(s, out),
				['я', s @ ..] => super::t46::n58331(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t46::n30636(s, out),
		['л', s @ ..] => super::t46::n30637(s, out),
		['м', s @ ..] => super::t46::n30638(s, out),
		['н', s @ ..] => super::t46::n30639(s, out),
		['п', s @ ..] => super::t46::n30640(s, out),
		['р', s @ ..] => super::t46::n30641(s, out),
		['с', s @ ..] => super::t46::n30642(s, out),
		['т', s @ ..] => super::t46::n30643(s, out),
		['х', s @ ..] => super::t46::n30644(s, out),
		_ => false,
	}
}

/// Words ending -свик.
pub(crate) fn n10888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10856);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 30430);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 57800);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85420);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107248);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 122015);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131076);
													match s {
														[] => true,
														['р', s @ ..] => super::t46::n137776(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ювик.
pub(crate) fn n10889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10857);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30431);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 57801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явик.
pub(crate) fn n10890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10858);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 30432);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 57802);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 85422);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107249);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122016);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -персонажик.
pub(crate) fn n132435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131078);
	s.is_empty()
}

/// Words ending -опа-козлик.
pub(crate) fn n132436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131079);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136365);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ереклик.
pub(crate) fn n86367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85460);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 107265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -склик.
pub(crate) fn n30757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30539);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 57879);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 57880);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тклик.
pub(crate) fn n30758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30540);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 57881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыклик.
pub(crate) fn n30759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30541);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 57882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллик.
pub(crate) fn n10938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30542);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 57883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30543);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 57884);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 85462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млик.
pub(crate) fn n10939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10906);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 30544);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 57885);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олик.
pub(crate) fn n10940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10907);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57886);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85463);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107266);
									s.is_empty()
								}
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
			super::push(out, 30546);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 57887);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 30547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 57888);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 57889);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 85465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 30548);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57890);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85466);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107267);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122025);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30549);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57891);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 57892);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 85468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30551);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 30552);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 57894);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 57895);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85469);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 107268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 57896);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 57897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30553);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57898);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 85471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 57899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 30554);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 57900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлик.
pub(crate) fn n10941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10908);
	match s {
		[] => true,
		['а', 'к', 'б', s @ ..] => {
			super::push(out, 30555);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 57901);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30556);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 57902);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 85474);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 57903);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30557);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 57904);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 57905);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85477);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слик.
pub(crate) fn n10942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10909);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30558);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 30559);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 57906);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлик.
pub(crate) fn n10943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10910);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 30560);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 57907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улик.
pub(crate) fn n10944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10911);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 30561);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 30562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 57908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30563);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30564);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57909);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 85478);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107271);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122026);
											s.is_empty()
										}
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
					super::push(out, 57910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флик.
pub(crate) fn n10945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10912);
	s.is_empty()
}

/// Words ending -хлик.
pub(crate) fn n10946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10913);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30565);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 57911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30566);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 57912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылик.
pub(crate) fn n10947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10914);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 30567);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 57913);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юлик.
pub(crate) fn n10948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10915);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 30568);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57914);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялик.
pub(crate) fn n10949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10916);
	s.is_empty()
}

/// Words ending -аэродинамик.
pub(crate) fn n132437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131080);
	s.is_empty()
}

/// Words ending -бец-академик.
pub(crate) fn n132438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131081);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136366);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139460);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 141260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -воин-химик.
pub(crate) fn n132439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131082);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ик-органик.
pub(crate) fn n132440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131083);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136368);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139461);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 141261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-механик.
pub(crate) fn n123307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122032);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 131084);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 136369);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 139462);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141262);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-механик.
pub(crate) fn n123308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122033);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 131085);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 136370);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 139463);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 141263);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь-механик.
pub(crate) fn n123309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122034);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 131086);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 136371);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 139464);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 141264);
									s.is_empty()
								}
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
			super::push(out, 131087);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 136372);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 139465);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чаник.
pub(crate) fn n30812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30593);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 57939);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85494);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шаник.
pub(crate) fn n30813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30594);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 57940);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 57941);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85495);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ощаник.
pub(crate) fn n58534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57942);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 85496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бник.
pub(crate) fn n10961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10928);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30596);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57943);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 57944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 57945);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 85498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 57946);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30597);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 57947);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 57948);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85500);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107284);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122036);
											s.is_empty()
										}
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
					super::push(out, 57949);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85501);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 85502);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107285);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122037);
											s.is_empty()
										}
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
					super::push(out, 57950);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 85503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 57951);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85504);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107286);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122038);
											s.is_empty()
										}
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
					super::push(out, 57952);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85506);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 85507);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107287);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 122039);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30598);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57953);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 85508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30599);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57954);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 85509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30600);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 57955);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85510);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 107288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 57956);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 85511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 57957);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85512);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 57958);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57959);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 57960);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 85514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30602);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 57961);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85515);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 57962);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 85516);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107291);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122040);
											s.is_empty()
										}
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
					super::push(out, 57963);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 57964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85518);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107292);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 85519);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107293);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122042);
											s.is_empty()
										}
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
					super::push(out, 57965);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 85520);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122043);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', 'р', '\u{1}', s @ ..] => {
			super::push(out, 30603);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 30604);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 57966);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 85521);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вник.
pub(crate) fn n10962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10929);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30605);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57967);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85522);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 57968);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85523);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107297);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122044);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 131088);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 57969);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85524);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107298);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122045);
											s.is_empty()
										}
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
					super::push(out, 57970);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85525);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107299);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122046);
											s.is_empty()
										}
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
					super::push(out, 57971);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 85526);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107300);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 122047);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131089);
													match s {
														[] => true,
														['р', s @ ..] => super::t46::n137785(s, out),
														_ => false,
													}
												}
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
							super::push(out, 85527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 57972);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85528);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 57973);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 85529);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107302);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 85530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 57974);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107303);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 107304);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122049);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131090);
													match s {
														[] => true,
														['р', s @ ..] => super::t46::n137786(s, out),
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
									super::push(out, 107305);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122050);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131091);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 107306);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122051);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30606);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 57975);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 85532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 57976);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85533);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 57977);
					s.is_empty()
				}
				['н', s @ ..] => super::t46::n58571(s, out),
				['ч', s @ ..] => super::t46::n58572(s, out),
				['ш', s @ ..] => {
					super::push(out, 57980);
					match s {
						[] => true,
						['п', s @ ..] => super::t46::n86448(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t46::n30826(s, out),
		['л', s @ ..] => super::t46::n30827(s, out),
		['о', s @ ..] => super::t46::n30828(s, out),
		['р', s @ ..] => super::t46::n30829(s, out),
		['ы', s @ ..] => super::t46::n30830(s, out),
		['я', s @ ..] => super::t46::n30831(s, out),
		_ => false,
	}
}

/// Words ending -гник.
pub(crate) fn n10963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10930);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30613);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 58004);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дник.
pub(crate) fn n10964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10931);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30614);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58005);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 85596);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107372);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122096);
											s.is_empty()
										}
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
					super::push(out, 58006);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 58007);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85597);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 107373);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122097);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 85598);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 85599);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 85600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58008);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 85601);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107374);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122098);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131109);
													match s {
														[] => true,
														['-', s @ ..] => super::t46::n137791(s, out),
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
									super::push(out, 107375);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 107376);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 85602);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107377);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85603);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58009);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85604);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58010);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85605);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 107380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 85606);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107381);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122101);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131110);
													match s {
														[] => true,
														['в', s @ ..] => super::t46::n137792(s, out),
														_ => false,
													}
												}
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
							super::push(out, 85607);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58011);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 85608);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 85609);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107383);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122102);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131111);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 107384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85610);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 85611);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107386);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122103);
											s.is_empty()
										}
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
					super::push(out, 58012);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85612);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t46::n30834(s, out),
		['з', s @ ..] => {
			super::push(out, 30616);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58020);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85629);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 58021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85630);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 107404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 85631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 58022);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85632);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107405);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122117);
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t46::n132479(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t46::n30836(s, out),
		['л', s @ ..] => super::t46::n30837(s, out),
		['н', s @ ..] => super::t46::n30838(s, out),
		['о', s @ ..] => super::t46::n30839(s, out),
		['р', s @ ..] => super::t46::n30840(s, out),
		['у', s @ ..] => super::t46::n30841(s, out),
		['ы', s @ ..] => super::t46::n30842(s, out),
		['ю', s @ ..] => super::t46::n30843(s, out),
		['я', s @ ..] => super::t46::n30844(s, out),
		_ => false,
	}
}

/// Words ending -еник.
pub(crate) fn n10965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10932);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30626);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 30627);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58047);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 85679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 30628);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 58048);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85680);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 107452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 58049);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85681);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 107453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 58050);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85682);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107454);
									s.is_empty()
								}
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
			super::push(out, 30629);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58051);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 85683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58052);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85684);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 107455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 58053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 30630);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58054);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85685);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107456);
									s.is_empty()
								}
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
			super::push(out, 30631);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58055);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 85686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 58056);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85687);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107457);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122150);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131147);
													match s {
														[] => true,
														['с', 'е', 'б', '\u{1}', s @ ..] => super::t46::n137818(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 122151);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131148);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 131149);
													match s {
														[] => true,
														['е', s @ ..] => super::t46::n137819(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58057);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58058);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85689);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 107458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 58059);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85690);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107459);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122152);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 131150);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30632);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 58060);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 30633);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85692);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 85693);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85694);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85695);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 85696);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 107462);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жник.
pub(crate) fn n10966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10933);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30634);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85697);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 107463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58063);
					s.is_empty()
				}
				['л', 'в', s @ ..] => {
					super::push(out, 58064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58065);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85699);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 107464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58066);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 85700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58067);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 85701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58068);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85702);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107465);
									match s {
										[] => true,
										['м', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131151);
													match s {
														[] => true,
														['з', s @ ..] => super::t46::n137820(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 131152);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 85703);
							match s {
								[] => true,
								['и', 'т', 'я', 'п', '\u{1}', s @ ..] => {
									super::push(out, 107466);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 107467);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 122154);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131153);
													match s {
														[] => true,
														['н', s @ ..] => super::t46::n137821(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30635);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58069);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 85704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 58070);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85705);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58071);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 85706);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107469);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85707);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58072);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85708);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 107471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 85709);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 85710);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107472);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122156);
											s.is_empty()
										}
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
					super::push(out, 58073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85711);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 85712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t46::n86625(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t46::n58668(s, out),
				['р', s @ ..] => super::t46::n58669(s, out),
				['т', s @ ..] => super::t46::n58670(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t46::n30855(s, out),
		['л', s @ ..] => super::t46::n30856(s, out),
		['о', s @ ..] => super::t46::n30857(s, out),
		['р', s @ ..] => super::t46::n30858(s, out),
		['у', s @ ..] => super::t46::n30859(s, out),
		['ы', s @ ..] => super::t46::n30860(s, out),
		['ю', s @ ..] => {
			super::push(out, 30642);
			s.is_empty()
		}
		['я', s @ ..] => super::t46::n30862(s, out),
		_ => false,
	}
}

/// Words ending -зник.
pub(crate) fn n10967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10934);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30644);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58103);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58104);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85752);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107507);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85753);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 85754);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58105);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 85755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107510);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 107511);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85756);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58106);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 58107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58108);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 85758);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122185);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 122186);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131173);
													match s {
														[] => true,
														['б', s @ ..] => super::t46::n137833(s, out),
														_ => false,
													}
												}
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
							super::push(out, 85759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30645);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58109);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 85760);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58110);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85761);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107515);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122187);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 131174);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58111);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 85763);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58112);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85764);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107518);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 122188);
											s.is_empty()
										}
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
					super::push(out, 58113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85765);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 85766);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 107519);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 85767);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107520);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 122189);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131175);
													s.is_empty()
												}
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
							super::push(out, 85768);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 107521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85769);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 107522);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 107523);
									s.is_empty()
								}
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
			super::push(out, 30646);
			match s {
				[] => true,
				['р', 'д', s @ ..] => {
					super::push(out, 58114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 85770);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107524);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122190);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30647);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58115);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 58116);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 58117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85771);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 85772);
							match s {
								[] => true,
								['т', s @ ..] => super::t46::n108666(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t46::n58713(s, out),
				['х', s @ ..] => {
					super::push(out, 58119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t46::n30867(s, out),
		['у', s @ ..] => super::t46::n30868(s, out),
		['ы', s @ ..] => {
			super::push(out, 30650);
			s.is_empty()
		}
		['ю', s @ ..] => super::t46::n30870(s, out),
		['я', s @ ..] => super::t46::n30871(s, out),
		_ => false,
	}
}

/// Words ending -иник.
pub(crate) fn n10968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10935);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30653);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58134);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85790);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107538);
									s.is_empty()
								}
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
			super::push(out, 30654);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 30655);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30656);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58136);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 58137);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 85791);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107539);
									s.is_empty()
								}
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
			super::push(out, 30657);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 58138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 30658);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 30659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йник.
pub(crate) fn n10969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30660);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85792);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85793);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107541);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122199);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131179);
													match s {
														[] => true,
														['б', s @ ..] => super::t46::n137836(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58139);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58140);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58141);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85795);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 85796);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 58142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85797);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 85798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 58143);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 85799);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85800);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107544);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122200);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131180);
													match s {
														[] => true,
														['п', s @ ..] => super::t46::n137837(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30661);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['о', 'р', 'о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 107545);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 107546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85801);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107547);
									s.is_empty()
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
							super::push(out, 85802);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 85803);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 107548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 85804);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107549);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122201);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 85805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'а', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85806);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 85807);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107551);
									match s {
										[] => true,
										['р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 122202);
											s.is_empty()
										}
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
					super::push(out, 58144);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 85808);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85809);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 85810);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58145);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58146);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85812);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58147);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 85813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58148);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85814);
							match s {
								[] => true,
								['з', s @ ..] => super::t46::n108697(s, out),
								['т', s @ ..] => {
									super::push(out, 107557);
									match s {
										[] => true,
										['с', s @ ..] => super::t46::n123480(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t46::n86728(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t46::n58749(s, out),
				['ч', s @ ..] => super::t46::n58750(s, out),
				['ш', s @ ..] => super::t46::n58751(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t46::n30881(s, out),
		['о', s @ ..] => super::t46::n30882(s, out),
		['у', s @ ..] => super::t46::n30883(s, out),
		_ => false,
	}
}

/// Words ending -кник.
pub(crate) fn n10970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10937);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 30664);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30665);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58166);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 85839);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107576);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122211);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131186);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 107577);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 107578);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 122212);
											s.is_empty()
										}
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
					super::push(out, 58167);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 85840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мник.
pub(crate) fn n10971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10938);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30666);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 58168);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 85841);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58169);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107580);
									s.is_empty()
								}
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
			super::push(out, 30667);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58170);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 85843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 58171);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85844);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107582);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122213);
											s.is_empty()
										}
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
					super::push(out, 58172);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85845);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 107583);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122214);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 85846);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107584);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122215);
											s.is_empty()
										}
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
					super::push(out, 58173);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107585);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 107586);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122216);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 131187);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t46::n137841(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58174);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58175);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 58176);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85847);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107587);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 122217);
											s.is_empty()
										}
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
					super::push(out, 58177);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 85848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30668);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 85849);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122218);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 122219);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131188);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n137842(s, out),
														_ => false,
													}
												}
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
							super::push(out, 85850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 85851);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107588);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122220);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131189);
													s.is_empty()
												}
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
							super::push(out, 85852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 58178);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30669);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30670);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58179);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85854);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 107589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 85855);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107590);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122221);
											s.is_empty()
										}
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
					super::push(out, 58180);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85856);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 85857);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107592);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122222);
											s.is_empty()
										}
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
					super::push(out, 58181);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 85858);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t46::n86773(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t46::n58786(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t46::n30891(s, out),
		_ => false,
	}
}

/// Words ending -нник.
pub(crate) fn n10972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10939);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30672);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85867);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 85868);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107599);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122228);
											s.is_empty()
										}
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
					super::push(out, 58187);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 58188);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85869);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 85870);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107601);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 122230);
											s.is_empty()
										}
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
					super::push(out, 58189);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 85871);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107602);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122231);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 131192);
													match s {
														[] => true,
														['о', s @ ..] => super::t46::n137845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58190);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85872);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 107603);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122232);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131193);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58191);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85873);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 107604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58192);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 85874);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107605);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122233);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131194);
													match s {
														[] => true,
														['к', s @ ..] => super::t46::n137846(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58193);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85875);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 85876);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 107606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 85877);
							match s {
								[] => true,
								['а', 'к', '\u{1}', s @ ..] => {
									super::push(out, 107607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 58194);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 85878);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 107608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 85879);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107609);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t46::n58799(s, out),
				['с', s @ ..] => {
					super::push(out, 58196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 85884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t46::n58801(s, out),
				['ф', s @ ..] => super::t46::n58802(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t46::n30893(s, out),
		['и', s @ ..] => super::t46::n30894(s, out),
		['о', s @ ..] => super::t46::n30895(s, out),
		['у', s @ ..] => super::t46::n30896(s, out),
		['ы', s @ ..] => super::t46::n30897(s, out),
		['я', s @ ..] => super::t46::n30898(s, out),
		_ => false,
	}
}

/// Words ending -оник.
pub(crate) fn n10973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10940);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30679);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58253);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 58254);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 58255);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86019);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'и', s @ ..] => {
			super::push(out, 30680);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58256);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 30681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58257);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 86021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 58258);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 86022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58260);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 58261);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 86025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58263);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 58264);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86027);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 107754);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122354);
											s.is_empty()
										}
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
					super::push(out, 58265);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86028);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 107755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 58266);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30683);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58267);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30684);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58268);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 58269);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86031);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 58270);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 107757);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122355);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131265);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 107758);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122356);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131266);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107759);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 122357);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 131267);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 107760);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122358);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131268);
													s.is_empty()
												}
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
							super::push(out, 86032);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107761);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 122359);
											s.is_empty()
										}
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
					super::push(out, 58271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30685);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58273);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 58274);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86033);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107762);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 122360);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 131269);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t46::n137880(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 58275);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86034);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58276);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86035);
							match s {
								[] => true,
								['д', s @ ..] => super::t46::n108910(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 86036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58277);
					s.is_empty()
				}
				['ь', s @ ..] => super::t46::n58884(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t46::n30908(s, out),
		_ => false,
	}
}

/// Words ending -пник.
pub(crate) fn n10974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10941);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30688);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58282);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 58283);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58284);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 86040);
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
				['л', s @ ..] => {
					super::push(out, 58285);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86041);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107766);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122363);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 131271);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58286);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30689);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 58287);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86043);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107767);
									s.is_empty()
								}
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
			super::push(out, 30690);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30691);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58289);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 58290);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86044);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107768);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122364);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'о', '\u{1}', s @ ..] => {
					super::push(out, 58291);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58292);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86045);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 107769);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122365);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 107770);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122366);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 107771);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122367);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30692);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58293);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86046);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 107772);
									s.is_empty()
								}
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
			super::push(out, 30693);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58294);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58296);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 30694);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рник.
pub(crate) fn n10975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30695);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86050);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 86051);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 107775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86052);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107776);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122368);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122369);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 122370);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 131272);
													match s {
														[] => true,
														['н', s @ ..] => super::t46::n137882(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58299);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86053);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107777);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 122371);
											s.is_empty()
										}
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
					super::push(out, 58300);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86054);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 58301);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86055);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 107779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58302);
					s.is_empty()
				}
				['м', 'о', 'к', 'а', s @ ..] => {
					super::push(out, 58303);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 58304);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86058);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107780);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 107781);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122372);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58306);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86060);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86061);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 86062);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107783);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 107784);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122374);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 107785);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', 'е', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 107786);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122376);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131273);
													match s {
														[] => true,
														['д', s @ ..] => super::t46::n137883(s, out),
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
									super::push(out, 107787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 86063);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107788);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122377);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 86064);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 107789);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 122378);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 86065);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 107790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 58308);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 58309);
					match s {
						[] => true,
						['а', s @ ..] => super::t46::n86985(s, out),
						['у', s @ ..] => super::t46::n86986(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 58310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t46::n30918(s, out),
		['и', s @ ..] => super::t46::n30919(s, out),
		['о', s @ ..] => super::t46::n30920(s, out),
		['у', s @ ..] => super::t46::n30921(s, out),
		['ы', s @ ..] => super::t46::n30922(s, out),
		['я', s @ ..] => super::t46::n30923(s, out),
		_ => false,
	}
}

/// Words ending -сник.
pub(crate) fn n10976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10943);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30702);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58353);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86130);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 58354);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 58355);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58356);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86133);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58357);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86134);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58358);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86135);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 86136);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107859);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 122413);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86137);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107860);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122414);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 131288);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58359);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86138);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107861);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 122415);
											s.is_empty()
										}
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
					super::push(out, 58360);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86139);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107862);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122416);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30703);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58361);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58362);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86141);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86142);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 86143);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107864);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122417);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86144);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 107865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58364);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86145);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122418);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 86146);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107867);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122419);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 131289);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58365);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107868);
									s.is_empty()
								}
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
			super::push(out, 30704);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58366);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86148);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107869);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122420);
											s.is_empty()
										}
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
					super::push(out, 58367);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86149);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 58368);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86150);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 107871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t46::n30927(s, out),
		['р', s @ ..] => {
			super::push(out, 30706);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58375);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 58376);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30707);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58377);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t46::n30930(s, out),
		['ы', s @ ..] => {
			super::push(out, 30709);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 58386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 30710);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58387);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86177);
							match s {
								[] => true,
								['п', s @ ..] => super::t46::n109042(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => super::t46::n30933(s, out),
		['ю', s @ ..] => super::t46::n30934(s, out),
		['я', s @ ..] => super::t46::n30935(s, out),
		_ => false,
	}
}

/// Words ending -тник.
pub(crate) fn n10977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10944);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30714);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58394);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 58395);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86184);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 86185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58396);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86186);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86187);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107899);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122438);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 86188);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 107900);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122439);
											s.is_empty()
										}
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
					super::push(out, 58397);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86189);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 86190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58398);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86191);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 107901);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 107902);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122440);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 107903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 86192);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 86193);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107905);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 86194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 86195);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 107907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', 'у', 'б', 'д', s @ ..] => {
							super::push(out, 86196);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107908);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122442);
											s.is_empty()
										}
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
					super::push(out, 58399);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86197);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 107909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 86198);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 58400);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86199);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 107911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 86200);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 107912);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 122443);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131300);
													match s {
														[] => true,
														['с', s @ ..] => super::t46::n137891(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58401);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 86201);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 86202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86203);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 107913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 86204);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 107914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 58402);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86205);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 107915);
									match s {
										[] => true,
										['б', s @ ..] => super::t46::n123721(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => super::t46::n59014(s, out),
				['ч', s @ ..] => super::t46::n59015(s, out),
				['ш', s @ ..] => super::t46::n59016(s, out),
				['щ', s @ ..] => super::t46::n59017(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t47::n30937(s, out),
		['и', s @ ..] => super::t47::n30938(s, out),
		['к', s @ ..] => super::t47::n30939(s, out),
		['н', s @ ..] => super::t47::n30940(s, out),
		['о', s @ ..] => super::t47::n30941(s, out),
		['р', s @ ..] => super::t47::n30942(s, out),
		['с', s @ ..] => super::t47::n30943(s, out),
		['у', s @ ..] => super::t47::n30944(s, out),
		['ф', s @ ..] => super::t47::n30945(s, out),
		['ы', s @ ..] => super::t47::n30946(s, out),
		['ь', s @ ..] => super::t47::n30947(s, out),
		['ю', s @ ..] => super::t47::n30948(s, out),
		['я', s @ ..] => super::t47::n30949(s, out),
		_ => false,
	}
}

/// Words ending -уник.
pub(crate) fn n10978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10945);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 30728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фник.
pub(crate) fn n10979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10946);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30729);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58483);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86354);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 108068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122558);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30730);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 58484);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хник.
pub(crate) fn n10980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10947);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30732);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 58485);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86356);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108069);
									match s {
										[] => true,
										['с', '-', s @ ..] => {
											super::push(out, 122559);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 131363);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n137925(s, out),
														_ => false,
													}
												}
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
							super::push(out, 86357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цник.
pub(crate) fn n10981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10948);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30733);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58486);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86358);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108070);
									s.is_empty()
								}
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
			super::push(out, 30734);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58487);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86359);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

