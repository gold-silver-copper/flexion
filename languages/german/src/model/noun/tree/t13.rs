use flexion::Path;

/// Words ending -rung.
pub(crate) fn n6158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6097);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15147);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 15148);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 25420);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							super::push(out, 33692);
							match s {
								[] => true,
								['w', 'r', s @ ..] => {
									super::push(out, 39245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['o', 'f', 'r', s @ ..] => {
									super::push(out, 39246);
									s.is_empty()
								}
								['ö', 'f', s @ ..] => {
									match s {
										[] => true,
										['e', 'i', s @ ..] => {
											super::push(out, 42439);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 43963);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 42440);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 43964);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25421);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33693);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 33694);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33695);
							s.is_empty()
						}
						['ö', 'z', 'r', 'e', 'v', 'e', s @ ..] => {
							super::push(out, 33696);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'c', 'i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33697);
							match s {
								[] => true,
								['p', 's', 's', 'a', 'g', 'd', s @ ..] => {
									super::push(out, 39249);
									s.is_empty()
								}
								['r', 't', s @ ..] => {
									super::push(out, 39250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33698);
							match s {
								[] => true,
								['r', 'e', 'v', 'l', s @ ..] => {
									super::push(out, 39251);
									match s {
										[] => true,
										['a', 'i', s @ ..] => {
											super::push(out, 42441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t15::n26373(s, out),
				['k', 'l', s @ ..] => {
					super::push(out, 25423);
					match s {
						[] => true,
						['ö', 'v', 'e', 'b', 'n', 'e', 'd', s @ ..] => {
							super::push(out, 33709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'm', 'ä', 'd', 'n', 'e', 'z', s @ ..] => {
					super::push(out, 25424);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 33710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'ü', 'h', s @ ..] => {
							super::push(out, 33711);
							match s {
								[] => true,
								['c', 's', 'n', 'i', 'e', 'n', s @ ..] => {
									super::push(out, 39262);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'ä', s @ ..] => {
							super::push(out, 33712);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 39263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', s @ ..] => {
							super::push(out, 33713);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'e', 'b', 'f', s @ ..] => {
											super::push(out, 42452);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 42453);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'e', 't', 's', 's', 'n', s @ ..] => {
					super::push(out, 25425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15149);
			match s {
				[] => true,
				['a', s @ ..] => super::t15::n26378(s, out),
				['e', s @ ..] => {
					super::push(out, 25427);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 25428);
					s.is_empty()
				}
				['ä', s @ ..] => super::t15::n26381(s, out),
				['ü', s @ ..] => super::t15::n26382(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t15::n15475(s, out),
		['r', s @ ..] => {
			super::push(out, 15151);
			s.is_empty()
		}
		['ä', s @ ..] => super::t15::n15477(s, out),
		['ö', s @ ..] => {
			super::push(out, 15153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sung.
pub(crate) fn n6159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6098);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 15154);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25432);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 33729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', 's', 'm', s @ ..] => {
							super::push(out, 33730);
							s.is_empty()
						}
						['r', 'g', s @ ..] => {
							super::push(out, 33731);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 33732);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39282);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42464);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15155);
			match s {
				[] => true,
				['l', 'r', 'h', s @ ..] => {
					super::push(out, 25433);
					match s {
						[] => true,
						['a', 'w', 'r', 'e', 'v', 's', s @ ..] => {
							super::push(out, 33733);
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
				['a', 'f', 'r', 'e', 'v', 'n', s @ ..] => {
					super::push(out, 25434);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 33734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25435);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 25436);
					match s {
						[] => true,
						['l', 'f', 'n', 'i', 'e', 'e', 'b', '-', s @ ..] => {
							super::push(out, 33735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', 'l', 'f', 'u', s @ ..] => {
			super::push(out, 15156);
			match s {
				[] => true,
				['a', 's', 'n', s @ ..] => {
					super::push(out, 25437);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tung.
pub(crate) fn n6160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6099);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15157);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 15158);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25438);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', 's', 't', 'r', 'i', 'w', 'e', 'b', 'l', s @ ..] => {
									super::push(out, 39283);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 39284);
									s.is_empty()
								}
								['n', 'e', 'p', s @ ..] => {
									super::push(out, 39285);
									s.is_empty()
								}
								['r', 'e', 't', s @ ..] => {
									super::push(out, 39286);
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
						['g', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 39287);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39288);
									match s {
										[] => true,
										['n', 'e', 's', s @ ..] => {
											super::push(out, 42465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', 'm', s @ ..] => {
							super::push(out, 33736);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 39289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 25439);
					match s {
						[] => true,
						['l', 'm', s @ ..] => {
							super::push(out, 33737);
							match s {
								[] => true,
								['u', 'm', s @ ..] => {
									super::push(out, 39290);
									s.is_empty()
								}
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
			super::push(out, 15159);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33738);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 39291);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 39292);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 39293);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 39294);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39295);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39296);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42466);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 42467);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 42468);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33739);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 42470);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 43986);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 42471);
											match s {
												[] => true,
												['e', 't', s @ ..] => {
													super::push(out, 43987);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', '-', s @ ..] => {
									super::push(out, 39297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'e', 'b', 'n', 'e', 'd', s @ ..] => {
									super::push(out, 39298);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 39299);
									s.is_empty()
								}
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
			super::push(out, 15160);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33740);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42472);
											s.is_empty()
										}
										['r', 'e', 'v', s @ ..] => {
											match s {
												[] => true,
												['h', 'c', 'a', s @ ..] => {
													super::push(out, 43988);
													s.is_empty()
												}
												['n', 'e', 't', 'a', 'd', 'h', s @ ..] => {
													super::push(out, 43989);
													s.is_empty()
												}
												['r', 'e', 't', s @ ..] => {
													super::push(out, 43990);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 39300);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 42473);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'e', 'b', 'm', s @ ..] => {
									super::push(out, 39301);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 42474);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'b', 'n', s @ ..] => {
							super::push(out, 33741);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '-', s @ ..] => {
							super::push(out, 33742);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 39303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t15::n15488(s, out),
		['r', s @ ..] => {
			super::push(out, 15162);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25440);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 33746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 25441);
					match s {
						[] => true,
						['w', 't', 'n', 'a', 'r', 'e', 'v', 't', s @ ..] => {
							super::push(out, 33747);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 39311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t15::n15490(s, out),
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => super::t15::n26405(s, out),
				['e', 'r', 'o', s @ ..] => super::t15::n26406(s, out),
				['ü', 'h', 'r', 'e', 'v', 'n', s @ ..] => super::t15::n26407(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n15492(s, out),
		_ => false,
	}
}

/// Words ending -uung.
pub(crate) fn n6161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6100);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25450);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 25451);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 33757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 's', 'n', 'a', 'h', s @ ..] => {
					super::push(out, 25452);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 25453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', 't', 'e', s @ ..] => {
			super::push(out, 15163);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 33758);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33759);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wung.
pub(crate) fn n6162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6101);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 15164);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33761);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42484);
											s.is_empty()
										}
										['r', 'u', 't', 'k', 'n', 'u', 'j', 'n', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42485);
											s.is_empty()
										}
										['s', 't', 'f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 42486);
											s.is_empty()
										}
										['t', 'k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42487);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', 'u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39318);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 39319);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39320);
									match s {
										[] => true,
										['e', 's', 'r', 'ö', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42488);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 42489);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 42490);
											match s {
												[] => true,
												['t', 'k', 'n', 'u', 'j', 'n', 'o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 43995);
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
										['g', 'e', 'i', 'r', 'k', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 42491);
											s.is_empty()
										}
										['t', 'f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 42492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39321);
									match s {
										[] => true,
										['k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 33762);
							match s {
								[] => true,
								['u', 's', 'g', 'n', 'u', 'n', s @ ..] => {
									super::push(out, 39322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15165);
			match s {
				[] => true,
				['i', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 25454);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zung.
pub(crate) fn n6163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6102);
	match s {
		[] => true,
		['i', 'e', 'h', 'e', s @ ..] => {
			super::push(out, 15166);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', s @ ..] => {
			super::push(out, 15167);
			s.is_empty()
		}
		['n', 'a', s @ ..] => {
			super::push(out, 15168);
			match s {
				[] => true,
				['l', 'f', 'p', 't', s @ ..] => {
					super::push(out, 25456);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 33764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15169);
			match s {
				[] => true,
				['ü', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 33765);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33766);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 39323);
									s.is_empty()
								}
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
				['e', s @ ..] => {
					super::push(out, 25457);
					match s {
						[] => true,
						['s', 'h', s @ ..] => {
							super::push(out, 33767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'r', 'e', 'd', s @ ..] => {
							super::push(out, 33768);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 39324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'n', 'n', s @ ..] => {
					super::push(out, 25458);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 33770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'h', s @ ..] => {
					super::push(out, 25459);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', 'n', s @ ..] => {
									super::push(out, 39325);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 39326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßung.
pub(crate) fn n6164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6103);
	s.is_empty()
}

/// The word mengenausgleich.
pub(crate) fn n46950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43996);
	s.is_empty()
}

/// The word interessenausgleich.
pub(crate) fn n46951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43997);
	s.is_empty()
}

/// Words ending -tenausgleich.
pub(crate) fn n46952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43998);
	s.is_empty()
}

/// The word zahlungsausgleich.
pub(crate) fn n46953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43999);
	s.is_empty()
}

/// Words ending -tungsausgleich.
pub(crate) fn n46954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44000);
	s.is_empty()
}

/// Words ending -reich.
pub(crate) fn n15651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15313);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 25560);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 25561);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 25562);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 33829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 25563);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 33830);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 39372);
							s.is_empty()
						}
						['e', 'n', s @ ..] => {
							super::push(out, 39373);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 39374);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 39375);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 39376);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 42499);
									match s {
										[] => true,
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 44001);
											s.is_empty()
										}
										['r', 't', 'n', 'e', 'z', '\u{1}', s @ ..] => {
											super::push(out, 44002);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44003);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 42500);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 42501);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 42502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 39377);
							match s {
								[] => true,
								['a', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 42503);
									s.is_empty()
								}
								['i', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 42504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 39378);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 44004);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 44005);
											s.is_empty()
										}
										['s', 'i', s @ ..] => {
											super::push(out, 44006);
											s.is_empty()
										}
										['t', 't', s @ ..] => {
											super::push(out, 44007);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 39379);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 42505);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 44008);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 44009);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 39380);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44010);
											match s {
												[] => true,
												['w', s @ ..] => {
													super::push(out, 44686);
													s.is_empty()
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
														['f', s @ ..] => super::t15::n47976(s, out),
														['g', s @ ..] => super::t15::n47977(s, out),
														_ => false,
													}
												}
												['u', s @ ..] => {
													match s {
														[] => true,
														['d', 'l', s @ ..] => super::t15::n47978(s, out),
														['h', s @ ..] => super::t15::n47979(s, out),
														['r', s @ ..] => super::t15::n47980(s, out),
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
									super::push(out, 42506);
									match s {
										[] => true,
										['e', 'n', 't', s @ ..] => {
											super::push(out, 44011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => super::t15::n45278(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 39381);
							match s {
								[] => true,
								['h', 'c', 'i', s @ ..] => {
									super::push(out, 42508);
									s.is_empty()
								}
								['i', s @ ..] => super::t15::n45280(s, out),
								['o', s @ ..] => super::t15::n45281(s, out),
								_ => false,
							}
						}
						['z', s @ ..] => super::t15::n41793(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => super::t15::n35626(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t15::n26539(s, out),
		['k', s @ ..] => super::t15::n26540(s, out),
		['l', s @ ..] => super::t15::n26541(s, out),
		['n', s @ ..] => super::t15::n26542(s, out),
		['r', s @ ..] => super::t15::n26543(s, out),
		['s', s @ ..] => super::t15::n26544(s, out),
		['t', s @ ..] => super::t15::n26545(s, out),
		['ß', s @ ..] => super::t15::n26546(s, out),
		_ => false,
	}
}

/// Words ending -lich.
pub(crate) fn n6264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6201);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 15315);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15316);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word koofmich.
pub(crate) fn n15655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15317);
	s.is_empty()
}

/// Words ending -nich.
pub(crate) fn n6266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6203);
	s.is_empty()
}

/// Words ending -pich.
pub(crate) fn n6267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6204);
	match s {
		[] => true,
		['p', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25574);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33842);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['l', 'o', 'w', 'm', 'u', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39397);
									s.is_empty()
								}
								['r', 'e', 'k', 'c', 'e', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 39398);
									s.is_empty()
								}
								['ö', '\u{1}', s @ ..] => {
									super::push(out, 39399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', 'ä', 'l', '\u{1}', s @ ..] => {
							super::push(out, 33843);
							s.is_empty()
						}
						['n', 'e', 'k', 'c', 'i', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 33844);
							s.is_empty()
						}
						['r', 'e', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rich.
pub(crate) fn n6268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6205);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 15318);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25575);
					match s {
						[] => true,
						['ü', s @ ..] => {
							super::push(out, 33846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 25576);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 25577);
					s.is_empty()
				}
				['t', 't', s @ ..] => {
					super::push(out, 25578);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 33847);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39400);
									s.is_empty()
								}
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
			super::push(out, 15319);
			match s {
				[] => true,
				['h', 'ä', 'f', '\u{1}', s @ ..] => {
					super::push(out, 25579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15320);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25580);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 25581);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33848);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 39401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', s @ ..] => {
							super::push(out, 33849);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 39402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 33850);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 33851);
							s.is_empty()
						}
						['l', 'e', 's', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33852);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 39404);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 39405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 33853);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 39406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 39407);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 33854);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 33855);
							s.is_empty()
						}
						['ö', 'm', '\u{1}', s @ ..] => {
							super::push(out, 33856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sich.
pub(crate) fn n6269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6206);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 25582);
					s.is_empty()
				}
				['i', 'f', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33857);
							s.is_empty()
						}
						['g', 'r', 'e', 'b', 'n', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 33858);
							s.is_empty()
						}
						['h', 'c', 'a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 33859);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33860);
							s.is_empty()
						}
						['s', 's', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 33861);
							s.is_empty()
						}
						['t', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tich.
pub(crate) fn n6270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6207);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 15321);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33863);
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 39409);
									match s {
										[] => true,
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42515);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 33864);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 39410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 33865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'i', s @ ..] => {
					super::push(out, 25583);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 25584);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 33866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15322);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25585);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 39411);
									s.is_empty()
								}
								['r', 'e', 'u', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 39412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25586);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 33867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 25587);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 33868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 25588);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 33869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wich.
pub(crate) fn n6271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6208);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 15323);
			match s {
				[] => true,
				['n', 'a', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33870);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 33871);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 33872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -felgulasch.
pub(crate) fn n46973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44016);
	s.is_empty()
}

/// Words ending -selgulasch.
pub(crate) fn n46974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44017);
	s.is_empty()
}

/// Words ending -isch.
pub(crate) fn n6296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6233);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15380);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 15381);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 15382);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25633);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 33886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 25634);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'i', 'l', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 15383);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 15384);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 25635);
					s.is_empty()
				}
				['d', 'a', s @ ..] => {
					super::push(out, 25636);
					s.is_empty()
				}
				['n', 'ä', 'l', 'r', 'e', 'u', s @ ..] => {
					super::push(out, 25637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15385);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 25638);
					match s {
						[] => true,
						['y', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 33888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 25639);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 39421);
									s.is_empty()
								}
								['l', 'i', s @ ..] => {
									super::push(out, 39422);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42517);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 42518);
											match s {
												[] => true,
												['c', 'o', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 44018);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 't', s @ ..] => {
											super::push(out, 42519);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 42520);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'i', 't', 'l', s @ ..] => {
									super::push(out, 39423);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42521);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42522);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'l', 'a', 'm', s @ ..] => {
									super::push(out, 39424);
									s.is_empty()
								}
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
			super::push(out, 15386);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 25640);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 33889);
							s.is_empty()
						}
						['g', 'n', s @ ..] => {
							super::push(out, 33890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 25641);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 39425);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 42523);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 39426);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 42524);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 33891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25642);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', 's', s @ ..] => {
									super::push(out, 39427);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 39428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25643);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'u', 'e', s @ ..] => {
							super::push(out, 33892);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 33893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 25644);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 25645);
					match s {
						[] => true,
						['t', 'i', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 33894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 15387);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 15388);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					super::push(out, 25646);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33895);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 39429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 33896);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39430);
									s.is_empty()
								}
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
			super::push(out, 15389);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 15390);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25647);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 25648);
					match s {
						[] => true,
						['ä', 'r', 'f', 'u', s @ ..] => {
							super::push(out, 33897);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 39431);
									match s {
										[] => true,
										['l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42525);
											s.is_empty()
										}
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
					super::push(out, 25649);
					match s {
						[] => true,
						['ä', s @ ..] => {
							super::push(out, 33898);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 33899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15391);
			match s {
				[] => true,
				['ä', 'f', 't', 's', 'e', s @ ..] => {
					super::push(out, 25651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 15392);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25652);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39432);
									s.is_empty()
								}
								['d', 'r', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 39433);
									s.is_empty()
								}
								['i', 'r', 'a', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 39434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25653);
					match s {
						[] => true,
						['g', 'n', s @ ..] => {
							super::push(out, 33900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', 'r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 25654);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 25655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15393);
			match s {
				[] => true,
				['a', s @ ..] => super::t15::n26640(s, out),
				['i', s @ ..] => {
					super::push(out, 25657);
					match s {
						[] => true,
						['e', 't', 's', s @ ..] => {
							super::push(out, 33905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t15::n26642(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15394);
			s.is_empty()
		}
		['p', s @ ..] => super::t15::n15736(s, out),
		['r', s @ ..] => super::t15::n15737(s, out),
		['s', s @ ..] => super::t15::n15738(s, out),
		['t', s @ ..] => super::t15::n15739(s, out),
		['u', s @ ..] => super::t15::n15740(s, out),
		['v', s @ ..] => super::t15::n15741(s, out),
		['w', s @ ..] => super::t15::n15742(s, out),
		['z', s @ ..] => super::t15::n15743(s, out),
		['ß', s @ ..] => super::t15::n15744(s, out),
		['ä', s @ ..] => super::t15::n15745(s, out),
		['ö', 'r', 'ä', 'f', '\u{1}', s @ ..] => super::t15::n15746(s, out),
		_ => false,
	}
}

/// Words ending -arsch.
pub(crate) fn n15756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15415);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 25691);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 25692);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33945);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 39460);
							s.is_empty()
						}
						['g', 'ä', 's', '\u{1}', s @ ..] => {
							super::push(out, 39461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'f', s @ ..] => {
					super::push(out, 33946);
					match s {
						[] => true,
						['i', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 39462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 33947);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 33948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 25693);
			match s {
				[] => true,
				['n', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 33949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 25694);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 25695);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 33950);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39463);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 42535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 33951);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 33952);
					match s {
						[] => true,
						['e', 't', 's', s @ ..] => {
							super::push(out, 39464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 33953);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 39465);
							match s {
								[] => true,
								['u', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 42536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 33954);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 39466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 33955);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 39467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 25696);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 39468);
							s.is_empty()
						}
						['u', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 39469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 25697);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 42537);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 42538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 39470);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['h', 'ü', 'h', '\u{1}', s @ ..] => {
									super::push(out, 42539);
									s.is_empty()
								}
								['n', 'ä', 'm', '\u{1}', s @ ..] => {
									super::push(out, 42540);
									s.is_empty()
								}
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
			super::push(out, 25698);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 25699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tsch.
pub(crate) fn n6303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6240);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15420);
			match s {
				[] => true,
				['d', 'a', 'r', 'e', 'd', 'd', 'a', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 25706);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 25707);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33957);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 33958);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39471);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 33959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 25708);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 25709);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 25710);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 33960);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 33961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25711);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 33962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 25712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15421);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 25713);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33963);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 33964);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 33965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 25714);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 33966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 25715);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 25716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15422);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 25718);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39473);
									match s {
										[] => true,
										['r', 'a', 'w', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 42541);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 39474);
									s.is_empty()
								}
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
			super::push(out, 15423);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 25719);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 25720);
					match s {
						[] => true,
						['e', 'r', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 33968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25721);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 33969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 25722);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 39475);
									s.is_empty()
								}
								['i', 'e', 'l', 'z', 'n', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 39476);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', 'r', 'ö', 'h', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42542);
											s.is_empty()
										}
										['t', 's', 'i', 'r', 'u', 'j', '\u{1}', s @ ..] => {
											super::push(out, 42543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 't', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 25723);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 25724);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 33970);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25725);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 33972);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 25726);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 33974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 15425);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 25727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 15426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usch.
pub(crate) fn n6304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6241);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15427);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25728);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 25729);
					s.is_empty()
				}
				['l', 'p', 'e', 'e', 'f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 25730);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 25731);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 33975);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 33976);
							s.is_empty()
						}
						['f', 'f', s @ ..] => {
							super::push(out, 33977);
							s.is_empty()
						}
						['m', 'l', s @ ..] => {
							super::push(out, 33978);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									super::push(out, 39478);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 39479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 25732);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33979);
							match s {
								[] => true,
								['a', 'g', s @ ..] => {
									super::push(out, 39480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 33980);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 33981);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 33982);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 33983);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									super::push(out, 39481);
									match s {
										[] => true,
										['a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 42544);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33984);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42545);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42546);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t15::n47721(s, out),
														['n', 'o', s @ ..] => super::t15::n47722(s, out),
														['r', s @ ..] => super::t15::n47723(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 42547);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42548);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15428);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 25733);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33985);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 39482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'd', 'n', 'a', s @ ..] => {
							super::push(out, 33986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25734);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 25735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 15429);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 25736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15430);
			match s {
				[] => true,
				['c', 's', 't', '\u{1}', s @ ..] => {
					super::push(out, 25737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15431);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 15432);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 15433);
			match s {
				[] => true,
				['r', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33988);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 33989);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39483);
									match s {
										[] => true,
										['e', 'f', 'p', '\u{1}', s @ ..] => {
											super::push(out, 42549);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'c', 'r', s @ ..] => {
							super::push(out, 33990);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 33991);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 39484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', s @ ..] => {
							super::push(out, 33992);
							s.is_empty()
						}
						['n', 'e', 'r', 'o', 't', 'o', 'r', s @ ..] => {
							super::push(out, 33993);
							s.is_empty()
						}
						['r', 'e', 'i', s @ ..] => {
							super::push(out, 33994);
							s.is_empty()
						}
						['s', 's', 'u', s @ ..] => {
							super::push(out, 33995);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33996);
							match s {
								[] => true,
								['h', 'c', 'a', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42550);
											s.is_empty()
										}
										['n', '\u{1}', s @ ..] => {
											super::push(out, 42551);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 33997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ysch.
pub(crate) fn n6306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6243);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 15435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äsch.
pub(crate) fn n6307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6244);
	s.is_empty()
}

/// Words ending -ösch.
pub(crate) fn n6308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6245);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 15436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üsch.
pub(crate) fn n6309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6246);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 15437);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15438);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 15439);
			s.is_empty()
		}
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 15440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -brauch.
pub(crate) fn n26744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25755);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 34000);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42553);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 42554);
							match s {
								[] => true,
								['c', 'a', 'r', 'p', 's', 's', s @ ..] => {
									super::push(out, 44019);
									match s {
										[] => true,
										['g', 'n', 'a', 'g', 'm', 'u', '\u{1}', s @ ..] => {
											super::push(out, 44693);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 42555);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 44020);
									s.is_empty()
								}
								['r', 'a', s @ ..] => {
									super::push(out, 44021);
									s.is_empty()
								}
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
			super::push(out, 34001);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 42556);
							match s {
								[] => true,
								['i', 'g', 'r', 'e', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 44022);
									s.is_empty()
								}
								['l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 44023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'f', 'o', 't', 's', 't', s @ ..] => {
							super::push(out, 42557);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 44024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 42558);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'z', s @ ..] => {
									super::push(out, 44025);
									s.is_empty()
								}
								['i', 's', s @ ..] => {
									super::push(out, 44026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 42559);
							s.is_empty()
						}
						['s', 'a', s @ ..] => {
							super::push(out, 42560);
							match s {
								[] => true,
								['g', 'd', s @ ..] => {
									super::push(out, 44027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 42561);
							match s {
								[] => true,
								['m', 'a', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 44028);
									s.is_empty()
								}
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
			super::push(out, 34002);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 39491);
					s.is_empty()
				}
				['s', 'i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42562);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 42563);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44029);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 44030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							super::push(out, 42564);
							match s {
								[] => true,
								['t', 't', s @ ..] => {
									super::push(out, 44031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 't', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 42565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ß', s @ ..] => {
			super::push(out, 34003);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39492);
					s.is_empty()
				}
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42566);
							s.is_empty()
						}
						['s', 't', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 42567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nrauch.
pub(crate) fn n26749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25760);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 34008);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 39493);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 42568);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 42569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 't', 'ü', s @ ..] => {
					super::push(out, 39494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrauch.
pub(crate) fn n26751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25762);
	s.is_empty()
}

/// Words ending -srauch.
pub(crate) fn n26752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25763);
	s.is_empty()
}

/// Words ending -trauch.
pub(crate) fn n26753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25764);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 34009);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 39495);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 42570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 42571);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 44032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 42572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vrauch.
pub(crate) fn n26754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25765);
	s.is_empty()
}

/// Words ending -zrauch.
pub(crate) fn n26755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25766);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 34010);
			match s {
				[] => true,
				['o', 'h', 'n', 'e', 'h', 'c', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 39496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßrauch.
pub(crate) fn n26756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25767);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 34011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -buch.
pub(crate) fn n6316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6253);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 15456);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['h', 's', 'g', s @ ..] => {
							super::push(out, 34012);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 34013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15457);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 25768);
					match s {
						[] => true,
						['f', 'a', 'r', 'g', 'o', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 34014);
							s.is_empty()
						}
						['g', 'o', 'l', 'o', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'y', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 39497);
									s.is_empty()
								}
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39498);
									s.is_empty()
								}
								['o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 39499);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 39500);
									s.is_empty()
								}
								['o', 't', 'a', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', 'e', 'm', 'o', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 34015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 25769);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 34016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'e', s @ ..] => {
			super::push(out, 15458);
			match s {
				[] => true,
				['r', 'd', 'l', s @ ..] => {
					super::push(out, 25770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15459);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 34017);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 34018);
							match s {
								[] => true,
								['u', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39502);
									s.is_empty()
								}
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
			super::push(out, 15460);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 34019);
							match s {
								[] => true,
								['s', 'a', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 39503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 34020);
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
									super::push(out, 39504);
									s.is_empty()
								}
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
			super::push(out, 15461);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 25771);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 34021);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 34022);
							s.is_empty()
						}
						['t', 'n', 'e', 'm', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 34023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 34024);
							match s {
								[] => true,
								['l', 'i', 'b', 'p', s @ ..] => {
									super::push(out, 39505);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39506);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 42573);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 34025);
							match s {
								[] => true,
								['r', 'ö', 'w', 'h', s @ ..] => {
									super::push(out, 39507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'e', 'l', 's', s @ ..] => {
					super::push(out, 25772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 15462);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 25773);
					match s {
						[] => true,
						['u', 'k', 'n', s @ ..] => {
							super::push(out, 34026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 15463);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 15464);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25774);
					s.is_empty()
				}
				['i', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 25775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -euch.
pub(crate) fn n6317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6254);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 15465);
			match s {
				[] => true,
				['a', 't', 'k', 'o', '\u{1}', s @ ..] => {
					super::push(out, 25776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -luch.
pub(crate) fn n6318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15466);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 15467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nuch.
pub(crate) fn n6319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6256);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 15468);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ouch.
pub(crate) fn n6320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6257);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 15469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25777);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 25778);
					match s {
						[] => true,
						['t', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 34027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ruch.
pub(crate) fn n6321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6258);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15471);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 15472);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25779);
					match s {
						[] => true,
						['a', 's', 'n', s @ ..] => {
							super::push(out, 34028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 25780);
					s.is_empty()
				}
				['l', 'i', s @ ..] => {
					super::push(out, 25781);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 25782);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'o', 'p', s @ ..] => {
									super::push(out, 39508);
									s.is_empty()
								}
								['m', 'm', 'a', 's', 'u', 'z', 'f', s @ ..] => {
									super::push(out, 39509);
									match s {
										[] => true,
										['u', 'a', 'l', 's', 'i', 'e', 'r', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42574);
											s.is_empty()
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
								['b', s @ ..] => {
									super::push(out, 39510);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 39511);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 42575);
											match s {
												[] => true,
												['u', 'k', '\u{1}', s @ ..] => {
													super::push(out, 44033);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 25783);
					match s {
						[] => true,
						['f', 'e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34029);
							s.is_empty()
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 34030);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 34031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25784);
					match s {
						[] => true,
						['l', 'a', 'h', 'l', s @ ..] => {
							super::push(out, 34032);
							match s {
								[] => true,
								['e', 'k', 'n', 'e', 'h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42576);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42577);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 34033);
							match s {
								[] => true,
								['e', 'd', 'e', 'i', 'r', 'f', 's', 'u', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 39512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', 's', 'i', s @ ..] => {
							super::push(out, 34034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15473);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 34035);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 34036);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 39513);
									s.is_empty()
								}
								['l', 'h', s @ ..] => {
									super::push(out, 39514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'f', 'o', 't', 's', 'b', 'e', s @ ..] => {
							super::push(out, 34037);
							s.is_empty()
						}
						['h', 'c', 's', 'i', 'e', s @ ..] => {
							super::push(out, 34038);
							s.is_empty()
						}
						['i', 'e', s @ ..] => {
							super::push(out, 34039);
							s.is_empty()
						}
						['l', 'a', 'k', s @ ..] => {
							super::push(out, 34040);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 34041);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39515);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 39516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							super::push(out, 34042);
							match s {
								[] => true,
								['f', 'p', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 39517);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42578);
											s.is_empty()
										}
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 42579);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', 'i', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 39518);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 39519);
									s.is_empty()
								}
								['k', 'n', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 39520);
									s.is_empty()
								}
								['m', 'u', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39521);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['o', 'r', 't', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 42580);
											s.is_empty()
										}
										['r', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42581);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'r', 'a', 'g', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 39522);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42582);
											s.is_empty()
										}
										['ü', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42583);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'i', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 39523);
									s.is_empty()
								}
								['ß', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 39524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 34043);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'g', 'b', s @ ..] => {
									super::push(out, 39525);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 39526);
									s.is_empty()
								}
								['i', 'n', 'g', s @ ..] => {
									super::push(out, 39527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'r', s @ ..] => {
							super::push(out, 34044);
							s.is_empty()
						}
						['z', 'r', 'a', s @ ..] => {
							super::push(out, 34045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t15::n15815(s, out),
		_ => false,
	}
}

/// Words ending -such.
pub(crate) fn n6322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6259);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 15475);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25785);
					match s {
						[] => true,
						['n', 'e', 't', 'n', s @ ..] => {
							super::push(out, 34051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 25786);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 39533);
									s.is_empty()
								}
								['t', 't', 'i', 'r', 't', 'k', s @ ..] => {
									super::push(out, 39534);
									s.is_empty()
								}
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
			super::push(out, 15476);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', s @ ..] => {
							super::push(out, 34052);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 39535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', s @ ..] => {
							super::push(out, 34053);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 34054);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['g', 'i', 'd', s @ ..] => {
											super::push(out, 42586);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 44036);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 42587);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['b', s @ ..] => super::t15::n47728(s, out),
														['i', 't', s @ ..] => super::t15::n47729(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'o', 'i', 't', 'a', 'd', s @ ..] => {
									super::push(out, 39536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', s @ ..] => {
							super::push(out, 34055);
							s.is_empty()
						}
						['ß', 'o', s @ ..] => {
							super::push(out, 34056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tuch.
pub(crate) fn n6323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6260);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 15477);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 15478);
			match s {
				[] => true,
				['n', 'a', 'h', 'e', 't', s @ ..] => {
					super::push(out, 25787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 15479);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 25788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15480);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25789);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 34057);
							s.is_empty()
						}
						['k', 'c', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 34058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15481);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'a', 't', 'n', s @ ..] => {
							super::push(out, 34059);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 34060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15482);
			match s {
				[] => true,
				['ü', 'f', '\u{1}', s @ ..] => {
					super::push(out, 25790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'h', s @ ..] => {
			super::push(out, 15483);
			match s {
				[] => true,
				['c', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 25791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äuch.
pub(crate) fn n6324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6261);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 15484);
			match s {
				[] => true,
				['t', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 25792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -alfischerei.
pub(crate) fn n46999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44039);
	s.is_empty()
}

/// Words ending -nfischerei.
pub(crate) fn n47000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44040);
	s.is_empty()
}

/// Words ending -ofischerei.
pub(crate) fn n47001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44041);
	s.is_empty()
}

/// Words ending -aurerei.
pub(crate) fn n26893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25885);
	s.is_empty()
}

/// Words ending -serei.
pub(crate) fn n15992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15631);
	s.is_empty()
}

/// Words ending -terei.
pub(crate) fn n15993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15632);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 25886);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 34119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'b', 'n', s @ ..] => {
			super::push(out, 25887);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 25888);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 34120);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 34121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 25889);
			match s {
				[] => true,
				['ö', s @ ..] => {
					super::push(out, 34122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 25890);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 34123);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39561);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 34124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uerei.
pub(crate) fn n15994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15633);
	s.is_empty()
}

/// Words ending -verei.
pub(crate) fn n15995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15634);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 34125);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 34126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zerei.
pub(crate) fn n15996(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'ä', s @ ..] => {
			super::push(out, 25891);
			s.is_empty()
		}
		['t', 'i', 'n', 'h', 'c', 's', 'd', s @ ..] => {
			super::push(out, 25892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dgießerei.
pub(crate) fn n15997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15635);
	s.is_empty()
}

/// Words ending -frei.
pub(crate) fn n6517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6450);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrei.
pub(crate) fn n6518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6451);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 15637);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34127);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 34128);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 39563);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 39564);
									match s {
										[] => true,
										['f', 'p', 'm', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42596);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['e', 'w', 'ö', 'm', '\u{1}', s @ ..] => {
													super::push(out, 44042);
													s.is_empty()
												}
												['n', 'i', 'z', '\u{1}', s @ ..] => {
													super::push(out, 44043);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42597);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 34129);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 34130);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 34131);
							match s {
								[] => true,
								['e', 'b', 'u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 39565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 34132);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 34133);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 34134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 15638);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 25893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nrei.
pub(crate) fn n6519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6452);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 15639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orei.
pub(crate) fn n6520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6453);
	s.is_empty()
}

/// Words ending -rrei.
pub(crate) fn n6521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6454);
	match s {
		[] => true,
		['o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 15640);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfleck.
pub(crate) fn n27058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26041);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 34178);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 39586);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 42600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 39587);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 39588);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 42601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nleck.
pub(crate) fn n16306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15938);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 26046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sleck.
pub(crate) fn n16307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15939);
	match s {
		[] => true,
		['a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 26047);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word heckmeck.
pub(crate) fn n6885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6812);
	s.is_empty()
}

/// Words ending -neck.
pub(crate) fn n6886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6813);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 15940);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15941);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 26049);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 34179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -peck.
pub(crate) fn n6887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6814);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 15942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26050);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 26051);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 26052);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 26053);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26054);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 26055);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 39589);
									s.is_empty()
								}
								['t', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 39590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 26056);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 26057);
					match s {
						[] => true,
						['k', 'c', 'ü', 't', 's', 'h', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 34180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 26058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reck.
pub(crate) fn n6888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6815);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15943);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 15944);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26059);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 34181);
							s.is_empty()
						}
						['s', 'u', 'ä', 'm', '\u{1}', s @ ..] => {
							super::push(out, 34182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'g', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 26060);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 26061);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 39591);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39592);
									match s {
										[] => true,
										['ä', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42602);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'n', 'h', 'ü', 'h', '\u{1}', s @ ..] => {
					super::push(out, 26062);
					s.is_empty()
				}
				['ß', 'i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 26063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15945);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 26064);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39593);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39594);
									match s {
										[] => true,
										['e', 'n', 'h', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 42603);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39595);
									s.is_empty()
								}
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
			super::push(out, 15946);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 26065);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 34183);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 34184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39596);
									s.is_empty()
								}
								['l', 'e', 'g', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 39597);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39598);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['g', 'r', 'ü', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42604);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 42605);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15947);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -seck.
pub(crate) fn n6889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6816);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 15948);
			match s {
				[] => true,
				['c', 'e', 's', 'r', s @ ..] => {
					super::push(out, 26066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teck.
pub(crate) fn n6890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6817);
	match s {
		[] => true,
		['h', 'c', 'e', s @ ..] => {
			super::push(out, 15950);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 15951);
			match s {
				[] => true,
				['e', 'b', 'k', 'i', 't', 's', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 26067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weck.
pub(crate) fn n6891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6818);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15952);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15953);
			s.is_empty()
		}
		['t', 'r', 'o', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 15954);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 15955);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 26068);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 34185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zeck.
pub(crate) fn n6892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6819);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15956);
			s.is_empty()
		}
		['t', 'a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 15957);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 15958);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gegenschock.
pub(crate) fn n47004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44044);
	s.is_empty()
}

/// Words ending -erblock.
pub(crate) fn n36053(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 39619);
			s.is_empty()
		}
		['n', 'l', 'l', 'e', 'k', '\u{1}', s @ ..] => {
			super::push(out, 39620);
			s.is_empty()
		}
		['p', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 39621);
			s.is_empty()
		}
		['s', 'u', 'ä', 'h', '\u{1}', s @ ..] => {
			super::push(out, 39622);
			s.is_empty()
		}
		['t', 'h', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 39623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orblock.
pub(crate) fn n36054(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 39624);
			s.is_empty()
		}
		['t', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 39625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tblock.
pub(crate) fn n27117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26096);
	match s {
		[] => true,
		['h', 'c', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 34213);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 34214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zblock.
pub(crate) fn n27118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26097);
	s.is_empty()
}

/// Words ending -dlock.
pub(crate) fn n16360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15992);
	match s {
		[] => true,
		['a', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 26098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -elock.
pub(crate) fn n16361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15993);
	match s {
		[] => true,
		['v', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 26099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -flock.
pub(crate) fn n16362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15994);
	match s {
		[] => true,
		['p', 'l', s @ ..] => {
			super::push(out, 26100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -glock.
pub(crate) fn n16363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15995);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mlock.
pub(crate) fn n16364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15996);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 26102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulock.
pub(crate) fn n16365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15997);
	match s {
		[] => true,
		['h', '\u{1}', s @ ..] => {
			super::push(out, 26103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylock.
pub(crate) fn n16366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15998);
	match s {
		[] => true,
		['h', 's', '\u{1}', s @ ..] => {
			super::push(out, 26104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mock.
pub(crate) fn n6911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6838);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 15999);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 26105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nock.
pub(crate) fn n6912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6839);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16001);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 16002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pock.
pub(crate) fn n6913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6840);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 16003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rock.
pub(crate) fn n6914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6841);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16004);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 16005);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26106);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 26107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'i', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 16006);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 16007);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26108);
					s.is_empty()
				}
				['r', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 26109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', 'l', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 26110);
					s.is_empty()
				}
				['s', 'i', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 26111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 16008);
			match s {
				[] => true,
				['a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 26112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 16009);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 26113);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 26114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16010);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 16011);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26115);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 34215);
							s.is_empty()
						}
						['k', 'c', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 34216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34217);
							s.is_empty()
						}
						['m', 'u', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 34218);
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
				['r', 'o', 'f', 'i', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 26116);
					s.is_empty()
				}
				['ü', 't', 's', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 26117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16012);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 34219);
							s.is_empty()
						}
						['g', 'r', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 34220);
							s.is_empty()
						}
						['t', 't', s @ ..] => {
							super::push(out, 34221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16013);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 16014);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 16015);
			match s {
				[] => true,
				['f', 'i', 't', 's', 'i', 'e', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 26118);
					s.is_empty()
				}
				['m', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 26119);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 26120);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 26121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'e', 't', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 16016);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 16017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tock.
pub(crate) fn n6915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6842);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['b', 'u', s @ ..] => {
					super::push(out, 26122);
					match s {
						[] => true,
						['a', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 34222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 26123);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26124);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 34223);
							match s {
								[] => true,
								['r', 'o', 'p', 'r', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 39626);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 34224);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 39628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 34225);
							match s {
								[] => true,
								['o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 39629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 26125);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 34226);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 39630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 26126);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 26127);
					match s {
						[] => true,
						['k', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 34227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 26128);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 34228);
							s.is_empty()
						}
						['u', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wock.
pub(crate) fn n6916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6843);
	match s {
		[] => true,
		['u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 16018);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zock.
pub(crate) fn n6917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6844);
	s.is_empty()
}

/// Words ending -druck.
pub(crate) fn n16400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16027);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 26137);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 34236);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 39633);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 39634);
							match s {
								[] => true,
								['e', 'f', 'e', 'i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 42611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'h', s @ ..] => {
							super::push(out, 39635);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39636);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 42612);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 42613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 34237);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 39637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 34238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'l', s @ ..] => {
			super::push(out, 26138);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 26139);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 34239);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 34240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 26140);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 34241);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42614);
									s.is_empty()
								}
								['h', 'c', 'i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 42615);
									s.is_empty()
								}
								['r', 'e', 't', 's', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 42616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 34242);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 34243);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 39638);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 42617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 39639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 34244);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 39640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 26141);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 39641);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 39642);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 39643);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44045);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44046);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 39644);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 42618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 34245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 26142);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 26143);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 34246);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 39645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 26144);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 26145);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 39646);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 39647);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 39648);
							match s {
								[] => true,
								['m', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 42619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 39649);
							match s {
								[] => true,
								['n', 'i', 'n', s @ ..] => {
									super::push(out, 42620);
									match s {
										[] => true,
										['e', 'g', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 44047);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 39650);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39651);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 39652);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 42621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 39653);
							match s {
								[] => true,
								['t', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 42622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 34247);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 39654);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 42623);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 44048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 39655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 26146);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 26147);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 26148);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 34248);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 39656);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 39657);
							match s {
								[] => true,
								['n', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 42624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 39658);
							match s {
								[] => true,
								['s', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 42625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 39659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 34249);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 34250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 26149);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 34251);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 34252);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 34253);
					s.is_empty()
				}
				['g', s @ ..] => super::t15::n36099(s, out),
				['n', s @ ..] => super::t15::n36100(s, out),
				['t', s @ ..] => super::t15::n36101(s, out),
				['u', s @ ..] => super::t15::n36102(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t15::n27175(s, out),
		['u', s @ ..] => {
			super::push(out, 26151);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 34260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t15::n27177(s, out),
		['ß', s @ ..] => super::t15::n27178(s, out),
		_ => false,
	}
}

/// Words ending -politik.
pub(crate) fn n27270(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 34307);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 39684);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 39685);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 39686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 34308);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 42640);
							s.is_empty()
						}
						['i', 'z', s @ ..] => {
							super::push(out, 42641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 39687);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 39688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 34309);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 42642);
							s.is_empty()
						}
						['h', 'c', 'a', s @ ..] => {
							super::push(out, 42643);
							s.is_empty()
						}
						['i', 'l', s @ ..] => {
							super::push(out, 42644);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 42645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 39689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 34310);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 42646);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 39690);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 39691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39692);
					s.is_empty()
				}
				['g', 'n', s @ ..] => {
					super::push(out, 39693);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 44057);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 44058);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 44059);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 44929);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 44930);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 44931);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 44703);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 44060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', 'i', 't', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 42648);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 39694);
					match s {
						[] => true,
						['f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', 'l', s @ ..] => {
							super::push(out, 42650);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 42651);
							s.is_empty()
						}
						['ä', 't', 'i', 'r', s @ ..] => {
							super::push(out, 42652);
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
				['h', s @ ..] => {
					super::push(out, 39695);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 39696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'n', 'a', s @ ..] => {
			super::push(out, 34311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kritik.
pub(crate) fn n27271(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 34312);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 34313);
			s.is_empty()
		}
		['l', 'e', s @ ..] => {
			super::push(out, 34314);
			s.is_empty()
		}
		['m', 'u', s @ ..] => {
			super::push(out, 34315);
			s.is_empty()
		}
		['r', 'u', 't', 'l', s @ ..] => {
			super::push(out, 34316);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 34317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ktik.
pub(crate) fn n7001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6925);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16083);
			match s {
				[] => true,
				['d', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 34318);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 39697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 34319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', 'o', s @ ..] => {
					super::push(out, 26233);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 'd', s @ ..] => {
							super::push(out, 34320);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 34321);
							s.is_empty()
						}
						['s', 'g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 39698);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39699);
									s.is_empty()
								}
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
			super::push(out, 16084);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16085);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 16086);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntik.
pub(crate) fn n7003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6927);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 26234);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 34322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 26235);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 34323);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 34324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16087);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptik.
pub(crate) fn n7005(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16089);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16090);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 26236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stik.
pub(crate) fn n7006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6929);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16091);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 26237);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26238);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 34325);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 39700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 34326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39701);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 39702);
									match s {
										[] => true,
										['e', 'w', 'n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 42653);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 42654);
											s.is_empty()
										}
										['n', 'a', s @ ..] => {
											super::push(out, 42655);
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													super::push(out, 44061);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 42656);
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 44062);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26239);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 34327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 26240);
					match s {
						[] => true,
						['m', 'y', 'g', 'n', s @ ..] => {
							super::push(out, 34328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 26241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16092);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 26242);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 26243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16093);
			match s {
				[] => true,
				['d', 'a', 'k', s @ ..] => {
					super::push(out, 26244);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26245);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 34329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 26246);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 34330);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 39703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'e', s @ ..] => {
							super::push(out, 34331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 26247);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 34332);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 39704);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42657);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'a', 'r', 'p', 's', 'r', s @ ..] => {
											super::push(out, 42658);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 42659);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42660);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 34333);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 34334);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 34335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 26248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 16094);
			match s {
				[] => true,
				['n', 'g', 'a', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 34336);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 34337);
							s.is_empty()
						}
						['o', 't', s @ ..] => {
							super::push(out, 34338);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 34339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utik.
pub(crate) fn n7007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6930);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16095);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 26249);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 26250);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 26251);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -serhandwerk.
pub(crate) fn n47029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44068);
	s.is_empty()
}

/// Words ending -egshandwerk.
pub(crate) fn n47030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44069);
	s.is_empty()
}

/// Words ending -gwerk.
pub(crate) fn n16596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16199);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 39744);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 39745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 34403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 26342);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 34404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'b', s @ ..] => {
			match s {
				[] => true,
				['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 34405);
					s.is_empty()
				}
				['e', 'l', 'h', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39746);
							s.is_empty()
						}
						['n', 'u', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 39747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'k', 'c', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 34406);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 39748);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'h', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44070);
											s.is_empty()
										}
										['n', 'u', 'a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 44071);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 42683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 39749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 39750);
							s.is_empty()
						}
						['f', 'p', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 39751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 34407);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lwerk.
pub(crate) fn n16600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16203);
	match s {
		[] => true,
		['e', 'z', s @ ..] => {
			super::push(out, 26346);
			match s {
				[] => true,
				['r', 'u', 'w', '\u{1}', s @ ..] => {
					super::push(out, 34409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'b', s @ ..] => {
			super::push(out, 26347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nwerk.
pub(crate) fn n16601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16204);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', s @ ..] => {
					super::push(out, 34410);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					super::push(out, 34411);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 39752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 26348);
			match s {
				[] => true,
				['e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 34412);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rwerk.
pub(crate) fn n16602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16205);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 34413);
					match s {
						[] => true,
						['c', 'u', s @ ..] => {
							super::push(out, 39753);
							match s {
								[] => true,
								['ä', 'r', '\u{1}', s @ ..] => {
									super::push(out, 42684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 34414);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 39754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 34415);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 34416);
					match s {
						[] => true,
						['a', 'm', 'l', s @ ..] => {
							super::push(out, 39755);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 39756);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44072);
											s.is_empty()
										}
										['h', 'c', 's', 'i', 't', '\u{1}', s @ ..] => {
											super::push(out, 44073);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26349);
			match s {
				[] => true,
				['u', 'f', 'e', s @ ..] => {
					super::push(out, 34417);
					match s {
						[] => true,
						['d', 'r', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 39757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -swerk.
pub(crate) fn n16603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16206);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 26350);
			match s {
				[] => true,
				['l', 'i', 'h', 'r', s @ ..] => {
					super::push(out, 34418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'n', 'u', 's', s @ ..] => {
			super::push(out, 26351);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 26352);
			match s {
				[] => true,
				['ä', 't', 'i', 'z', 'i', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 34419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -twerk.
pub(crate) fn n16604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16207);
	match s {
		[] => true,
		['f', 'a', 'r', 'k', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 34420);
					s.is_empty()
				}
				['e', 'm', 'r', 'ä', 'w', 'b', s @ ..] => {
					super::push(out, 34421);
					s.is_empty()
				}
				['s', 'g', s @ ..] => {
					super::push(out, 34422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'a', s @ ..] => {
			super::push(out, 26353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34423);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uwerk.
pub(crate) fn n16605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16208);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 34424);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 34425);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 34426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zwerk.
pub(crate) fn n16606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16209);
	match s {
		[] => true,
		['t', 'e', 'n', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 34427);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 39759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 34428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -henmaterial.
pub(crate) fn n47035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44074);
	s.is_empty()
}

/// Words ending -hlenmaterial.
pub(crate) fn n47036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44075);
	s.is_empty()
}

/// Words ending -renmaterial.
pub(crate) fn n47037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44076);
	s.is_empty()
}

/// Words ending -genkapital.
pub(crate) fn n47038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44077);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 44704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ienkapital.
pub(crate) fn n47039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44078);
	s.is_empty()
}

/// Words ending -angskapital.
pub(crate) fn n47040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44079);
	s.is_empty()
}

/// Words ending -dungskapital.
pub(crate) fn n47041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44080);
	match s {
		[] => true,
		['n', 'ü', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 44705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -portal.
pub(crate) fn n27530(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34486);
			s.is_empty()
		}
		['n', 'e', 'd', s @ ..] => {
			super::push(out, 34487);
			match s {
				[] => true,
				['n', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 39792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 34488);
			s.is_empty()
		}
		['t', 'e', s @ ..] => {
			super::push(out, 34489);
			match s {
				[] => true,
				['n', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 39793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stal.
pub(crate) fn n7202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7124);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16361);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 34490);
							s.is_empty()
						}
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 34491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16362);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 16363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttal.
pub(crate) fn n7203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7125);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 16364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utal.
pub(crate) fn n7204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7126);
	s.is_empty()
}

/// Words ending -xtal.
pub(crate) fn n7205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7127);
	s.is_empty()
}

/// The word flußtal.
pub(crate) fn n7206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7128);
	s.is_empty()
}

/// Words ending -endel.
pub(crate) fn n16839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16436);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 26497);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 26498);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 34514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 26499);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 26500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -indel.
pub(crate) fn n16840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16437);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 26501);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 26502);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 26503);
			s.is_empty()
		}
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 26504);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 26505);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 34515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 26506);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34516);
					s.is_empty()
				}
				['f', 'f', 'o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 34517);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 34518);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 42700);
									s.is_empty()
								}
								['s', 't', 'a', s @ ..] => {
									super::push(out, 42701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'l', 'o', 'w', 'm', 'u', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 34519);
					s.is_empty()
				}
				['n', 'e', 'h', 'c', 's', 'ö', 'h', '\u{1}', s @ ..] => {
					super::push(out, 34520);
					s.is_empty()
				}
				['r', 'e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 34521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 26507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ondel.
pub(crate) fn n16841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16438);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 26508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -undel.
pub(crate) fn n16842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16439);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 26509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ändel.
pub(crate) fn n16843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16440);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 26510);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 26511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ündel.
pub(crate) fn n16844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16441);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 26512);
			match s {
				[] => true,
				['n', 'e', 'p', s @ ..] => {
					super::push(out, 34522);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 39806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 26513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odel.
pub(crate) fn n7249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7168);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 16442);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 16443);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 26514);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 34523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26515);
					s.is_empty()
				}
				['k', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 26516);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdel.
pub(crate) fn n7250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7169);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 16445);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16446);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udel.
pub(crate) fn n7251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7170);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 16447);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 16448);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 16449);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16450);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 16451);
			match s {
				[] => true,
				['ß', s @ ..] => {
					super::push(out, 26518);
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							super::push(out, 34524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16452);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 26519);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 26520);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 26521);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ädel.
pub(crate) fn n7252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7171);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 16453);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 16454);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 26522);
					match s {
						[] => true,
						['h', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 34525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ödel.
pub(crate) fn n7253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7172);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 16455);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16456);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26523);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 26524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 26525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üdel.
pub(crate) fn n7254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7173);
	s.is_empty()
}

/// Words ending -mangel.
pub(crate) fn n27648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26566);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 34546);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 34547);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 34548);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 39815);
					match s {
						[] => true,
						['c', 's', 'ä', 'w', '\u{1}', s @ ..] => {
							super::push(out, 42703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'f', s @ ..] => {
					super::push(out, 39816);
					match s {
						[] => true,
						['ä', 'r', 'k', 'h', s @ ..] => {
							super::push(out, 42704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 34549);
			match s {
				[] => true,
				['f', 'o', 't', 's', 'r', 'h', s @ ..] => {
					super::push(out, 39817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 34550);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39818);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 42705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 't', s @ ..] => {
							super::push(out, 42706);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 42707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 39819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 34551);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 39820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 34552);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 39821);
					s.is_empty()
				}
				['g', 'n', 'u', 'r', s @ ..] => {
					super::push(out, 39822);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 39823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', s @ ..] => {
			super::push(out, 34553);
			s.is_empty()
		}
		['z', 'l', s @ ..] => {
			super::push(out, 34554);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sangel.
pub(crate) fn n27651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26569);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 34559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tangel.
pub(crate) fn n27652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26570);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 34560);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 34561);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßangel.
pub(crate) fn n27653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26571);
	match s {
		[] => true,
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 34562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ingel.
pub(crate) fn n16902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16497);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 26572);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 26573);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 34563);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 39827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 34564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 26574);
			s.is_empty()
		}
		['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 26575);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 26576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ungel.
pub(crate) fn n16903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16498);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 26577);
			match s {
				[] => true,
				['c', 's', 'd', 't', s @ ..] => {
					super::push(out, 34565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ängel.
pub(crate) fn n16904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16499);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 26578);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 26579);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üngel.
pub(crate) fn n16905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16500);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 26580);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 26581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ogel.
pub(crate) fn n7272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7191);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 16501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 16502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16503);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 16504);
			match s {
				[] => true,
				['u', 'p', s @ ..] => {
					super::push(out, 26585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rgel.
pub(crate) fn n7273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7192);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16505);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 26586);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 26587);
					match s {
						[] => true,
						['q', '\u{1}', s @ ..] => {
							super::push(out, 34567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16506);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 26588);
					match s {
						[] => true,
						['s', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 34568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 26589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16507);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 26590);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 26591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 16508);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 16509);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 26592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'z', '\u{1}', s @ ..] => {
			super::push(out, 16510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tgel.
pub(crate) fn n7274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7193);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 16511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugel.
pub(crate) fn n7275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7194);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16512);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 16513);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26594);
					match s {
						[] => true,
						['l', 'a', 'h', 'd', s @ ..] => {
							super::push(out, 34569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'r', 'e', s @ ..] => {
					super::push(out, 26595);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 26596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 16514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ögel.
pub(crate) fn n7276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7195);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 16515);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ügel.
pub(crate) fn n7277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7196);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 16516);
			s.is_empty()
		}
		['l', 'f', 'e', s @ ..] => {
			super::push(out, 16517);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 26598);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 34570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16518);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 26599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ebeispiel.
pub(crate) fn n42300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39840);
	match s {
		[] => true,
		['d', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 42713);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word hörbeispiel.
pub(crate) fn n42302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39842);
	s.is_empty()
}

/// Words ending -inkspiel.
pub(crate) fn n27706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26622);
	match s {
		[] => true,
		['r', 't', '\u{1}', s @ ..] => {
			super::push(out, 34591);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lspiel.
pub(crate) fn n27707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26623);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 34592);
			s.is_empty()
		}
		['l', 'a', 'b', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39843);
					match s {
						[] => true,
						['e', 'd', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 42714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 39844);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 42715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'h', s @ ..] => {
			super::push(out, 34593);
			match s {
				[] => true,
				['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 39845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mspiel.
pub(crate) fn n27708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26624);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 34594);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 34595);
			s.is_empty()
		}
		['u', 'i', s @ ..] => {
			super::push(out, 34596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enspiel.
pub(crate) fn n27709(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 34597);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 34598);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 39846);
					s.is_empty()
				}
				['n', 'a', 'd', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 39847);
					s.is_empty()
				}
				['r', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 39848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', 'o', 'r', 'e', s @ ..] => {
			super::push(out, 34599);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 34600);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 34601);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 39849);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 42716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 39850);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 42717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rspiel.
pub(crate) fn n27710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26625);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 39851);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 42718);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 42719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 39852);
					s.is_empty()
				}
				['l', 'l', 'i', s @ ..] => {
					super::push(out, 39853);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 42720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 39854);
					match s {
						[] => true,
						['s', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 42721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sspiel.
pub(crate) fn n27711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26626);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 39855);
					s.is_empty()
				}
				['n', 'u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 42722);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 42723);
							match s {
								[] => true,
								['a', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 44081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'h', s @ ..] => {
			super::push(out, 34602);
			s.is_empty()
		}
		['n', 'o', s @ ..] => {
			super::push(out, 34603);
			match s {
				[] => true,
				['i', 't', 'a', 'k', s @ ..] => {
					super::push(out, 39856);
					match s {
						[] => true,
						['i', 'f', 'i', 'l', 'a', 'u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 44082);
									s.is_empty()
								}
								['-', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 44083);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 44084);
									s.is_empty()
								}
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
			super::push(out, 34604);
			match s {
				[] => true,
				['a', 'p', 'z', s @ ..] => {
					super::push(out, 39857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 34605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tspiel.
pub(crate) fn n27712(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 34606);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 34607);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 34608);
			match s {
				[] => true,
				['a', 'w', s @ ..] => {
					super::push(out, 39859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 34609);
			s.is_empty()
		}
		['t', 'e', 'r', 'b', 'd', s @ ..] => {
			super::push(out, 34610);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yspiel.
pub(crate) fn n27713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26627);
	match s {
		[] => true,
		['b', 'g', 'u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 34611);
			s.is_empty()
		}
		['d', 'n', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 34612);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 39860);
					s.is_empty()
				}
				['k', 'c', 'o', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42724);
							s.is_empty()
						}
						['s', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 42725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 34613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßspiel.
pub(crate) fn n27714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26628);
	s.is_empty()
}

/// Words ending -riel.
pub(crate) fn n7288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7207);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16536);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 16537);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 16538);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siel.
pub(crate) fn n7289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7208);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16539);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiel.
pub(crate) fn n7290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7209);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 16540);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26630);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 26631);
					match s {
						[] => true,
						['ü', s @ ..] => {
							super::push(out, 34614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 26632);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26633);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['f', 'i', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34615);
							s.is_empty()
						}
						['p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34616);
							s.is_empty()
						}
						['s', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 34617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'm', 'm', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 26634);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 26635);
					s.is_empty()
				}
				['z', 'l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 26636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -viel.
pub(crate) fn n7291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7210);
	s.is_empty()
}

/// Words ending -wiel.
pub(crate) fn n7292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7211);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ziel.
pub(crate) fn n7293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7212);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 16542);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 34618);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 34619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtikel.
pub(crate) fn n27748(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 39862);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 39863);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 39864);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 39865);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 39866);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 39867);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 42728);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 44085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 42729);
							match s {
								[] => true,
								['i', 't', 's', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 44086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 42730);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 42731);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 44087);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 44088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 42732);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 42733);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44089);
									match s {
										[] => true,
										['l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 44706);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 44090);
									match s {
										[] => true,
										['n', 'u', 'f', s @ ..] => {
											super::push(out, 44707);
											match s {
												[] => true,
												['u', 't', 's', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 44932);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 44091);
									match s {
										[] => true,
										['c', 'ä', 'r', 'p', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 44708);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 44092);
									match s {
										[] => true,
										['u', 'k', 's', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 44709);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 44093);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 44710);
											match s {
												[] => true,
												['i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 44933);
													s.is_empty()
												}
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
							super::push(out, 42734);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 44094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 42735);
							match s {
								[] => true,
								['u', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44711);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44712);
											s.is_empty()
										}
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
					super::push(out, 39868);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 39869);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 39870);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 39871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 34624);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 39872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nkel.
pub(crate) fn n7298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7217);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16566);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16567);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 26661);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 26662);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 26663);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 34625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 26664);
					match s {
						[] => true,
						['r', 'ä', s @ ..] => {
							super::push(out, 34626);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 34627);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 39873);
									s.is_empty()
								}
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
			super::push(out, 16568);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 16569);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26665);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 26666);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 26667);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 26668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 26669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34629);
							s.is_empty()
						}
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34630);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 34631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 16570);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 26670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 16571);
			match s {
				[] => true,
				['d', 's', s @ ..] => {
					super::push(out, 26671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rkel.
pub(crate) fn n7300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7219);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16573);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26672);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 26673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -skel.
pub(crate) fn n7301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7220);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 16575);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 26674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 16576);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 26675);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 34632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 26676);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 26677);
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 34633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 26678);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 34634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -aukel.
pub(crate) fn n16982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16577);
	match s {
		[] => true,
		['h', 'c', 's', 'e', s @ ..] => {
			super::push(out, 26679);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 34635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äkel.
pub(crate) fn n7303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7222);
	s.is_empty()
}

/// Words ending -aupel.
pub(crate) fn n17023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16617);
	match s {
		[] => true,
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 26740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rupel.
pub(crate) fn n17024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16618);
	s.is_empty()
}

/// Words ending -tupel.
pub(crate) fn n17025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16619);
	s.is_empty()
}

/// Words ending -äpel.
pub(crate) fn n7326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7245);
	s.is_empty()
}

/// Words ending -öpel.
pub(crate) fn n7327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7246);
	s.is_empty()
}

/// Words ending -assel.
pub(crate) fn n17049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16640);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 26770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34681);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 34682);
					match s {
						[] => true,
						['l', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42737);
									s.is_empty()
								}
								['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 42738);
									s.is_empty()
								}
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
			super::push(out, 26771);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					super::push(out, 34683);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 34684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 26772);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -essel.
pub(crate) fn n17050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16641);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 26773);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 26774);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 34685);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 39891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 26775);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 26776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ussel.
pub(crate) fn n17053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16644);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 26778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34686);
					s.is_empty()
				}
				['l', 'e', 'b', 'a', 'n', 'h', 'c', 'u', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 34687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 26779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -össel.
pub(crate) fn n17054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16645);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 26780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üssel.
pub(crate) fn n17055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16646);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 26781);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 34688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 26782);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 26783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tsel.
pub(crate) fn n7344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7263);
	match s {
		[] => true,
		['ä', s @ ..] => {
			super::push(out, 16647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usel.
pub(crate) fn n7345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7264);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16648);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 26784);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['-', 's', s @ ..] => {
									super::push(out, 39892);
									s.is_empty()
								}
								['s', 't', s @ ..] => {
									super::push(out, 39893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 26785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 16649);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16650);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 26786);
					match s {
						[] => true,
						['t', 's', 'l', s @ ..] => {
							super::push(out, 34689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 16651);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 16652);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 16653);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 26788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stel.
pub(crate) fn n7358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7277);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16678);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 16679);
			match s {
				[] => true,
				['n', 'e', 's', 'u', 'a', 't', 'n', s @ ..] => {
					super::push(out, 26814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 16680);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 16681);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 34707);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 39898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 34708);
							match s {
								[] => true,
								['n', 'a', 'w', 'z', 'd', s @ ..] => {
									super::push(out, 39899);
									s.is_empty()
								}
								['r', 'e', 'i', 'v', 'd', s @ ..] => {
									super::push(out, 39900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', 'i', 'e', 'r', 'd', 'd', 'n', 'u', 'r', s @ ..] => {
							super::push(out, 34709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 16682);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16683);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 16684);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 16685);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 26815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 26816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16686);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 26817);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 26818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 16687);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 16688);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 26819);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 26820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttel.
pub(crate) fn n7359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7278);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16689);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 26821);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 26822);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 34711);
							match s {
								[] => true,
								['m', 'e', 'r', 'b', 'n', s @ ..] => {
									super::push(out, 39901);
									match s {
										[] => true,
										['e', 'b', 'i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 42740);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16690);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 26823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 26824);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 34713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39902);
									s.is_empty()
								}
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
			super::push(out, 16691);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 26825);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 26826);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 34714);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 39903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 34715);
							match s {
								[] => true,
								['p', 'm', 'a', 'k', '-', s @ ..] => {
									super::push(out, 39904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 34716);
							match s {
								[] => true,
								['e', 'n', 'z', 'r', 'a', 'd', s @ ..] => {
									super::push(out, 39905);
									match s {
										[] => true,
										['r', 'a', 't', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 42741);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 34717);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 42742);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 44095);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'd', s @ ..] => {
											super::push(out, 42743);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 44096);
													s.is_empty()
												}
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
							super::push(out, 34718);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 39906);
									match s {
										[] => true,
										['n', 'u', 'r', 'e', 'i', s @ ..] => {
											super::push(out, 42744);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 44097);
													match s {
														[] => true,
														['r', 'a', 't', 'e', 'r', '\u{1}', s @ ..] => super::t15::n47745(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'o', 'i', 't', 'a', s @ ..] => {
									super::push(out, 39907);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 42745);
											match s {
												[] => true,
												['i', 'l', 'a', 't', 'i', 'v', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 44098);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39908);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42746);
											s.is_empty()
										}
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
					super::push(out, 26827);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 34719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 26828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 16692);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 26829);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 26830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 16693);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 26831);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 26832);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 26833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 16694);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 16695);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word minderwertigkeitsgefühl.
pub(crate) fn n47063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44101);
	s.is_empty()
}

/// Words ending -stigkeitsgefühl.
pub(crate) fn n47064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44102);
	s.is_empty()
}

/// Words ending -hteil.
pub(crate) fn n17199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16782);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 34771);
					match s {
						[] => true,
						['n', 't', s @ ..] => {
							super::push(out, 39932);
							match s {
								[] => true,
								['r', 'o', 'd', 'n', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 42755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 34772);
					s.is_empty()
				}
				['u', 'r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39933);
							s.is_empty()
						}
						['n', 'e', 'd', 'n', 'u', 'k', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 39934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 26917);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lteil.
pub(crate) fn n17201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16784);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 26920);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 26921);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 34773);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 34774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 26922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mteil.
pub(crate) fn n17202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16785);
	match s {
		[] => true,
		['r', 'o', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34775);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 34776);
					s.is_empty()
				}
				['r', 'h', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 34777);
					s.is_empty()
				}
				['s', 'g', 'n', 'u', 't', 'f', 'ü', 'l', '\u{1}', s @ ..] => {
					super::push(out, 34778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nteil.
pub(crate) fn n17203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16786);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 26923);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 34779);
					s.is_empty()
				}
				['f', 'f', 'o', 't', 's', 't', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 34780);
					s.is_empty()
				}
				['h', 'c', 't', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 34781);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 34782);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'e', 'm', 'f', 'f', 'o', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 42756);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', 'r', 'g', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 44103);
											s.is_empty()
										}
										['r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 44104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 42757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 39935);
							match s {
								[] => true,
								['n', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 42758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'w', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 39936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 34783);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 39937);
							s.is_empty()
						}
						['t', 'e', 'f', 'r', 'e', 'p', 'r', 'ö', 'k', '\u{1}', s @ ..] => {
							super::push(out, 39938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 26924);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 34784);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 34785);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 34786);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 34787);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 34788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 26925);
			match s {
				[] => true,
				['e', 't', 'l', 'e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 39939);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rteil.
pub(crate) fn n17204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16787);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 26926);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 39941);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 39942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 26927);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 34789);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'a', 'j', 'y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 42759);
									s.is_empty()
								}
								['f', 'u', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 42760);
									s.is_empty()
								}
								['g', 'u', 'z', 'n', 'a', 'f', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 42761);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 42762);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 42763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'm', 'ü', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 34790);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 34791);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 39943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 34792);
					match s {
						[] => true,
						['h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 39944);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 42764);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 42765);
									s.is_empty()
								}
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
			super::push(out, 26928);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 34793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39945);
							s.is_empty()
						}
						['l', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 39946);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 39947);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['c', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 44105);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 44106);
											s.is_empty()
										}
										['ß', s @ ..] => {
											super::push(out, 44107);
											match s {
												[] => true,
												['ö', 'r', 'g', '\u{1}', s @ ..] => {
													super::push(out, 44714);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'u', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 39948);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 39949);
							match s {
								[] => true,
								['b', 'r', 'e', 'w', 'e', 'b', 't', 't', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 42766);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 42767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 39950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 26929);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34794);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 34795);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 34796);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 34797);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 34798);
					match s {
						[] => true,
						['e', 'd', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 39951);
							s.is_empty()
						}
						['g', 'n', 'u', 'l', s @ ..] => {
							super::push(out, 39952);
							match s {
								[] => true,
								['l', 'e', 't', 's', 't', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 42768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', 'm', 'u', 'ä', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42769);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 42770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'h', 'c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 42771);
									match s {
										[] => true,
										['r', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 44108);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'r', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 42772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 34799);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 39953);
							s.is_empty()
						}
						['r', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 39954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', 'h', 'e', 'b', 'u', 'z', '\u{1}', s @ ..] => {
			super::push(out, 26930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -steil.
pub(crate) fn n17205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16788);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 26931);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 26932);
			match s {
				[] => true,
				['n', 'u', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', 'i', 'b', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 39955);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 39956);
							match s {
								[] => true,
								['f', 'ü', 'l', '\u{1}', s @ ..] => {
									super::push(out, 42773);
									s.is_empty()
								}
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
			super::push(out, 26933);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 26934);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 39957);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 39958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 26935);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 34800);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 34801);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tteil.
pub(crate) fn n17206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16789);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 26936);
			match s {
				[] => true,
				['c', 'i', 'l', 'f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 34802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 26937);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 39959);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 39960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 26938);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 26939);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 26940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uteil.
pub(crate) fn n17207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16790);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34803);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 34804);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 34805);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 34806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zteil.
pub(crate) fn n17208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16791);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 34807);
					match s {
						[] => true,
						['s', 'r', s @ ..] => {
							super::push(out, 39961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 34808);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42774);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 42775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßteil.
pub(crate) fn n17209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16792);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 26941);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 39962);
							s.is_empty()
						}
						['l', 'h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 39963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 26942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weil.
pub(crate) fn n7420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7339);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 16793);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 16794);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word basketball.
pub(crate) fn n47071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44109);
	s.is_empty()
}

/// Words ending -dbasketball.
pub(crate) fn n47072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44110);
	s.is_empty()
}

/// The word beachbasketball.
pub(crate) fn n47073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44111);
	s.is_empty()
}

/// Words ending -lbasketball.
pub(crate) fn n47074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44112);
	s.is_empty()
}

/// Words ending -otball.
pub(crate) fn n28090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26986);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 34830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stball.
pub(crate) fn n28091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26987);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 34831);
			s.is_empty()
		}
		['e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 34832);
			s.is_empty()
		}
		['u', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 34833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yball.
pub(crate) fn n17257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16837);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 26988);
			match s {
				[] => true,
				['l', 'l', 'o', 'v', 'z', s @ ..] => {
					super::push(out, 34834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 26989);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 34835);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 34836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßball.
pub(crate) fn n17259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16839);
	match s {
		[] => true,
		['u', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34837);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 34838);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 39970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 34839);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 34840);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 34841);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 34842);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 39971);
							match s {
								[] => true,
								['m', 'a', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 42776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -call.
pub(crate) fn n7450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7369);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16840);
			match s {
				[] => true,
				['y', 'b', '-', 'l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 26990);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dall.
pub(crate) fn n7451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7370);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 16841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fall.
pub(crate) fn n7452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7371);
	match s {
		[] => true,
		['-', 'i', s @ ..] => {
			super::push(out, 16842);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 16843);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', 'i', s @ ..] => {
							super::push(out, 34843);
							s.is_empty()
						}
						['n', 'e', 'z', s @ ..] => {
							super::push(out, 34844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16844);
			match s {
				[] => true,
				['b', 'r', s @ ..] => {
					super::push(out, 26991);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 39972);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 39973);
									s.is_empty()
								}
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
			super::push(out, 16845);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 16846);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34845);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 34846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16847);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 16848);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 16849);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 26993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16850);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', 'p', 'm', 'a', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34847);
							s.is_empty()
						}
						['n', 'e', 't', 's', 'o', s @ ..] => {
							super::push(out, 34848);
							s.is_empty()
						}
						['r', 'e', 'b', 'e', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 34849);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 34850);
							s.is_empty()
						}
						['t', 'u', 'w', '\u{1}', s @ ..] => {
							super::push(out, 34851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 26994);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 34852);
							match s {
								[] => true,
								['n', 'ü', 's', '\u{1}', s @ ..] => {
									super::push(out, 39974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 34853);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 39975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 26995);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 34854);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 34855);
							match s {
								[] => true,
								['k', 's', '\u{1}', s @ ..] => {
									super::push(out, 39976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 34856);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 's', 'e', 'r', 'd', 'h', 'ä', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42777);
											s.is_empty()
										}
										['l', 'l', 'ä', 'f', 'z', 'l', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 42778);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'a', 'f', 'f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39977);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 42779);
											s.is_empty()
										}
										['t', 'k', 'a', s @ ..] => {
											match s {
												[] => true,
												['e', 'r', '\u{1}', s @ ..] => {
													super::push(out, 44113);
													s.is_empty()
												}
												['r', 't', '\u{1}', s @ ..] => {
													super::push(out, 44114);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 't', 'o', s @ ..] => {
							super::push(out, 34857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16851);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 34858);
							match s {
								[] => true,
								['ü', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 42780);
											s.is_empty()
										}
										['k', 'n', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42781);
											s.is_empty()
										}
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 44115);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 44116);
													s.is_empty()
												}
												['p', 'i', 'e', 'n', 'k', '\u{1}', s @ ..] => {
													super::push(out, 44117);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'u', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42782);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 42783);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 34859);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									super::push(out, 39978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 34860);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 39979);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 'm', s @ ..] => {
									super::push(out, 39980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 34861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16852);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 26996);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39981);
									s.is_empty()
								}
								['m', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 39982);
									s.is_empty()
								}
								['r', 'a', s @ ..] => {
									super::push(out, 39983);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39984);
									match s {
										[] => true,
										['e', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42784);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 42785);
											match s {
												[] => true,
												['d', 'e', 'r', 'k', '\u{1}', s @ ..] => {
													super::push(out, 44118);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'n', 'e', 'i', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t15::n45582(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t15::n17273(s, out),
		['u', 'z', 'n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => super::t15::n17274(s, out),
		_ => false,
	}
}

/// Words ending -gall.
pub(crate) fn n7453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7372);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 16855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hall.
pub(crate) fn n7454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16856);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 16857);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34862);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 34863);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 34864);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 34865);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 39986);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 39987);
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 42787);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ö', s @ ..] => {
									super::push(out, 39988);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 42788);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 't', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 34866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 16858);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 16859);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 16860);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 26997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16861);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mall.
pub(crate) fn n7455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nall.
pub(crate) fn n7456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7375);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 16863);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 26999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rall.
pub(crate) fn n7457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7376);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 16864);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16865);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 27000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 16866);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 27001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sall.
pub(crate) fn n7458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7377);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16867);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16868);
			match s {
				[] => true,
				['e', 't', 't', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 27002);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tall.
pub(crate) fn n7459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7378);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16869);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 27003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34867);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 34868);
							s.is_empty()
						}
						['h', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 34869);
							s.is_empty()
						}
						['l', 'e', 'd', 'e', 'b', 'l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 34870);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 34871);
							match s {
								[] => true,
								['u', 'i', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 42789);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42790);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 42791);
											match s {
												[] => true,
												['n', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 44119);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'e', 's', 'i', 'e', 't', 'h', 'c', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 34872);
							s.is_empty()
						}
						['r', 'e', 'w', s @ ..] => {
							super::push(out, 34873);
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 39989);
									s.is_empty()
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
										['a', s @ ..] => {
											super::push(out, 42792);
											match s {
												[] => true,
												['g', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
													super::push(out, 44120);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 42793);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 39990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39991);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39992);
									match s {
										[] => true,
										['u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42794);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39993);
									s.is_empty()
								}
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
			super::push(out, 16870);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16871);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27005);
					match s {
						[] => true,
						['n', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 34874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 27006);
					s.is_empty()
				}
				['g', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 27007);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 27008);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 27009);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 34875);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42795);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 42796);
											match s {
												[] => true,
												['e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 44121);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 42797);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 42798);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 44122);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 42799);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42800);
											match s {
												[] => true,
												['i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 44123);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27010);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27011);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27012);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vall.
pub(crate) fn n7460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7379);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 16872);
			match s {
				[] => true,
				['e', 't', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['h', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 34876);
							s.is_empty()
						}
						['r', 'h', 'e', 'k', 'm', 'u', '\u{1}', s @ ..] => {
							super::push(out, 34877);
							s.is_empty()
						}
						['s', 'g', 'n', 'u', 'n', 'h', 'c', 'e', 'r', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 34878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wall.
pub(crate) fn n7461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7380);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16873);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 27014);
					match s {
						[] => true,
						['k', 'n', s @ ..] => {
							super::push(out, 34879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 16874);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 27015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 16875);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 16876);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 27016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 16877);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16878);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16879);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 16880);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 27018);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 16881);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 16882);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 16883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dschulsystem.
pub(crate) fn n47088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44126);
	s.is_empty()
}

/// Words ending -hschulsystem.
pub(crate) fn n47089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44127);
	s.is_empty()
}

/// Words ending -odensystem.
pub(crate) fn n47090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44128);
	s.is_empty()
}

/// Words ending -ochensystem.
pub(crate) fn n47091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44129);
	s.is_empty()
}

/// Words ending -ersystem.
pub(crate) fn n42537(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', 'l', s @ ..] => {
			super::push(out, 42816);
			s.is_empty()
		}
		['i', 'e', s @ ..] => {
			super::push(out, 42817);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 42818);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 42819);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 42820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssystem.
pub(crate) fn n36903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34973);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 40056);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 42821);
					match s {
						[] => true,
						['g', 'r', 'e', 'b', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 44130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 40057);
			match s {
				[] => true,
				['e', 'i', 'r', 't', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 42822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 40058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tsystem.
pub(crate) fn n36904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34974);
	match s {
		[] => true,
		['n', 'e', 'm', 'e', 'g', 'a', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 40059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vsystem.
pub(crate) fn n36905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34975);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 40060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zsystem.
pub(crate) fn n36906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34976);
	match s {
		[] => true,
		['i', 'e', 'h', s @ ..] => {
			super::push(out, 40061);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word epigramm.
pub(crate) fn n36924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34994);
	s.is_empty()
}

/// Words ending -ngramm.
pub(crate) fn n28311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27174);
	match s {
		[] => true,
		['e', 's', s @ ..] => {
			super::push(out, 34995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ogramm.
pub(crate) fn n28312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27175);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 34996);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 34997);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 34998);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 34999);
			match s {
				[] => true,
				['d', 'r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 40069);
					s.is_empty()
				}
				['z', 'o', 's', 's', s @ ..] => {
					super::push(out, 40070);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 42836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 35000);
			match s {
				[] => true,
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 40071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 35001);
			match s {
				[] => true,
				['i', 't', 'e', 'r', 'o', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 40072);
					s.is_empty()
				}
				['o', 'm', 's', 'u', 's', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 40073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 35002);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 35003);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 40074);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 42837);
							s.is_empty()
						}
						['m', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 42838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 40075);
					s.is_empty()
				}
				['o', 'e', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 40076);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42839);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 42840);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 44133);
									s.is_empty()
								}
								['l', 'i', 'a', 'm', '-', 'e', '\u{1}', s @ ..] => {
									super::push(out, 44134);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 44135);
									s.is_empty()
								}
								['v', 'd', 'e', '\u{1}', s @ ..] => {
									super::push(out, 44136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 44137);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 44138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', 'r', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 42841);
							s.is_empty()
						}
						['h', 'e', 's', 'n', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 42842);
							s.is_empty()
						}
						['i', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 42843);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 44139);
									s.is_empty()
								}
								['h', 'a', 'w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44715);
											s.is_empty()
										}
										['n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 44716);
											s.is_empty()
										}
										['z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 44717);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'o', 'r', 't', 'n', 'o', 'k', 'n', 'e', 't', 'r', 'u', 'b', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 44140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 42844);
							match s {
								[] => true,
								['l', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 44141);
									s.is_empty()
								}
								['r', 'o', 'f', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 44142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => super::t15::n47106(s, out),
								['o', 'd', 'a', 'h', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 44143);
									s.is_empty()
								}
								['r', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 44144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 42845);
							match s {
								[] => true,
								['l', s @ ..] => super::t15::n47109(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t15::n45648(s, out),
						['s', s @ ..] => super::t15::n45649(s, out),
						['t', s @ ..] => super::t15::n45650(s, out),
						['u', 'a', 'b', 'f', 'u', 'a', s @ ..] => super::t15::n45651(s, out),
						['z', s @ ..] => super::t15::n45652(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t15::n42566(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t15::n36934(s, out),
		_ => false,
	}
}

/// Words ending -kramm.
pub(crate) fn n17611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17166);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27176);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word churfürstenthum.
pub(crate) fn n47127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44158);
	s.is_empty()
}

/// The word kurfürstenthum.
pub(crate) fn n47128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44159);
	s.is_empty()
}

/// The word ichthyopterygium.
pub(crate) fn n42611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40116);
	s.is_empty()
}

/// Words ending -hium.
pub(crate) fn n7871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7780);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 17295);
			match s {
				[] => true,
				['s', 'i', '\u{1}', s @ ..] => {
					super::push(out, 27308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27309);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lium.
pub(crate) fn n7872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7781);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17296);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17297);
			match s {
				[] => true,
				['g', 'n', 'a', 'v', 'e', 's', s @ ..] => {
					super::push(out, 27311);
					s.is_empty()
				}
				['h', 'p', s @ ..] => {
					super::push(out, 27312);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 27313);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 35100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17298);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17299);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17300);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 27315);
					s.is_empty()
				}
				['p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 27316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mium.
pub(crate) fn n7873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7782);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 17301);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17302);
			match s {
				[] => true,
				['r', 'g', '-', s @ ..] => {
					super::push(out, 27317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 17303);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17304);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 27318);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 27319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 17305);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 17306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nium.
pub(crate) fn n7874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7783);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17307);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 27320);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27321);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 35101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17308);
			match s {
				[] => true,
				['g', 'n', s @ ..] => {
					super::push(out, 27322);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 27323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17309);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35102);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 35103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					super::push(out, 27324);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 27325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 17310);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 17311);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27326);
					match s {
						[] => true,
						['i', 'b', 'n', s @ ..] => {
							super::push(out, 35104);
							s.is_empty()
						}
						['l', 'i', s @ ..] => {
							super::push(out, 35105);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17312);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 27327);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35107);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 40117);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 40118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 27328);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 35108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35109);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 35110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27329);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 35111);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35113);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 35114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 27330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27331);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 27332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 17313);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27333);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27334);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 35115);
							match s {
								[] => true,
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 40119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 27335);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pium.
pub(crate) fn n7875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7784);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 17314);
			match s {
				[] => true,
				['z', 'i', 't', s @ ..] => {
					super::push(out, 27336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17315);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 17316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rium.
pub(crate) fn n7876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7785);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17317);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27337);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 27338);
					s.is_empty()
				}
				['i', 'l', s @ ..] => {
					super::push(out, 27339);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 27340);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27341);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35118);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 35119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 17318);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27342);
					match s {
						[] => true,
						['l', 'i', 'u', 'q', 'e', '\u{1}', s @ ..] => {
							super::push(out, 35120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27343);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 27344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17319);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 27345);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 35121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35122);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 35123);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42861);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 42862);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 44160);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['a', 'k', 'i', 'd', 's', s @ ..] => {
									super::push(out, 40120);
									s.is_empty()
								}
								['i', 'n', 'i', 'm', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'ß', 'u', 'a', '-', s @ ..] => {
											super::push(out, 42863);
											s.is_empty()
										}
										['r', 'h', s @ ..] => {
											super::push(out, 42864);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													match s {
														[] => true,
														['e', 'i', 'r', 'k', 's', s @ ..] => super::t15::n47763(s, out),
														['n', 'u', 'g', 'i', 'd', 'i', 'e', 't', 'r', 'e', 'v', '-', s @ ..] => super::t15::n47764(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 44161);
													match s {
														[] => true,
														['h', 'e', 'k', 'r', 'e', 'v', 's', s @ ..] => super::t15::n47765(s, out),
														_ => false,
													}
												}
												['t', 'i', 'e', 'h', 'd', 'n', 'u', 's', 'e', 'g', 's', s @ ..] => {
													super::push(out, 44162);
													s.is_empty()
												}
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
							super::push(out, 35124);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40121);
									s.is_empty()
								}
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
			super::push(out, 17320);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 27346);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27347);
					match s {
						[] => true,
						['o', 'b', 'o', 'h', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 35125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17321);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 27348);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 27349);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 35126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27350);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 35127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 27351);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 35128);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 40122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'a', s @ ..] => {
							super::push(out, 35129);
							match s {
								[] => true,
								['f', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 40123);
									s.is_empty()
								}
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
			super::push(out, 17322);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 17323);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 27352);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 27353);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 27354);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27355);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27356);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 17324);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 27358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 17325);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sium.
pub(crate) fn n7877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7786);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17326);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17327);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 27360);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 17328);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 17329);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 17330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tium.
pub(crate) fn n7878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7787);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17331);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 17332);
			match s {
				[] => true,
				['o', 'n', s @ ..] => {
					super::push(out, 27362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 17333);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17334);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17335);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					super::push(out, 27363);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 17336);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 17337);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35130);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 35131);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 27365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17338);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 17339);
			match s {
				[] => true,
				['e', 's', 'n', s @ ..] => {
					super::push(out, 27366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17340);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 17341);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 27367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 17342);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lloquium.
pub(crate) fn n7879(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17343);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vium.
pub(crate) fn n7880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7788);
	match s {
		[] => true,
		['i', 'v', s @ ..] => {
			super::push(out, 17345);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17346);
			s.is_empty()
		}
		['u', 'l', 'p', s @ ..] => {
			super::push(out, 17347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wium.
pub(crate) fn n7881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7789);
	s.is_empty()
}

/// Words ending -xium.
pub(crate) fn n7882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7790);
	match s {
		[] => true,
		['e', 'h', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27368);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27369);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zium.
pub(crate) fn n7883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7791);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 27371);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 17348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -entum.
pub(crate) fn n17894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17420);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 27418);
			match s {
				[] => true,
				['r', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 35155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 27419);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35156);
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 40136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 35157);
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 40137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 27420);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 27421);
			match s {
				[] => true,
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40138);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40139);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 42869);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 42870);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 44729);
													s.is_empty()
												}
												['m', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 44730);
													s.is_empty()
												}
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
							super::push(out, 40140);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 40141);
							match s {
								[] => true,
								['g', 'n', 'u', 'r', 'e', 'i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 42871);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 42872);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 42873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 40142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 35158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 27422);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 40143);
							s.is_empty()
						}
						['s', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 40144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 27423);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 35159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 27424);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 27425);
			match s {
				[] => true,
				['a', 'm', 'r', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 35161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 27426);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 35162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 27427);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35163);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 40145);
							match s {
								[] => true,
								['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 42874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 35164);
					match s {
						[] => true,
						['d', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 40146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 35165);
					match s {
						[] => true,
						['e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 40147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 35166);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 42875);
									match s {
										[] => true,
										['z', 'n', 'u', 'n', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 44163);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 'm', 'm', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 42876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 35167);
					match s {
						[] => true,
						['i', 'r', 'h', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42877);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 42878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 40148);
							match s {
								[] => true,
								['ü', 'f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44164);
											s.is_empty()
										}
										['r', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 44165);
											s.is_empty()
										}
										['ß', 'o', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 44166);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otum.
pub(crate) fn n7925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7832);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 17422);
			s.is_empty()
		}
		['t', 'k', s @ ..] => {
			super::push(out, 17423);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 27429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 17424);
			match s {
				[] => true,
				['r', 'e', 'd', 'e', 'i', 'l', 'g', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 27430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtum.
pub(crate) fn n7927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7834);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17426);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 27431);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 35168);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 27432);
					match s {
						[] => true,
						['c', 'e', 'r', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 35170);
							s.is_empty()
						}
						['t', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 35171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 27433);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 35172);
							match s {
								[] => true,
								['a', 't', 'e', 'g', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 40149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27434);
					match s {
						[] => true,
						['h', 'e', 'n', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 35173);
							s.is_empty()
						}
						['ö', 'r', '\u{1}', s @ ..] => {
							super::push(out, 35174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 27435);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35175);
							match s {
								[] => true,
								['g', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 40150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27436);
					match s {
						[] => true,
						['y', 't', 'r', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 40151);
									s.is_empty()
								}
								['ä', 'm', '\u{1}', s @ ..] => {
									super::push(out, 40152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 27437);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35176);
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'ä', s @ ..] => {
							super::push(out, 35177);
							match s {
								[] => true,
								['s', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 40154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 27438);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 35178);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 40155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', s @ ..] => {
							super::push(out, 35179);
							match s {
								[] => true,
								['i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 35180);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 40157);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42879);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', 'r', s @ ..] => {
					super::push(out, 27439);
					match s {
						[] => true,
						['i', 's', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 35181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17427);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35182);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 35183);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 40158);
									s.is_empty()
								}
								['t', 'o', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 40159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 35184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stum.
pub(crate) fn n7928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7835);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 17428);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 17429);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40160);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42880);
											match s {
												[] => true,
												['z', 'r', 'u', 'w', '\u{1}', s @ ..] => {
													super::push(out, 44167);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 42881);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'z', 'n', 'a', 'l', 'f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40161);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42882);
											match s {
												[] => true,
												['a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 44168);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 42883);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 42884);
											match s {
												[] => true,
												['n', 'u', 'g', s @ ..] => {
													super::push(out, 44169);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 42885);
											match s {
												[] => true,
												['o', 'i', 't', 'a', 'l', 'u', 'p', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 44170);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 42886);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 40162);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 40163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 35185);
							match s {
								[] => true,
								['ö', 'm', '\u{1}', s @ ..] => {
									super::push(out, 40164);
									s.is_empty()
								}
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
			super::push(out, 17430);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27441);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 27442);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 35186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 17431);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 17432);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27443);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttum.
pub(crate) fn n7929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7836);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 17433);
			match s {
				[] => true,
				['i', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 27444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17434);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 27445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 17435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utum.
pub(crate) fn n7930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7837);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 17436);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 17437);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ztum.
pub(crate) fn n7932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7839);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 17438);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ersatzfahrplan.
pub(crate) fn n47142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44171);
	s.is_empty()
}

/// Words ending -berschreiben.
pub(crate) fn n47768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44731);
	s.is_empty()
}

/// Words ending -terschreiben.
pub(crate) fn n47769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44732);
	s.is_empty()
}

/// Words ending -verschreiben.
pub(crate) fn n47770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44733);
	s.is_empty()
}

/// Words ending -liden.
pub(crate) fn n18135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17653);
	s.is_empty()
}

/// Words ending -niden.
pub(crate) fn n18136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17654);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 27604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -riden.
pub(crate) fn n18137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17655);
	s.is_empty()
}

/// Words ending -lden.
pub(crate) fn n8101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8004);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17656);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17657);
			match s {
				[] => true,
				['b', 'n', s @ ..] => {
					super::push(out, 27605);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 35273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 17658);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 27606);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 27607);
					match s {
						[] => true,
						['c', 's', 'r', s @ ..] => {
							super::push(out, 35274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mden.
pub(crate) fn n8102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8005);
	s.is_empty()
}

/// Words ending -nden.
pub(crate) fn n8103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8006);
	match s {
		[] => true,
		['e', 'p', s @ ..] => {
			super::push(out, 17659);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 27608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oden.
pub(crate) fn n8104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8007);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 17661);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 27609);
					match s {
						[] => true,
						['e', 'k', s @ ..] => {
							super::push(out, 35275);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 40206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 27610);
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 35276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 17662);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17663);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 17664);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17665);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 27613);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rden.
pub(crate) fn n8105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8008);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17666);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17667);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 27615);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 35277);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 40207);
									match s {
										[] => true,
										['s', 'e', 'b', 'n', s @ ..] => {
											super::push(out, 42921);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 35278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 17668);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17669);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 27616);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27617);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 27618);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27619);
					match s {
						[] => true,
						['e', 'u', s @ ..] => {
							super::push(out, 35279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27620);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40208);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 17670);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27621);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 27622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uden.
pub(crate) fn n8106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8009);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 17671);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17672);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 27623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öden.
pub(crate) fn n8107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8010);
	s.is_empty()
}

/// Words ending -üden.
pub(crate) fn n8108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8011);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 17673);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 17674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ufen.
pub(crate) fn n8123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8026);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17699);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 27648);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 27649);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27650);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27651);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27652);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17700);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 17701);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 17702);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rigen.
pub(crate) fn n18207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17724);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 27693);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 35316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tigen.
pub(crate) fn n18208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17725);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 27694);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 35317);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lgen.
pub(crate) fn n8130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8033);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17726);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ngen.
pub(crate) fn n8131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8034);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17728);
			match s {
				[] => true,
				['f', 'r', 'e', 't', s @ ..] => {
					super::push(out, 27695);
					s.is_empty()
				}
				['l', 'r', s @ ..] => {
					super::push(out, 27696);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							super::push(out, 35318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 27697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17729);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', 'n', 'a', s @ ..] => {
							super::push(out, 35319);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 35320);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 42924);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42925);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17730);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 27698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 17731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ogen.
pub(crate) fn n8132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8035);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 17732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27699);
					s.is_empty()
				}
				['e', 'd', s @ ..] => {
					super::push(out, 27700);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 35321);
							s.is_empty()
						}
						['p', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 35322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'i', 'n', 'e', 'z', 'm', 'u', 'k', 'r', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 35323);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 35324);
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
								['g', 'i', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 40219);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 40220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'o', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 35325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'd', 'i', 'p', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 27701);
					s.is_empty()
				}
				['r', 'e', 'i', 'p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 27702);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27703);
					match s {
						[] => true,
						['i', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 35326);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35327);
							s.is_empty()
						}
						['u', 't', s @ ..] => {
							super::push(out, 35328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', 'm', 'a', 'g', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 35329);
							s.is_empty()
						}
						['o', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 35330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', 'l', 'f', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 27704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17733);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 17734);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17735);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 17736);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27706);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 27707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27708);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 27709);
					s.is_empty()
				}
				['s', 'h', 'c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 27710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 17738);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 17739);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 17740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rgen.
pub(crate) fn n8133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8036);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17741);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27711);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17742);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27713);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 27714);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 17743);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 27716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sgen.
pub(crate) fn n8134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8037);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 17744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tgen.
pub(crate) fn n8135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8038);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 17745);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugen.
pub(crate) fn n8136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8039);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 17746);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ygen.
pub(crate) fn n8137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8040);
	match s {
		[] => true,
		['x', 'o', '\u{1}', s @ ..] => {
			super::push(out, 17747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ägen.
pub(crate) fn n8138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8041);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 17748);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ögen.
pub(crate) fn n8139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8042);
	match s {
		[] => true,
		['m', 'r', 'e', 'v', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 27719);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 27720);
					match s {
						[] => true,
						['c', 'a', 's', s @ ..] => {
							super::push(out, 35331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					super::push(out, 27721);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 27722);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35332);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 35333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 35334);
							match s {
								[] => true,
								['n', 'u', s @ ..] => {
									match s {
										[] => true,
										['d', 'i', s @ ..] => {
											super::push(out, 42926);
											s.is_empty()
										}
										['l', 'h', s @ ..] => {
											super::push(out, 42927);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42928);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 35335);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35336);
							s.is_empty()
						}
						['t', 'f', s @ ..] => {
							super::push(out, 35337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ügen.
pub(crate) fn n8140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8043);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 17749);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 17751);
			match s {
				[] => true,
				['g', 'r', 'e', 'v', 's', s @ ..] => {
					super::push(out, 27724);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -achen.
pub(crate) fn n18236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17752);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 27725);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 27726);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 35338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 27727);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 27728);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 35339);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 27729);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 27730);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 35340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 27731);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 35341);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 35342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 27732);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 35343);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 40223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 35344);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 35345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 27733);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hadchen.
pub(crate) fn n37307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35347);
	s.is_empty()
}

/// Words ending -ndchen.
pub(crate) fn n28925(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 35348);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 40224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', 'l', s @ ..] => {
			super::push(out, 35349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ädchen.
pub(crate) fn n28926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27736);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 35350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -echen.
pub(crate) fn n18239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17755);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 27737);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40225);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 40226);
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
				['b', s @ ..] => {
					super::push(out, 35351);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							super::push(out, 40227);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 40228);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 40229);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 40230);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 44180);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44181);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 44734);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 44735);
													match s {
														[] => true,
														['l', 'b', '\u{1}', s @ ..] => super::t15::n48004(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 44182);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 44183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 40231);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 40232);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 42930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 35352);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 35353);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 35354);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 35355);
					match s {
						[] => true,
						['s', 'r', s @ ..] => {
							super::push(out, 40233);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 44184);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 44185);
											s.is_empty()
										}
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
					super::push(out, 35356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 35357);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 35358);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fchen.
pub(crate) fn n18240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17756);
	s.is_empty()
}

/// Words ending -gchen.
pub(crate) fn n18241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17757);
	s.is_empty()
}

/// Words ending -hchen.
pub(crate) fn n18242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17758);
	s.is_empty()
}

/// Words ending -ichen.
pub(crate) fn n18243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17759);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 27738);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 35359);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 35360);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40234);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 40235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 35361);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 35362);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 40236);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 42931);
									s.is_empty()
								}
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
			super::push(out, 27739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kchen.
pub(crate) fn n18244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17760);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 27740);
			match s {
				[] => true,
				['ü', 't', 's', 'h', s @ ..] => {
					super::push(out, 35363);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 40237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 27741);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 35364);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lchen.
pub(crate) fn n18245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17761);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 27742);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 35365);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 40238);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 40239);
							s.is_empty()
						}
						['u', 'a', s @ ..] => {
							super::push(out, 40240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 35366);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 35367);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 40241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 35368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 't', 's', s @ ..] => {
			super::push(out, 27743);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 27744);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					super::push(out, 35370);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 40242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mchen.
pub(crate) fn n18246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17762);
	match s {
		[] => true,
		['m', 'ä', 'l', s @ ..] => {
			super::push(out, 27745);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nchen.
pub(crate) fn n18247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17763);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 27746);
			match s {
				[] => true,
				['ä', 'b', s @ ..] => {
					super::push(out, 35371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 27747);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 27748);
			match s {
				[] => true,
				['ä', 'm', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40243);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 42932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'g', s @ ..] => {
							super::push(out, 40244);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 42933);
									s.is_empty()
								}
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
			super::push(out, 27749);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 27750);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 35372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 27751);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 35373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ochen.
pub(crate) fn n18248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17764);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 27752);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 35374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 27753);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 27754);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 35375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 27755);
			match s {
				[] => true,
				['k', 'n', 'e', 't', s @ ..] => {
					super::push(out, 35376);
					match s {
						[] => true,
						['n', 'a', 'f', s @ ..] => {
							super::push(out, 40245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 27756);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 27757);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 27758);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 35377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pchen.
pub(crate) fn n18249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17765);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 27759);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 35378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rchen.
pub(crate) fn n18250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17766);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 27760);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 35379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 27761);
			match s {
				[] => true,
				['t', 't', s @ ..] => {
					super::push(out, 35380);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							super::push(out, 40246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 27762);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 35381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 27763);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 35382);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -schen.
pub(crate) fn n18251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17767);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 27764);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 27765);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 27766);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 27767);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 27768);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35383);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 35384);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 35385);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 35386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 27769);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 27770);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 27771);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 35387);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 35388);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 35389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 27772);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 35390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 35391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 35392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 27773);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 27774);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 27775);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 35394);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 40249);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 42934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 40250);
							match s {
								[] => true,
								['t', 'a', s @ ..] => {
									super::push(out, 42935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 40251);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 35395);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 35396);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 35397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 27776);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 35398);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 35399);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 35400);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 35401);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 35402);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 35403);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 40253);
							match s {
								[] => true,
								['ß', s @ ..] => {
									super::push(out, 42937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 40254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 27777);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 27778);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 35404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tchen.
pub(crate) fn n18252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17768);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 27779);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 35405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 27780);
			s.is_empty()
		}
		['s', 'r', 'ü', 'd', s @ ..] => {
			super::push(out, 27781);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 27782);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35406);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 40255);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 40256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'l', 'b', 'l', s @ ..] => {
					super::push(out, 35407);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							super::push(out, 40257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 27783);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 27784);
			s.is_empty()
		}
		['ü', 'm', s @ ..] => {
			super::push(out, 27785);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uchen.
pub(crate) fn n18253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17769);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 27786);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 35409);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 40258);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 40259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 35410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 27787);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 27788);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 27789);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 27790);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 27791);
			match s {
				[] => true,
				['p', 's', 'n', 'a', s @ ..] => {
					super::push(out, 35411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 27792);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 35412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 27793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zchen.
pub(crate) fn n18254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17770);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 27794);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 35413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 27795);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35414);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 40260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 35415);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 35416);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 40261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßchen.
pub(crate) fn n18255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17771);
	s.is_empty()
}

/// Words ending -ächen.
pub(crate) fn n18256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17772);
	s.is_empty()
}

/// Words ending -ehen.
pub(crate) fn n8143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8046);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 27796);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27797);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40262);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 40263);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 40264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 35417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 27798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 17773);
			match s {
				[] => true,
				['c', 's', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 35418);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35419);
							s.is_empty()
						}
						['t', 'i', s @ ..] => {
							super::push(out, 35420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17774);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 17775);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 27799);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17776);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 17777);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 27801);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27802);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							super::push(out, 35421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 17778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ghen.
pub(crate) fn n8144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8047);
	s.is_empty()
}

/// Words ending -ihen.
pub(crate) fn n8145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8048);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17779);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27803);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27804);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ohen.
pub(crate) fn n8146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8049);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 17780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -phen.
pub(crate) fn n8147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8050);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17781);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17782);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 17783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -then.
pub(crate) fn n8148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8051);
	match s {
		[] => true,
		['e', 'y', s @ ..] => {
			super::push(out, 17784);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 17785);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 17786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uhen.
pub(crate) fn n8149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8052);
	s.is_empty()
}

/// Words ending -ähen.
pub(crate) fn n8150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8053);
	s.is_empty()
}

/// Words ending -öhen.
pub(crate) fn n8151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8054);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 17787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ühen.
pub(crate) fn n8152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8055);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 27805);
					s.is_empty()
				}
				['g', 'n', 'e', s @ ..] => {
					super::push(out, 27806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eken.
pub(crate) fn n8172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8075);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 17822);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 17823);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 17824);
			match s {
				[] => true,
				['n', 'ä', 'm', '\u{1}', s @ ..] => {
					super::push(out, 27855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 17825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iken.
pub(crate) fn n8173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8076);
	match s {
		[] => true,
		['l', 'l', 'i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 17826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kken.
pub(crate) fn n8174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8077);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17827);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 27856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lken.
pub(crate) fn n8175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8078);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27857);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 27858);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 27859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 27860);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 27861);
					match s {
						[] => true,
						['o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 35442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27862);
					match s {
						[] => true,
						['o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 35443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 27863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17830);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17831);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 27864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 17832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mken.
pub(crate) fn n8176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8079);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nken.
pub(crate) fn n8177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8080);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17834);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27865);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 27866);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 35444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 27867);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27868);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27869);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 27870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17835);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 27871);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 35445);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 35446);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 35447);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40274);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 40275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 35448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17836);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 27873);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 35449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27874);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17837);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27876);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 17838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oken.
pub(crate) fn n8178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8081);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 17839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rken.
pub(crate) fn n8179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8082);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17840);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17841);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 27878);
					match s {
						[] => true,
						['t', 'ü', 'b', '\u{1}', s @ ..] => {
							super::push(out, 35450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 17842);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17843);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 27879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 17844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sken.
pub(crate) fn n8180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8083);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17845);
			s.is_empty()
		}
		['t', 'e', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 17846);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 17847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uken.
pub(crate) fn n8181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8084);
	match s {
		[] => true,
		['a', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 17848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üken.
pub(crate) fn n8183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8086);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 17849);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kommen.
pub(crate) fn n29110(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 35469);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 35470);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 42943);
							s.is_empty()
						}
						['s', 'g', 'a', s @ ..] => {
							super::push(out, 42944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 35471);
			s.is_empty()
		}
		['f', 'u', 'a', 't', s @ ..] => {
			super::push(out, 35472);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 35473);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 35474);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 35475);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 35476);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 35477);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 40280);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 40281);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 40282);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 42945);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 44186);
									s.is_empty()
								}
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
					super::push(out, 40283);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 40284);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 42946);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 42947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 35478);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 35479);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 35480);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wurzelnomen.
pub(crate) fn n29112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27916);
	s.is_empty()
}

/// Words ending -onomen.
pub(crate) fn n29113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27917);
	s.is_empty()
}

/// Words ending -snomen.
pub(crate) fn n29114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27918);
	s.is_empty()
}

/// Words ending -änomen.
pub(crate) fn n29115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27919);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 35481);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 40285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 35482);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 40286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oomen.
pub(crate) fn n18389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17900);
	s.is_empty()
}

/// Words ending -romen.
pub(crate) fn n18390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17901);
	s.is_empty()
}

/// Words ending -rmen.
pub(crate) fn n8200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8103);
	s.is_empty()
}

/// Words ending -smen.
pub(crate) fn n8201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17902);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tmen.
pub(crate) fn n8202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8105);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17903);
			s.is_empty()
		}
		_ => false,
	}
}

