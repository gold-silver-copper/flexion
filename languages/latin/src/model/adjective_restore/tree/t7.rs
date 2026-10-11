use flexion::Path;

/// Words ending -ficans.
pub(crate) fn n2383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2114);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8307);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12039);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8308);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12041);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8309);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12043);
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
							super::push(out, 12044);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8310);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8311);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12046);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12047);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12048);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12049);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8312);
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
				['r', s @ ..] => {
					super::push(out, 8313);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12051);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12052);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12053);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12054);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12055);
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
						['t', s @ ..] => {
							super::push(out, 12056);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8314);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8315);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12058);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12059);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14925);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 14926);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14927);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 16807);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 16808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12060);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14928);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 14929);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12061);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12062);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14930);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14931);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8316);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12063);
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
							super::push(out, 12064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -plicans.
pub(crate) fn n5802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4694);
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
			super::push(out, 8317);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12066);
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
			super::push(out, 8318);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12067);
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
			super::push(out, 8319);
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
			super::push(out, 8320);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12068);
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

/// Words ending -amicans.
pub(crate) fn n5804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4696);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -emicans.
pub(crate) fn n5805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4697);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -imicans.
pub(crate) fn n5806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4698);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 8321);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8322);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8323);
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

/// Words ending -rmicans.
pub(crate) fn n5807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4699);
	match s {
		[] => true,
		['o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nicans.
pub(crate) fn n2387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2118);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4700);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4701);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4702);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -picans.
pub(crate) fn n2388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2119);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8324);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8325);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ricans.
pub(crate) fn n2389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4703);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8326);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12069);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4704);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8327);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8328);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8329);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12071);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4705);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8330);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12073);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12074);
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
					super::push(out, 8331);
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
					super::push(out, 8332);
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
		['g', s @ ..] => {
			super::push(out, 4706);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4707);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4708);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4709);
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
			super::push(out, 4710);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 8333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicans.
pub(crate) fn n2390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2121);
	s.is_empty()
}

/// Words ending -ticans.
pub(crate) fn n2391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2122);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4711);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4712);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4713);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8334);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vicans.
pub(crate) fn n2392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2123);
	match s {
		[] => true,
		['a', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4715);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mulans.
pub(crate) fn n2559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2253);
	match s {
		[] => true,
		['e', 'a', s @ ..] => {
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
		['i', 's', s @ ..] => {
			super::push(out, 4863);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8398);
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
		['u', 'c', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ipulans.
pub(crate) fn n6114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4865);
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

/// Words ending -opulans.
pub(crate) fn n6115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4866);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8399);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 8400);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12097);
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

/// Words ending -sulans.
pub(crate) fn n2561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2255);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4867);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 4868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tulans.
pub(crate) fn n2562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2256);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4869);
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
			super::push(out, 4870);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4871);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8401);
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
							super::push(out, 12098);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 14938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 12099);
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

/// Words ending -minans.
pub(crate) fn n2617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2301);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4903);
			match s {
				[] => true,
				['l', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 4904);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8438);
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
					super::push(out, 8439);
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
					super::push(out, 8440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12119);
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
		['i', s @ ..] => {
			super::push(out, 4905);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8441);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8442);
					match s {
						[] => true,
						['c', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 4906);
			match s {
				[] => true,
				['u', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4907);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 8443);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8444);
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
					super::push(out, 8445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12120);
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
		['r', s @ ..] => {
			super::push(out, 4909);
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
									super::push(out, 14939);
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
		['u', s @ ..] => {
			super::push(out, 4910);
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
		_ => false,
	}
}

/// Words ending -ctinans.
pub(crate) fn n6204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4919);
	s.is_empty()
}

/// Words ending -ntinans.
pub(crate) fn n6205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4920);
	s.is_empty()
}

/// Words ending -stinans.
pub(crate) fn n6206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4921);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8447);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 12121);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8448);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14940);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14941);
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
					super::push(out, 12124);
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

/// Words ending -utinans.
pub(crate) fn n6207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4922);
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

/// Words ending -uinans.
pub(crate) fn n2621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2305);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8449);
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
					super::push(out, 8450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vinans.
pub(crate) fn n2622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2306);
	s.is_empty()
}

/// Words ending -merans.
pub(crate) fn n2673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2340);
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
		['u', s @ ..] => {
			super::push(out, 4959);
			match s {
				[] => true,
				['n', s @ ..] => {
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
							super::push(out, 12131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12132);
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
		['y', s @ ..] => {
			super::push(out, 4960);
			match s {
				[] => true,
				['l', 'o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nerans.
pub(crate) fn n2674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2341);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4961);
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
						['e', s @ ..] => {
							super::push(out, 12133);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 14943);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14944);
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
			super::push(out, 4962);
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
			super::push(out, 4963);
			match s {
				[] => true,
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

/// Words ending -perans.
pub(crate) fn n2675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2342);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 4964);
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
						['p', 'o', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 4965);
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
			super::push(out, 4966);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8488);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8489);
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
					super::push(out, 8490);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8491);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8492);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8493);
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
			super::push(out, 4967);
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

/// Words ending -serans.
pub(crate) fn n2676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2343);
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

/// Words ending -terans.
pub(crate) fn n2677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2344);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4968);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12134);
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

/// Words ending -uerans.
pub(crate) fn n2678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2345);
	s.is_empty()
}

/// Words ending -verans.
pub(crate) fn n2679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2346);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 's', 'd', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word corporans.
pub(crate) fn n19449(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word accorporans.
pub(crate) fn n19450(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word adcorporans.
pub(crate) fn n19451(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word incorporans.
pub(crate) fn n19452(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rorans.
pub(crate) fn n2700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2366);
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

/// Words ending -torans.
pub(crate) fn n2701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2367);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12143);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 12144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8503);
					match s {
						[] => true,
						['a', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 4981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vorans.
pub(crate) fn n2702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2368);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4982);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4983);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8504);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8505);
					s.is_empty()
				}
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'd', 'i', 'c', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xorans.
pub(crate) fn n2703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2369);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word reflagitans.
pub(crate) fn n26237(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word efflagitans.
pub(crate) fn n26238(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -oagitans.
pub(crate) fn n12447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8540);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xagitans.
pub(crate) fn n12448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8541);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -igitans.
pub(crate) fn n6497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5042);
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

/// Words ending -ogitans.
pub(crate) fn n6498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5043);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8542);
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rgitans.
pub(crate) fn n6499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5044);
	s.is_empty()
}

/// Words ending -ugitans.
pub(crate) fn n6500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5045);
	s.is_empty()
}

/// Words ending -litans.
pub(crate) fn n2765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2426);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5046);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5047);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5048);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5049);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8544);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12162);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12163);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12164);
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
					super::push(out, 8545);
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
			super::push(out, 5050);
			match s {
				[] => true,
				['e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5051);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mitans.
pub(crate) fn n2766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2427);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5052);
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
									super::push(out, 14949);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14950);
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
			super::push(out, 5053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8546);
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
			super::push(out, 5054);
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
			super::push(out, 5055);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nitans.
pub(crate) fn n2767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2428);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5056);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5057);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pitans.
pub(crate) fn n2768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2429);
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

/// Words ending -ritans.
pub(crate) fn n2769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2430);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5059);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5060);
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5061);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5062);
			s.is_empty()
		}
		['r', 'i', s @ ..] => {
			super::push(out, 5063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sitans.
pub(crate) fn n2770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2431);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5064);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5065);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5066);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8547);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5067);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5068);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5069);
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

/// Words ending -titans.
pub(crate) fn n2771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2432);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12165);
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
					super::push(out, 8550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5070);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uitans.
pub(crate) fn n2772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2433);
	match s {
		[] => true,
		['q', 'e', s @ ..] => {
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

/// Words ending -vitans.
pub(crate) fn n2773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2434);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5071);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5072);
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
			super::push(out, 5073);
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

/// Words ending -escens.
pub(crate) fn n2891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2523);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5164);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8617);
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
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8618);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12191);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12192);
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
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8619);
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
					super::push(out, 8620);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8621);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12193);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12194);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14957);
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
		['c', s @ ..] => {
			super::push(out, 5165);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8623);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8624);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12196);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8625);
					match s {
						[] => true,
						['u', 'd', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 8626);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14958);
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
					super::push(out, 8627);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12198);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 14959);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14960);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 14961);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5166);
			match s {
				[] => true,
				['i', 'r', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12200);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8628);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
				['r', s @ ..] => {
					super::push(out, 8629);
					match s {
						[] => true,
						['a', s @ ..] => {
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
						['o', 's', s @ ..] => {
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
					super::push(out, 8630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5167);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5168);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 8631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 12201);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => super::t9::n26306(s, out),
								_ => false,
							}
						}
						['f', s @ ..] => super::t9::n19688(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t9::n6778(s, out),
		['l', s @ ..] => super::t9::n6779(s, out),
		['m', s @ ..] => super::t9::n6780(s, out),
		['n', s @ ..] => super::t9::n6781(s, out),
		['p', s @ ..] => super::t9::n6782(s, out),
		['r', s @ ..] => super::t9::n6783(s, out),
		['s', s @ ..] => super::t9::n6784(s, out),
		['t', s @ ..] => super::t9::n6785(s, out),
		['u', s @ ..] => super::t9::n6786(s, out),
		['v', s @ ..] => super::t9::n6787(s, out),
		_ => false,
	}
}

/// Words ending -miscens.
pub(crate) fn n6793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5184);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 8675);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8676);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12238);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12239);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14983);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14984);
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
		['m', s @ ..] => {
			super::push(out, 8677);
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
			super::push(out, 8678);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8679);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12241);
					match s {
						[] => true,
						['p', s @ ..] => {
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
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12242);
					match s {
						[] => true,
						['d', s @ ..] => {
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
									super::push(out, 16815);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -riscens.
pub(crate) fn n6796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5187);
	s.is_empty()
}

/// Words ending -tiscens.
pub(crate) fn n6797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5188);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8682);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viscens.
pub(crate) fn n6798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5189);
	s.is_empty()
}

/// Words ending -oscens.
pub(crate) fn n2893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2525);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8684);
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
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 14985);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'c', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8685);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
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
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12243);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ividens.
pub(crate) fn n6899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5250);
	match s {
		[] => true,
		['d', s @ ..] => {
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

/// Words ending -nvidens.
pub(crate) fn n6900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5251);
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

/// Words ending -ovidens.
pub(crate) fn n6901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5252);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rvidens.
pub(crate) fn n6902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5253);
	match s {
		[] => true,
		['e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pendens.
pub(crate) fn n6910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5258);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8717);
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', s @ ..] => {
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
		['n', 'i', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 8718);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -prendens.
pub(crate) fn n12912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8720);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12256);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 14992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tendens.
pub(crate) fn n6912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5260);
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
			super::push(out, 8721);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
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

/// Words ending -vendens.
pub(crate) fn n6913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5261);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8722);
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

/// Words ending -indens.
pub(crate) fn n2927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2555);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8723);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8724);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ondens.
pub(crate) fn n2928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2556);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5263);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12257);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 14993);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14994);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 12258);
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
		['r', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5264);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8725);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12259);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 12260);
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
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -undens.
pub(crate) fn n2929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2557);
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
				['f', s @ ..] => {
					super::push(out, 8726);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
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
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8727);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12262);
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
					super::push(out, 8728);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 8729);
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
		['t', s @ ..] => {
			super::push(out, 5265);
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
				['n', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -ringens.
pub(crate) fn n7007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5301);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 8755);
			match s {
				[] => true,
				['c', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12270);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14996);
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
					super::push(out, 12271);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12272);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14997);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14998);
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
		['t', s @ ..] => {
			super::push(out, 8756);
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
							super::push(out, 14999);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 16817);
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
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15000);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16818);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16819);
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
		_ => false,
	}
}

/// Words ending -ungens.
pub(crate) fn n2964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2582);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8757);
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
		['i', s @ ..] => {
			super::push(out, 5304);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 8758);
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
					super::push(out, 8759);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8760);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 12273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8761);
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
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5305);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5306);
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
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
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

/// The word tabefaciens.
pub(crate) fn n35186(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -gefaciens.
pub(crate) fn n20006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12283);
	match s {
		[] => true,
		['r', 'e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lefaciens.
pub(crate) fn n20007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12284);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15003);
			match s {
				[] => true,
				['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 15004);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mefaciens.
pub(crate) fn n20008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12285);
	match s {
		[] => true,
		['e', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nefaciens.
pub(crate) fn n20009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12286);
	match s {
		[] => true,
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pefaciens.
pub(crate) fn n20010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12287);
	match s {
		[] => true,
		['e', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 't', 's', s @ ..] => {
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

/// Words ending -refaciens.
pub(crate) fn n20011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12288);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16821);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16822);
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
		['b', s @ ..] => {
			super::push(out, 15006);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15007);
			match s {
				[] => true,
				['o', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15008);
			match s {
				[] => true,
				['u', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 15009);
			match s {
				[] => true,
				['d', 'b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uefaciens.
pub(crate) fn n20013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12289);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 15010);
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 15011);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 15012);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16823);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16824);
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
		_ => false,
	}
}

/// Words ending -vefaciens.
pub(crate) fn n20014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12290);
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

/// The word lucrifaciens.
pub(crate) fn n13157(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lfaciens.
pub(crate) fn n13158(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ofaciens.
pub(crate) fn n13159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8776);
	match s {
		[] => true,
		['r', 'e', 'a', '\u{1}', s @ ..] => {
			super::push(out, 12291);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word satisfaciens.
pub(crate) fn n13160(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -iaciens.
pub(crate) fn n7057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5321);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
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
		_ => false,
	}
}

/// The word laciens.
pub(crate) fn n7058(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word acciens.
pub(crate) fn n2980(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word speciens.
pub(crate) fn n2981(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -iciens.
pub(crate) fn n2982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2591);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5322);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5323);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5324);
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
			super::push(out, 5325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8777);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8778);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8780);
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
			super::push(out, 5326);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8781);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12292);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 12293);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12294);
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
					super::push(out, 8782);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 8783);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12296);
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
					super::push(out, 8784);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8785);
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
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8786);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 8787);
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
					super::push(out, 8788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12297);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12298);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 12299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12300);
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
					super::push(out, 8789);
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
					super::push(out, 8790);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8791);
					match s {
						[] => true,
						['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 5328);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8793);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
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
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5329);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5331);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8795);
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
		['p', s @ ..] => super::t9::n7070(s, out),
		['r', s @ ..] => {
			super::push(out, 5333);
			match s {
				[] => true,
				['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -lciens.
pub(crate) fn n2983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2592);
	match s {
		[] => true,
		['u', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nciens.
pub(crate) fn n2984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2593);
	match s {
		[] => true,
		['i', 'v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12309);
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
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ociens.
pub(crate) fn n2985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2594);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5334);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5335);
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

/// Words ending -rciens.
pub(crate) fn n2986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2595);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5336);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12310);
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
				['s', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 5337);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12311);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15018);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15019);
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
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sciens.
pub(crate) fn n2987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2596);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5338);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5339);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8798);
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

/// The word exciens.
pub(crate) fn n2988(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -muniens.
pub(crate) fn n7131(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12334);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12335);
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
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word fruniens.
pub(crate) fn n7133(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word adesuriens.
pub(crate) fn n20190(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -turiens.
pub(crate) fn n7164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5381);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8856);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15029);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8857);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 8858);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cutiens.
pub(crate) fn n7178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5386);
	match s {
		[] => true,
		['r', 'e', 'p', s @ ..] => {
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

/// Words ending -viloquens.
pub(crate) fn n20411(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15036);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 15037);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lloquens.
pub(crate) fn n13752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8957);
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

/// Words ending -mloquens.
pub(crate) fn n13753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8958);
	match s {
		[] => true,
		['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word conloquens.
pub(crate) fn n13754(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -oloquens.
pub(crate) fn n13755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8959);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rloquens.
pub(crate) fn n13756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8960);
	match s {
		[] => true,
		['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rquens.
pub(crate) fn n3203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2731);
	match s {
		[] => true,
		['o', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8961);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 12385);
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
				['s', 'i', 'd', '\u{1}', s @ ..] => {
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

/// Words ending -onicus.
pub(crate) fn n3328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2825);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5653);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9061);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9062);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9064);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9065);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9067);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5655);
			match s {
				[] => true,
				['e', 'l', 'a', 'c', 'o', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9070);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5656);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9072);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', s @ ..] => {
			super::push(out, 5657);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5658);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9074);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9075);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12419);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'l', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5659);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5660);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9076);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9078);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5661);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9080);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5662);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9082);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12421);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', 'a', 's', 'o', s @ ..] => {
			super::push(out, 5663);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5664);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -metricus.
pub(crate) fn n13993(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 12430);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 12431);
			match s {
				[] => true,
				['y', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12432);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15045);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12433);
			match s {
				[] => true,
				['e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stricus.
pub(crate) fn n7782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5691);
	match s {
		[] => true,
		['a', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'l', s @ ..] => {
			super::push(out, 9112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uricus.
pub(crate) fn n3346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2843);
	match s {
		[] => true,
		['a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5692);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5693);
			match s {
				[] => true,
				['s', 'd', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yricus.
pub(crate) fn n3347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2844);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5695);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// The word mathematicus.
pub(crate) fn n35194(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -naticus.
pub(crate) fn n7800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5707);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9135);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9136);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9137);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oaticus.
pub(crate) fn n7801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5708);
	s.is_empty()
}

/// Words ending -paticus.
pub(crate) fn n7802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5709);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9139);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 9140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -raticus.
pub(crate) fn n7803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5710);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9141);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 9142);
			match s {
				[] => true,
				['o', 'n', 'h', 'c', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 12442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9143);
			match s {
				[] => true,
				['a', 'u', 'q', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 9144);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9145);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -taticus.
pub(crate) fn n7804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5711);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 9147);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uaticus.
pub(crate) fn n7805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5712);
	s.is_empty()
}

/// Words ending -vaticus.
pub(crate) fn n7806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5713);
	s.is_empty()
}

/// Words ending -cticus.
pub(crate) fn n3355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2852);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5714);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9149);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5715);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9151);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12443);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5716);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5717);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'a', s @ ..] => {
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
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eticus.
pub(crate) fn n3356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2853);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5719);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5720);
			s.is_empty()
		}
		['d', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 5721);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5722);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9153);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9154);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5723);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9156);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9157);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9158);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12445);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'v', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 5724);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5725);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5726);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9159);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5727);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9161);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12447);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12448);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12449);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 12450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9162);
					match s {
						[] => true,
						['a', 'm', 'o', s @ ..] => {
							super::push(out, 12451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5728);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5729);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9163);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9164);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 12452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9165);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9166);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 9167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5730);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iticus.
pub(crate) fn n3357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2854);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 5732);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5733);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9168);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12453);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12454);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9169);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12456);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15056);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15057);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15058);
									s.is_empty()
								}
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
			super::push(out, 5734);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9170);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9171);
					match s {
						[] => true,
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 12457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5735);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9173);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5736);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9175);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5737);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9177);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12458);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9178);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lticus.
pub(crate) fn n3358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2855);
	s.is_empty()
}

/// Words ending -nticus.
pub(crate) fn n3359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2856);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5738);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9179);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9180);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9181);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 15059);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 15060);
									s.is_empty()
								}
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
			super::push(out, 5739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oticus.
pub(crate) fn n3360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2857);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5740);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5741);
			s.is_empty()
		}
		['e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 5742);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5743);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5744);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9182);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5745);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9184);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5746);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9186);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 12460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9187);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9188);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 5747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pticus.
pub(crate) fn n3361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2858);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5748);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12461);
							match s {
								[] => true,
								['r', 't', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5749);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5750);
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
		['y', s @ ..] => {
			super::push(out, 5751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sticus.
pub(crate) fn n3362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2859);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9190);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5753);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9191);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12462);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5754);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9193);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5755);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9195);
					s.is_empty()
				}
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9196);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9197);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9198);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9199);
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
		_ => false,
	}
}

/// Words ending -tticus.
pub(crate) fn n3363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2860);
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
		_ => false,
	}
}

/// Words ending -uticus.
pub(crate) fn n3364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2861);
	match s {
		[] => true,
		['a', 'n', 'o', s @ ..] => {
			super::push(out, 5756);
			match s {
				[] => true,
				['r', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yticus.
pub(crate) fn n3365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2862);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					super::push(out, 9201);
					match s {
						[] => true,
						['r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 12464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12465);
							match s {
								[] => true,
								['r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 15061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'h', 'c', 'c', 'a', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -candus.
pub(crate) fn n3430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2911);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5802);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9215);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9216);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9217);
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
				['v', s @ ..] => {
					super::push(out, 9218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5803);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5804);
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
						['e', s @ ..] => {
							super::push(out, 12471);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5805);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9219);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12472);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
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
						['f', s @ ..] => {
							super::push(out, 12473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9220);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12474);
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
				['r', s @ ..] => {
					super::push(out, 9222);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15062);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
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
					super::push(out, 9223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12475);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15064);
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
							super::push(out, 12476);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15065);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15066);
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
							super::push(out, 12477);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12478);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'b', 'u', 's', '\u{1}', s @ ..] => {
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
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t9::n7940(s, out),
		['l', s @ ..] => {
			super::push(out, 5807);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
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
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
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
						['s', s @ ..] => {
							super::push(out, 12506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15106);
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
		['n', s @ ..] => super::t9::n7942(s, out),
		['o', s @ ..] => super::t9::n7943(s, out),
		['r', s @ ..] => super::t9::n7944(s, out),
		['s', s @ ..] => super::t9::n7945(s, out),
		['u', s @ ..] => {
			super::push(out, 5812);
			match s {
				[] => true,
				['d', s @ ..] => super::t9::n14198(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dandus.
pub(crate) fn n3431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2912);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5813);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5814);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9245);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15111);
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
				['o', s @ ..] => {
					super::push(out, 9246);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9247);
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
				['r', s @ ..] => {
					super::push(out, 9248);
					match s {
						[] => true,
						['e', 'h', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9249);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12522);
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
			super::push(out, 5815);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9250);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 15112);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15113);
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
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9251);
					match s {
						[] => true,
						['f', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9252);
					match s {
						[] => true,
						['i', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9253);
					match s {
						[] => true,
						['o', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15114);
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
				['m', s @ ..] => {
					super::push(out, 9254);
					match s {
						[] => true,
						['r', 'o', 'f', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 9255);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'u', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9256);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9257);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5816);
			match s {
				[] => true,
				['u', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5817);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9258);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15115);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 15116);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 16874);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 16875);
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
							super::push(out, 12523);
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
					super::push(out, 9259);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15117);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 15118);
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
				['u', s @ ..] => super::t9::n14216(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t9::n7953(s, out),
		['r', s @ ..] => super::t9::n7954(s, out),
		['u', s @ ..] => super::t9::n7955(s, out),
		_ => false,
	}
}

/// The word collineandus.
pub(crate) fn n26837(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -tineandus.
pub(crate) fn n20734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12533);
	s.is_empty()
}

/// Words ending -uneandus.
pub(crate) fn n14232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9268);
	s.is_empty()
}

/// Words ending -reandus.
pub(crate) fn n7959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5824);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12534);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12535);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 15125);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 15126);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ueandus.
pub(crate) fn n7960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5825);
	match s {
		[] => true,
		['q', 'a', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -fandus.
pub(crate) fn n3433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2914);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
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
			super::push(out, 5826);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gandus.
pub(crate) fn n3434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2915);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5827);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9269);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9270);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9271);
					match s {
						[] => true,
						['f', 'u', s @ ..] => {
							super::push(out, 12536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12537);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12538);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12539);
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
		['e', s @ ..] => {
			super::push(out, 5828);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9273);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9274);
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
							super::push(out, 12540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15127);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15128);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 15129);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15130);
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
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12541);
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
				['r', s @ ..] => {
					super::push(out, 9276);
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
								['e', s @ ..] => {
									super::push(out, 15131);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 5829);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12542);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 12543);
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
							super::push(out, 12544);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15132);
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
						['f', s @ ..] => {
							super::push(out, 12545);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => super::t9::n20766(s, out),
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => super::t9::n20768(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t9::n14249(s, out),
				['r', s @ ..] => super::t9::n14250(s, out),
				['t', s @ ..] => super::t9::n14251(s, out),
				['v', s @ ..] => super::t9::n14252(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t9::n7971(s, out),
		['n', s @ ..] => super::t9::n7972(s, out),
		['o', s @ ..] => super::t9::n7973(s, out),
		['r', s @ ..] => super::t9::n7974(s, out),
		['u', s @ ..] => super::t9::n7975(s, out),
		_ => false,
	}
}

/// Words ending -handus.
pub(crate) fn n3435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2916);
	match s {
		[] => true,
		['c', 'c', s @ ..] => {
			super::push(out, 5835);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12570);
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
		_ => false,
	}
}

/// Words ending -iandus.
pub(crate) fn n3436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2917);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5836);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9288);
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
				['i', s @ ..] => {
					super::push(out, 9289);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9290);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9291);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15144);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9292);
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
								['n', s @ ..] => {
									super::push(out, 15146);
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
				['u', s @ ..] => {
					super::push(out, 9293);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12571);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16880);
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
							super::push(out, 12572);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
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
			super::push(out, 5837);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9294);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9295);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12573);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9296);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 12575);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12576);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9297);
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
				['u', s @ ..] => {
					super::push(out, 9298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5838);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9299);
					s.is_empty()
				}
				['a', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9300);
					match s {
						[] => true,
						['c', 'n', 'o', 'c', s @ ..] => {
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
						['m', 'u', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12578);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9301);
					match s {
						[] => true,
						['f', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 16881);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['p', 'm', 'a', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 5840);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5841);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9302);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15147);
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
				['i', s @ ..] => {
					super::push(out, 9303);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9305);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => super::t9::n20829(s, out),
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => super::t9::n14286(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t9::n7984(s, out),
		['s', s @ ..] => super::t9::n7985(s, out),
		['t', s @ ..] => super::t9::n7986(s, out),
		['v', s @ ..] => super::t9::n7987(s, out),
		_ => false,
	}
}

/// Words ending -landus.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2918);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5847);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 9324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12590);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5848);
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
		['e', s @ ..] => {
			super::push(out, 5849);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9326);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9327);
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
				['g', s @ ..] => {
					super::push(out, 9328);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9329);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9330);
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
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 9331);
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
		['f', s @ ..] => {
			super::push(out, 5850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 9332);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 16884);
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
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9333);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12592);
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
					super::push(out, 9334);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 15150);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 15151);
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
		['g', s @ ..] => {
			super::push(out, 5851);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5852);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9335);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12593);
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
							super::push(out, 12594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9336);
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
				['g', s @ ..] => {
					super::push(out, 9337);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15152);
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
					super::push(out, 9338);
					match s {
						[] => true,
						['i', 'n', s @ ..] => super::t9::n20871(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t9::n14327(s, out),
				['p', s @ ..] => super::t9::n14328(s, out),
				['t', s @ ..] => super::t9::n14329(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t9::n7994(s, out),
		['o', s @ ..] => super::t9::n7995(s, out),
		['p', s @ ..] => {
			super::push(out, 5855);
			s.is_empty()
		}
		['u', s @ ..] => super::t10::n7997(s, out),
		_ => false,
	}
}

/// Words ending -mandus.
pub(crate) fn n3438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2919);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5857);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9366);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9367);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9368);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9369);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12646);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12647);
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
					super::push(out, 9370);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9371);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
									super::push(out, 15169);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 16895);
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
									super::push(out, 15170);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 5858);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9372);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9373);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9374);
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
		['i', s @ ..] => {
			super::push(out, 5859);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9375);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 9376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 12648);
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
							super::push(out, 12649);
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
					super::push(out, 9377);
					match s {
						[] => true,
						['a', s @ ..] => {
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
					super::push(out, 9378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12650);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12651);
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
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9379);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12652);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9380);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12654);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15171);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => super::t10::n14369(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5860);
			s.is_empty()
		}
		['m', s @ ..] => super::t10::n8002(s, out),
		['o', s @ ..] => super::t10::n8003(s, out),
		['r', s @ ..] => super::t10::n8004(s, out),
		['s', s @ ..] => super::t10::n8005(s, out),
		['u', s @ ..] => super::t10::n8006(s, out),
		_ => false,
	}
}

/// Words ending -nandus.
pub(crate) fn n3439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2920);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5866);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9395);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9396);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12656);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15181);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15182);
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
							super::push(out, 12657);
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
				['s', s @ ..] => {
					super::push(out, 9397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12658);
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
			super::push(out, 5867);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9398);
					s.is_empty()
				}
				['i', 'l', 'a', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 9399);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15183);
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
				['t', 'a', 'c', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 5868);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9400);
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
					super::push(out, 9401);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12659);
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
							super::push(out, 12660);
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
									super::push(out, 15184);
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
									super::push(out, 15185);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 16902);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 16903);
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
									super::push(out, 15186);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 16904);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 16905);
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
				['u', s @ ..] => super::t10::n14394(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t10::n8010(s, out),
		['m', s @ ..] => {
			super::push(out, 5870);
			match s {
				[] => true,
				['a', 'd', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 5871);
			s.is_empty()
		}
		['o', s @ ..] => super::t10::n8013(s, out),
		['r', s @ ..] => super::t10::n8014(s, out),
		['u', s @ ..] => {
			super::push(out, 5874);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oandus.
pub(crate) fn n3440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2921);
	match s {
		[] => true,
		['b', s @ ..] => {
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

/// Words ending -pandus.
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2922);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5875);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'n', s @ ..] => {
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
			super::push(out, 5876);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9426);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12699);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12700);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15223);
									match s {
										[] => true,
										['m', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 17723);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n36542(s, out),
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
									super::push(out, 15224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9427);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9428);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 12701);
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
			super::push(out, 5877);
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5878);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5879);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9429);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9430);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12702);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15225);
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
		['s', s @ ..] => {
			super::push(out, 5880);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5881);
			match s {
				[] => true,
				['c', 'c', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -randus.
pub(crate) fn n3442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2923);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5882);
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
				['l', s @ ..] => {
					super::push(out, 9431);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12704);
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
									super::push(out, 15226);
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
						['i', s @ ..] => {
							super::push(out, 12705);
							match s {
								[] => true,
								['h', s @ ..] => {
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
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 9432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12706);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 15227);
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
						['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
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
		['b', s @ ..] => {
			super::push(out, 5883);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9433);
					match s {
						[] => true,
						['l', 'e', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15228);
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
						['n', 'e', 't', 'n', s @ ..] => {
							super::push(out, 12707);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9434);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15229);
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
						['r', s @ ..] => {
							super::push(out, 12710);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 12711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9435);
					match s {
						[] => true,
						['u', s @ ..] => {
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9436);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 15230);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 15231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9437);
					match s {
						[] => true,
						['c', 'u', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15232);
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
		['c', s @ ..] => {
			super::push(out, 5884);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9438);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15233);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 15234);
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
					super::push(out, 9439);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 12712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9440);
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
		['d', s @ ..] => {
			super::push(out, 5885);
			s.is_empty()
		}
		['e', s @ ..] => super::t10::n8029(s, out),
		['g', s @ ..] => super::t10::n8030(s, out),
		['i', s @ ..] => super::t10::n8031(s, out),
		['o', s @ ..] => super::t10::n8032(s, out),
		['p', s @ ..] => super::t10::n8033(s, out),
		['r', s @ ..] => super::t10::n8034(s, out),
		['t', s @ ..] => super::t10::n8035(s, out),
		['u', s @ ..] => super::t10::n8036(s, out),
		['y', s @ ..] => super::t10::n8037(s, out),
		_ => false,
	}
}

/// Words ending -sandus.
pub(crate) fn n3443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2924);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5895);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9494);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5896);
			match s {
				[] => true,
				['u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12768);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 5897);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9496);
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
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 12769);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15271);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 15272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'e', 'r', 'p', s @ ..] => {
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
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 9497);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9498);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15273);
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
		['p', s @ ..] => {
			super::push(out, 5898);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9499);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5899);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
							super::push(out, 12771);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15275);
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
						['i', 'g', 'r', 'e', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5900);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9502);
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
				['e', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9503);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12772);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', 'c', s @ ..] => {
					super::push(out, 9504);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 12774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5901);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9505);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9506);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tandus.
pub(crate) fn n3444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2925);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5902);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9507);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12775);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12776);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12779);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12780);
							match s {
								[] => true,
								['e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 12781);
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
		['c', s @ ..] => {
			super::push(out, 5903);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9509);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12782);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 16941);
											match s {
												[] => true,
												['i', 'r', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12783);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12784);
							match s {
								[] => true,
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
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 17727);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n36546(s, out),
														_ => false,
													}
												}
												['r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t10::n14522(s, out),
				['i', s @ ..] => {
					super::push(out, 9511);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15286);
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
							super::push(out, 12793);
							match s {
								[] => true,
								['f', s @ ..] => super::t10::n27367(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t10::n14524(s, out),
				['u', s @ ..] => super::t10::n14525(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t10::n8047(s, out),
		['i', s @ ..] => super::t10::n8048(s, out),
		['l', s @ ..] => super::t10::n8049(s, out),
		['n', s @ ..] => super::t10::n8050(s, out),
		['o', s @ ..] => super::t10::n8051(s, out),
		['p', s @ ..] => super::t10::n8052(s, out),
		['r', s @ ..] => super::t10::n8053(s, out),
		['s', s @ ..] => super::t10::n8054(s, out),
		['u', s @ ..] => super::t10::n8055(s, out),
		_ => false,
	}
}

/// Words ending -uandus.
pub(crate) fn n3445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2926);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5913);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9564);
					match s {
						[] => true,
						['v', 'e', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 9565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5914);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5915);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5916);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9566);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 12869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 5917);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12870);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15336);
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
						['t', 'n', 'o', 'c', s @ ..] => {
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
		['q', s @ ..] => {
			super::push(out, 5918);
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
				['e', s @ ..] => {
					super::push(out, 9567);
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
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9568);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 15337);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 15338);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 15339);
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
						['t', s @ ..] => {
							super::push(out, 12871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9569);
					match s {
						[] => true,
						['i', 'p', 'o', 'r', 'p', s @ ..] => {
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
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5919);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5920);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9570);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15340);
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
				['c', s @ ..] => {
					super::push(out, 9571);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12872);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12873);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9572);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9573);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9574);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9575);
					match s {
						[] => true,
						['e', 'a', s @ ..] => {
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
					super::push(out, 9576);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15341);
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
		_ => false,
	}
}

/// Words ending -vandus.
pub(crate) fn n3446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2927);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5921);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9577);
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
				['n', s @ ..] => {
					super::push(out, 9578);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9579);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12875);
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
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 12876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5922);
			match s {
				[] => true,
				['l', s @ ..] => {
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
							super::push(out, 12877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15342);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15343);
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
							super::push(out, 12878);
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5923);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12879);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
		['l', s @ ..] => {
			super::push(out, 5924);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12880);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
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
			super::push(out, 5926);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 12882);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 15344);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15345);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 12883);
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
								['n', s @ ..] => {
									super::push(out, 15346);
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
				['u', s @ ..] => {
					super::push(out, 9580);
					match s {
						[] => true,
						['c', s @ ..] => {
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
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5927);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 12884);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12885);
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

/// Words ending -xandus.
pub(crate) fn n3447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2928);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5928);
			match s {
				[] => true,
				['l', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 5929);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9581);
					match s {
						[] => true,
						['p', 'm', 'a', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 9582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12886);
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
		['i', s @ ..] => {
			super::push(out, 5930);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9583);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zandus.
pub(crate) fn n3448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2929);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9585);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9586);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9587);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9588);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12887);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9589);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9590);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9591);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12890);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12891);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12892);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9592);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12894);
							match s {
								[] => true,
								['m', 'o', s @ ..] => {
									super::push(out, 15347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12895);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 15348);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ribendus.
pub(crate) fn n14634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9600);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 12905);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 16964);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 16965);
							match s {
								[] => true,
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
							super::push(out, 16966);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17732);
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
							super::push(out, 16967);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 16968);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 16969);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17733);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 17734);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17735);
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
							super::push(out, 16970);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16971);
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
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 16972);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 16973);
							match s {
								[] => true,
								['s', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 16974);
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
					super::push(out, 15351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 12906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -umbendus.
pub(crate) fn n14636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9602);
	match s {
		[] => true,
		['c', 'o', s @ ..] => {
			super::push(out, 12907);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbendus.
pub(crate) fn n8079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5935);
	match s {
		[] => true,
		['o', 's', s @ ..] => {
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
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -ubendus.
pub(crate) fn n8080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5936);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 9603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12908);
					match s {
						[] => true,
						['d', 'i', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9604);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15353);
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
		['n', s @ ..] => {
			super::push(out, 9605);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12909);
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

/// Words ending -cendus.
pub(crate) fn n3450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2931);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5937);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5938);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5939);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12910);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12911);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15354);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'a', s @ ..] => {
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12912);
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
							super::push(out, 12913);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12914);
							match s {
								[] => true,
								['e', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 's', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9608);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5941);
			match s {
				[] => true,
				['u', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12915);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15355);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15356);
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
						['m', 'o', 'c', '\u{1}', s @ ..] => {
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
			super::push(out, 5942);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15357);
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
						['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 5943);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12917);
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
				['d', s @ ..] => {
					super::push(out, 9610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15358);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 15359);
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
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12919);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5944);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
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
		['s', s @ ..] => super::t10::n8089(s, out),
		['u', s @ ..] => super::t10::n8090(s, out),
		_ => false,
	}
}

/// Words ending -dendus.
pub(crate) fn n3451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2932);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5947);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9616);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12952);
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
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9618);
					match s {
						[] => true,
						['s', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 9619);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5948);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5949);
			match s {
				[] => true,
				['a', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 5950);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9620);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9621);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 9622);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12954);
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
						['e', s @ ..] => {
							super::push(out, 12955);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15379);
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
								['s', s @ ..] => {
									super::push(out, 15380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12956);
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
							super::push(out, 12957);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12958);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
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
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12959);
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
					super::push(out, 9624);
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
				['p', s @ ..] => {
					super::push(out, 9625);
					match s {
						[] => true,
						['p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9626);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12960);
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
						['e', s @ ..] => {
							super::push(out, 12961);
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
				['s', s @ ..] => {
					super::push(out, 9627);
					match s {
						[] => true,
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 9628);
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
		['i', s @ ..] => super::t10::n8095(s, out),
		['n', s @ ..] => super::t10::n8096(s, out),
		['o', s @ ..] => {
			super::push(out, 5953);
			match s {
				[] => true,
				['l', s @ ..] => super::t10::n14682(s, out),
				['r', s @ ..] => super::t10::n14683(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t10::n8098(s, out),
		['u', s @ ..] => super::t10::n8099(s, out),
		_ => false,
	}
}

/// Words ending -gendus.
pub(crate) fn n3452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2933);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5956);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 5957);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9648);
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
						['g', 'e', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
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
										['t', 'n', 'i', s @ ..] => {
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
								_ => false,
							}
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
				['t', s @ ..] => {
					super::push(out, 9649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12996);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15410);
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
							super::push(out, 12997);
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
							super::push(out, 12998);
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
		['i', s @ ..] => {
			super::push(out, 5958);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9650);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9651);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13000);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15411);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 15412);
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
							super::push(out, 13001);
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
						['i', s @ ..] => {
							super::push(out, 13002);
							match s {
								[] => true,
								['c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13003);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15413);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15414);
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
						['r', s @ ..] => {
							super::push(out, 13004);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13005);
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
				['l', s @ ..] => super::t10::n14700(s, out),
				['r', s @ ..] => super::t10::n14701(s, out),
				['s', s @ ..] => {
					super::push(out, 9655);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 9656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t10::n8103(s, out),
		['n', s @ ..] => super::t10::n8104(s, out),
		['o', s @ ..] => {
			super::push(out, 5961);
			s.is_empty()
		}
		['r', s @ ..] => super::t10::n8106(s, out),
		['u', s @ ..] => super::t10::n8107(s, out),
		_ => false,
	}
}

/// Words ending -hendus.
pub(crate) fn n3453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2934);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5964);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
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
							super::push(out, 13030);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15435);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15436);
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
							super::push(out, 13031);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
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
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
		['e', s @ ..] => {
			super::push(out, 5965);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13032);
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
							super::push(out, 13033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15437);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15438);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 15439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15440);
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
							super::push(out, 13034);
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
							super::push(out, 13035);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13036);
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
							super::push(out, 13037);
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

/// Words ending -iendus.
pub(crate) fn n3454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2935);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5966);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5967);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => super::t10::n14717(s, out),
				['c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'p', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => super::t10::n14720(s, out),
				['l', s @ ..] => {
					super::push(out, 9668);
					match s {
						[] => true,
						['u', 'f', s @ ..] => super::t10::n21759(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t10::n14722(s, out),
				['r', s @ ..] => super::t10::n14723(s, out),
				['s', s @ ..] => super::t10::n14724(s, out),
				['x', 'e', '\u{1}', s @ ..] => super::t10::n14725(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t10::n8112(s, out),
		['e', s @ ..] => super::t10::n8113(s, out),
		['f', s @ ..] => super::t10::n8114(s, out),
		['g', s @ ..] => super::t10::n8115(s, out),
		['l', s @ ..] => super::t10::n8116(s, out),
		['m', s @ ..] => super::t10::n8117(s, out),
		['n', s @ ..] => super::t10::n8118(s, out),
		['p', s @ ..] => super::t10::n8119(s, out),
		['r', s @ ..] => super::t10::n8120(s, out),
		['t', s @ ..] => super::t10::n8121(s, out),
		['v', s @ ..] => super::t10::n8122(s, out),
		_ => false,
	}
}

/// Words ending -lendus.
pub(crate) fn n3455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2936);
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
				['v', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5979);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9713);
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
		['l', s @ ..] => {
			super::push(out, 5981);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13089);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 13090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15515);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 17067);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 17068);
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
									super::push(out, 15516);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
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
						['v', s @ ..] => {
							super::push(out, 13091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15517);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 15518);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17069);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 17070);
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
									super::push(out, 15519);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 15520);
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
				['o', s @ ..] => {
					super::push(out, 9714);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5982);
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
						['e', 'r', '\u{1}', s @ ..] => {
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
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
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
		['p', s @ ..] => super::t10::n8128(s, out),
		['u', s @ ..] => {
			super::push(out, 5984);
			match s {
				[] => true,
				['c', s @ ..] => super::t10::n14789(s, out),
				['s', s @ ..] => super::t10::n14790(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

