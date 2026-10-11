use flexion::Path;

/// Words ending -öte.
pub(crate) fn n1322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1320);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5726);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14566);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14567);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14568);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14569);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14570);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14571);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üte.
pub(crate) fn n1323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5727);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 5728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14572);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 14573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5729);
			match s {
				[] => true,
				['b', 'f', 'u', s @ ..] => {
					super::push(out, 14574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tze.
pub(crate) fn n1363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1361);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14653);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5815);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5816);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14655);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14656);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14657);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14658);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24866);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5817);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14659);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14660);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14661);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24868);
							match s {
								[] => true,
								['l', 'e', 'b', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 33236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14662);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 24869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5818);
			match s {
				[] => true,
				['p', 'e', s @ ..] => {
					super::push(out, 14663);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 24870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 14664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5819);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 5820);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14665);
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 5821);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14667);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14668);
					match s {
						[] => true,
						['c', 's', 'd', s @ ..] => {
							super::push(out, 24871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14669);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14670);
					match s {
						[] => true,
						['g', 'n', 'e', 't', 'n', s @ ..] => {
							super::push(out, 24872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word containerschiff.
pub(crate) fn n45017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42277);
	s.is_empty()
}

/// Words ending -riff.
pub(crate) fn n5930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5871);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14721);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 24904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 14722);
			match s {
				[] => true,
				['d', 'n', 'a', 'h', 'i', s @ ..] => {
					super::push(out, 24905);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 24906);
					match s {
						[] => true,
						['b', 's', 't', 'i', 'e', 'h', 'k', s @ ..] => {
							super::push(out, 33257);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 33258);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 24907);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 24908);
					match s {
						[] => true,
						['o', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 33259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 24909);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 38970);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 38971);
									match s {
										[] => true,
										['i', 'r', 'e', 't', 'n', 'a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 42278);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'f', s @ ..] => {
									super::push(out, 38972);
									match s {
										[] => true,
										['f', 'a', 'w', 'e', 'i', 'm', 'e', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 42279);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'g', 'n', 'u', 'i', s @ ..] => {
									super::push(out, 38973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'z', '-', s @ ..] => {
					super::push(out, 24910);
					match s {
						[] => true,
						['r', 'e', 'v', 'r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 33260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 14723);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 24911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -off.
pub(crate) fn n1395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5874);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 5875);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14725);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 14726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5876);
			match s {
				[] => true,
				['c', 's', 'i', 'b', s @ ..] => {
					match s {
						[] => true,
						['-', 'z', 't', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 24912);
							s.is_empty()
						}
						['z', 't', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 24913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5877);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5878);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14727);
					s.is_empty()
				}
				['s', s @ ..] => super::t12::n15003(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5879);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14729);
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5880);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14730);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uff.
pub(crate) fn n1396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5881);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14731);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 24929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5882);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5883);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14732);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14733);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 24930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5884);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5885);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14736);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14737);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5887);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5888);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5889);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öff.
pub(crate) fn n1398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	match s {
		[] => true,
		['t', '\u{1}', s @ ..] => {
			super::push(out, 5891);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word kinderfriedhof.
pub(crate) fn n45042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42294);
	s.is_empty()
}

/// The word selbstmörderfriedhof.
pub(crate) fn n45043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42295);
	s.is_empty()
}

/// The word wadenkrampf.
pub(crate) fn n45044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42296);
	s.is_empty()
}

/// Words ending -tenkrampf.
pub(crate) fn n45045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42297);
	match s {
		[] => true,
		['s', 'u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 43900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -opf.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1427);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5937);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14772);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14773);
					s.is_empty()
				}
				['l', 'e', 'g', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 14774);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14775);
					match s {
						[] => true,
						['u', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5938);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 24962);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 24963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 14776);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24964);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'l', s @ ..] => {
							super::push(out, 24966);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 24967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14777);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'n', 'k', s @ ..] => {
									super::push(out, 33297);
									match s {
										[] => true,
										['c', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 38996);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 33298);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 33299);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 38997);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 24968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14778);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'n', s @ ..] => {
									super::push(out, 33300);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 33301);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', s @ ..] => {
											super::push(out, 38998);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38999);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 33302);
									s.is_empty()
								}
								['z', 't', 'a', 'k', s @ ..] => {
									super::push(out, 33303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', s @ ..] => {
							super::push(out, 24969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 24970);
							match s {
								[] => true,
								['n', 'ä', 'm', '\u{1}', s @ ..] => {
									super::push(out, 33304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 24971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['f', 'l', s @ ..] => {
							super::push(out, 24972);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 24973);
							s.is_empty()
						}
						['u', 'a', s @ ..] => {
							super::push(out, 24974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 14779);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 14780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5939);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24975);
							s.is_empty()
						}
						['g', 'u', s @ ..] => {
							super::push(out, 24976);
							s.is_empty()
						}
						['l', 'h', s @ ..] => {
							super::push(out, 24977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5940);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14781);
					match s {
						[] => true,
						['p', 't', 'u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5941);
			match s {
				[] => true,
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24979);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 24981);
							s.is_empty()
						}
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 33305);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 33306);
									match s {
										[] => true,
										['c', 's', 'i', s @ ..] => {
											match s {
												[] => true,
												['e', 'l', 'f', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n46848(s, out),
														['d', 'n', 'i', 'r', '\u{1}', s @ ..] => super::t12::n46849(s, out),
														_ => false,
													}
												}
												['f', '\u{1}', s @ ..] => {
													super::push(out, 42298);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', 'e', 'm', s @ ..] => {
									super::push(out, 33307);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 33308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 24982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'u', 'e', s @ ..] => {
					super::push(out, 14783);
					s.is_empty()
				}
				['s', 'i', s @ ..] => {
					super::push(out, 14784);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 14785);
					match s {
						[] => true,
						['l', 'e', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 24983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5942);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word mitgeschöpf.
pub(crate) fn n25877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24990);
	s.is_empty()
}

/// The word zusatzverkauf.
pub(crate) fn n45048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42299);
	s.is_empty()
}

/// Words ending -lauf.
pub(crate) fn n6016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5957);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14808);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'g', s @ ..] => {
									super::push(out, 39010);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 39011);
									s.is_empty()
								}
								['t', 'f', s @ ..] => {
									super::push(out, 39012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 33326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 14809);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 33327);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 14810);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 14811);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33329);
							s.is_empty()
						}
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 33330);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 33331);
							s.is_empty()
						}
						['u', 's', 'r', 'e', 'd', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 33332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 25008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 14812);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14813);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25009);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 25010);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 14814);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 14815);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25011);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 33334);
							match s {
								[] => true,
								['p', 'e', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 39013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 25012);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 39014);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 39015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 33335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 14816);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 33336);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 39016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 33337);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 39017);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 42300);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 33338);
							match s {
								[] => true,
								['n', 'e', 'ß', s @ ..] => {
									super::push(out, 39018);
									s.is_empty()
								}
								['s', 'n', 'o', 'i', 't', s @ ..] => {
									super::push(out, 39019);
									s.is_empty()
								}
								['t', 'a', 'h', s @ ..] => {
									super::push(out, 39020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 25013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14817);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 25014);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25015);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', 'k', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 42301);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 42302);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 42303);
											match s {
												[] => true,
												['u', 'l', 'b', '\u{1}', s @ ..] => {
													super::push(out, 43903);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 25016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 14818);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 25017);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 25018);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 33339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 25019);
					s.is_empty()
				}
				['u', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 25020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -verruf.
pub(crate) fn n15117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14825);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sruf.
pub(crate) fn n6028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5969);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 14826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -truf.
pub(crate) fn n6029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5970);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 14827);
			s.is_empty()
		}
		['o', 'n', s @ ..] => {
			match s {
				[] => true,
				['-', 'r', s @ ..] => {
					super::push(out, 25024);
					s.is_empty()
				}
				['n', 'e', 'k', s @ ..] => {
					super::push(out, 25025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 14828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -alumschlag.
pub(crate) fn n46851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43904);
	s.is_empty()
}

/// Words ending -ioumschlag.
pub(crate) fn n46852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43905);
	s.is_empty()
}

/// Words ending -erumschlag.
pub(crate) fn n46853(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 44661);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['t', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 44916);
					s.is_empty()
				}
				['ü', 'g', '\u{1}', s @ ..] => {
					super::push(out, 44917);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word zungenschlag.
pub(crate) fn n46854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43906);
	s.is_empty()
}

/// The word ausschlag.
pub(crate) fn n46855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43907);
	s.is_empty()
}

/// The word hautausschlag.
pub(crate) fn n46856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43908);
	s.is_empty()
}

/// The word totschlag.
pub(crate) fn n46857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43909);
	s.is_empty()
}

/// Words ending -verlag.
pub(crate) fn n25933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25036);
	s.is_empty()
}

/// Words ending -slag.
pub(crate) fn n6039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5980);
	match s {
		[] => true,
		['l', 'e', 'g', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 14835);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tlag.
pub(crate) fn n6040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5981);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulag.
pub(crate) fn n6041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5982);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 14837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrungsvertrag.
pub(crate) fn n46858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43910);
	match s {
		[] => true,
		['ü', 'f', 'b', 'a', 's', 'i', 'n', 'b', 'e', 'g', 'r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 44662);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word koalitionsvertrag.
pub(crate) fn n46859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43911);
	s.is_empty()
}

/// Words ending -mittag.
pub(crate) fn n25970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25068);
	match s {
		[] => true,
		['g', 'a', 't', 's', s @ ..] => {
			super::push(out, 33372);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 33373);
			match s {
				[] => true,
				['c', 'a', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42317);
							s.is_empty()
						}
						['g', 'a', 't', 's', s @ ..] => {
							super::push(out, 42318);
							s.is_empty()
						}
						['i', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 42319);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 42320);
							match s {
								[] => true,
								['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 43912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'b', 'm', 'e', s @ ..] => {
							match s {
								[] => true,
								['t', 'p', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 43913);
									s.is_empty()
								}
								['z', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 43914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'n', 'i', 'l', 'h', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 42321);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 42322);
							match s {
								[] => true,
								['n', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 43915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'o', 'v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39042);
					s.is_empty()
				}
				['g', 'a', 't', 's', s @ ..] => {
					super::push(out, 39043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 33374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39044);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word samnitenkrieg.
pub(crate) fn n45074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42323);
	s.is_empty()
}

/// The word abstieg.
pub(crate) fn n25986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25083);
	s.is_empty()
}

/// The word aufstieg.
pub(crate) fn n25987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25084);
	s.is_empty()
}

/// Words ending -anstieg.
pub(crate) fn n25988(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 33395);
			s.is_empty()
		}
		['s', 'r', s @ ..] => {
			super::push(out, 33396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sstieg.
pub(crate) fn n25989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25085);
	s.is_empty()
}

/// Words ending -leg.
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1452);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6005);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6006);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14870);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 14871);
					match s {
						[] => true,
						['i', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33397);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 33398);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33399);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 'd', 'n', 'ü', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 33400);
									s.is_empty()
								}
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
			super::push(out, 6007);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14872);
					match s {
						[] => true,
						['k', 'n', 'e', 't', 's', s @ ..] => {
							super::push(out, 25086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reg.
pub(crate) fn n1455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1453);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6008);
			match s {
				[] => true,
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 14873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -teg.
pub(crate) fn n1456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1454);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14874);
					s.is_empty()
				}
				['n', 'e', 'h', s @ ..] => {
					super::push(out, 14875);
					match s {
						[] => true,
						['c', 's', 'i', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 25087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14876);
					match s {
						[] => true,
						['e', 't', 't', 'e', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 25088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weg.
pub(crate) fn n1457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1455);
	match s {
		[] => true,
		['b', 'u', s @ ..] => {
			super::push(out, 6009);
			match s {
				[] => true,
				['h', 'c', 's', 'h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 14877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6010);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14878);
					match s {
						[] => true,
						['i', 'v', 'r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 25089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'u', 'n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 6011);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6012);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 25090);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 33401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'l', s @ ..] => {
							super::push(out, 25091);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 25092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14879);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 25093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6013);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14880);
					match s {
						[] => true,
						['o', 'k', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 25094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'u', 't', s @ ..] => {
					super::push(out, 14881);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 25095);
							match s {
								[] => true,
								['e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 33402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'r', s @ ..] => {
			super::push(out, 6014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -önig.
pub(crate) fn n6088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6028);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 25101);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33408);
							match s {
								[] => true,
								['n', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 39062);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 39063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 33409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 25102);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25103);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 25104);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', 'o', 'i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 33412);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 33413);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 33414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 25105);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 39064);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 42325);
											s.is_empty()
										}
										['i', 'k', 'u', 'e', 'l', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 42326);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'e', s @ ..] => {
									super::push(out, 39065);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 42327);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', 's', s @ ..] => {
									super::push(out, 39066);
									match s {
										[] => true,
										['i', 'w', 'z', '\u{1}', s @ ..] => {
											super::push(out, 42328);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 39067);
									match s {
										[] => true,
										['n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 39068);
									s.is_empty()
								}
								['m', 'h', 'ö', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39069);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42330);
											match s {
												[] => true,
												['m', 'r', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 43916);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'a', 'm', 'r', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 42331);
											s.is_empty()
										}
										['ä', 'd', '\u{1}', s @ ..] => {
											super::push(out, 42332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 39070);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42333);
											s.is_empty()
										}
										['o', 'g', s @ ..] => {
											super::push(out, 42334);
											match s {
												[] => true,
												['t', 's', s @ ..] => {
													match s {
														[] => true,
														['e', 'w', '\u{1}', s @ ..] => super::t12::n47690(s, out),
														['o', '\u{1}', s @ ..] => super::t12::n47691(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 42335);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 39071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 33415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', s @ ..] => {
					super::push(out, 25106);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 33416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25107);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 39072);
									s.is_empty()
								}
								['s', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 39073);
									s.is_empty()
								}
								['ä', 'n', 'o', 'm', 's', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 39074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 25108);
					match s {
						[] => true,
						['t', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 33417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 33418);
							match s {
								[] => true,
								['e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 39075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 33419);
							match s {
								[] => true,
								['e', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 39076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntweinessig.
pub(crate) fn n26021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25111);
	s.is_empty()
}

/// Words ending -ressig.
pub(crate) fn n26022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25112);
	s.is_empty()
}

/// Words ending -sessig.
pub(crate) fn n26023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25113);
	s.is_empty()
}

/// Words ending -issig.
pub(crate) fn n15194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14888);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 25114);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word fittig.
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1471);
	s.is_empty()
}

/// Words ending -zig.
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1472);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 6031);
			match s {
				[] => true,
				['c', 'e', 's', 'd', 'n', 'u', 's', s @ ..] => {
					super::push(out, 14889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6032);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6033);
			match s {
				[] => true,
				['e', 'i', 'v', 'd', 'n', 'u', 'i', 'e', 'w', s @ ..] => {
					super::push(out, 14890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'i', s @ ..] => {
			super::push(out, 6034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßig.
pub(crate) fn n1475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1473);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ang.
pub(crate) fn n1480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1478);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6039);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14898);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6040);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14900);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14901);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14902);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 25116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14903);
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							super::push(out, 25117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14904);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							super::push(out, 25118);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 33423);
									s.is_empty()
								}
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 33424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14905);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25119);
							match s {
								[] => true,
								['s', 'r', 'e', 'v', s @ ..] => {
									super::push(out, 33425);
									s.is_empty()
								}
								['t', 'r', s @ ..] => {
									super::push(out, 33426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 25120);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 25121);
							match s {
								[] => true,
								['e', 'n', 'e', 'n', 'o', 't', s @ ..] => {
									super::push(out, 33428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 25122);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 33429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14906);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33430);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 33431);
									s.is_empty()
								}
								['n', 'o', s @ ..] => {
									super::push(out, 33432);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 33433);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 33434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14907);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 25123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6041);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14908);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 14909);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25124);
							match s {
								[] => true,
								['r', 'r', 'o', 't', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 33435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14910);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 25125);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 25126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 14911);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 14912);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 25127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14913);
					match s {
						[] => true,
						['c', 'r', 'u', s @ ..] => {
							super::push(out, 25128);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 33436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 25129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14914);
					match s {
						[] => true,
						['c', 'ü', 'r', s @ ..] => super::t12::n26042(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14915);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25130);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 33441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => super::t12::n26044(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t12::n15222(s, out),
				['n', s @ ..] => super::t12::n15223(s, out),
				['r', s @ ..] => super::t12::n15224(s, out),
				['s', s @ ..] => super::t12::n15225(s, out),
				['t', s @ ..] => super::t12::n15226(s, out),
				['u', s @ ..] => super::t12::n15227(s, out),
				['z', s @ ..] => super::t12::n15228(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t12::n6103(s, out),
		['i', s @ ..] => super::t12::n6104(s, out),
		['j', s @ ..] => super::t12::n6105(s, out),
		['k', s @ ..] => super::t12::n6106(s, out),
		['l', s @ ..] => super::t12::n6107(s, out),
		['m', s @ ..] => super::t12::n6108(s, out),
		['r', s @ ..] => super::t12::n6109(s, out),
		['s', s @ ..] => super::t12::n6110(s, out),
		['t', s @ ..] => super::t12::n6111(s, out),
		['u', s @ ..] => super::t12::n6112(s, out),
		['w', s @ ..] => super::t12::n6113(s, out),
		['y', s @ ..] => super::t12::n6114(s, out),
		_ => false,
	}
}

/// Words ending -ing.
pub(crate) fn n1482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1480);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6057);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 25177);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 25178);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 25179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14967);
					match s {
						[] => true,
						['o', 'b', 'm', s @ ..] => {
							super::push(out, 25180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6058);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14968);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14969);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25181);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 25182);
							match s {
								[] => true,
								['o', 's', 'd', 'w', 'o', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33473);
									s.is_empty()
								}
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
			super::push(out, 6059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14970);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 14971);
					match s {
						[] => true,
						['r', 't', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 25183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 14972);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25184);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 25185);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 33474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 25186);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['m', 'u', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 39111);
											s.is_empty()
										}
										['r', 'e', 'e', 'b', 'd', 'r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 39112);
											s.is_empty()
										}
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
					super::push(out, 14973);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14974);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25187);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 25188);
							match s {
								[] => true,
								['m', 'u', 'g', '\u{1}', s @ ..] => {
									super::push(out, 33475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 25189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14975);
					match s {
						[] => true,
						['i', 't', 's', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 25190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14976);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25191);
							match s {
								[] => true,
								['t', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 33476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 25192);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 25193);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 25194);
							match s {
								[] => true,
								['a', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 33477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 25195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14977);
					match s {
						[] => true,
						['e', 'h', 'c', 's', 'i', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 25196);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 25197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33478);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 33479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14978);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 25198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14979);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25199);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 25200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14980);
					match s {
						[] => true,
						['e', 'd', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 25201);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 25202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14981);
					match s {
						[] => true,
						['r', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 25203);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 25204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 14982);
					match s {
						[] => true,
						['i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 25205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6060);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6061);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14984);
					match s {
						[] => true,
						['u', 's', 'h', s @ ..] => {
							super::push(out, 25206);
							match s {
								[] => true,
								['c', 'u', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33480);
									s.is_empty()
								}
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
			super::push(out, 6062);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['s', 's', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 25207);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 25208);
							match s {
								[] => true,
								['s', 'e', 'm', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 33481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 14985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6063);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14986);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25209);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 33482);
									s.is_empty()
								}
								['o', 'c', s @ ..] => {
									super::push(out, 33483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 25210);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 25211);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33485);
									match s {
										[] => true,
										['w', 's', '-', 'e', 'd', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 39113);
											s.is_empty()
										}
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
					super::push(out, 14987);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25212);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 25213);
							match s {
								[] => true,
								['f', 't', 'a', s @ ..] => {
									super::push(out, 33486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t12::n15296(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t12::n6125(s, out),
		['l', s @ ..] => super::t12::n6126(s, out),
		['m', s @ ..] => super::t12::n6127(s, out),
		['n', s @ ..] => super::t12::n6128(s, out),
		['p', s @ ..] => super::t12::n6129(s, out),
		['r', s @ ..] => super::t12::n6130(s, out),
		['s', s @ ..] => super::t12::n6131(s, out),
		['t', s @ ..] => super::t12::n6132(s, out),
		['v', s @ ..] => super::t12::n6133(s, out),
		['w', s @ ..] => {
			super::push(out, 6073);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 15074);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => super::t12::n6135(s, out),
		['y', s @ ..] => super::t12::n6136(s, out),
		['z', s @ ..] => super::t12::n6137(s, out),
		_ => false,
	}
}

/// Words ending -ung.
pub(crate) fn n1484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1482);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6086);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15091);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 25340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', 'h', 'c', 's', 'r', 'e', 'v', '-', s @ ..] => {
					super::push(out, 15092);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 25341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', 'r', 'h', 'c', 's', 's', 't', s @ ..] => {
					super::push(out, 15093);
					match s {
						[] => true,
						['h', 'c', 'i', 'h', 'c', 's', 'e', 'g', 'n', s @ ..] => {
							super::push(out, 25342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 15094);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 25343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					super::push(out, 15095);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 25344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 's', s @ ..] => {
					super::push(out, 15096);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33579);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 33580);
									s.is_empty()
								}
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
			super::push(out, 6087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15097);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 15098);
					match s {
						[] => true,
						['l', 'r', s @ ..] => {
							super::push(out, 25345);
							match s {
								[] => true,
								['e', 'v', 'o', s @ ..] => {
									super::push(out, 33581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 15099);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25346);
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 33582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 15100);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 25347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15101);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 33583);
									match s {
										[] => true,
										['c', 's', 's', s @ ..] => {
											super::push(out, 39156);
											match s {
												[] => true,
												['u', 'a', 'n', s @ ..] => {
													super::push(out, 42364);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', 'k', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', s @ ..] => {
											match s {
												[] => true,
												['n', 'e', 'h', s @ ..] => {
													super::push(out, 42365);
													s.is_empty()
												}
												['r', 'e', 'u', s @ ..] => {
													super::push(out, 42366);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 39157);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 39158);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 42367);
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 42368);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'a', 't', 'n', s @ ..] => {
													super::push(out, 42369);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 42370);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'i', s @ ..] => {
											super::push(out, 39159);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'r', 'e', 'v', 'n', s @ ..] => {
									super::push(out, 33584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t12::n15416(s, out),
				['m', s @ ..] => {
					super::push(out, 15103);
					match s {
						[] => true,
						['e', 'r', 'f', 'r', 'e', 'b', s @ ..] => {
							super::push(out, 25352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15104);
					match s {
						[] => true,
						['e', s @ ..] => super::t12::n26285(s, out),
						['i', s @ ..] => super::t12::n26286(s, out),
						['u', 's', s @ ..] => super::t12::n26287(s, out),
						['ü', s @ ..] => super::t12::n26288(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t12::n15419(s, out),
				['ü', 'm', 'r', 'e', 'l', s @ ..] => {
					super::push(out, 15105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t12::n6149(s, out),
		['g', s @ ..] => super::t12::n6150(s, out),
		['h', s @ ..] => super::t12::n6151(s, out),
		['j', s @ ..] => super::t12::n6152(s, out),
		['k', s @ ..] => super::t12::n6153(s, out),
		['l', s @ ..] => super::t12::n6154(s, out),
		['m', s @ ..] => super::t12::n6155(s, out),
		['n', s @ ..] => super::t12::n6156(s, out),
		['p', 'p', 'e', s @ ..] => super::t12::n6157(s, out),
		['r', s @ ..] => super::t13::n6158(s, out),
		['s', s @ ..] => super::t13::n6159(s, out),
		['t', s @ ..] => super::t13::n6160(s, out),
		['u', s @ ..] => super::t13::n6161(s, out),
		['w', s @ ..] => super::t13::n6162(s, out),
		['z', s @ ..] => super::t13::n6163(s, out),
		['ß', s @ ..] => super::t13::n6164(s, out),
		_ => false,
	}
}

/// Words ending -nburg.
pub(crate) fn n15537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15204);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 33785);
					match s {
						[] => true,
						['n', 'a', 'r', 'b', 't', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 39333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 33786);
					match s {
						[] => true,
						['i', 'e', 'w', '-', 'h', 'c', 's', 'i', 'h', 'c', 'e', 'i', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 39334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sburg.
pub(crate) fn n15539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15206);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 25474);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tburg.
pub(crate) fn n15540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15207);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 25475);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 33787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iurg.
pub(crate) fn n6197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6135);
	match s {
		[] => true,
		['m', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 15208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lurg.
pub(crate) fn n6198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6136);
	s.is_empty()
}

/// Words ending -rurg.
pub(crate) fn n6199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6137);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 15209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -turg.
pub(crate) fn n6200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fahrzeug.
pub(crate) fn n35583(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 'u', s @ ..] => {
			super::push(out, 39341);
			s.is_empty()
		}
		['l', 'i', s @ ..] => {
			super::push(out, 39342);
			s.is_empty()
		}
		['n', 'e', 'm', s @ ..] => {
			super::push(out, 39343);
			s.is_empty()
		}
		['t', 's', s @ ..] => {
			super::push(out, 39344);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 42494);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 42495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tzeug.
pub(crate) fn n15556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15223);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 25495);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 25496);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word putzzeug.
pub(crate) fn n26466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25497);
	s.is_empty()
}

/// Words ending -ßzeug.
pub(crate) fn n15558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15225);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 25498);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 25499);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 33795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fug.
pub(crate) fn n1504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1502);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6145);
			s.is_empty()
		}
		['d', 'ö', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 6146);
			s.is_empty()
		}
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 6147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hug.
pub(crate) fn n1505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1503);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6148);
			match s {
				[] => true,
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 15226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jug.
pub(crate) fn n1506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1504);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lug.
pub(crate) fn n1507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1505);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6151);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 6152);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 15227);
					match s {
						[] => true,
						['e', 's', 's', 'a', 'l', 'k', '-', 'e', 't', 's', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 25500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'b', s @ ..] => {
					super::push(out, 15228);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15229);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 25501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'n', 'e', 's', s @ ..] => {
					super::push(out, 15230);
					match s {
						[] => true,
						['h', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 25502);
							s.is_empty()
						}
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 25503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'k', s @ ..] => {
					super::push(out, 15231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6153);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 15232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6154);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 15233);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rug.
pub(crate) fn n1508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1506);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 6155);
			match s {
				[] => true,
				['f', 'r', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 15234);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 15235);
					s.is_empty()
				}
				['l', 'l', s @ ..] => {
					super::push(out, 15236);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6156);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					match s {
						[] => true,
						['l', 'h', s @ ..] => {
							super::push(out, 25504);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 25505);
							match s {
								[] => true,
								['e', 'u', s @ ..] => {
									super::push(out, 33796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 't', s @ ..] => {
							super::push(out, 25506);
							match s {
								[] => true,
								['a', 'r', 'i', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 33797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 25507);
							match s {
								[] => true,
								['s', 'b', s @ ..] => {
									super::push(out, 33798);
									match s {
										[] => true,
										['l', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 39345);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zug.
pub(crate) fn n1509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1507);
	match s {
		[] => true,
		['-', 'r', s @ ..] => {
			super::push(out, 6157);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 6158);
			match s {
				[] => true,
				['l', 'e', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25508);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 25509);
							match s {
								[] => true,
								['x', s @ ..] => {
									super::push(out, 33799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 25510);
							match s {
								[] => true,
								['m', 'm', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 33800);
									s.is_empty()
								}
								['t', 'n', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 33801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'r', s @ ..] => {
							super::push(out, 25511);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 25512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6159);
			match s {
				[] => true,
				['b', 'z', s @ ..] => {
					super::push(out, 15238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6160);
			match s {
				[] => true,
				['c', 'a', 'n', s @ ..] => {
					super::push(out, 15239);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 25513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6161);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15240);
					s.is_empty()
				}
				['l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 15241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'm', s @ ..] => {
			super::push(out, 6162);
			match s {
				[] => true,
				['i', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 15242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6163);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 25514);
							s.is_empty()
						}
						['h', 'c', 'u', s @ ..] => {
							super::push(out, 25515);
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 33802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 25516);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 33803);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 25517);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 39346);
											s.is_empty()
										}
										['s', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 39347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'h', 'c', 'u', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 25518);
							s.is_empty()
						}
						['s', 'g', 'n', 'i', 'n', 'i', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 25519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6164);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 25520);
							match s {
								[] => true,
								['ü', '\u{1}', s @ ..] => {
									super::push(out, 33804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', s @ ..] => {
							super::push(out, 25521);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 33805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 25522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33806);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['l', 'h', 'a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 39348);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 39349);
											s.is_empty()
										}
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
					super::push(out, 15244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6165);
			match s {
				[] => true,
				['g', 'n', 'u', 'n', s @ ..] => {
					super::push(out, 15245);
					match s {
						[] => true,
						['f', 'f', 'ö', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 25523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', s @ ..] => {
					super::push(out, 15246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6166);
			match s {
				[] => true,
				['f', 'i', s @ ..] => {
					super::push(out, 15247);
					match s {
						[] => true,
						['r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 25524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 15248);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15249);
					match s {
						[] => true,
						['e', 'l', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 25525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15250);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 33807);
									match s {
										[] => true,
										['a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 39350);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 't', s @ ..] => {
									super::push(out, 33808);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 39351);
											s.is_empty()
										}
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
					super::push(out, 15251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6167);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 6168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -chach.
pub(crate) fn n15617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15280);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 33816);
					match s {
						[] => true,
						['e', 'n', 'i', 'l', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 39354);
							s.is_empty()
						}
						['t', 'e', 'l', 'l', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 39355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 33817);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 33818);
					s.is_empty()
				}
				['e', 'h', s @ ..] => {
					super::push(out, 33819);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 39356);
							s.is_empty()
						}
						['l', 'e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 39357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 33820);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 39358);
							s.is_empty()
						}
						['r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 39359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 33821);
					match s {
						[] => true,
						['e', 'u', s @ ..] => {
							super::push(out, 39360);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 42496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 33822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -llach.
pub(crate) fn n15620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15283);
	match s {
		[] => true,
		['a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 25540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rlach.
pub(crate) fn n15621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15284);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 25541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mach.
pub(crate) fn n6246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6183);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15285);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 15286);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 33823);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 39361);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 39362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 33824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15287);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15288);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 15289);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 15290);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 25542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nach.
pub(crate) fn n6247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15291);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 25543);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 25544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 15292);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oach.
pub(crate) fn n6248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6185);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 15293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25545);
					s.is_empty()
				}
				['f', 'u', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 25546);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rach.
pub(crate) fn n6249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6186);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 15294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sach.
pub(crate) fn n6250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6187);
	s.is_empty()
}

/// Words ending -ech.
pub(crate) fn n1529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1527);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6188);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 15295);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 15296);
					match s {
						[] => true,
						['ß', s @ ..] => {
							super::push(out, 25547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6189);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25548);
							s.is_empty()
						}
						['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 25549);
							s.is_empty()
						}
						['n', 'n', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 25550);
							s.is_empty()
						}
						['r', 'e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 25551);
							s.is_empty()
						}
						['z', 'l', s @ ..] => {
							super::push(out, 25552);
							s.is_empty()
						}
						['ß', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 25553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6190);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 15297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6191);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 15298);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15299);
					match s {
						[] => true,
						['e', 'k', 'r', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 25554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 15300);
					match s {
						[] => true,
						['d', 'n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 25555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 15301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6192);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15302);
					match s {
						[] => true,
						['n', 'i', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 25556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 15303);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33826);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33827);
									match s {
										[] => true,
										['ü', s @ ..] => {
											super::push(out, 39363);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6193);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6194);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 15304);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15305);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 15306);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 15307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ich.
pub(crate) fn n1530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1528);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6195);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 6196);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 15308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 6197);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6198);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 6199);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6200);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 15309);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 15310);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 25557);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39364);
											s.is_empty()
										}
										['l', 'ö', '\u{1}', s @ ..] => {
											super::push(out, 39365);
											s.is_empty()
										}
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
					super::push(out, 15311);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15312);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 25559);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 33828);
									match s {
										[] => true,
										['e', 'v', 't', s @ ..] => {
											super::push(out, 39366);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'a', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 39367);
											match s {
												[] => true,
												['c', 'u', 'r', 'd', '\u{1}', s @ ..] => {
													super::push(out, 42497);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 39368);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 39369);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['g', 'n', 'e', 'm', '\u{1}', s @ ..] => super::t13::n46950(s, out),
														['s', 's', 'e', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => super::t13::n46951(s, out),
														['t', s @ ..] => super::t13::n46952(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'u', s @ ..] => {
													match s {
														[] => true,
														['l', 'h', 'a', 'z', '\u{1}', s @ ..] => super::t13::n46953(s, out),
														['t', s @ ..] => super::t13::n46954(s, out),
														_ => false,
													}
												}
												['n', 'o', 'i', 't', s @ ..] => {
													super::push(out, 42498);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'f', s @ ..] => {
											super::push(out, 39370);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 39371);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t13::n15651(s, out),
				['t', s @ ..] => {
					super::push(out, 15314);
					match s {
						[] => true,
						['r', 'e', 't', s @ ..] => {
							super::push(out, 25572);
							s.is_empty()
						}
						['s', 'g', s @ ..] => {
							super::push(out, 25573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t13::n6264(s, out),
		['m', s @ ..] => {
			super::push(out, 6202);
			match s {
				[] => true,
				['f', 'o', 'o', 'k', '\u{1}', s @ ..] => super::t13::n15655(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t13::n6266(s, out),
		['p', s @ ..] => super::t13::n6267(s, out),
		['r', s @ ..] => super::t13::n6268(s, out),
		['s', s @ ..] => super::t13::n6269(s, out),
		['t', s @ ..] => super::t13::n6270(s, out),
		['w', s @ ..] => super::t13::n6271(s, out),
		_ => false,
	}
}

/// Words ending -lch.
pub(crate) fn n1531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1529);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6209);
			match s {
				[] => true,
				['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 15325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6210);
			match s {
				[] => true,
				['k', 'n', s @ ..] => {
					super::push(out, 15326);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25589);
							s.is_empty()
						}
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 25590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6211);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 15328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25592);
							s.is_empty()
						}
						['-', 'e', 'e', 's', 'n', 'e', 'd', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 25593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 15329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25594);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 25595);
							s.is_empty()
						}
						['e', 'g', s @ ..] => {
							super::push(out, 25596);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 25597);
							s.is_empty()
						}
						['k', 'n', s @ ..] => {
							super::push(out, 25598);
							s.is_empty()
						}
						['l', 'e', 'g', s @ ..] => {
							super::push(out, 25599);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 33873);
									s.is_empty()
								}
								['n', 'n', s @ ..] => {
									super::push(out, 33874);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 39413);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													super::push(out, 42516);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 33875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'f', s @ ..] => {
							super::push(out, 25600);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 25601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15330);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 25602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 15331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6212);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6213);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 15332);
					match s {
						[] => true,
						['n', 'e', 'i', s @ ..] => {
							super::push(out, 25603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nch.
pub(crate) fn n1532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1530);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6214);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 15333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6215);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 15334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6216);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15335);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15336);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 15337);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 6217);
			match s {
				[] => true,
				['m', 'r', 'e', 'n', 'i', 'z', s @ ..] => {
					super::push(out, 15339);
					match s {
						[] => true,
						['u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 25605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -och.
pub(crate) fn n1533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1531);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 6218);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15340);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15341);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 25606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 6219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15342);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 6220);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15344);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 25607);
							s.is_empty()
						}
						['t', 'h', 'c', 'ü', 'r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 25608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15345);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15346);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 33878);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 33879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 't', 's', 'i', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 25609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 15347);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6221);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 15348);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 25611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 15349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'u', 'r', s @ ..] => {
					super::push(out, 15350);
					s.is_empty()
				}
				['e', 's', 'u', 'ä', s @ ..] => {
					super::push(out, 15351);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 25613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 15352);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 15353);
					match s {
						[] => true,
						['c', 's', 'r', 'a', 'n', s @ ..] => {
							super::push(out, 25614);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 39414);
											s.is_empty()
										}
										['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 39415);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 39416);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'c', 'i', s @ ..] => {
					super::push(out, 15354);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15355);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15356);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 25615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 15357);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 25616);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 25617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15358);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 33880);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'm', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 33881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'o', '\u{1}', s @ ..] => {
							super::push(out, 25618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 15359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6223);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 15361);
					match s {
						[] => true,
						['t', 'i', 'm', 'r', 'e', 'h', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 25619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rch.
pub(crate) fn n1534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1532);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6225);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6226);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 15363);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33882);
									s.is_empty()
								}
								['e', 'n', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 33883);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 33884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 15364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6227);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 15365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6228);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 15366);
					match s {
						[] => true,
						['c', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 25620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sch.
pub(crate) fn n1535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1533);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6229);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15368);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 15369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15370);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 25623);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33885);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39418);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 25624);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'l', s @ ..] => {
											super::push(out, 39419);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 39420);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['f', s @ ..] => super::t13::n46973(s, out),
														['s', s @ ..] => super::t13::n46974(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15371);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 25625);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 25626);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 25627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15372);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 15373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15374);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 25629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 15375);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 25630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 15376);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 25631);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 25632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6230);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15377);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 15378);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 15379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6232);
			s.is_empty()
		}
		['i', s @ ..] => super::t13::n6296(s, out),
		['l', s @ ..] => {
			super::push(out, 6234);
			match s {
				[] => true,
				['e', 'w', 'r', 'e', 'd', 'u', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 15406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6235);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6236);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15407);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 25682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 15408);
					match s {
						[] => true,
						['m', '-', s @ ..] => {
							super::push(out, 25683);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15409);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 25684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 15410);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 25685);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 33940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 25686);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 25687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 15411);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 25688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6237);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 15412);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25689);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 25690);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 33942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15413);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['h', 't', s @ ..] => {
									super::push(out, 33943);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 33944);
									s.is_empty()
								}
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
			super::push(out, 6238);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 15414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6239);
			match s {
				[] => true,
				['a', s @ ..] => super::t13::n15756(s, out),
				['e', s @ ..] => {
					super::push(out, 15416);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 25700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15417);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 25701);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 25702);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 25703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 15418);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 25704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 15419);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 25705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t13::n6303(s, out),
		['u', s @ ..] => super::t13::n6304(s, out),
		['w', s @ ..] => {
			super::push(out, 6242);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 15434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t13::n6306(s, out),
		['ä', s @ ..] => super::t13::n6307(s, out),
		['ö', s @ ..] => super::t13::n6308(s, out),
		['ü', s @ ..] => super::t13::n6309(s, out),
		_ => false,
	}
}

/// Words ending -tch.
pub(crate) fn n1536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1534);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6247);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 15441);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 25738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 15442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25739);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 25740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 15443);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 15444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6248);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 15445);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6249);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 15447);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 15448);
					match s {
						[] => true,
						['s', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 25741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 15449);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 25742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6250);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uch.
pub(crate) fn n1537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1535);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6252);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15450);
					match s {
						[] => true,
						['l', 'e', 'b', 'b', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 25743);
							s.is_empty()
						}
						['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 25744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 15451);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 15452);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 25745);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 39486);
											s.is_empty()
										}
										['i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 39487);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 25746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25747);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 25748);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 33998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 25749);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39488);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42552);
													s.is_empty()
												}
												_ => false,
											}
										}
										['f', 'f', 'o', 't', 's', 'r', s @ ..] => {
											super::push(out, 39489);
											s.is_empty()
										}
										['n', 'e', 't', 't', s @ ..] => {
											super::push(out, 39490);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 25750);
							match s {
								[] => true,
								['ä', s @ ..] => {
									super::push(out, 33999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 25751);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 25752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 15454);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25753);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 25754);
							s.is_empty()
						}
						['b', s @ ..] => super::t13::n26744(s, out),
						['d', s @ ..] => {
							super::push(out, 25756);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 25757);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 25758);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 34005);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 34006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 25759);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t13::n26749(s, out),
						['o', s @ ..] => {
							super::push(out, 25761);
							s.is_empty()
						}
						['r', s @ ..] => super::t13::n26751(s, out),
						['s', s @ ..] => super::t13::n26752(s, out),
						['t', s @ ..] => super::t13::n26753(s, out),
						['v', s @ ..] => super::t13::n26754(s, out),
						['z', s @ ..] => super::t13::n26755(s, out),
						['ß', s @ ..] => super::t13::n26756(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => super::t13::n6316(s, out),
		['e', s @ ..] => super::t13::n6317(s, out),
		['l', s @ ..] => super::t13::n6318(s, out),
		['n', s @ ..] => super::t13::n6319(s, out),
		['o', s @ ..] => super::t13::n6320(s, out),
		['r', s @ ..] => super::t13::n6321(s, out),
		['s', s @ ..] => super::t13::n6322(s, out),
		['t', s @ ..] => super::t13::n6323(s, out),
		['ä', s @ ..] => super::t13::n6324(s, out),
		_ => false,
	}
}

/// Words ending -äch.
pub(crate) fn n1538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1536);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 6262);
			match s {
				[] => true,
				['p', 's', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 25793);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 25794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word arschgeweih.
pub(crate) fn n45365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42588);
	s.is_empty()
}

/// The word hirschgeweih.
pub(crate) fn n45366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42589);
	s.is_empty()
}

/// Words ending -paragraph.
pub(crate) fn n45367(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'l', 'e', s @ ..] => {
			super::push(out, 44037);
			match s {
				[] => true,
				['g', 'z', 'n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 44699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'h', s @ ..] => {
			super::push(out, 44038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -luth.
pub(crate) fn n6393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6328);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 15543);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 15544);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 15545);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 15546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -muth.
pub(crate) fn n6394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6329);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15547);
			s.is_empty()
		}
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 15548);
			s.is_empty()
		}
		['s', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 15549);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wuth.
pub(crate) fn n6396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6331);
	s.is_empty()
}

/// Words ending -wth.
pub(crate) fn n1573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1571);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 6332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yth.
pub(crate) fn n1574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1572);
	match s {
		[] => true,
		['l', 'o', 'k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üth.
pub(crate) fn n1575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1573);
	match s {
		[] => true,
		['m', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 6334);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word baseballhandschuh.
pub(crate) fn n45368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42590);
	s.is_empty()
}

/// Words ending -ellhandschuh.
pub(crate) fn n45369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42591);
	s.is_empty()
}

/// Words ending -enei.
pub(crate) fn n6509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6442);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 15612);
			match s {
				[] => true,
				['n', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 25868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 15613);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 15614);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 15615);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 25869);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 25870);
					s.is_empty()
				}
				['ö', 'r', 'k', 'd', 'l', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 25871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -koei.
pub(crate) fn n6513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6446);
	s.is_empty()
}

/// Words ending -pei.
pub(crate) fn n1633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1631);
	match s {
		[] => true,
		['o', 'i', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 6447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rei.
pub(crate) fn n1634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1632);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15616);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15617);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 34101);
									s.is_empty()
								}
								['r', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 34102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 25872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'f', 'f', 'o', 't', 'r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 15618);
					s.is_empty()
				}
				['n', 'e', 'n', s @ ..] => {
					super::push(out, 15619);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15620);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 25873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 15621);
					match s {
						[] => true,
						['i', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 25874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'e', 'i', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 15622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6449);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', 'w', 'd', s @ ..] => {
							super::push(out, 25875);
							s.is_empty()
						}
						['r', 'e', 'g', 'h', s @ ..] => {
							super::push(out, 25876);
							s.is_empty()
						}
						['u', 'ä', 'r', 'e', s @ ..] => {
							super::push(out, 25877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 15623);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 15624);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 15625);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 25878);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 25879);
							s.is_empty()
						}
						['ä', s @ ..] => {
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 34103);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 34104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 15626);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'd', s @ ..] => {
									super::push(out, 34105);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34106);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 34107);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 39555);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 42592);
													s.is_empty()
												}
												['f', s @ ..] => {
													super::push(out, 42593);
													match s {
														[] => true,
														['l', 'a', s @ ..] => super::t13::n46999(s, out),
														['n', s @ ..] => super::t13::n47000(s, out),
														['o', s @ ..] => super::t13::n47001(s, out),
														_ => false,
													}
												}
												['w', s @ ..] => {
													super::push(out, 42594);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ä', s @ ..] => {
											super::push(out, 39556);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39557);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 39558);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ü', s @ ..] => {
									super::push(out, 34108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 15627);
					match s {
						[] => true,
						['c', 'u', s @ ..] => {
							super::push(out, 25880);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 34109);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 25881);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34110);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 34111);
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
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 34112);
									s.is_empty()
								}
								['l', 'e', s @ ..] => {
									super::push(out, 34113);
									s.is_empty()
								}
								['n', 'r', s @ ..] => {
									super::push(out, 34114);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 39560);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', s @ ..] => {
									super::push(out, 34115);
									s.is_empty()
								}
								['ß', 'i', s @ ..] => {
									super::push(out, 34116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'u', s @ ..] => {
							super::push(out, 25882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'ä', s @ ..] => {
					super::push(out, 15628);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15629);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 25883);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 34117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 25884);
							match s {
								[] => true,
								['ä', s @ ..] => {
									super::push(out, 34118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'u', s @ ..] => {
					super::push(out, 15630);
					match s {
						[] => true,
						['a', s @ ..] => super::t13::n26893(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t13::n15992(s, out),
				['t', s @ ..] => super::t13::n15993(s, out),
				['u', s @ ..] => super::t13::n15994(s, out),
				['v', s @ ..] => super::t13::n15995(s, out),
				['z', s @ ..] => super::t13::n15996(s, out),
				['ß', 'e', 'i', 'g', 'd', s @ ..] => super::t13::n15997(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t13::n6517(s, out),
		['h', s @ ..] => super::t13::n6518(s, out),
		['n', s @ ..] => super::t13::n6519(s, out),
		['o', s @ ..] => super::t13::n6520(s, out),
		['r', s @ ..] => super::t13::n6521(s, out),
		_ => false,
	}
}

/// Words ending -sei.
pub(crate) fn n1635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1633);
	match s {
		[] => true,
		['g', 'n', 'u', 'h', 'c', 's', 'a', 'r', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
			super::push(out, 6455);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6456);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15641);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15642);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 6457);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tei.
pub(crate) fn n1636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1634);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6459);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 15644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6460);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['-', 't', s @ ..] => {
							super::push(out, 25894);
							s.is_empty()
						}
						['l', 'm', s @ ..] => {
							super::push(out, 25895);
							s.is_empty()
						}
						['s', 's', s @ ..] => {
							super::push(out, 25896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 15645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6461);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 15646);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 25897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zei.
pub(crate) fn n1637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1635);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6462);
			match s {
				[] => true,
				['l', 'o', 'p', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 25898);
							s.is_empty()
						}
						['n', 'e', 'f', 'a', 'h', 'g', s @ ..] => {
							super::push(out, 25899);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 25900);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 34135);
									s.is_empty()
								}
								['t', 'ä', s @ ..] => {
									super::push(out, 34136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 34137);
									s.is_empty()
								}
								['t', 'u', 'h', 'c', 's', 'r', s @ ..] => {
									super::push(out, 34138);
									s.is_empty()
								}
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
			super::push(out, 6463);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 15647);
					match s {
						[] => true,
						['b', 'e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 25901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßei.
pub(crate) fn n1638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1636);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ikuli.
pub(crate) fn n16086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15722);
	s.is_empty()
}

/// Words ending -okuli.
pub(crate) fn n16087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15723);
	s.is_empty()
}

/// Words ending -muli.
pub(crate) fn n6648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6581);
	s.is_empty()
}

/// Words ending -zuli.
pub(crate) fn n6649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6582);
	s.is_empty()
}

/// Words ending -zli.
pub(crate) fn n1690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1688);
	match s {
		[] => true,
		['n', 'ü', 'b', '\u{1}', s @ ..] => {
			super::push(out, 6583);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6584);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 15724);
					match s {
						[] => true,
						['p', 'i', 'l', 'z', 't', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 25933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 15725);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öli.
pub(crate) fn n1691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1689);
	s.is_empty()
}

/// Words ending -üli.
pub(crate) fn n1692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1690);
	s.is_empty()
}

/// Words ending -mack.
pub(crate) fn n6868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6795);
	match s {
		[] => true,
		['h', 'c', 's', 'e', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25986);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 25987);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 34150);
							match s {
								[] => true,
								['ä', 'k', '\u{1}', s @ ..] => {
									super::push(out, 39570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 25988);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 34151);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 34152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 25989);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 25990);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 34153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 25991);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 34154);
							match s {
								[] => true,
								['s', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 39571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 25992);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 34156);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 34157);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 34158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 25993);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 34159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 25994);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 39572);
									match s {
										[] => true,
										['u', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 42598);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 39573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 25995);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 34160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 25996);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 34161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 25997);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 34162);
							match s {
								[] => true,
								['k', 's', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 39574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'n', 'i', 'm', 'r', 'e', 'f', 'f', 'e', 'f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 25998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nack.
pub(crate) fn n6869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6796);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15887);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 15888);
			match s {
				[] => true,
				['c', 's', 'k', s @ ..] => {
					super::push(out, 25999);
					match s {
						[] => true,
						['c', 'i', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 34163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 15889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15890);
			match s {
				[] => true,
				['e', 'b', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 26001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 15891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pack.
pub(crate) fn n6870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6797);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 15892);
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 26002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15893);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 15894);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26003);
					match s {
						[] => true,
						['p', 'm', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 34164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15895);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 15896);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 26004);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rack.
pub(crate) fn n6871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6798);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 15897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 15898);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 15899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 15900);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 26007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 15901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26008);
					s.is_empty()
				}
				['g', 'u', 'e', 'z', 'g', 'u', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 26009);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26010);
					s.is_empty()
				}
				['n', 'e', 'g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 26011);
					s.is_empty()
				}
				['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 26012);
					s.is_empty()
				}
				['s', 'f', 'f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 26013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sack.
pub(crate) fn n6872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6799);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 15902);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 26014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'l', s @ ..] => {
			super::push(out, 15903);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 26015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'f', 'ö', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 15904);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 15905);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 26016);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 34165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'w', 'm', s @ ..] => {
					super::push(out, 26017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'h', 'c', 'i', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 15906);
			s.is_empty()
		}
		['r', 'e', 'f', 'a', s @ ..] => {
			super::push(out, 15907);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 15908);
			match s {
				[] => true,
				['f', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 26018);
					s.is_empty()
				}
				['h', 'c', 'i', 'r', 'h', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 26019);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 34166);
							s.is_empty()
						}
						['e', 'm', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 34167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 26020);
					s.is_empty()
				}
				['t', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 26021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ß', 'e', s @ ..] => {
			super::push(out, 15909);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 26022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zack.
pub(crate) fn n6874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6801);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 15911);
			match s {
				[] => true,
				['k', 'c', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 26024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 15912);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 15913);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 15914);
			match s {
				[] => true,
				['c', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 26025);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eck.
pub(crate) fn n1808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1806);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6802);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 6803);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 15915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 6804);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15917);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15918);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26026);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15919);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 26027);
							match s {
								[] => true,
								['u', 'a', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39575);
											s.is_empty()
										}
										['n', 'h', 'e', 'z', '\u{1}', s @ ..] => {
											super::push(out, 39576);
											s.is_empty()
										}
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
					super::push(out, 15920);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 34169);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 34170);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39577);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 39578);
											s.is_empty()
										}
										['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 39579);
											s.is_empty()
										}
										['t', 'l', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 39580);
											s.is_empty()
										}
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
					super::push(out, 15921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6806);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 15922);
					s.is_empty()
				}
				['n', 'ü', 'f', '\u{1}', s @ ..] => {
					super::push(out, 15923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6807);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15924);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15925);
					match s {
						[] => true,
						['d', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 26028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 15926);
					match s {
						[] => true,
						['z', 'n', 'a', 'w', 'z', 'd', s @ ..] => {
							super::push(out, 26029);
							match s {
								[] => true,
								['n', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 39581);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 39582);
											match s {
												[] => true,
												['h', 'c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 42599);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6808);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15927);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15928);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 15929);
					match s {
						[] => true,
						['f', 'f', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 26031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 15930);
					match s {
						[] => true,
						['e', 'i', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 26032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6809);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15931);
					match s {
						[] => true,
						['r', 'd', s @ ..] => {
							match s {
								[] => true,
								['-', 'a', s @ ..] => {
									super::push(out, 34171);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 34172);
									s.is_empty()
								}
								['r', 'e', 'm', s @ ..] => {
									super::push(out, 34173);
									s.is_empty()
								}
								['s', 'g', s @ ..] => {
									super::push(out, 34174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 6810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6811);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15933);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15934);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 15935);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26034);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 26035);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26036);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 39583);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 39584);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 34175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 26037);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26038);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26039);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26040);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34176);
									match s {
										[] => true,
										['n', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 39585);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 34177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t13::n27058(s, out),
						['s', s @ ..] => {
							super::push(out, 26042);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26043);
							s.is_empty()
						}
						['z', 't', 'o', s @ ..] => {
							super::push(out, 26044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 15936);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15937);
					match s {
						[] => true,
						['ö', '\u{1}', s @ ..] => {
							super::push(out, 26045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t13::n16306(s, out),
				['s', s @ ..] => super::t13::n16307(s, out),
				_ => false,
			}
		}
		['m', 'k', 'c', 'e', 'h', '\u{1}', s @ ..] => super::t13::n6885(s, out),
		['n', s @ ..] => super::t13::n6886(s, out),
		['p', s @ ..] => super::t13::n6887(s, out),
		['r', s @ ..] => super::t13::n6888(s, out),
		['s', s @ ..] => super::t13::n6889(s, out),
		['t', s @ ..] => super::t13::n6890(s, out),
		['w', s @ ..] => super::t13::n6891(s, out),
		['z', s @ ..] => super::t13::n6892(s, out),
		_ => false,
	}
}

/// Words ending -ick.
pub(crate) fn n1809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1807);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6820);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 6821);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26069);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26070);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34186);
									match s {
										[] => true,
										['g', 's', 'g', s @ ..] => {
											super::push(out, 39599);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 34187);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39600);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6822);
			match s {
				[] => true,
				['-', 'e', 's', 'u', 'o', 'h', 'd', 'n', 'u', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 15960);
					s.is_empty()
				}
				['n', 'i', 'e', 'f', 'f', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 15961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6823);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15962);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26071);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34188);
									s.is_empty()
								}
								['n', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 34189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 26072);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 26073);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 26074);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26075);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26076);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34191);
									match s {
										[] => true,
										['g', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 39601);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 34192);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 39602);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 39603);
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
								['e', s @ ..] => {
									super::push(out, 34193);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 39604);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 34194);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 39605);
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
								['d', s @ ..] => {
									super::push(out, 34195);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 34196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 15963);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 15964);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 15965);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 15966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6824);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 15967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6825);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15968);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 15969);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 26079);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26080);
							match s {
								[] => true,
								['r', 'e', 'i', 'r', 'r', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 34197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 26081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6826);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6827);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 15971);
					match s {
						[] => true,
						['m', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 26082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 15972);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26083);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26084);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26085);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34198);
									s.is_empty()
								}
								['d', 'n', 'ü', 'z', '\u{1}', s @ ..] => {
									super::push(out, 34199);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34200);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 39606);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42606);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 39607);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 34201);
									match s {
										[] => true,
										['n', 'i', 'l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 39608);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 34202);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 34203);
									s.is_empty()
								}
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
			super::push(out, 6828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15973);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6829);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 15975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lck.
pub(crate) fn n1810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1808);
	match s {
		[] => true,
		['o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 6830);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word getränck.
pub(crate) fn n1811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1809);
	s.is_empty()
}

/// Words ending -ock.
pub(crate) fn n1812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1810);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6831);
			match s {
				[] => true,
				['f', 'a', s @ ..] => {
					super::push(out, 15976);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 26086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 15977);
					s.is_empty()
				}
				['i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 15978);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15979);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 26087);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 26088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 15980);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15981);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15982);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15983);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 15984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6832);
			match s {
				[] => true,
				['h', 'c', 't', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 15985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6833);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 15986);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 15987);
					match s {
						[] => true,
						['u', 't', 'a', 'r', 'a', 'p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 26090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6834);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 6835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15988);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 15989);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 34204);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34205);
									match s {
										[] => true,
										['z', 't', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 39609);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 34206);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 42607);
													match s {
														[] => true,
														['e', 'g', '\u{1}', s @ ..] => super::t13::n47004(s, out),
														_ => false,
													}
												}
												['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
													super::push(out, 42608);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 34207);
									s.is_empty()
								}
								['s', 'n', 'o', 'i', 't', 'a', 'l', 'f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 34208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 6836);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 15990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6837);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15991);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26091);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26092);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 26093);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34210);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'i', 'e', 'z', '\u{1}', s @ ..] => {
											super::push(out, 39610);
											s.is_empty()
										}
										['k', 'c', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 39611);
											s.is_empty()
										}
										['l', s @ ..] => {
											match s {
												[] => true,
												['h', 'a', 'z', '\u{1}', s @ ..] => {
													super::push(out, 42609);
													s.is_empty()
												}
												['l', 'e', 'z', '\u{1}', s @ ..] => {
													super::push(out, 42610);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'e', 'h', 't', '\u{1}', s @ ..] => {
											super::push(out, 39612);
											s.is_empty()
										}
										['s', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 39613);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 39614);
											s.is_empty()
										}
										['z', 'z', 'i', 'k', 's', '\u{1}', s @ ..] => {
											super::push(out, 39615);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 34211);
									s.is_empty()
								}
								['i', 'e', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39616);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 39617);
											s.is_empty()
										}
										['k', 'l', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 39618);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 't', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 34212);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26094);
							match s {
								[] => true,
								['e', s @ ..] => super::t13::n36053(s, out),
								['o', s @ ..] => super::t13::n36054(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26095);
							s.is_empty()
						}
						['t', s @ ..] => super::t13::n27117(s, out),
						['z', s @ ..] => super::t13::n27118(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t13::n16360(s, out),
				['e', s @ ..] => super::t13::n16361(s, out),
				['f', s @ ..] => super::t13::n16362(s, out),
				['g', s @ ..] => super::t13::n16363(s, out),
				['m', s @ ..] => super::t13::n16364(s, out),
				['u', s @ ..] => super::t13::n16365(s, out),
				['y', s @ ..] => super::t13::n16366(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t13::n6911(s, out),
		['n', s @ ..] => super::t13::n6912(s, out),
		['p', s @ ..] => super::t13::n6913(s, out),
		['r', s @ ..] => super::t13::n6914(s, out),
		['t', s @ ..] => super::t13::n6915(s, out),
		['w', s @ ..] => super::t13::n6916(s, out),
		['z', '\u{1}', s @ ..] => super::t13::n6917(s, out),
		_ => false,
	}
}

/// The word werck.
pub(crate) fn n1813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1811);
	s.is_empty()
}

/// Words ending -uck.
pub(crate) fn n1814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1812);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 6845);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6846);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16020);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6847);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 6848);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6849);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16023);
					match s {
						[] => true,
						['m', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 26129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6850);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26130);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26131);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 26132);
							s.is_empty()
						}
						['m', 'u', s @ ..] => {
							super::push(out, 26133);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									match s {
										[] => true,
										['s', 't', 'h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 39631);
											s.is_empty()
										}
										['t', 's', 'i', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 39632);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34230);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 34231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 34232);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 34233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 't', s @ ..] => {
							super::push(out, 26134);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26135);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34234);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 34235);
									s.is_empty()
								}
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
			super::push(out, 6851);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16025);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 16026);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t13::n16400(s, out),
				['s', s @ ..] => {
					super::push(out, 16028);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16029);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16030);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6853);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16031);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äck.
pub(crate) fn n1815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1813);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6854);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26155);
							match s {
								[] => true,
								['t', 'h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 34261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'l', s @ ..] => {
							super::push(out, 26156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6855);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26157);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 26158);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ück.
pub(crate) fn n1816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1814);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 6856);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26160);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26161);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26162);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26163);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34262);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 34263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26164);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26165);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 34264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26166);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6857);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 16032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6858);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 26168);
							s.is_empty()
						}
						['d', 'n', 'u', 'f', s @ ..] => {
							super::push(out, 26169);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 26170);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 34265);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 34266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 26171);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['s', 'i', 'e', 'l', 'f', 'd', s @ ..] => {
											super::push(out, 39664);
											s.is_empty()
										}
										['u', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39665);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ü', 'r', 'f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39666);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 39667);
											s.is_empty()
										}
										['r', 'e', 't', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 39668);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 26172);
							s.is_empty()
						}
						['k', 'r', 'a', s @ ..] => {
							super::push(out, 26173);
							s.is_empty()
						}
						['l', 'e', 'z', s @ ..] => {
							super::push(out, 26174);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 34267);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 39669);
											s.is_empty()
										}
										['r', 'u', 'g', '\u{1}', s @ ..] => {
											super::push(out, 39670);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 34268);
									match s {
										[] => true,
										['l', 'e', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 39671);
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
								['e', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 39672);
											match s {
												[] => true,
												['m', 'a', 'k', '\u{1}', s @ ..] => {
													super::push(out, 42635);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', 'i', s @ ..] => {
											super::push(out, 39673);
											match s {
												[] => true,
												['e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 42636);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', 'l', s @ ..] => {
									super::push(out, 34269);
									match s {
										[] => true,
										['f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42637);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 42638);
													s.is_empty()
												}
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
							super::push(out, 26175);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26176);
							match s {
								[] => true,
								['s', 'n', 'u', 'k', 's', s @ ..] => {
									super::push(out, 34270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
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

/// Words ending -nik.
pub(crate) fn n1838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1836);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6900);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 16056);
					s.is_empty()
				}
				['h', 'c', 'e', 'm', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26198);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16057);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6901);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16059);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16060);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16061);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6902);
			match s {
				[] => true,
				['c', 'e', 't', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 26200);
							s.is_empty()
						}
						['g', 'u', 'e', 'z', 'r', 'h', 'a', 'f', 't', s @ ..] => {
							super::push(out, 26201);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26202);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 34279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'n', 'n', s @ ..] => {
							super::push(out, 26203);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 26204);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 34280);
									s.is_empty()
								}
								['r', 't', s @ ..] => {
									super::push(out, 34281);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 34282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26205);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									super::push(out, 34283);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 34284);
									s.is_empty()
								}
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['g', 'r', s @ ..] => {
											super::push(out, 39674);
											s.is_empty()
										}
										['l', 'e', 'g', s @ ..] => {
											super::push(out, 39675);
											s.is_empty()
										}
										['r', 'e', 'u', s @ ..] => {
											super::push(out, 39676);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 34285);
									s.is_empty()
								}
								['n', 'o', 'i', 't', 'a', 'm', s @ ..] => {
									super::push(out, 34286);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 34287);
									s.is_empty()
								}
								['t', 'i', s @ ..] => {
									super::push(out, 34288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'f', s @ ..] => {
							super::push(out, 26206);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26207);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34289);
									s.is_empty()
								}
								['t', 'u', 'h', 'c', 's', 'z', s @ ..] => {
									super::push(out, 34290);
									s.is_empty()
								}
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
			super::push(out, 6903);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6904);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6905);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26208);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 34291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6906);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6907);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 16065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pik.
pub(crate) fn n6984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6908);
	s.is_empty()
}

/// Words ending -epik.
pub(crate) fn n6985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6909);
	s.is_empty()
}

/// Words ending -spik.
pub(crate) fn n6986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6910);
	s.is_empty()
}

/// Words ending -upik.
pub(crate) fn n6987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6911);
	s.is_empty()
}

/// Words ending -rik.
pub(crate) fn n1840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1838);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6912);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6913);
			match s {
				[] => true,
				['a', 'f', 'm', s @ ..] => {
					super::push(out, 16066);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							super::push(out, 26210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6914);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6915);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6916);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26211);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26212);
							match s {
								[] => true,
								['h', 'r', 't', s @ ..] => {
									super::push(out, 34292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 26213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6917);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26214);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 16067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 6918);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 16068);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sik.
pub(crate) fn n1841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1839);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 6919);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6920);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26216);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26217);
							match s {
								[] => true,
								['c', 'o', 'r', 'a', s @ ..] => {
									super::push(out, 34293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 26218);
							s.is_empty()
						}
						['m', 'l', s @ ..] => {
							super::push(out, 26219);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 34294);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 34295);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 34296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 26220);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 26221);
							match s {
								[] => true,
								['g', 'a', s @ ..] => {
									super::push(out, 34297);
									s.is_empty()
								}
								['i', 'v', s @ ..] => {
									super::push(out, 34298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 34299);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 34300);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 34301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16070);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 26222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 6921);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tik.
pub(crate) fn n1842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1840);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6922);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16071);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16072);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 34302);
									match s {
										[] => true,
										['t', 'a', s @ ..] => {
											super::push(out, 39677);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 34303);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 39678);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 39679);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 34304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 26223);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26224);
							match s {
								[] => true,
								['a', 'r', 'g', s @ ..] => {
									match s {
										[] => true,
										['l', 'a', s @ ..] => {
											super::push(out, 39680);
											s.is_empty()
										}
										['t', 'x', s @ ..] => {
											super::push(out, 39681);
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
								['s', s @ ..] => {
									super::push(out, 34305);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 39682);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 39683);
											match s {
												[] => true,
												['a', 'd', s @ ..] => {
													super::push(out, 42639);
													s.is_empty()
												}
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
							super::push(out, 26225);
							match s {
								[] => true,
								['o', 'f', 'n', 'i', 'o', 'i', s @ ..] => {
									super::push(out, 34306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26226);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6923);
			match s {
				[] => true,
				['g', 'o', s @ ..] => {
					super::push(out, 16074);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 26228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16075);
					match s {
						[] => true,
						['t', 'o', s @ ..] => {
							super::push(out, 26229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 16076);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16077);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26230);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16078);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16079);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 26232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6924);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16081);
					match s {
						[] => true,
						['o', 'p', s @ ..] => super::t13::n27270(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16082);
					match s {
						[] => true,
						['k', s @ ..] => super::t13::n27271(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t13::n7001(s, out),
		['l', s @ ..] => {
			super::push(out, 6926);
			s.is_empty()
		}
		['n', s @ ..] => super::t13::n7003(s, out),
		['o', s @ ..] => {
			super::push(out, 6928);
			match s {
				[] => true,
				['x', s @ ..] => {
					super::push(out, 16088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t13::n7005(s, out),
		['s', s @ ..] => super::t13::n7006(s, out),
		['u', s @ ..] => super::t13::n7007(s, out),
		['y', s @ ..] => {
			super::push(out, 6931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wik.
pub(crate) fn n1843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1841);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6932);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6933);
			match s {
				[] => true,
				['h', 'c', 's', 'l', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 16096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6934);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16097);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xik.
pub(crate) fn n1844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1842);
	s.is_empty()
}

/// Words ending -tenbank.
pub(crate) fn n36210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34353);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 39706);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 39707);
			match s {
				[] => true,
				['a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 42661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 'l', s @ ..] => {
			super::push(out, 39708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rbank.
pub(crate) fn n16516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16123);
	match s {
		[] => true,
		['e', 'w', s @ ..] => {
			super::push(out, 26265);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 34354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sbank.
pub(crate) fn n16517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16124);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 26266);
			match s {
				[] => true,
				['d', 'n', 'u', s @ ..] => {
					super::push(out, 34355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 26267);
			s.is_empty()
		}
		['s', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 26268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltbank.
pub(crate) fn n27315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26269);
	s.is_empty()
}

/// Words ending -zbank.
pub(crate) fn n16519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16126);
	s.is_empty()
}

/// Words ending -oßbank.
pub(crate) fn n16520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16127);
	s.is_empty()
}

/// Words ending -dank.
pub(crate) fn n7029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6952);
	s.is_empty()
}

/// Words ending -hank.
pub(crate) fn n7030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6953);
	match s {
		[] => true,
		['c', 's', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 26270);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 26271);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39709);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lank.
pub(crate) fn n7031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6954);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 16128);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 16129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rank.
pub(crate) fn n7032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6955);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16130);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 26272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 16131);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 34356);
							match s {
								[] => true,
								['g', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 39711);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39712);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 34357);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 39713);
									match s {
										[] => true,
										['e', 'n', 'i', 'b', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42663);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 34358);
							match s {
								[] => true,
								['l', 'o', 'h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42664);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42665);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16132);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'n', 'e', 'k', 'r', 'ü', 't', '\u{1}', s @ ..] => {
			super::push(out, 16133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tank.
pub(crate) fn n7033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6956);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16134);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16135);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 16136);
			match s {
				[] => true,
				['f', 'o', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'e', 'r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39714);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 39715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 34359);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 39716);
									s.is_empty()
								}
								['u', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 39717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'f', 'a', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 16137);
			match s {
				[] => true,
				['c', 'u', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 26274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 16138);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 26275);
					match s {
						[] => true,
						['u', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 34361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 26276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 16139);
			match s {
				[] => true,
				['e', 's', 'e', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 26277);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 26278);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26279);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 26280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16140);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 26281);
					match s {
						[] => true,
						['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 26282);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 34363);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 39718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 34364);
							match s {
								[] => true,
								['n', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39719);
									s.is_empty()
								}
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
			super::push(out, 16141);
			match s {
				[] => true,
				['y', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 26283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16142);
			match s {
				[] => true,
				['e', 's', 's', 'a', 'w', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 34365);
							s.is_empty()
						}
						['h', 'c', 's', 'ö', 'l', '\u{1}', s @ ..] => {
							super::push(out, 34366);
							s.is_empty()
						}
						['l', 'h', 'ü', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26284);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 26285);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 34368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39720);
									s.is_empty()
								}
								['g', 'i', 's', 's', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 39721);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 39722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 26286);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39723);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 39724);
									match s {
										[] => true,
										['ß', 'i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 42666);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', 's', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 39725);
									s.is_empty()
								}
								['i', 'm', 'm', 'u', 'g', '\u{1}', s @ ..] => {
									super::push(out, 39726);
									s.is_empty()
								}
								['k', 'a', 'i', 'n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39727);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['f', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 44063);
													s.is_empty()
												}
												['s', 'e', 'i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 44064);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'h', 'o', 'k', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42667);
											s.is_empty()
										}
										['ö', '\u{1}', s @ ..] => {
											super::push(out, 42668);
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
												['h', 'c', 'i', 'e', 'l', '\u{1}', s @ ..] => {
													super::push(out, 44065);
													s.is_empty()
												}
												['k', 'a', 'o', 'l', 'k', '\u{1}', s @ ..] => {
													super::push(out, 44066);
													s.is_empty()
												}
												['t', 't', 'e', 'r', 'a', 'g', 'i', 'z', '\u{1}', s @ ..] => {
													super::push(out, 44067);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'r', 'u', '\u{1}', s @ ..] => {
											super::push(out, 42669);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 39728);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42670);
											s.is_empty()
										}
										['d', 'r', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42671);
											s.is_empty()
										}
										['e', 'd', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 42672);
											s.is_empty()
										}
										['g', 'n', 'u', 's', 'e', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 42673);
											s.is_empty()
										}
										['i', 'n', 'l', 'u', 'ä', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42674);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ß', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 39729);
									s.is_empty()
								}
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
			super::push(out, 16144);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 16145);
			match s {
				[] => true,
				['t', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 26287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wank.
pub(crate) fn n7034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6957);
	s.is_empty()
}

/// Words ending -zank.
pub(crate) fn n7035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6958);
	s.is_empty()
}

/// Words ending -enk.
pub(crate) fn n1854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1852);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 6959);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6960);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26288);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 26289);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34369);
									match s {
										[] => true,
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 39730);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 26290);
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									super::push(out, 34370);
									match s {
										[] => true,
										['e', 'r', s @ ..] => {
											super::push(out, 39731);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 26291);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 34371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26292);
							match s {
								[] => true,
								['e', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 34372);
									s.is_empty()
								}
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
			super::push(out, 6961);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16147);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 34373);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 34374);
									match s {
										[] => true,
										['g', 'o', 'b', s @ ..] => {
											match s {
												[] => true,
												['l', 'l', 'e', '\u{1}', s @ ..] => {
													super::push(out, 42675);
													s.is_empty()
												}
												['n', 'e', 'l', 'l', 'e', '\u{1}', s @ ..] => {
													super::push(out, 42676);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'u', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 39732);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'f', s @ ..] => {
									super::push(out, 34375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6962);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16149);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 26293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ink.
pub(crate) fn n1855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1853);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6963);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16150);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16151);
					match s {
						[] => true,
						['ü', 'r', 'g', '-', s @ ..] => {
							super::push(out, 26295);
							match s {
								[] => true,
								['f', 'p', 'o', 'k', 'z', 'r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 34376);
									s.is_empty()
								}
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
			super::push(out, 6964);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 16152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6965);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 6966);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6967);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6968);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 16155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6969);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16156);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6970);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16158);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16159);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onk.
pub(crate) fn n1856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1854);
	s.is_empty()
}

/// Words ending -unk.
pub(crate) fn n1857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1855);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6971);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16160);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 16161);
					match s {
						[] => true,
						['b', 'c', '\u{1}', s @ ..] => {
							super::push(out, 26297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'u', 'r', 'n', 'e', 't', 'i', 'l', 'l', 'e', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 16162);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 26298);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26299);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 26300);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 34377);
									s.is_empty()
								}
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
			super::push(out, 6972);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 6973);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6974);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6975);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 16165);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16166);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26301);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26302);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 34378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 26303);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34379);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26304);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 6976);
			s.is_empty()
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 6977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -änk.
pub(crate) fn n1858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1856);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 6978);
			match s {
				[] => true,
				['t', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'i', 'm', '-', s @ ..] => {
							super::push(out, 26306);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26307);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 34381);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 34382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'l', s @ ..] => {
							super::push(out, 26308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rpark.
pub(crate) fn n16584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16187);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 26321);
			match s {
				[] => true,
				['l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 34397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'u', s @ ..] => {
			super::push(out, 26322);
			match s {
				[] => true,
				['e', 't', 'n', 'e', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 34398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 26323);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word schlosspark.
pub(crate) fn n36261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34399);
	s.is_empty()
}

/// Words ending -tpark.
pub(crate) fn n16586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16189);
	match s {
		[] => true,
		['a', 'v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 26325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -qark.
pub(crate) fn n7069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6992);
	s.is_empty()
}

/// Words ending -uark.
pub(crate) fn n7070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6993);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 16190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26326);
					s.is_empty()
				}
				['e', 't', 'h', 'c', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 26327);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 26328);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 26329);
					s.is_empty()
				}
				['t', 'h', 'c', 'u', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 26330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -drk.
pub(crate) fn n1872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1870);
	s.is_empty()
}

/// Words ending -erk.
pub(crate) fn n1873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1871);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 6994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16191);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16192);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6995);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16194);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 26331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 16195);
					match s {
						[] => true,
						['e', 'i', 'r', 't', 'g', s @ ..] => {
							super::push(out, 26332);
							s.is_empty()
						}
						['u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 26333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 16196);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 26334);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34400);
									match s {
										[] => true,
										['h', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 42677);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 42678);
													match s {
														[] => true,
														['e', 's', s @ ..] => super::t13::n47029(s, out),
														_ => false,
													}
												}
												['s', s @ ..] => {
													super::push(out, 42679);
													match s {
														[] => true,
														['g', 'e', s @ ..] => super::t13::n47030(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 42680);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 34401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16197);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26336);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 26337);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 26338);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 16198);
					match s {
						[] => true,
						['i', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 26340);
							s.is_empty()
						}
						['p', 'ö', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 26341);
							s.is_empty()
						}
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 34402);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39739);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 39740);
											s.is_empty()
										}
										['d', 'n', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39741);
											s.is_empty()
										}
										['n', 'e', 't', 't', s @ ..] => {
											match s {
												[] => true,
												['a', 'l', 'p', 't', 's', 'e', 'f', '\u{1}', s @ ..] => {
													super::push(out, 42681);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 42682);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 't', 'i', 'e', 'l', 'b', 'l', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 39742);
											s.is_empty()
										}
										['t', 'o', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => super::t13::n16596(s, out),
				['h', s @ ..] => {
					super::push(out, 16200);
					match s {
						[] => true,
						['c', 'u', s @ ..] => {
							super::push(out, 26343);
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 34408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16201);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 26344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 16202);
					match s {
						[] => true,
						['c', 'i', s @ ..] => {
							super::push(out, 26345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t13::n16600(s, out),
				['n', s @ ..] => super::t13::n16601(s, out),
				['r', s @ ..] => super::t13::n16602(s, out),
				['s', s @ ..] => super::t13::n16603(s, out),
				['t', s @ ..] => super::t13::n16604(s, out),
				['u', s @ ..] => super::t13::n16605(s, out),
				['z', s @ ..] => super::t13::n16606(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irk.
pub(crate) fn n1874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1872);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6998);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 6999);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26354);
							s.is_empty()
						}
						['l', 'h', s @ ..] => {
							super::push(out, 26355);
							s.is_empty()
						}
						['s', 'g', 'n', 'u', 'r', 'e', 'i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 26356);
							s.is_empty()
						}
						['t', 'h', 'c', 'i', 'l', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 26357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ork.
pub(crate) fn n1875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1873);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7000);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 7001);
			match s {
				[] => true,
				['t', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 16212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -egal.
pub(crate) fn n7128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7050);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 16246);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 26369);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 34433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16247);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 26370);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 34434);
							s.is_empty()
						}
						['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 34435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 26371);
					match s {
						[] => true,
						['e', 'b', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 34436);
							s.is_empty()
						}
						['h', 'ü', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'ü', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34438);
							s.is_empty()
						}
						['r', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 34439);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['f', 'i', 'r', 'h', 'c', 's', 't', 'i', 'e', 'z', '\u{1}', s @ ..] => {
									super::push(out, 39761);
									s.is_empty()
								}
								['k', 'a', '\u{1}', s @ ..] => {
									super::push(out, 39762);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 26372);
					s.is_empty()
				}
				['s', 'p', 'a', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 26373);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 26374);
					match s {
						[] => true,
						['k', 'r', 'a', 'm', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 34440);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 34441);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 39763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'r', 'ü', 'w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 26375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -igal.
pub(crate) fn n7129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7051);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 16248);
			s.is_empty()
		}
		['r', 'd', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 16249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tingal.
pub(crate) fn n16649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16250);
	s.is_empty()
}

/// Words ending -rgal.
pub(crate) fn n7131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7053);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hal.
pub(crate) fn n1904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1902);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7054);
			match s {
				[] => true,
				['s', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 16252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7055);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16253);
					s.is_empty()
				}
				['b', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 16254);
					s.is_empty()
				}
				['g', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 16255);
					s.is_empty()
				}
				['h', 'c', 's', 't', 'e', '\u{1}', s @ ..] => {
					super::push(out, 16256);
					s.is_empty()
				}
				['l', 'e', 's', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 16257);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16258);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26376);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 34442);
									s.is_empty()
								}
								['m', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 34443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 26377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16259);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 26378);
							s.is_empty()
						}
						['t', 's', 'n', 'ü', 'm', '\u{1}', s @ ..] => {
							super::push(out, 26379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'n', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 16260);
					s.is_empty()
				}
				['z', 't', 'ö', '\u{1}', s @ ..] => {
					super::push(out, 16261);
					s.is_empty()
				}
				['ß', 'u', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 16262);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ial.
pub(crate) fn n1905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1903);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7056);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16263);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7057);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7058);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16266);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7059);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7060);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7061);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7062);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 16271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7063);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16272);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16273);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26380);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26381);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'i', s @ ..] => {
											super::push(out, 39764);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 39765);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['h', s @ ..] => super::t13::n47035(s, out),
														['l', 'h', s @ ..] => super::t13::n47036(s, out),
														['r', s @ ..] => super::t13::n47037(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', 'i', 'e', s @ ..] => {
											super::push(out, 39766);
											match s {
												[] => true,
												['w', 'h', s @ ..] => {
													super::push(out, 42685);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16274);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26382);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 34445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16275);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 26383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7064);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7065);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16278);
					match s {
						[] => true,
						['e', 't', 'o', 'p', 'm', s @ ..] => {
							super::push(out, 26385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 7066);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16279);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 16280);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16281);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 7067);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16283);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 26386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16284);
					match s {
						[] => true,
						['e', 't', 'o', 'p', 'm', s @ ..] => {
							super::push(out, 26387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jal.
pub(crate) fn n1906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1904);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kal.
pub(crate) fn n1907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1905);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7069);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16285);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34446);
									s.is_empty()
								}
								['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 34447);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['f', 'i', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 39767);
											s.is_empty()
										}
										['k', 'c', 'a', 'r', 'b', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 39768);
											s.is_empty()
										}
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
					super::push(out, 16286);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 26388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7070);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7071);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26389);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 26390);
							s.is_empty()
						}
						['e', 's', 'i', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 26391);
							s.is_empty()
						}
						['f', 'r', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 26392);
							s.is_empty()
						}
						['h', 'c', 's', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 26393);
							s.is_empty()
						}
						['l', 'h', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 26394);
							s.is_empty()
						}
						['m', 'm', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 26395);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26396);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 34448);
									s.is_empty()
								}
								['p', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 34449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26397);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39769);
											s.is_empty()
										}
										['r', 'h', 'a', 'f', 'n', 'r', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 39770);
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
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', 'l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
													super::push(out, 42686);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 42687);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'l', 'f', 's', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 39771);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 34450);
									s.is_empty()
								}
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 34451);
									s.is_empty()
								}
								['t', 'f', 'ä', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 34452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26398);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['n', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 34453);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 34454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16289);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 16290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26399);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26400);
							s.is_empty()
						}
						['n', 'e', 'g', 'n', 'u', 'z', 'r', 'e', 'd', 'r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 26401);
							s.is_empty()
						}
						['s', 'n', s @ ..] => {
							super::push(out, 26402);
							match s {
								[] => true,
								['o', 'i', 't', 'k', 'u', 'd', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 34455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26403);
							match s {
								[] => true,
								['n', 'e', 'z', 'k', 'a', '\u{1}', s @ ..] => {
									super::push(out, 34456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lal.
pub(crate) fn n1908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1906);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 7072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mal.
pub(crate) fn n1909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1907);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7073);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7074);
			match s {
				[] => true,
				['n', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 16291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7075);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 7076);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16292);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34457);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 34458);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 39772);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 34459);
									match s {
										[] => true,
										['g', 'e', 'i', 'r', 'k', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 39773);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'e', s @ ..] => {
									super::push(out, 34460);
									match s {
										[] => true,
										['j', 'w', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 39774);
											s.is_empty()
										}
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
					super::push(out, 16293);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 39775);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 39776);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'p', s @ ..] => {
									super::push(out, 34461);
									s.is_empty()
								}
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
			super::push(out, 7077);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7078);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 16294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7079);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16295);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7080);
			match s {
				[] => true,
				['t', 'r', 'u', 'b', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 16297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nal.
pub(crate) fn n1910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1908);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7081);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16298);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 16299);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 16300);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 26405);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 16301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26407);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 26408);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 26409);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26410);
							match s {
								[] => true,
								['e', 's', 'n', 'r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 34463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 26411);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 34464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 26412);
							match s {
								[] => true,
								['u', 'i', 'r', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 34465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 26413);
							s.is_empty()
						}
						['r', 'e', 's', s @ ..] => {
							super::push(out, 26414);
							match s {
								[] => true,
								['s', 'a', 'w', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 34466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26415);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 26416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16302);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 16303);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16304);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 16305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7082);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 16306);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16307);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34468);
									s.is_empty()
								}
								['n', 'e', 'f', 'f', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 34469);
									s.is_empty()
								}
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
			super::push(out, 7083);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26417);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 34470);
									match s {
										[] => true,
										['r', 'h', 'c', 'n', 'y', 's', '-', 't', 's', 'a', 't', 's', 'u', 'a', '-', 'd', 'l', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39777);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 34471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26418);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 26419);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 34472);
									s.is_empty()
								}
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
			super::push(out, 7084);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16308);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 16309);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16310);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 16311);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26422);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 34473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26423);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 't', s @ ..] => {
											super::push(out, 39778);
											s.is_empty()
										}
										['t', 'p', s @ ..] => {
											super::push(out, 39779);
											s.is_empty()
										}
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
					super::push(out, 16312);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 26424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7085);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7086);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16313);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26425);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7087);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 16315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7088);
			match s {
				[] => true,
				['b', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 16316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oal.
pub(crate) fn n1911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1909);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pal.
pub(crate) fn n1912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1910);
	match s {
		[] => true,
		['o', 's', s @ ..] => {
			super::push(out, 7090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ral.
pub(crate) fn n1913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1911);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7091);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16317);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26427);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39780);
											s.is_empty()
										}
										['e', 'n', 'i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 39781);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['e', 'm', 'm', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 42688);
													s.is_empty()
												}
												['g', 'e', 'i', 'r', 'k', 'r', 'e', 'g', 'r', 'ü', 'b', '\u{1}', s @ ..] => {
													super::push(out, 42689);
													s.is_empty()
												}
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
							super::push(out, 26428);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39782);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 39783);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 39784);
											match s {
												[] => true,
												['e', 's', 'r', 'a', '\u{1}', s @ ..] => {
													super::push(out, 42690);
													s.is_empty()
												}
												['o', 't', '\u{1}', s @ ..] => {
													super::push(out, 42691);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 39785);
											s.is_empty()
										}
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
					super::push(out, 16318);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26429);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7092);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7093);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 16320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7094);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7095);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16321);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 26431);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 16322);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 16323);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26432);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 39786);
											s.is_empty()
										}
										['k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42692);
													s.is_empty()
												}
												['s', 'b', 'a', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 42693);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7096);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7097);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7098);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16325);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16326);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sal.
pub(crate) fn n1914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1912);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7099);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 7100);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16329);
					s.is_empty()
				}
				['ü', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 16330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7101);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7102);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26434);
							s.is_empty()
						}
						['w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 26435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7103);
			match s {
				[] => true,
				['ü', 'm', '\u{1}', s @ ..] => {
					super::push(out, 16331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7104);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 7105);
			match s {
				[] => true,
				['c', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 16332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7106);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7107);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16333);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16334);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34475);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 34476);
									s.is_empty()
								}
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
			super::push(out, 7108);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16335);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 16336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7109);
			match s {
				[] => true,
				['h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 16337);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tal.
pub(crate) fn n1915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1913);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7111);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7112);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7113);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16340);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 26436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7114);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16342);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16343);
					match s {
						[] => true,
						['n', 'ä', 'h', '\u{1}', s @ ..] => {
							super::push(out, 26437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 16344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7115);
			match s {
				[] => true,
				['r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 16345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7116);
			match s {
				[] => true,
				['c', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 16346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7117);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16347);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34477);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 34478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 16348);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16349);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16350);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26440);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39787);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 42694);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 42695);
													match s {
														[] => true,
														['g', s @ ..] => super::t13::n47038(s, out),
														['i', s @ ..] => super::t13::n47039(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 39788);
											match s {
												[] => true,
												['g', 'n', s @ ..] => {
													match s {
														[] => true,
														['a', s @ ..] => super::t13::n47040(s, out),
														['u', 'd', s @ ..] => super::t13::n47041(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', 'm', s @ ..] => {
											super::push(out, 39789);
											match s {
												[] => true,
												['a', 's', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 42696);
													s.is_empty()
												}
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
							super::push(out, 26441);
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 34479);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 34480);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									super::push(out, 34481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 16351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7118);
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 16352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7119);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16353);
					match s {
						[] => true,
						['n', 'n', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 26442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16354);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 26443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7120);
			match s {
				[] => true,
				['o', 'r', 't', 's', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 16355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7121);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16356);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 26444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16357);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26445);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34482);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 39790);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 26446);
							s.is_empty()
						}
						['l', 'l', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 26447);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26448);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 34483);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 39791);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 34484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 26449);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 34485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 26450);
							s.is_empty()
						}
						['t', 'i', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 26451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7122);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7123);
			match s {
				[] => true,
				['a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 16358);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16359);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 26452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16360);
					match s {
						[] => true,
						['p', s @ ..] => super::t13::n27530(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t13::n7202(s, out),
		['t', s @ ..] => super::t13::n7203(s, out),
		['u', s @ ..] => super::t13::n7204(s, out),
		['x', s @ ..] => super::t13::n7205(s, out),
		['ß', 'u', 'l', 'f', '\u{1}', s @ ..] => super::t13::n7206(s, out),
		_ => false,
	}
}

/// Words ending -ual.
pub(crate) fn n1916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1914);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7129);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7130);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7131);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 16366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 7132);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7133);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7134);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -val.
pub(crate) fn n1917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1915);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7135);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7136);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7137);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 16370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7138);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7139);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wal.
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1916);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 7140);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16372);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 26453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yal.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1917);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7141);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 16373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zal.
pub(crate) fn n1920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1918);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 7142);
			match s {
				[] => true,
				['e', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 16374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -auchnabel.
pub(crate) fn n45485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42697);
	s.is_empty()
}

/// Words ending -rbel.
pub(crate) fn n7238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7157);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16406);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 26470);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 26471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16407);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 26472);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 26473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 16408);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 26474);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 26475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'z', '\u{1}', s @ ..] => {
			super::push(out, 16409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ubel.
pub(crate) fn n7239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7158);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16410);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 26476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 16411);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16412);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 26477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 16413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öbel.
pub(crate) fn n7241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7160);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 16414);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 16415);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 16416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cel.
pub(crate) fn n1928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1926);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 7162);
			s.is_empty()
		}
		_ => false,
	}
}

