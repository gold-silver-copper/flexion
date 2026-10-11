use flexion::Path;

/// Words ending -вший.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['а', s @ ..] => super::t2::n767(s, out),
		['е', s @ ..] => super::t2::n768(s, out),
		['и', s @ ..] => super::t2::n769(s, out),
		['о', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 5403);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 5404);
							s.is_empty()
						}
						['п', 'ы', s @ ..] => {
							super::push(out, 5405);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'о', 'п', 'ы', s @ ..] => {
					super::push(out, 1946);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t2::n771(s, out),
		['ы', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['б', 'ы', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				['л', 'п', 'ы', s @ ..] => {
					super::push(out, 1953);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'ы', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 5433);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 5434);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5435);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 5436);
							match s {
								[] => true,
								['в', s @ ..] => {
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
		['я', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1956);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							super::push(out, 5437);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 5438);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 11960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 5440);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5441);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11961);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 11962);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19444);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5442);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5443);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 11963);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 11964);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 5444);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 5445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1957);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5446);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 11965);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 5447);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 5448);
							match s {
								[] => true,
								['с', s @ ..] => super::t2::n12566(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 5449);
							match s {
								[] => true,
								['ы', s @ ..] => super::t2::n12567(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t2::n2027(s, out),
				['л', s @ ..] => super::t2::n2028(s, out),
				['м', s @ ..] => super::t2::n2029(s, out),
				['н', s @ ..] => super::t2::n2030(s, out),
				['о', s @ ..] => super::t2::n2031(s, out),
				['р', s @ ..] => super::t2::n2032(s, out),
				['у', s @ ..] => super::t2::n2033(s, out),
				['ъ', s @ ..] => super::t2::n2034(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йший.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 1974);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5479);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11988);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 19470);
											s.is_empty()
										}
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
					super::push(out, 1975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1976);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1977);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5480);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5481);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 11989);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 19471);
											s.is_empty()
										}
										['т', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 25626);
													match s {
														[] => true,
														['а', s @ ..] => super::t2::n30987(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 25627);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 19472);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 25628);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 11990);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 11991);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 19473);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 25629);
													s.is_empty()
												}
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
							super::push(out, 5482);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 11992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5483);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 11993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t2::n2048(s, out),
				['м', s @ ..] => super::t2::n2049(s, out),
				['н', s @ ..] => super::t2::n2050(s, out),
				['п', s @ ..] => super::t2::n2051(s, out),
				['р', s @ ..] => super::t2::n2052(s, out),
				['т', s @ ..] => super::t2::n2053(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерший.
pub(crate) fn n790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5513);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5514);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5515);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5516);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5517);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5518);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5519);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12047);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 12048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5520);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5521);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5522);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 12049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5523);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 5524);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тший.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1996);
					match s {
						[] => true,
						['ц', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 12052);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 12053);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 12054);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 12055);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19517);
											s.is_empty()
										}
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
					super::push(out, 1997);
					s.is_empty()
				}
				['м', 'ы', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уший.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хший.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 5526);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12056);
									s.is_empty()
								}
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
			super::push(out, 786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чший.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щший.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	s.is_empty()
}

/// Words ending -ьший.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	s.is_empty()
}

/// Words ending -юший.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 2002);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word надлежащий.
pub(crate) fn n26929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25655);
	s.is_empty()
}

/// Words ending -енадлежащий.
pub(crate) fn n26930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25656);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 29540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инадлежащий.
pub(crate) fn n26931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25657);
	s.is_empty()
}

/// Words ending -зжащий.
pub(crate) fn n2078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2007);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12061);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12062);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожащий.
pub(crate) fn n2079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2008);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5532);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5533);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5534);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5535);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					super::push(out, 12064);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужащий.
pub(crate) fn n2081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2010);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5536);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5537);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5538);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12067);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12068);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжащий.
pub(crate) fn n2083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2012);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5540);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пащий.
pub(crate) fn n804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 791);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чащий.
pub(crate) fn n805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 792);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5543);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5544);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5545);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 5546);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 12070);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12071);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12072);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 12073);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2015);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2016);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5548);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5549);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12076);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19522);
									s.is_empty()
								}
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
			super::push(out, 2018);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5551);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5553);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5554);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5555);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12078);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12079);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12080);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5556);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5557);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19524);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19525);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 25658);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2021);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12081);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5559);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5560);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5561);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5562);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5563);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5564);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5566);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5567);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2023);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5569);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шащий.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 793);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2025);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5570);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 5571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2028);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['р', 'е', s @ ..] => {
					super::push(out, 5573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2030);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5574);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'к', s @ ..] => {
					super::push(out, 5575);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2032);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5577);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12087);
							match s {
								[] => true,
								['н', 'г', 'о', '\u{1}', s @ ..] => {
									super::push(out, 19526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щащий.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 794);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2033);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2034);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'о', s @ ..] => {
			match s {
				[] => true,
				['м', '\u{1}', s @ ..] => {
					super::push(out, 5581);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бщий.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 2038);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 5584);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ещий.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2040);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2041);
					match s {
						[] => true,
						['л', 'з', '\u{1}', s @ ..] => {
							super::push(out, 5586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ищий.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 797);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ощий.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ущий.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2043);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5587);
							match s {
								[] => true,
								['г', 'а', s @ ..] => {
									super::push(out, 12088);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 19527);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 801);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2044);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2046);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2048);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12089);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 12090);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 19528);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2049);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5590);
							match s {
								[] => true,
								['к', 'е', s @ ..] => {
									super::push(out, 12091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2050);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12092);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 12093);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 19529);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 12094);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19530);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 19531);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 19532);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5592);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 12095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 2051);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5593);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2052);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5595);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 12096);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12097);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 19533);
											s.is_empty()
										}
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
					super::push(out, 2053);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2054);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 5597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 2056);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 5600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2059);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5601);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 5602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 806);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2064);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2065);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5603);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5604);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5605);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12099);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 19534);
											s.is_empty()
										}
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
					super::push(out, 2066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 5606);
							s.is_empty()
						}
						['о', 'л', 'а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 5607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 808);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2072);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2073);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 5608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2074);
					match s {
						[] => true,
						['т', s @ ..] => super::t2::n5847(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2075);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2077);
					s.is_empty()
				}
				['ь', s @ ..] => super::t2::n2152(s, out),
				['я', s @ ..] => super::t2::n2153(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t2::n822(s, out),
		['р', s @ ..] => super::t2::n823(s, out),
		['с', s @ ..] => super::t2::n824(s, out),
		['т', s @ ..] => super::t2::n825(s, out),
		['ц', s @ ..] => super::t2::n826(s, out),
		['ч', s @ ..] => super::t2::n827(s, out),
		['ш', s @ ..] => super::t2::n828(s, out),
		['щ', s @ ..] => super::t2::n829(s, out),
		_ => false,
	}
}

/// Words ending -ющий.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['а', s @ ..] => super::t2::n830(s, out),
		['д', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2119);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 5745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12342);
									s.is_empty()
								}
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
			super::push(out, 819);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2120);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5746);
							match s {
								[] => true,
								['ж', 'р', '\u{1}', s @ ..] => {
									super::push(out, 12343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 5747);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 12344);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 19767);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2121);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2122);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 5749);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 12345);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 19768);
											s.is_empty()
										}
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
					super::push(out, 2123);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5750);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 12346);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 19769);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 19770);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 25811);
													s.is_empty()
												}
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
							super::push(out, 5751);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 12347);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 19771);
											s.is_empty()
										}
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
					super::push(out, 2124);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5752);
							match s {
								[] => true,
								['т', 'с', 'е', s @ ..] => {
									super::push(out, 12348);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 19772);
											s.is_empty()
										}
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
					super::push(out, 2125);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5753);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 5754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2127);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2128);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 5755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2132);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2133);
					match s {
						[] => true,
						['е', 'ъ', s @ ..] => {
							super::push(out, 5758);
							match s {
								[] => true,
								['б', 'о', 'е', s @ ..] => {
									super::push(out, 12349);
									match s {
										[] => true,
										['с', 'в', '\u{1}', s @ ..] => {
											super::push(out, 19773);
											s.is_empty()
										}
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
					super::push(out, 2134);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2136);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 5759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['д', 'о', 'л', 'о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 5760);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 5761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				['н', 'и', 'л', 'д', '\u{1}', s @ ..] => {
					super::push(out, 2139);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2140);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 2143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2144);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2150);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2152);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 5765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2153);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t2::n839(s, out),
		['ь', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2173);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5825);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5826);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 12412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['л', s @ ..] => super::t2::n2252(s, out),
				['о', s @ ..] => {
					super::push(out, 2177);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t2::n842(s, out),
		_ => false,
	}
}

/// Words ending -ящий.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2182);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2186);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 5848);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 12427);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12428);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 19851);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 12429);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 19852);
											s.is_empty()
										}
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
					super::push(out, 2189);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 12430);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 12431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2190);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 5849);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 5850);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 12432);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12433);
									match s {
										[] => true,
										['г', '\u{1}', s @ ..] => {
											super::push(out, 19853);
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
								['\u{1}', s @ ..] => {
									super::push(out, 12434);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 12435);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 12436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 2191);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5851);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5852);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 5853);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5854);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 12439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19854);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 12440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19855);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2195);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 5856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2196);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 5857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2197);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5858);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 5859);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5860);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 5861);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 12441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12442);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 12443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5862);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t2::n846(s, out),
		['е', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2210);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12470);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t2::n13087(s, out),
								['б', s @ ..] => super::t2::n13088(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t2::n2287(s, out),
				['о', s @ ..] => super::t2::n2288(s, out),
				['р', s @ ..] => super::t2::n2289(s, out),
				['у', s @ ..] => super::t2::n2290(s, out),
				['ь', s @ ..] => super::t2::n2291(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t2::n849(s, out),
		['м', s @ ..] => super::t2::n850(s, out),
		['н', s @ ..] => super::t2::n851(s, out),
		['о', s @ ..] => super::t2::n852(s, out),
		['п', s @ ..] => super::t2::n853(s, out),
		['р', s @ ..] => super::t2::n854(s, out),
		['с', s @ ..] => super::t2::n855(s, out),
		['т', s @ ..] => super::t2::n856(s, out),
		['ф', s @ ..] => super::t2::n857(s, out),
		_ => false,
	}
}

/// Words ending -льной.
pub(crate) fn n918(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2337);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2338);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12588);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12589);
							match s {
								[] => true,
								['н', 'в', 'е', s @ ..] => {
									super::push(out, 19911);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 25893);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дяной.
pub(crate) fn n919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	match s {
		[] => true,
		['о', 'в', '-', s @ ..] => {
			super::push(out, 2339);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word непростой.
pub(crate) fn n27177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25894);
	s.is_empty()
}

/// Words ending -цатиглавый.
pub(crate) fn n27178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25895);
	s.is_empty()
}

/// Words ending -латоглавый.
pub(crate) fn n27179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25896);
	s.is_empty()
}

/// Words ending -лотоглавый.
pub(crate) fn n27180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25897);
	s.is_empty()
}

/// Words ending -евый.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2389);
					s.is_empty()
				}
				['т', 'н', 'и', 'м', '\u{1}', s @ ..] => {
					super::push(out, 2390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2391);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2392);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 6145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 2393);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2394);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 944);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2397);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['а', 'с', 'с', s @ ..] => {
					super::push(out, 2398);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2399);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6149);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6150);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 12620);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 19923);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 12621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 6151);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 12622);
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
						['б', s @ ..] => {
							super::push(out, 6152);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 6153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 12623);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 19924);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 19925);
											match s {
												[] => true,
												['-', 'о', s @ ..] => {
													match s {
														[] => true,
														['н', 'м', 'е', 'т', '\u{1}', s @ ..] => super::t2::n31130(s, out),
														['т', s @ ..] => super::t2::n31131(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2401);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							super::push(out, 6154);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 12624);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 19926);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'м', s @ ..] => {
							super::push(out, 6155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6156);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 12626);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 19927);
											s.is_empty()
										}
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
					super::push(out, 2405);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6157);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 12627);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 19928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 6158);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12628);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 19929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 6159);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6160);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 12629);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 19930);
											s.is_empty()
										}
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
					super::push(out, 2406);
					match s {
						[] => true,
						['о', 'л', s @ ..] => {
							super::push(out, 6161);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 12630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2407);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t2::n973(s, out),
		['н', s @ ..] => super::t2::n974(s, out),
		['о', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 951);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2443);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6220);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 12663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 6221);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2444);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 6222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 952);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2445);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2446);
					match s {
						[] => true,
						['м', s @ ..] => super::t2::n6490(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t2::n2540(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t2::n978(s, out),
		['у', s @ ..] => super::t2::n979(s, out),
		['ц', s @ ..] => super::t3::n980(s, out),
		['ч', s @ ..] => super::t3::n981(s, out),
		['ш', s @ ..] => super::t3::n982(s, out),
		['щ', s @ ..] => super::t3::n983(s, out),
		['ь', s @ ..] => super::t3::n984(s, out),
		_ => false,
	}
}

/// Words ending -звый.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6249);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6250);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 12680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 6251);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12681);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 12682);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 19951);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 25910);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивый.
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12683);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 19952);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 25911);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 12684);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 12685);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 19953);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 12686);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 19954);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 25912);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 25913);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 25914);
													match s {
														[] => true,
														['р', s @ ..] => super::t3::n31140(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 962);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2474);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 6252);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 12687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2475);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2476);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2478);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6255);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 12688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2479);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 6256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t3::n990(s, out),
		['н', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2501);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6307);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6308);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 12757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6309);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6310);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 12758);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 20000);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 12759);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 12760);
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 20001);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 20002);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 20003);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 20004);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6311);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6313);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 12761);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20005);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 25935);
													s.is_empty()
												}
												['л', s @ ..] => {
													super::push(out, 25936);
													match s {
														[] => true,
														['з', s @ ..] => super::t3::n31147(s, out),
														_ => false,
													}
												}
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
							super::push(out, 6314);
							match s {
								[] => true,
								['л', s @ ..] => super::t3::n13388(s, out),
								_ => false,
							}
						}
						['ь', s @ ..] => super::t3::n6589(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t3::n2607(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t3::n995(s, out),
		['ш', s @ ..] => super::t3::n996(s, out),
		_ => false,
	}
}

/// Words ending -овый.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2532);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 6331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6332);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 6333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2534);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2535);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 6334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6335);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 6336);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 12772);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 20017);
											s.is_empty()
										}
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
					super::push(out, 2536);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2537);
					match s {
						[] => true,
						['е', 'г', '\u{1}', s @ ..] => {
							super::push(out, 6338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2538);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12773);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12774);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 20018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 6340);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 6341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2539);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6342);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 6343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2541);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 2542);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6345);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2543);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2544);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 6348);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t3::n13404(s, out),
								_ => false,
							}
						}
						['ч', s @ ..] => super::t3::n6624(s, out),
						['ш', s @ ..] => super::t3::n6625(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t3::n2641(s, out),
				['и', s @ ..] => super::t3::n2642(s, out),
				['л', s @ ..] => super::t3::n2643(s, out),
				['н', s @ ..] => super::t3::n2644(s, out),
				['о', s @ ..] => super::t3::n2645(s, out),
				['р', s @ ..] => super::t3::n2646(s, out),
				['у', s @ ..] => super::t3::n2647(s, out),
				['ь', s @ ..] => super::t3::n2648(s, out),
				['я', s @ ..] => super::t3::n2649(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t3::n1001(s, out),
		['ж', s @ ..] => super::t3::n1002(s, out),
		['з', s @ ..] => super::t3::n1003(s, out),
		['к', s @ ..] => super::t3::n1004(s, out),
		['л', s @ ..] => super::t3::n1005(s, out),
		['м', s @ ..] => super::t3::n1006(s, out),
		['н', s @ ..] => super::t3::n1007(s, out),
		['п', s @ ..] => super::t3::n1008(s, out),
		['р', s @ ..] => super::t3::n1009(s, out),
		['с', s @ ..] => super::t3::n1010(s, out),
		['т', s @ ..] => super::t3::n1011(s, out),
		['ф', s @ ..] => super::t3::n1012(s, out),
		['х', s @ ..] => super::t3::n1013(s, out),
		['ц', s @ ..] => super::t3::n1014(s, out),
		['ч', s @ ..] => super::t3::n1015(s, out),
		['ш', s @ ..] => super::t3::n1016(s, out),
		['щ', s @ ..] => super::t3::n1017(s, out),
		_ => false,
	}
}

/// Words ending -твый.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 994);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2729);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13371);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 13372);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 20439);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 995);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2730);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явый.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 997);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2731);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					super::push(out, 2732);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2734);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'е', 'м', 'с', '\u{1}', s @ ..] => {
					super::push(out, 2735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 999);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word алмазно-твердый.
pub(crate) fn n27419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26126);
	s.is_empty()
}

/// Words ending -ряблый.
pub(crate) fn n2884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2782);
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			super::push(out, 6873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -глый.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6874);
							match s {
								[] => true,
								['е', 'б', '\u{1}', s @ ..] => {
									super::push(out, 13413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', 'р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6876);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2786);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 6878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13414);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13415);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 13416);
									match s {
										[] => true,
										['л', 'о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 20456);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1030);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 2788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -длый.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1032);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -елый.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2790);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 2791);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6879);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13417);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20457);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 26127);
													match s {
														[] => true,
														['е', 'з', '\u{1}', s @ ..] => super::t3::n31216(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 20458);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 20459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 13418);
									match s {
										[] => true,
										['л', 'е', 'ж', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 26128);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 26129);
													match s {
														[] => true,
														['о', 'н', 'р', 'е', 'ч', '\u{1}', s @ ..] => super::t3::n31217(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2792);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6880);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 6881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2793);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6882);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 6883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2794);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6884);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 6885);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 13421);
									match s {
										[] => true,
										['е', 'л', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 26130);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 26131);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t3::n31218(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2795);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2796);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6886);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 6887);
							match s {
								[] => true,
								['о', 'с', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20460);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 20461);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 6888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 2799);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 13423);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 13424);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 20462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 13425);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 20463);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1039);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6889);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6890);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13426);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20464);
											s.is_empty()
										}
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
					super::push(out, 2802);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6891);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6892);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 13427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 6893);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13428);
									s.is_empty()
								}
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
			super::push(out, 1041);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				['е', s @ ..] => super::t3::n2908(s, out),
				['м', s @ ..] => {
					super::push(out, 2805);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t3::n2910(s, out),
				['с', s @ ..] => super::t3::n2911(s, out),
				['я', s @ ..] => super::t3::n2912(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t3::n1072(s, out),
		['р', s @ ..] => super::t3::n1073(s, out),
		['с', s @ ..] => super::t3::n1074(s, out),
		['т', s @ ..] => super::t3::n1075(s, out),
		['ц', s @ ..] => super::t3::n1076(s, out),
		['ш', s @ ..] => super::t3::n1077(s, out),
		_ => false,
	}
}

/// Words ending -злый.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1048);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6921);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13457);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13458);
									s.is_empty()
								}
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
			super::push(out, 1050);
			match s {
				[] => true,
				['е', 'м', '\u{1}', s @ ..] => {
					super::push(out, 2827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -илый.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2828);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2829);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 6922);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 6923);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 13459);
									s.is_empty()
								}
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
			super::push(out, 1053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2830);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2831);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2832);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2833);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 6925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клый.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2834);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13460);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13461);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 20481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 6926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2836);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2838);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2839);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 6929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13462);
									s.is_empty()
								}
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
			super::push(out, 1060);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 2840);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 6930);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6931);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 6932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олый.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2842);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 6933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1062);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2843);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2846);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2847);
					match s {
						[] => true,
						['г', 'л', 'о', 'д', '\u{1}', s @ ..] => {
							super::push(out, 6935);
							s.is_empty()
						}
						['к', 'о', 'р', 'и', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 6936);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 13464);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 20482);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'и', 'л', 'д', '\u{1}', s @ ..] => {
									super::push(out, 13465);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 13466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2848);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2849);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плый.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 2850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2851);
					match s {
						[] => true,
						['х', '\u{1}', s @ ..] => {
							super::push(out, 6937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6938);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['щ', '\u{1}', s @ ..] => {
					super::push(out, 2853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлый.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['о', 'г', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2854);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слый.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6941);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6942);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6943);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6944);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6945);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 6946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6947);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 6948);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6949);
							match s {
								[] => true,
								['л', 'г', 'у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20483);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 20484);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 26134);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 13467);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 20485);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6950);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13468);
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 20486);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 26135);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 26136);
													match s {
														[] => true,
														['л', s @ ..] => super::t3::n31219(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 20487);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 26137);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1068);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2858);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6951);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13469);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20488);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 20489);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6952);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 13470);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20490);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 13471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлый.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улый.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хлый.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 2861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1075);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2862);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 6953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2863);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2864);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 6954);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13472);
									s.is_empty()
								}
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
			super::push(out, 1076);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2865);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 2866);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6956);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 6957);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1078);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1079);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2870);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлый.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1081);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2871);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2872);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 6959);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2873);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 6960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ылый.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6961);
							s.is_empty()
						}
						['и', 'т', 'с', 'е', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 6962);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6963);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 13474);
									s.is_empty()
								}
								['к', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 20491);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 20492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 13475);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 13476);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 13477);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 20493);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 13478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 6964);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6965);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 6966);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13481);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 20494);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 13482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2876);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 6967);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 6968);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 6969);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 13483);
									s.is_empty()
								}
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
			super::push(out, 1085);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6970);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6971);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 6972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2878);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялый.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1086);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2879);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13484);
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 13485);
									s.is_empty()
								}
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
			super::push(out, 1088);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 2880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емый.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['а', s @ ..] => super::t3::n1121(s, out),
		['д', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 2900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1092);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['б', 'ы', 'з', 'е', s @ ..] => {
					super::push(out, 2901);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2902);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 13589);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 13590);
									s.is_empty()
								}
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
			super::push(out, 1094);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7055);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7056);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 7057);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t3::n3011(s, out),
				['г', s @ ..] => {
					super::push(out, 2905);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t3::n3013(s, out),
				['ж', s @ ..] => {
					super::push(out, 2907);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t3::n3015(s, out),
				['к', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				['л', s @ ..] => super::t3::n3017(s, out),
				['м', s @ ..] => {
					super::push(out, 2910);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t3::n3019(s, out),
				['р', s @ ..] => super::t3::n3020(s, out),
				['с', s @ ..] => super::t3::n3021(s, out),
				['т', s @ ..] => super::t3::n3022(s, out),
				['ф', s @ ..] => super::t3::n3023(s, out),
				['х', s @ ..] => super::t3::n3024(s, out),
				['ц', s @ ..] => super::t3::n3025(s, out),
				['ч', s @ ..] => super::t3::n3026(s, out),
				['ш', s @ ..] => super::t3::n3027(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t3::n1127(s, out),
		['ш', s @ ..] => super::t3::n1128(s, out),
		['щ', s @ ..] => super::t3::n1129(s, out),
		['ю', s @ ..] => super::t3::n1130(s, out),
		['я', s @ ..] => super::t3::n1131(s, out),
		_ => false,
	}
}

/// Words ending -имый.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2923);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 7099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2925);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2926);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2927);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 7101);
							s.is_empty()
						}
						['р', 'о', s @ ..] => {
							super::push(out, 7102);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 13637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2928);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2929);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 2930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2931);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7104);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 7105);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 7106);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 7107);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 13640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13641);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 13642);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2932);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 2933);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2934);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2935);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7109);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 7110);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13644);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 20684);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 13645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 7111);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2937);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7112);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13647);
									s.is_empty()
								}
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20685);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 20686);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 13648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2938);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2939);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2940);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7114);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 13649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2941);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2942);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7117);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 7118);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13650);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 20687);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 13651);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', 'е', s @ ..] => {
							super::push(out, 7119);
							match s {
								[] => true,
								['р', 'е', 'п', 'е', s @ ..] => {
									super::push(out, 13652);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 20688);
											s.is_empty()
										}
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
					super::push(out, 2943);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7120);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 7121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 7122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 7123);
							match s {
								[] => true,
								['т', 'у', s @ ..] => {
									super::push(out, 13655);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 7124);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13656);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 20690);
											s.is_empty()
										}
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
					super::push(out, 2944);
					match s {
						[] => true,
						['л', 'е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 7125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t3::n1136(s, out),
		['ж', s @ ..] => super::t3::n1137(s, out),
		['з', s @ ..] => super::t3::n1138(s, out),
		['л', s @ ..] => super::t3::n1139(s, out),
		['м', s @ ..] => super::t3::n1140(s, out),
		['н', s @ ..] => super::t3::n1141(s, out),
		['о', s @ ..] => super::t3::n1142(s, out),
		['п', s @ ..] => super::t3::n1143(s, out),
		['р', s @ ..] => super::t3::n1144(s, out),
		['с', s @ ..] => super::t3::n1145(s, out),
		['т', s @ ..] => super::t3::n1146(s, out),
		['ч', s @ ..] => super::t3::n1147(s, out),
		['ш', s @ ..] => super::t3::n1148(s, out),
		['щ', s @ ..] => super::t3::n1149(s, out),
		_ => false,
	}
}

/// Words ending -таный.
pub(crate) fn n1169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1138);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7299);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3057);
			s.is_empty()
		}
		['у', 'п', '\u{1}', s @ ..] => {
			super::push(out, 3058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чаный.
pub(crate) fn n1171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1140);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шаный.
pub(crate) fn n1172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1141);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3061);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щаный.
pub(crate) fn n1173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1142);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бный.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1143);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1144);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7302);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13768);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 20740);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 26298);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 13769);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 20741);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 7303);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7304);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 13771);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 20742);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 26299);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', 'н', 'в', s @ ..] => {
											super::push(out, 20743);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 20744);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 20745);
											s.is_empty()
										}
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
					super::push(out, 3065);
					match s {
						[] => true,
						['у', 'л', 'с', 'о', s @ ..] => {
							super::push(out, 7305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7306);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7307);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 13772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13773);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13774);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20746);
											s.is_empty()
										}
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
					super::push(out, 3067);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7308);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3069);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3070);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3071);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7309);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7310);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13776);
									s.is_empty()
								}
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
			super::push(out, 1145);
			match s {
				[] => true,
				['д', s @ ..] => super::t3::n3190(s, out),
				['л', s @ ..] => {
					super::push(out, 3073);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7315);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 13778);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 13779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 7316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3074);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7317);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 7318);
							s.is_empty()
						}
						['д', s @ ..] => super::t3::n7658(s, out),
						['к', s @ ..] => super::t3::n7659(s, out),
						['п', s @ ..] => super::t3::n7660(s, out),
						['т', s @ ..] => super::t3::n7661(s, out),
						['э', s @ ..] => super::t3::n7662(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t3::n3193(s, out),
				['ф', s @ ..] => super::t3::n3194(s, out),
				['щ', s @ ..] => super::t3::n3195(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t3::n1177(s, out),
		['у', s @ ..] => super::t3::n1178(s, out),
		['ы', s @ ..] => super::t3::n1179(s, out),
		['ю', s @ ..] => super::t3::n1180(s, out),
		_ => false,
	}
}

/// Words ending -вный.
pub(crate) fn n386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3084);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['р', 'е', 'д', 'о', 'м', s @ ..] => {
							super::push(out, 7338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['у', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13797);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 13798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13799);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 7340);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13801);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 20757);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 13802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13803);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13804);
									match s {
										[] => true,
										['о', 'с', s @ ..] => {
											super::push(out, 20758);
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
									super::push(out, 13805);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 13806);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7341);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 7342);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 7343);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 7344);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13809);
									s.is_empty()
								}
								['с', 'е', s @ ..] => {
									super::push(out, 13810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 7345);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13811);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 20759);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 26316);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3090);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 7346);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13813);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13814);
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									super::push(out, 13815);
									s.is_empty()
								}
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
			super::push(out, 1151);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3091);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 7347);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3092);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7348);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 13817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3093);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7349);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13818);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 13819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t3::n7695(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3094);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7351);
							match s {
								[] => true,
								['з', s @ ..] => super::t3::n14512(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t3::n3214(s, out),
				['ч', s @ ..] => super::t3::n3215(s, out),
				['ш', s @ ..] => super::t3::n3216(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t3::n1183(s, out),
		['о', s @ ..] => super::t3::n1184(s, out),
		['р', s @ ..] => super::t3::n1185(s, out),
		['у', s @ ..] => super::t3::n1186(s, out),
		['ы', s @ ..] => super::t3::n1187(s, out),
		['я', s @ ..] => super::t3::n1188(s, out),
		_ => false,
	}
}

/// Words ending -гный.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 1158);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дный.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3119);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 7394);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3120);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13876);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 13877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7395);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3123);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 7397);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 13878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 7398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3124);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7399);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 7400);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 13879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7401);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 13880);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 26377);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 26378);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 13881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 7402);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 7403);
							match s {
								[] => true,
								['а', 'к', 'о', s @ ..] => {
									super::push(out, 13882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7404);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 7405);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 7406);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 13883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7407);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 7408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7409);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 13885);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 20819);
											match s {
												[] => true,
												['о', 'р', 'е', 'в', 'е', 'с', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n31341(s, out),
														['-', s @ ..] => super::t3::n31342(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['а', 'р', 'т', 'ь', 'л', 'у', '\u{1}', s @ ..] => {
											super::push(out, 20820);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 20821);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 26379);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 20822);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 20823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 7410);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7411);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 7412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t3::n3249(s, out),
				['с', s @ ..] => {
					super::push(out, 3130);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7417);
							match s {
								[] => true,
								['ф', 'о', s @ ..] => {
									super::push(out, 13894);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 20828);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7418);
							match s {
								[] => true,
								['д', 'е', s @ ..] => {
									super::push(out, 13895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t3::n7769(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t3::n3253(s, out),
				['щ', s @ ..] => super::t3::n3254(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t3::n1191(s, out),
		['з', s @ ..] => super::t3::n1192(s, out),
		['и', s @ ..] => super::t3::n1193(s, out),
		['н', s @ ..] => super::t3::n1194(s, out),
		['о', s @ ..] => super::t3::n1195(s, out),
		['р', s @ ..] => super::t3::n1196(s, out),
		['у', s @ ..] => super::t3::n1197(s, out),
		['ы', s @ ..] => super::t3::n1198(s, out),
		['ь', s @ ..] => super::t3::n1199(s, out),
		['ю', s @ ..] => super::t3::n1200(s, out),
		['я', s @ ..] => super::t3::n1201(s, out),
		_ => false,
	}
}

/// Words ending -еный.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1171);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3199);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3200);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 7554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3202);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 7555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14068);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14069);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 20962);
											s.is_empty()
										}
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
					super::push(out, 3203);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7556);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3204);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3205);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3206);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3207);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7558);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 7559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3208);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7560);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 7561);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 14072);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 14073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 7562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 3209);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3210);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14074);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 14075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 7565);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 7566);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 3211);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7567);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7568);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 14078);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20963);
											s.is_empty()
										}
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
					super::push(out, 3212);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7569);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 14079);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 14080);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 14081);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 20964);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 7570);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14082);
									match s {
										[] => true,
										['ч', '\u{1}', s @ ..] => {
											super::push(out, 20965);
											s.is_empty()
										}
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
					super::push(out, 3213);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14083);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 14084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 7572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14085);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 14086);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
													super::push(out, 26465);
													s.is_empty()
												}
												['т', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
													super::push(out, 26466);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3214);
					s.is_empty()
				}
				['м', s @ ..] => super::t3::n3337(s, out),
				['о', s @ ..] => super::t3::n3338(s, out),
				['п', s @ ..] => super::t3::n3339(s, out),
				['с', s @ ..] => {
					super::push(out, 3218);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t3::n3341(s, out),
				['ш', s @ ..] => super::t3::n3342(s, out),
				['я', s @ ..] => super::t3::n3343(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t3::n1207(s, out),
		['н', s @ ..] => super::t3::n1208(s, out),
		['о', s @ ..] => super::t3::n1209(s, out),
		['р', s @ ..] => super::t3::n1210(s, out),
		['т', s @ ..] => super::t3::n1211(s, out),
		['ч', s @ ..] => super::t3::n1212(s, out),
		['ш', s @ ..] => super::t3::n1213(s, out),
		['щ', s @ ..] => super::t3::n1214(s, out),
		_ => false,
	}
}

/// Words ending -жный.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1184);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7632);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 7633);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 14117);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20977);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'х', 'р', 'а', '\u{1}', s @ ..] => {
							super::push(out, 7634);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 7635);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 14118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7636);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 14119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 7637);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 7638);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3255);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3256);
					match s {
						[] => true,
						['о', 'р', 'п', 'е', s @ ..] => {
							super::push(out, 7639);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 14122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3257);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3258);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3259);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7641);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14123);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 14124);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20978);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14125);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 20979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 7642);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 14126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 7643);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 7644);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 14127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 7645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3260);
					match s {
						[] => true,
						['у', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14128);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 14129);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 20980);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 14130);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 20981);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 20982);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 26469);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 20983);
											match s {
												[] => true,
												['а', 'ч', 'п', 'о', 'л', 'х', '\u{1}', s @ ..] => {
													super::push(out, 26470);
													s.is_empty()
												}
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
							super::push(out, 7646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3261);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7647);
							match s {
								[] => true,
								['р', 'д', 'о', s @ ..] => {
									super::push(out, 14131);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 20984);
											s.is_empty()
										}
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
					super::push(out, 3262);
					match s {
						[] => true,
						['и', 'к', 'э', 'з', s @ ..] => {
							super::push(out, 7648);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t3::n3385(s, out),
				['с', s @ ..] => super::t3::n3386(s, out),
				['т', s @ ..] => super::t3::n3387(s, out),
				['ч', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t3::n1216(s, out),
		['е', s @ ..] => super::t3::n1217(s, out),
		['и', s @ ..] => super::t3::n1218(s, out),
		['л', s @ ..] => super::t3::n1219(s, out),
		['о', s @ ..] => super::t4::n1220(s, out),
		['р', s @ ..] => super::t4::n1221(s, out),
		['у', s @ ..] => super::t4::n1222(s, out),
		['ы', s @ ..] => super::t4::n1223(s, out),
		['э', s @ ..] => super::t4::n1224(s, out),
		['ю', s @ ..] => super::t4::n1225(s, out),
		['я', s @ ..] => super::t4::n1226(s, out),
		_ => false,
	}
}

/// Words ending -зный.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3312);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3313);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3314);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7727);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 14193);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 14194);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 21030);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 26498);
													s.is_empty()
												}
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
							super::push(out, 7728);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 14195);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 21031);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 7729);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 14196);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 21032);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													super::push(out, 26499);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3315);
					s.is_empty()
				}
				['р', s @ ..] => super::t4::n3441(s, out),
				['т', s @ ..] => {
					super::push(out, 3317);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7733);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['ж', 'р', 'у', 'б', '-', s @ ..] => {
							super::push(out, 7734);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14202);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 21040);
											s.is_empty()
										}
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
					super::push(out, 3319);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7735);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 14203);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 21041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 7736);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14204);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 21042);
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 26512);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1197);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3320);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3321);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7737);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 14205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t4::n3447(s, out),
				['н', s @ ..] => super::t4::n3448(s, out),
				['п', s @ ..] => super::t4::n3449(s, out),
				['р', s @ ..] => super::t4::n3450(s, out),
				['т', s @ ..] => super::t4::n3451(s, out),
				['ь', s @ ..] => super::t4::n3452(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n1229(s, out),
		['о', s @ ..] => super::t4::n1230(s, out),
		['р', s @ ..] => super::t4::n1231(s, out),
		['у', s @ ..] => super::t4::n1232(s, out),
		['ю', s @ ..] => super::t4::n1233(s, out),
		['я', s @ ..] => super::t4::n1234(s, out),
		_ => false,
	}
}

/// Words ending -иный.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 3363);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 7803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1205);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3364);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 7804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3365);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7805);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 7806);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 7807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1206);
			match s {
				[] => true,
				['м', 'з', '\u{1}', s @ ..] => {
					super::push(out, 3367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1207);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1208);
			match s {
				[] => true,
				['е', 'ч', s @ ..] => {
					super::push(out, 3368);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 7808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'а', 'х', s @ ..] => {
			super::push(out, 1209);
			match s {
				[] => true,
				['у', 'л', 'г', '\u{1}', s @ ..] => {
					super::push(out, 3369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1210);
			s.is_empty()
		}
		['т', 'с', 'о', 'г', '\u{1}', s @ ..] => {
			super::push(out, 1211);
			s.is_empty()
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3370);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3371);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 7809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йный.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1213);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3372);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3373);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 7810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3374);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3375);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 7811);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 14274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3376);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 7812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 3377);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7813);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 14275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7814);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3378);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3379);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 7816);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 14276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14277);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14279);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 14280);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 21077);
											s.is_empty()
										}
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
					super::push(out, 3380);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7818);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 14281);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 14282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 7819);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7820);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 14283);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 14284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', 'у', 'р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 7821);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3382);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3383);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7823);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 14286);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 21078);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'ж', s @ ..] => {
							super::push(out, 7824);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 7825);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 7826);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 14287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7827);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 14288);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 14289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 14290);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 21079);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 14291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t4::n8204(s, out),
						['с', s @ ..] => {
							super::push(out, 7829);
							s.is_empty()
						}
						['у', s @ ..] => super::t4::n8206(s, out),
						_ => false,
					}
				}
				['м', 'е', 'с', s @ ..] => super::t4::n3512(s, out),
				['н', s @ ..] => super::t4::n3513(s, out),
				['п', s @ ..] => super::t4::n3514(s, out),
				['р', s @ ..] => super::t4::n3515(s, out),
				['с', s @ ..] => super::t4::n3516(s, out),
				['т', s @ ..] => super::t4::n3517(s, out),
				['ф', s @ ..] => super::t4::n3518(s, out),
				['ш', s @ ..] => super::t4::n3519(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n1247(s, out),
		['о', s @ ..] => super::t4::n1248(s, out),
		['у', s @ ..] => super::t4::n1249(s, out),
		_ => false,
	}
}

/// Words ending -кный.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1218);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1219);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лный.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7895);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 7896);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 14363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мный.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3416);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7897);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3417);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 7899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'к', 'е', 'р', 'р', s @ ..] => {
					super::push(out, 3418);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 7900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3419);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 7901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 7902);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 14365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1223);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3420);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 7904);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 7905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14366);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3421);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3422);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 7906);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 14368);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 21123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3423);
					match s {
						[] => true,
						['д', 'а', s @ ..] => {
							super::push(out, 7908);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 14369);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 21124);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 14370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 7909);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 14371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3424);
					match s {
						[] => true,
						['р', 'п', 'о', s @ ..] => {
							super::push(out, 7911);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 14372);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 21125);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													super::push(out, 26544);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 14373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3425);
					match s {
						[] => true,
						['б', 'о', 'р', 'п', 'о', 'к', s @ ..] => {
							super::push(out, 7912);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 14374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3426);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3427);
					match s {
						[] => true,
						['о', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 7913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3428);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 7914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 7915);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 7916);
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 14375);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 21126);
											s.is_empty()
										}
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
					super::push(out, 3429);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7917);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 7918);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 14376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7919);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 7920);
							match s {
								[] => true,
								['и', 'с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 21127);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 21128);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 21129);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 21130);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7921);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 14377);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 21131);
											s.is_empty()
										}
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
					super::push(out, 3431);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 3432);
					s.is_empty()
				}
				['х', s @ ..] => super::t4::n3566(s, out),
				['ч', s @ ..] => super::t4::n3567(s, out),
				['ъ', s @ ..] => super::t4::n3568(s, out),
				['ы', s @ ..] => super::t4::n3569(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n1256(s, out),
		['л', s @ ..] => super::t4::n1257(s, out),
		['м', s @ ..] => super::t4::n1258(s, out),
		['о', s @ ..] => super::t4::n1259(s, out),
		['р', s @ ..] => super::t4::n1260(s, out),
		['у', s @ ..] => super::t4::n1261(s, out),
		['ы', s @ ..] => super::t4::n1262(s, out),
		['ю', s @ ..] => super::t4::n1263(s, out),
		['я', s @ ..] => super::t4::n1264(s, out),
		_ => false,
	}
}

/// Words ending -нный.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['а', s @ ..] => super::t4::n1265(s, out),
		['е', s @ ..] => super::t4::n1266(s, out),
		['и', s @ ..] => {
			super::push(out, 1235);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3527);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8340);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 15243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 3528);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15244);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 15245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 8341);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 15246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8342);
							s.is_empty()
						}
						['р', s @ ..] => super::t4::n8735(s, out),
						['у', s @ ..] => {
							super::push(out, 8344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t4::n3663(s, out),
				['г', s @ ..] => super::t4::n3664(s, out),
				['д', s @ ..] => super::t4::n3665(s, out),
				['ж', s @ ..] => super::t4::n3666(s, out),
				['з', s @ ..] => super::t4::n3667(s, out),
				['к', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				['л', s @ ..] => super::t4::n3669(s, out),
				['м', s @ ..] => super::t4::n3670(s, out),
				['н', s @ ..] => super::t4::n3671(s, out),
				['п', s @ ..] => {
					super::push(out, 3538);
					s.is_empty()
				}
				['р', s @ ..] => super::t4::n3673(s, out),
				['с', s @ ..] => super::t4::n3674(s, out),
				['т', s @ ..] => super::t4::n3675(s, out),
				['у', s @ ..] => super::t4::n3676(s, out),
				['ф', s @ ..] => super::t4::n3677(s, out),
				['х', s @ ..] => super::t4::n3678(s, out),
				['ч', s @ ..] => super::t4::n3679(s, out),
				['ш', s @ ..] => super::t4::n3680(s, out),
				['щ', s @ ..] => super::t4::n3681(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t4::n1268(s, out),
		['у', s @ ..] => super::t4::n1269(s, out),
		['ы', s @ ..] => super::t4::n1270(s, out),
		['э', s @ ..] => super::t4::n1271(s, out),
		['ю', s @ ..] => super::t4::n1272(s, out),
		['я', s @ ..] => super::t4::n1273(s, out),
		_ => false,
	}
}

/// Words ending -оный.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3587);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 8526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1244);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1245);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пный.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1246);
			match s {
				[] => true,
				['т', 'э', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3591);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8527);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 8528);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 8529);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8530);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15455);
									s.is_empty()
								}
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
			super::push(out, 1248);
			match s {
				[] => true,
				['т', 'о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 8531);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 15456);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 15457);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 22489);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'п', s @ ..] => {
									super::push(out, 15458);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 22490);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 8532);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 15459);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 22491);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1249);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 3593);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15460);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 15461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 8535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3596);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8536);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 15462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8537);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 15463);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 22492);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 27691);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 22493);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 22494);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 27692);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 27693);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 22495);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 27694);
													s.is_empty()
												}
												['й', s @ ..] => {
													super::push(out, 27695);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 22496);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 22497);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 22498);
											s.is_empty()
										}
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
					super::push(out, 3597);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8538);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15464);
									match s {
										[] => true,
										['и', '-', s @ ..] => {
											super::push(out, 22499);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 27696);
													s.is_empty()
												}
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
								['-', s @ ..] => {
									super::push(out, 15465);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 22500);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 15466);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 22501);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 22502);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 27697);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 22503);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 27698);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3598);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1250);
			match s {
				[] => true,
				['а', 'к', 'о', 'п', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t4::n1283(s, out),
		_ => false,
	}
}

/// Words ending -рный.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1252);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 8543);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15471);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 15472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3604);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8545);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', 'с', 'и', s @ ..] => {
									super::push(out, 15473);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 22507);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 15474);
									s.is_empty()
								}
								['т', 'о', s @ ..] => {
									super::push(out, 15475);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 22508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 8546);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 15476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8547);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15477);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 15478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 8548);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8549);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8550);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 8551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3606);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 8552);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									super::push(out, 15479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 8553);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3607);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3608);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15480);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 15481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3609);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3610);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3611);
					match s {
						[] => true,
						['о', 'т', 'о', 'н', 'с', 'е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 8555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3612);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3613);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							super::push(out, 8556);
							s.is_empty()
						}
						['о', 'ф', s @ ..] => {
							super::push(out, 8557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3614);
					s.is_empty()
				}
				['р', 'г', s @ ..] => {
					super::push(out, 3615);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3616);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 8558);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15482);
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
						['е', s @ ..] => {
							super::push(out, 8559);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 8560);
							match s {
								[] => true,
								['з', 'о', 'п', 'е', 'д', '-', 'о', 'н', 'т', 'е', 'ч', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 15483);
									s.is_empty()
								}
								['л', 'и', s @ ..] => {
									super::push(out, 15484);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 15485);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15486);
									s.is_empty()
								}
								['ц', 'о', 'б', s @ ..] => {
									super::push(out, 15487);
									match s {
										[] => true,
										['м', 'о', 'р', 'т', 'и', s @ ..] => {
											super::push(out, 22509);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 27701);
													s.is_empty()
												}
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
							super::push(out, 8561);
							match s {
								[] => true,
								['е', 'м', 'у', s @ ..] => {
									super::push(out, 15488);
									match s {
										[] => true,
										['к', 'о', 'д', '\u{1}', s @ ..] => {
											super::push(out, 22510);
											s.is_empty()
										}
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
					super::push(out, 3617);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 8562);
							s.is_empty()
						}
						['т', 'о', s @ ..] => {
							super::push(out, 8563);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3618);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8564);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8565);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n1285(s, out),
		['и', s @ ..] => super::t4::n1286(s, out),
		['о', s @ ..] => super::t4::n1287(s, out),
		['у', s @ ..] => super::t4::n1288(s, out),
		['ы', s @ ..] => {
			super::push(out, 1257);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8763);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1258);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3687);
					s.is_empty()
				}
				['ш', 'о', s @ ..] => {
					super::push(out, 3688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t4::n1291(s, out),
		_ => false,
	}
}

/// Words ending -сный.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['б', 'л', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3690);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3691);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3692);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 8769);
							match s {
								[] => true,
								['е', 'н', s @ ..] => {
									super::push(out, 15682);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 15683);
									s.is_empty()
								}
								['о', 'с', s @ ..] => {
									super::push(out, 15684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 8770);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 15685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3693);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 8771);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15686);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 22641);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15687);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 15688);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 22642);
											match s {
												[] => true,
												['н', 'и', 'с', '-', 'о', 'л', 'е', 'б', '\u{1}', s @ ..] => {
													super::push(out, 27785);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 27786);
													match s {
														[] => true,
														['р', s @ ..] => super::t4::n32329(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													match s {
														[] => true,
														['м', s @ ..] => super::t4::n32330(s, out),
														['н', s @ ..] => super::t4::n32331(s, out),
														['р', s @ ..] => super::t4::n32332(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 27787);
													match s {
														[] => true,
														['л', 'е', 'ж', s @ ..] => super::t4::n32333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['а', 'р', 'ф', 'н', 'и', '\u{1}', s @ ..] => {
									super::push(out, 15689);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 15690);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 22643);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 8774);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 8775);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 15692);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 8776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 3696);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3698);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3699);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8777);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8779);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3701);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t4::n3844(s, out),
				['м', s @ ..] => super::t4::n3845(s, out),
				['р', s @ ..] => super::t4::n3846(s, out),
				['т', s @ ..] => super::t4::n3847(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n1294(s, out),
		['й', s @ ..] => {
			super::push(out, 1263);
			s.is_empty()
		}
		['к', s @ ..] => super::t4::n1296(s, out),
		['н', s @ ..] => super::t4::n1297(s, out),
		['о', s @ ..] => super::t4::n1298(s, out),
		['п', s @ ..] => super::t4::n1299(s, out),
		['р', s @ ..] => super::t4::n1300(s, out),
		['с', s @ ..] => super::t4::n1301(s, out),
		['у', s @ ..] => super::t4::n1302(s, out),
		['ы', s @ ..] => {
			super::push(out, 1271);
			s.is_empty()
		}
		['ь', s @ ..] => super::t4::n1304(s, out),
		['ю', s @ ..] => super::t4::n1305(s, out),
		['я', s @ ..] => super::t4::n1306(s, out),
		_ => false,
	}
}

/// Words ending -тный.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1275);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3757);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8860);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8861);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 8862);
							match s {
								[] => true,
								['а', 'з', 'о', s @ ..] => {
									super::push(out, 15767);
									match s {
										[] => true,
										['к', 'з', s @ ..] => {
											super::push(out, 22689);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 27805);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3758);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3759);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 8863);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 22690);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 22691);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 27806);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'г', s @ ..] => {
											super::push(out, 22692);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 22693);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 27807);
													s.is_empty()
												}
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
							super::push(out, 8864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8865);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 8866);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 15768);
									s.is_empty()
								}
								['р', 'п', 'о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 22694);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 27808);
													match s {
														[] => true,
														['и', s @ ..] => super::t4::n32340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', 'е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 22695);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8867);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 8868);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8869);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 15769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3762);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8870);
							match s {
								[] => true,
								['п', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 15770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 8871);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 8872);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15771);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 22696);
											s.is_empty()
										}
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
					super::push(out, 3763);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8873);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 15772);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 22697);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8874);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 8875);
							s.is_empty()
						}
						['х', s @ ..] => super::t4::n9303(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t4::n3907(s, out),
				['р', s @ ..] => super::t4::n3908(s, out),
				['с', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				['т', s @ ..] => super::t4::n3910(s, out),
				['ф', s @ ..] => super::t4::n3911(s, out),
				['х', s @ ..] => super::t4::n3912(s, out),
				['ч', s @ ..] => super::t4::n3913(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n1308(s, out),
		['и', s @ ..] => super::t4::n1309(s, out),
		['й', s @ ..] => super::t4::n1310(s, out),
		['к', s @ ..] => super::t4::n1311(s, out),
		['л', s @ ..] => super::t4::n1312(s, out),
		['н', s @ ..] => super::t4::n1313(s, out),
		['о', s @ ..] => super::t4::n1314(s, out),
		['р', s @ ..] => super::t4::n1315(s, out),
		['с', s @ ..] => super::t4::n1316(s, out),
		['т', s @ ..] => super::t4::n1317(s, out),
		['у', s @ ..] => super::t4::n1318(s, out),
		['ф', s @ ..] => super::t4::n1319(s, out),
		['х', s @ ..] => super::t4::n1320(s, out),
		['ш', s @ ..] => super::t4::n1321(s, out),
		['ы', s @ ..] => super::t4::n1322(s, out),
		['ь', s @ ..] => super::t4::n1323(s, out),
		['э', s @ ..] => super::t4::n1324(s, out),
		['ю', s @ ..] => super::t4::n1325(s, out),
		['я', s @ ..] => super::t4::n1326(s, out),
		_ => false,
	}
}

/// Words ending -фный.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['р', 'г', 'о', s @ ..] => {
					super::push(out, 3864);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3865);
					s.is_empty()
				}
				['ь', 'л', 'е', 'р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9065);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 15998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['р', 'а', 'т', 'х', s @ ..] => {
					super::push(out, 3866);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['р', 'т', 'о', 'т', s @ ..] => {
					super::push(out, 3867);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9068);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 15999);
									s.is_empty()
								}
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
			super::push(out, 1299);
			match s {
				[] => true,
				['о', 'м', 'о', 'е', s @ ..] => {
					super::push(out, 3868);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9069);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цный.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3870);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1301);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чный.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3872);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 16001);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 16002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3873);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3874);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9072);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9073);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3875);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9075);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 9076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3876);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9077);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 9078);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 9079);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9080);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 9081);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3877);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9082);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3878);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16009);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 16010);
									match s {
										[] => true,
										['е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 27879);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n32366(s, out),
														_ => false,
													}
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 27880);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3879);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9084);
							match s {
								[] => true,
								['е', 'л', s @ ..] => {
									super::push(out, 16011);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 22846);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 22847);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 27881);
													match s {
														[] => true,
														['н', s @ ..] => super::t4::n32367(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 22848);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 9085);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16012);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 16013);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 22849);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16014);
									match s {
										[] => true,
										['р', 'п', 'о', s @ ..] => {
											super::push(out, 22850);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 27882);
													match s {
														[] => true,
														['д', s @ ..] => super::t4::n32368(s, out),
														_ => false,
													}
												}
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
							super::push(out, 9086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16015);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 16016);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 22851);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 9087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3880);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n1335(s, out),
		['и', s @ ..] => super::t4::n1336(s, out),
		['л', s @ ..] => {
			super::push(out, 1305);
			s.is_empty()
		}
		['о', s @ ..] => super::t5::n1338(s, out),
		['у', s @ ..] => super::t5::n1339(s, out),
		['ы', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3939);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3940);
					match s {
						[] => true,
						['я', s @ ..] => super::t5::n9749(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3941);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3942);
					match s {
						[] => true,
						['е', 'р', 'о', s @ ..] => {
							super::push(out, 9299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t5::n1341(s, out),
		['я', s @ ..] => super::t5::n1342(s, out),
		_ => false,
	}
}

/// Words ending -шный.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3958);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3959);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3960);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9316);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 9317);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3961);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3962);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9319);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23111);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 23112);
											s.is_empty()
										}
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
					super::push(out, 3963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3965);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3966);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9321);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3967);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3968);
					match s {
						[] => true,
						['с', 'у', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16348);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 16349);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23113);
											s.is_empty()
										}
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
					super::push(out, 3969);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16350);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 16351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3971);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3972);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16352);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 16353);
									match s {
										[] => true,
										['к', '-', 'о', 'н', 'т', 'е', 'ч', 'с', '\u{1}', s @ ..] => {
											super::push(out, 23114);
											s.is_empty()
										}
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
					super::push(out, 3973);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3974);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3975);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9323);
							match s {
								[] => true,
								['ф', 'о', s @ ..] => {
									super::push(out, 16354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3976);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3977);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3978);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 3979);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3980);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9324);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 3981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3982);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 9325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3983);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3984);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9327);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9328);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16356);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 23115);
											s.is_empty()
										}
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
					super::push(out, 3985);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3986);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9329);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3987);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9330);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9331);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16358);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 23116);
											s.is_empty()
										}
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
					super::push(out, 3988);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3989);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 3991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3992);
					s.is_empty()
				}
				['д', s @ ..] => super::t5::n4138(s, out),
				['з', s @ ..] => {
					super::push(out, 3994);
					match s {
						[] => true,
						['а', 'п', s @ ..] => super::t5::n9791(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t5::n4140(s, out),
				['м', s @ ..] => super::t5::n4141(s, out),
				['н', s @ ..] => super::t5::n4142(s, out),
				['т', s @ ..] => super::t5::n4143(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t5::n1351(s, out),
		['э', s @ ..] => super::t5::n1352(s, out),
		['ю', s @ ..] => super::t5::n1353(s, out),
		['я', s @ ..] => super::t5::n1354(s, out),
		_ => false,
	}
}

/// Words ending -щный.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4004);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9346);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9348);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 16366);
									match s {
										[] => true,
										['а', 'л', 'в', '-', s @ ..] => {
											super::push(out, 23122);
											match s {
												[] => true,
												['о', 'н', 'б', 'е', 'н', '\u{1}', s @ ..] => {
													super::push(out, 27996);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 16367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 9349);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9350);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 16368);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 16369);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 16370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9351);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9352);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9353);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16372);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23123);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 16373);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 23124);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 16374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 9354);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9355);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 16376);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23125);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 16377);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 9356);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9357);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16378);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 23127);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1326);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4011);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьный.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['л', s @ ..] => super::t5::n1360(s, out),
		['р', s @ ..] => {
			super::push(out, 1329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юный.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	s.is_empty()
}

