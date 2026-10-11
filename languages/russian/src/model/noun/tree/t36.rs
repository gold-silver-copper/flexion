use flexion::Path;

/// Words ending -иток.
pub(crate) fn n11279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11240);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31572);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59858);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59859);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87763);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109244);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123313);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31573);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 59860);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87764);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 59861);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 87765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 59862);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 31574);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59864);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59865);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31575);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59866);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31576);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59868);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31577);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59869);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59870);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31578);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 31579);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 31580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кток.
pub(crate) fn n11280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11241);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 31581);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 59873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лток.
pub(crate) fn n11281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11242);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31582);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 59874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оток.
pub(crate) fn n11282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11243);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59875);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87772);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109246);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 123314);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 109247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59876);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 87773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 31584);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59877);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 87774);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 87775);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109248);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123315);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131768);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59878);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87776);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109249);
									s.is_empty()
								}
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
			super::push(out, 31585);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59879);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87777);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 87778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 31586);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59880);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31587);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31588);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59882);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59883);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59884);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 87781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31589);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59885);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59886);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87782);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109250);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123316);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 87783);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109251);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 109252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 87784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 59887);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87785);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109253);
									s.is_empty()
								}
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
			super::push(out, 31590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59888);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 59889);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87787);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109254);
									s.is_empty()
								}
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
			super::push(out, 31591);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59890);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87788);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109255);
									s.is_empty()
								}
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
			super::push(out, 31592);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59891);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59892);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87790);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 109256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 87791);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 109257);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123317);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 87792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59893);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87793);
							s.is_empty()
						}
						['т', '\u{1}', s @ ..] => {
							super::push(out, 87794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31593);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59894);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87795);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109258);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123318);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131769);
													s.is_empty()
												}
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
							super::push(out, 87796);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 109259);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123319);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131770);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59895);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59896);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87798);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 87799);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 109261);
									s.is_empty()
								}
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
			super::push(out, 31594);
			match s {
				[] => true,
				['о', s @ ..] => super::t47::n60528(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t47::n31850(s, out),
		['х', s @ ..] => super::t47::n31851(s, out),
		_ => false,
	}
}

/// Words ending -рток.
pub(crate) fn n11283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11244);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31597);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31598);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59901);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87802);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 87803);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 87804);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109263);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 123320);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131771);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сток.
pub(crate) fn n11284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11245);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31599);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 59902);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 87805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 31600);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31601);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 59903);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87806);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109264);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123321);
											s.is_empty()
										}
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
					super::push(out, 59904);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87807);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59905);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87808);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109266);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123322);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131772);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59906);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87809);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109267);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123323);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31602);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59907);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 59908);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 87810);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 59909);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 59910);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59911);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87812);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109269);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123324);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131773);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31603);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59912);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 87813);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109270);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123325);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 123326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 87814);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109271);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123327);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 131774);
													match s {
														[] => true,
														['в', s @ ..] => super::t47::n138139(s, out),
														_ => false,
													}
												}
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
							super::push(out, 87815);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109272);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123328);
											s.is_empty()
										}
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
					super::push(out, 59913);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87816);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 59914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87817);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 87818);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109274);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123329);
											s.is_empty()
										}
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
					super::push(out, 59915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t47::n60547(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31604);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59917);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87830);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109285);
									s.is_empty()
								}
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
			super::push(out, 31605);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59918);
					match s {
						[] => true,
						['в', s @ ..] => super::t47::n88778(s, out),
						['с', s @ ..] => {
							super::push(out, 87832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t47::n60550(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отток.
pub(crate) fn n31861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31606);
	s.is_empty()
}

/// Words ending -уток.
pub(crate) fn n11286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11247);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 31607);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59920);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87833);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109287);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123340);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131780);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31608);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59921);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59922);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87834);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109288);
									s.is_empty()
								}
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
			super::push(out, 31609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 31610);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31611);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59925);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87835);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109289);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123341);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131781);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31612);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59926);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шток.
pub(crate) fn n11287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11248);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 31613);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 59927);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 87836);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 109290);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123342);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31614);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59928);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31615);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыток.
pub(crate) fn n11288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11249);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31616);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59930);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эток.
pub(crate) fn n11289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11250);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 31617);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яток.
pub(crate) fn n11290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11251);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 31618);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31619);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59932);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87840);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109291);
									s.is_empty()
								}
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
			super::push(out, 31620);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59933);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичок.
pub(crate) fn n11298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11259);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31636);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59962);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87862);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 109300);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123346);
											s.is_empty()
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
								['б', s @ ..] => {
									super::push(out, 109301);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 109302);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123347);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131782);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['г', s @ ..] => {
							super::push(out, 87863);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109303);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123348);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131783);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'о', s @ ..] => {
							super::push(out, 87864);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109304);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123349);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109305);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123350);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131784);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 109306);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123351);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131785);
													s.is_empty()
												}
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
							super::push(out, 87865);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 87866);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 87867);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109308);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 109309);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123352);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131786);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31637);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59963);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'у', s @ ..] => {
			super::push(out, 31638);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31639);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87870);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 87871);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109310);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123353);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131787);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59965);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 87872);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 109311);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123354);
											s.is_empty()
										}
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
					super::push(out, 59966);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87873);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 87874);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 109312);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123355);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31640);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 31641);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87875);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 87876);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109313);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 31642);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59967);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 87877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олчок.
pub(crate) fn n31903(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 59969);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 59970);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87879);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 87880);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 109315);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 123356);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131788);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136728);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n141114(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анчок.
pub(crate) fn n31904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31644);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 59972);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -очок.
pub(crate) fn n11301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11262);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31645);
			s.is_empty()
		}
		['р', 'г', s @ ..] => {
			super::push(out, 31646);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчок.
pub(crate) fn n11302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11263);
	match s {
		[] => true,
		['а', 'х', '\u{1}', s @ ..] => {
			super::push(out, 31647);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31648);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59974);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87882);
							s.is_empty()
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
				['м', 'с', '\u{1}', s @ ..] => {
					super::push(out, 59975);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 59976);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учок.
pub(crate) fn n11303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11264);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31649);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 31650);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 59978);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87884);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109316);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'у', s @ ..] => {
					match s {
						[] => true,
						['р', 'у', s @ ..] => {
							super::push(out, 87885);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109317);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123358);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 87886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 31651);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31652);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31653);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31654);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 31655);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 59982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'р', s @ ..] => {
			super::push(out, 31656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59983);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109318);
									s.is_empty()
								}
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
			super::push(out, 31657);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59984);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычок.
pub(crate) fn n11304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11265);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31659);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 31660);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 31661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ючок.
pub(crate) fn n11305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11266);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ячок.
pub(crate) fn n11306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11267);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31663);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', 'о', s @ ..] => {
			super::push(out, 31664);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59987);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31665);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59988);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 87890);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109319);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123359);
											s.is_empty()
										}
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
					super::push(out, 59989);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87891);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 59990);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87892);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109321);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 59991);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 59992);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87893);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 59993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59994);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 87894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31666);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87896);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109323);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123360);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131789);
													s.is_empty()
												}
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
							super::push(out, 87897);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', 'и', 'а', s @ ..] => {
					super::push(out, 59996);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109325);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 59997);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87899);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109326);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123361);
											s.is_empty()
										}
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
					super::push(out, 59998);
					s.is_empty()
				}
				['п', 'е', s @ ..] => {
					super::push(out, 59999);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 87900);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 60000);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87901);
							s.is_empty()
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
				['в', s @ ..] => {
					super::push(out, 60001);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 60002);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31667);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 60003);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'б', '\u{1}', s @ ..] => {
					super::push(out, 60004);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 60005);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31668);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 60006);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87906);
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
				['с', s @ ..] => {
					super::push(out, 60007);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87907);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109328);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123362);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87908);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 87909);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 60008);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 31669);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 60009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 60010);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 60011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87911);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word футурошок.
pub(crate) fn n133149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131790);
	s.is_empty()
}

/// Words ending -стушок.
pub(crate) fn n60673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60036);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87926);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 109339);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 123365);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131791);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136729);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 139675);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n142856(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word чушок.
pub(crate) fn n31960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31693);
	s.is_empty()
}

/// Words ending -ышок.
pub(crate) fn n11314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11275);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 60037);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яшок.
pub(crate) fn n11315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11276);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31695);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 31696);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 60038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87928);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -менделеевск.
pub(crate) fn n138143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136730);
	s.is_empty()
}

/// Words ending -окуйбышевск.
pub(crate) fn n138144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136731);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139676);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ия-гротеск.
pub(crate) fn n133160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131801);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 136732);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139677);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 141411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ахтубинск.
pub(crate) fn n133162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131803);
	s.is_empty()
}

/// Words ending -акинск.
pub(crate) fn n60778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60141);
	s.is_empty()
}

/// Words ending -ткинск.
pub(crate) fn n60779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60142);
	s.is_empty()
}

/// Words ending -линск.
pub(crate) fn n32068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31799);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60143);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 88014);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 109390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 88015);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 109391);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 123392);
									s.is_empty()
								}
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
			super::push(out, 60144);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 88016);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 109392);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123393);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131804);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 60145);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 88017);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 88018);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 109393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 88019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -минск.
pub(crate) fn n32069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31800);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 60146);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 88020);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 109395);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нинск.
pub(crate) fn n32070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31801);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60147);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 60148);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 60149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пинск.
pub(crate) fn n32071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31802);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 60150);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 88021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 109396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 60151);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 88022);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 109397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 60152);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 88023);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 109398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринск.
pub(crate) fn n32072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31803);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60153);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 88024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 60154);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 88025);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 109399);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 123395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -синск.
pub(crate) fn n32073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31804);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 60155);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 88026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 60156);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 88027);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинск.
pub(crate) fn n32074(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60157);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 88028);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 109400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 60158);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 88029);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 109401);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 123396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинск.
pub(crate) fn n32075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31805);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 60159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чинск.
pub(crate) fn n32076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31806);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьинск.
pub(crate) fn n32077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31807);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 60161);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 88030);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 109402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онск.
pub(crate) fn n11377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11336);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 31808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янск.
pub(crate) fn n11378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11337);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31809);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 60162);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88031);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123397);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31810);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 60163);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109404);
									s.is_empty()
								}
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
			super::push(out, 31811);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сновишерск.
pub(crate) fn n133164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131805);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136733);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ес-пропуск.
pub(crate) fn n133165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131806);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рхангельск.
pub(crate) fn n133166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131807);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -но-курильск.
pub(crate) fn n133167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131808);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 136736);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 139678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жак-сюртук.
pub(crate) fn n133168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131809);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 136737);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паракаучук.
pub(crate) fn n133169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131810);
	s.is_empty()
}

/// Words ending -утилкаучук.
pub(crate) fn n133170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131811);
	s.is_empty()
}

/// Words ending -ин-сибиряк.
pub(crate) fn n133172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131813);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136738);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139680);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 141412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зар-вокзал.
pub(crate) fn n133176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131817);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136740);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 139681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инистериал.
pub(crate) fn n133177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131818);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -индастриал.
pub(crate) fn n133178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131819);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ультипотенциал.
pub(crate) fn n133179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131820);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -винциал.
pub(crate) fn n89183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88234);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 109492);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 123427);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 131821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яциал.
pub(crate) fn n32489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32210);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 60512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раворадикал.
pub(crate) fn n133181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131822);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -саббатикал.
pub(crate) fn n133182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131823);
	s.is_empty()
}

/// Words ending -иол-джамал.
pub(crate) fn n133183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131824);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 136745);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139683);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --профессионал.
pub(crate) fn n133184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131825);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 136746);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139684);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141413);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 142413);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 142961);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 143256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ернационал.
pub(crate) fn n133185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131826);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 136747);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word магистрал.
pub(crate) fn n133186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131827);
	s.is_empty()
}

/// Words ending -айт-портал.
pub(crate) fn n133187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131828);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бороздодел.
pub(crate) fn n133189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131830);
	s.is_empty()
}

/// Words ending -ацетонитрил.
pub(crate) fn n133191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131832);
	s.is_empty()
}

/// The word эозинофил.
pub(crate) fn n133192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131833);
	s.is_empty()
}

/// Words ending -рофил.
pub(crate) fn n32745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32458);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 60712);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 88354);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 109556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123457);
									s.is_empty()
								}
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
			super::push(out, 60713);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 88355);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 109557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'с', s @ ..] => {
			super::push(out, 60714);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 88356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 60715);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 88357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 60716);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 88358);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 109559);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -софил.
pub(crate) fn n32746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32459);
	s.is_empty()
}

/// Words ending -тофил.
pub(crate) fn n32747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32460);
	s.is_empty()
}

/// Words ending -раз-символ.
pub(crate) fn n133197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131838);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 136754);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139687);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роткоствол.
pub(crate) fn n133198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131839);
	s.is_empty()
}

/// Words ending -ом-ледокол.
pub(crate) fn n133199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131840);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 136755);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139688);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щина-посол.
pub(crate) fn n133200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131841);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136756);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139689);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 141415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-чехол.
pub(crate) fn n133201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131842);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136757);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 139690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word мультитул.
pub(crate) fn n133202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131843);
	s.is_empty()
}

/// Words ending -ллиангстрем.
pub(crate) fn n133205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131846);
	s.is_empty()
}

/// Words ending -нетто-объем.
pub(crate) fn n133206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131847);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -т-активизм.
pub(crate) fn n133207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131848);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 136760);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 139691);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 141416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -труктивизм.
pub(crate) fn n133208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131849);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136761);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 139692);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141417);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 142414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -релятивизм.
pub(crate) fn n133209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131850);
	s.is_empty()
}

/// Words ending -евнухоидизм.
pub(crate) fn n133210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131851);
	s.is_empty()
}

/// Words ending -оцкизм.
pub(crate) fn n61618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60974);
	s.is_empty()
}

/// Words ending -лизм.
pub(crate) fn n12245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12186);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 60975);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88518);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109641);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123500);
											match s {
												[] => true,
												['а', 'к', 'о', s @ ..] => {
													super::push(out, 131852);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138175(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 60976);
					match s {
						[] => true,
						['р', 'р', s @ ..] => {
							super::push(out, 88519);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109642);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123501);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131853);
													match s {
														[] => true,
														['г', s @ ..] => super::t47::n138176(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 88520);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109643);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123502);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131854);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'е', 'п', s @ ..] => {
							super::push(out, 88521);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 88522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 60977);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 60978);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88523);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109645);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123503);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131855);
													match s {
														[] => true,
														['о', s @ ..] => super::t47::n138177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'и', 'ц', 'а', s @ ..] => {
							super::push(out, 88524);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123504);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131856);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138178(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 123505);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131857);
													match s {
														[] => true,
														['б', s @ ..] => super::t47::n138179(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'р', s @ ..] => {
									super::push(out, 109646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 88525);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109647);
									match s {
										[] => true,
										['т', 'а', s @ ..] => {
											super::push(out, 123506);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131858);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							super::push(out, 88526);
							match s {
								[] => true,
								['и', 'л', 'а', s @ ..] => {
									super::push(out, 109648);
									match s {
										[] => true,
										['р', s @ ..] => super::t47::n124790(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', 'т', 'а', s @ ..] => {
							super::push(out, 88527);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t47::n61626(s, out),
				['у', s @ ..] => super::t47::n61627(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t47::n33120(s, out),
		['е', 'т', s @ ..] => super::t47::n33121(s, out),
		['и', s @ ..] => super::t47::n33122(s, out),
		['л', s @ ..] => super::t47::n33123(s, out),
		['о', s @ ..] => super::t47::n33124(s, out),
		['у', 'б', s @ ..] => super::t47::n33125(s, out),
		_ => false,
	}
}

/// Words ending -мизм.
pub(crate) fn n12246(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 32835);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 32836);
			s.is_empty()
		}
		['и', 'с', s @ ..] => {
			super::push(out, 32837);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 60984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'т', 'а', s @ ..] => {
			super::push(out, 32838);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 32839);
			match s {
				[] => true,
				['о', 'ф', 'н', s @ ..] => {
					super::push(out, 60985);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88536);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низм.
pub(crate) fn n12247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12187);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32840);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 60986);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 88537);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109654);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123510);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131861);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'у', s @ ..] => {
					super::push(out, 60987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 32841);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 32842);
			match s {
				[] => true,
				['л', 'а', s @ ..] => {
					super::push(out, 60988);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 60989);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 60990);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 88538);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 123511);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131862);
													match s {
														[] => true,
														['з', s @ ..] => super::t47::n138182(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 123512);
											s.is_empty()
										}
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
					super::push(out, 60991);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 60992);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 60993);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88539);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 88540);
							s.is_empty()
						}
						['у', 'п', s @ ..] => {
							super::push(out, 88541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 32843);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 60994);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32844);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', 'в', 'е', s @ ..] => {
							super::push(out, 88543);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109656);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'с', s @ ..] => {
							super::push(out, 88544);
							s.is_empty()
						}
						['ц', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109657);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 109658);
									match s {
										[] => true,
										['б', 'и', 'г', 'о', s @ ..] => {
											super::push(out, 123513);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131863);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138183(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 123514);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', 'е', 'т', s @ ..] => {
									super::push(out, 109659);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123515);
											s.is_empty()
										}
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
					super::push(out, 60995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 60996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32845);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 60997);
					match s {
						[] => true,
						['д', 'о', 'м', 'т', s @ ..] => {
							super::push(out, 88546);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109660);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123516);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131864);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'м', 'м', 'о', 'к', 'и', s @ ..] => {
			super::push(out, 32846);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 60998);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 88547);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', 'в', s @ ..] => {
			super::push(out, 32847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оизм.
pub(crate) fn n12248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12188);
	match s {
		[] => true,
		['а', 'м', s @ ..] => {
			super::push(out, 32848);
			s.is_empty()
		}
		['р', 'е', s @ ..] => {
			super::push(out, 32849);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 32850);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 60999);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 88548);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109662);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'н', 'и', 'с', s @ ..] => {
			super::push(out, 32851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пизм.
pub(crate) fn n12249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12189);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32852);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61000);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'т', 'о', 'е', s @ ..] => {
			super::push(out, 32853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ризм.
pub(crate) fn n12250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12190);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32854);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61001);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 61002);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 61003);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88550);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123518);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131865);
													s.is_empty()
												}
												_ => false,
											}
										}
										['э', s @ ..] => {
											super::push(out, 123519);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 88551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 61004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 32855);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61005);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 88552);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109663);
									s.is_empty()
								}
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
			super::push(out, 32856);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61006);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88553);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109664);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123520);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 131866);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['г', s @ ..] => {
					super::push(out, 61007);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88554);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 61008);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 61009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32857);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					super::push(out, 61010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'н', 'е', 'ц', 'о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61011);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88555);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109666);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123521);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131867);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'д', s @ ..] => {
					super::push(out, 61012);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88556);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109667);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123522);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 131868);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 61013);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88557);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109668);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123523);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131869);
													match s {
														[] => true,
														['а', s @ ..] => super::t47::n138184(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'т', 'у', 'ф', 'о', s @ ..] => {
			super::push(out, 32858);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61014);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 88558);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сизм.
pub(crate) fn n12251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12191);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32859);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61015);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 88559);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109670);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123524);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 131870);
													match s {
														[] => true,
														['с', s @ ..] => super::t47::n138185(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 61016);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88560);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109671);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123525);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['о', s @ ..] => {
					super::push(out, 61017);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 61018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32860);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 32861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тизм.
pub(crate) fn n12252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12192);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61019);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109672);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 109673);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123526);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 88561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 88562);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109674);
									match s {
										[] => true,
										['х', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138186(s, out),
														['т', s @ ..] => super::t47::n138187(s, out),
														_ => false,
													}
												}
												['и', 'р', s @ ..] => {
													super::push(out, 131871);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138188(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'о', s @ ..] => {
					super::push(out, 61020);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 88563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 61021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61022);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88564);
							match s {
								[] => true,
								['с', 'т', s @ ..] => {
									super::push(out, 109675);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123527);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131872);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138189(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'г', 'а', 'м', 'о', s @ ..] => {
					super::push(out, 61023);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 61024);
					s.is_empty()
				}
				['ч', 'с', 'а', 'н', 'к', 'а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 61025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 32862);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61026);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88565);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109676);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123528);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							super::push(out, 88566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'в', s @ ..] => {
					super::push(out, 61027);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 88567);
							s.is_empty()
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
				['и', s @ ..] => {
					super::push(out, 61028);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 88568);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109677);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 88569);
							match s {
								[] => true,
								['т', 'а', 'п', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123529);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 123530);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131873);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138190(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 123531);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131874);
													match s {
														[] => true,
														['е', s @ ..] => super::t47::n138191(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 61029);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 61030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32863);
			s.is_empty()
		}
		['с', 'о', s @ ..] => {
			super::push(out, 32864);
			match s {
				[] => true,
				['в', s @ ..] => super::t47::n61682(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t47::n33163(s, out),
		['ю', s @ ..] => super::t47::n33164(s, out),
		_ => false,
	}
}

/// Words ending -уизм.
pub(crate) fn n12253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12193);
	match s {
		[] => true,
		['д', 'н', s @ ..] => {
			super::push(out, 32866);
			s.is_empty()
		}
		['р', 'т', 'ь', s @ ..] => {
			super::push(out, 32867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -физм.
pub(crate) fn n12254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12194);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 32868);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 61035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хизм.
pub(crate) fn n12255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12195);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 32870);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 61036);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32871);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 61037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цизм.
pub(crate) fn n12256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12196);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
			super::push(out, 32873);
			match s {
				[] => true,
				['о', 'е', s @ ..] => {
					super::push(out, 61038);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 88575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 32874);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'о', 'т', 'а', 'к', 'о', s @ ..] => {
					super::push(out, 61039);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 61040);
					match s {
						[] => true,
						['с', 'у', s @ ..] => {
							super::push(out, 88576);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109678);
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
						['и', 'р', s @ ..] => {
							super::push(out, 88577);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 88578);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 88579);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109680);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123532);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 109681);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123533);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ашизм.
pub(crate) fn n12257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12197);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 32875);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 61041);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', 'о', 'р', s @ ..] => {
			super::push(out, 32876);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61042);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 88581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 61043);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88582);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109682);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123534);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -д-побратим.
pub(crate) fn n133235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131875);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136779);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139702);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141426);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 142420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хаджифатим.
pub(crate) fn n133236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131876);
	s.is_empty()
}

/// Words ending -ини-слалом.
pub(crate) fn n133241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131881);
	s.is_empty()
}

/// Words ending -ясомолпром.
pub(crate) fn n133242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131882);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136781);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 139703);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -электропром.
pub(crate) fn n133243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131883);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136782);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139704);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хоз-техникум.
pub(crate) fn n133247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131887);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 136784);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139706);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идраргирум.
pub(crate) fn n133248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131888);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 136785);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нтерсептум.
pub(crate) fn n133249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131889);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 136786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -индивидуум.
pub(crate) fn n133250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131890);
	s.is_empty()
}

/// Words ending -саскачеван.
pub(crate) fn n133253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131893);
	s.is_empty()
}

/// Words ending -орги-кардиган.
pub(crate) fn n133254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131894);
	s.is_empty()
}

/// Words ending -рейган.
pub(crate) fn n62020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61366);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 88790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лган.
pub(crate) fn n12577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12514);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33277);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 33278);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мган.
pub(crate) fn n12578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12515);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 33279);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нган.
pub(crate) fn n12579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12516);
	s.is_empty()
}

/// Words ending -оган.
pub(crate) fn n12580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12517);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 33280);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61369);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 88792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 33281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61370);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 61371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33282);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 61372);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рган.
pub(crate) fn n12581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12518);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33283);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88794);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 61374);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 61375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61376);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 61377);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 88795);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 61378);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88796);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109807);
									s.is_empty()
								}
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
			super::push(out, 33285);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тган.
pub(crate) fn n12582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12519);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33286);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 61380);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уган.
pub(crate) fn n12583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12520);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 33287);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33288);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 33289);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 61383);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 33290);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 61384);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 88799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фган.
pub(crate) fn n12584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12521);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33291);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61385);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыган.
pub(crate) fn n12585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12522);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 33292);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61386);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 88800);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109808);
									s.is_empty()
								}
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
			super::push(out, 33293);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61387);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 88801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33294);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61388);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 33295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61389);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 61390);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 88803);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109809);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123594);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131895);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юган.
pub(crate) fn n12586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12523);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 33296);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61391);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88804);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'к', 'а', 'т', '\u{1}', s @ ..] => {
			super::push(out, 33297);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 33298);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61392);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яган.
pub(crate) fn n12587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12524);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ик-великан.
pub(crate) fn n133256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131896);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136788);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 139708);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141428);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 142421);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ок-великан.
pub(crate) fn n133257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131897);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 136789);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139709);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141429);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142422);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142964);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ба-пеликан.
pub(crate) fn n133258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131898);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 136790);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рикан.
pub(crate) fn n33695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33386);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 61474);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 88851);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 109834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 61475);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 88852);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 109835);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123600);
									s.is_empty()
								}
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
			super::push(out, 61476);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 88853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итикан.
pub(crate) fn n62132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61478);
	s.is_empty()
}

/// Words ending -шикан.
pub(crate) fn n33697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33388);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 61479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щикан.
pub(crate) fn n33698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33389);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 61480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йкан.
pub(crate) fn n12633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12568);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ккан.
pub(crate) fn n12634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12569);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33391);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33392);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 61482);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкан.
pub(crate) fn n12635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12570);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33393);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 61484);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 61485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 33394);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33395);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61487);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 61488);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 61489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33396);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61490);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкан.
pub(crate) fn n12636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33397);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33398);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 33399);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окан.
pub(crate) fn n12637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12572);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33400);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61493);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 88859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109836);
									s.is_empty()
								}
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
			super::push(out, 33401);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пкан.
pub(crate) fn n12638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12573);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33402);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркан.
pub(crate) fn n12639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33403);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 61496);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88860);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109837);
									s.is_empty()
								}
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
			super::push(out, 33404);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 61497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33405);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61498);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 88861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33406);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 33407);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61499);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 88862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', '\u{1}', s @ ..] => {
					super::push(out, 61500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 33408);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61501);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 88863);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скан.
pub(crate) fn n12640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12575);
	s.is_empty()
}

/// Words ending -укан.
pub(crate) fn n12641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12576);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33409);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 33410);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 33411);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 33412);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 33413);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 33414);
			match s {
				[] => true,
				['с', 'и', '\u{1}', s @ ..] => {
					super::push(out, 61502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цкан.
pub(crate) fn n12642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12577);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33415);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкан.
pub(crate) fn n12643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12578);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33416);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ькан.
pub(crate) fn n12644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12579);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йб-улан.
pub(crate) fn n89832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88876);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 109845);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 123602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кулан.
pub(crate) fn n33760(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 61532);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 61533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тулан.
pub(crate) fn n33761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33448);
	s.is_empty()
}

/// Words ending -чулан.
pub(crate) fn n33762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33449);
	s.is_empty()
}

/// Words ending -ылан.
pub(crate) fn n12658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12591);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 33450);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word оман.
pub(crate) fn n33793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33476);
	s.is_empty()
}

/// Words ending -гоман.
pub(crate) fn n33794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33477);
	s.is_empty()
}

/// Words ending -доман.
pub(crate) fn n33795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33478);
	s.is_empty()
}

/// The word зоман.
pub(crate) fn n33796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33479);
	s.is_empty()
}

/// Words ending -коман.
pub(crate) fn n33797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33480);
	s.is_empty()
}

/// Words ending -оломан.
pub(crate) fn n33798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33481);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 61560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 88891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -номан.
pub(crate) fn n33799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33482);
	s.is_empty()
}

/// Words ending -роман.
pub(crate) fn n33800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33483);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 61561);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 61562);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 88892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 88893);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 88894);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 109850);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123604);
									s.is_empty()
								}
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
			super::push(out, 61563);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 88895);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 109851);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 123605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 109852);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123606);
									s.is_empty()
								}
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
			super::push(out, 61564);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 88896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соман.
pub(crate) fn n33801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33484);
	s.is_empty()
}

/// Words ending -томан.
pub(crate) fn n33802(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 61565);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 61566);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 61567);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 88897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фоман.
pub(crate) fn n33803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33485);
	s.is_empty()
}

/// Words ending -пман.
pub(crate) fn n12669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12602);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 33486);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рман.
pub(crate) fn n12670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12603);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33487);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61569);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 61570);
					s.is_empty()
				}
				['х', 'а', s @ ..] => {
					super::push(out, 61571);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 33488);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61572);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88899);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 61573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 61574);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88901);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 61575);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 88902);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 61576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33489);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 61577);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 61578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61579);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 61580);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88904);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109857);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123607);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 131900);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 61581);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88905);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109858);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123608);
											s.is_empty()
										}
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
					super::push(out, 61582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 61583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 61584);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88908);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109859);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123609);
											s.is_empty()
										}
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
					super::push(out, 61585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88909);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 88910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 61586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сман.
pub(crate) fn n12671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12604);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 33491);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61587);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 88912);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 61588);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88913);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109861);
									s.is_empty()
								}
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
			super::push(out, 33492);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61589);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88914);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109862);
									s.is_empty()
								}
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
			super::push(out, 33493);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61590);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 88915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33494);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61591);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 88916);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тман.
pub(crate) fn n12672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61592);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 61593);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 61594);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 61595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 33496);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33497);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 33498);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 61598);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88918);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уман.
pub(crate) fn n12673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12606);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33499);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 61599);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88919);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109865);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123610);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131901);
													match s {
														[] => true,
														['н', s @ ..] => super::t47::n138204(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 33500);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61600);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 88920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хман.
pub(crate) fn n12674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12607);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 61603);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 88922);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 88923);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 88924);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цман.
pub(crate) fn n12675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12608);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33502);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61604);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 61605);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 88925);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109869);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123611);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 131902);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138205(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 33503);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 61606);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чман.
pub(crate) fn n12676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12609);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 33504);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61607);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 61608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шман.
pub(crate) fn n12677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33505);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 33506);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61609);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьман.
pub(crate) fn n12678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12611);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33507);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61610);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 88927);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 88928);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 88929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 61611);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 88930);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109872);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123612);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 131903);
													s.is_empty()
												}
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
							super::push(out, 88931);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109873);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123613);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 88932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 61612);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 88933);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 109875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 61613);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 61614);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88935);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 88936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word табасаран.
pub(crate) fn n133264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131904);
	s.is_empty()
}

/// Words ending -втоветеран.
pub(crate) fn n133265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обиль-кран.
pub(crate) fn n133266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131906);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136794);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139712);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оран.
pub(crate) fn n12707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12640);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 33557);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 33558);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 33559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61657);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 88956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 33560);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61658);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33561);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 61659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33562);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 61660);
					match s {
						[] => true,
						['е', 'р', '-', s @ ..] => {
							super::push(out, 88958);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109885);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 123618);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 131907);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n138208(s, out),
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
									super::push(out, 109886);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 109887);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123619);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131908);
													match s {
														[] => true,
														['а', s @ ..] => super::t47::n138209(s, out),
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
									super::push(out, 109888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word сопран.
pub(crate) fn n89915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88959);
	s.is_empty()
}

/// Words ending -рран.
pub(crate) fn n12709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33564);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сран.
pub(crate) fn n12710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12643);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33565);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тран.
pub(crate) fn n12711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12644);
	match s {
		[] => true,
		['а', 'к', '\u{1}', s @ ..] => {
			super::push(out, 33566);
			s.is_empty()
		}
		['р', 'е', s @ ..] => {
			super::push(out, 33567);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уран.
pub(crate) fn n12712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12645);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 33568);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 33569);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33570);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 33571);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фран.
pub(crate) fn n12713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12646);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33572);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 61667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хран.
pub(crate) fn n12714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12647);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33573);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61668);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 61669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -афе-шантан.
pub(crate) fn n133269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131909);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтрамонтан.
pub(crate) fn n133270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131910);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136798);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 139714);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отан.
pub(crate) fn n12735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12667);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 33612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61705);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 61706);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 33613);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 33614);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 33615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61710);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стан.
pub(crate) fn n12738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12670);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33618);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 33619);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 33620);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 33621);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61711);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 61712);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 88981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109898);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 109899);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123624);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33622);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 33623);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 61713);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88982);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109900);
									s.is_empty()
								}
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
			super::push(out, 33624);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61714);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88983);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109901);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123625);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33625);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61715);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 88984);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109902);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттан.
pub(crate) fn n12739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12671);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33626);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 61716);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 88985);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109903);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123627);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131911);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138212(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утан.
pub(crate) fn n12740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12672);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 33627);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61717);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 88986);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109904);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123628);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131912);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 33628);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 33629);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88987);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109905);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123629);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33630);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61719);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 88988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -афтан.
pub(crate) fn n33956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33631);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 61720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хтан.
pub(crate) fn n12742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12674);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 33632);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 61721);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 88989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штан.
pub(crate) fn n12743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12675);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аэтан.
pub(crate) fn n12744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12676);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 33634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -амхан.
pub(crate) fn n34012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33687);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 61777);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 89016);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 109918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 61778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 89017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имхан.
pub(crate) fn n34013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33688);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 61779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89018);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 109919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 89019);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 109920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 61780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рымхан.
pub(crate) fn n62436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61781);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 89020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нхан.
pub(crate) fn n12776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12707);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33690);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61782);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89021);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109921);
									s.is_empty()
								}
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
			super::push(out, 33691);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61783);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -охан.
pub(crate) fn n12777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12708);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 33692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пхан.
pub(crate) fn n12778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12709);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 33693);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 61785);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхан.
pub(crate) fn n12779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33694);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61786);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 61787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 61788);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89024);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109922);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123631);
											s.is_empty()
										}
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
					super::push(out, 61789);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 89025);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 61790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89026);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 89027);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109924);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123632);
											s.is_empty()
										}
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
					super::push(out, 61791);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89028);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 109925);
									s.is_empty()
								}
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
			super::push(out, 33695);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61792);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89029);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 61793);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89030);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109927);
									s.is_empty()
								}
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
			super::push(out, 33696);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61794);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 61795);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109928);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 109929);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 89031);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 61796);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89032);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 109931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 61797);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33697);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61798);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33698);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 61800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 61801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 33699);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -схан.
pub(crate) fn n12780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12711);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33700);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61803);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 89039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61804);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89040);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109932);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 123633);
											s.is_empty()
										}
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
					super::push(out, 61805);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 89041);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109933);
									s.is_empty()
								}
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
			super::push(out, 33701);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 61806);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 89042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тхан.
pub(crate) fn n12781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12712);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33702);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61807);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 61808);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 89043);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109934);
									s.is_empty()
								}
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
			super::push(out, 33703);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61809);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 89044);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109935);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123634);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ухан.
pub(crate) fn n12782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12713);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33705);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61810);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 61811);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 61812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 33706);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 61813);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89046);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109937);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123635);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33707);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 33708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 33709);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чхан.
pub(crate) fn n12783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12714);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33710);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61816);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 33711);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61817);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 89048);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109938);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123636);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шхан.
pub(crate) fn n12784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12715);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 33712);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 33713);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61818);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щхан.
pub(crate) fn n12785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12716);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33714);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 33715);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыхан.
pub(crate) fn n12786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12717);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 33716);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61819);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89049);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьхан.
pub(crate) fn n12787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12718);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33717);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61820);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89050);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 61821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юхан.
pub(crate) fn n12788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33718);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 33719);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 61822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33720);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61823);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word метаноген.
pub(crate) fn n133274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131914);
	s.is_empty()
}

/// Words ending -антропоген.
pub(crate) fn n133275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131915);
	s.is_empty()
}

/// Words ending -ицберген.
pub(crate) fn n111120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109952);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 123642);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 131916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лерген.
pub(crate) fn n62520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61865);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 89073);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 109953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орген.
pub(crate) fn n34096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33769);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 61866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 89074);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word юрген.
pub(crate) fn n62522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61867);
	s.is_empty()
}

/// Words ending -сген.
pub(crate) fn n12825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12755);
	s.is_empty()
}

/// Words ending -тген.
pub(crate) fn n12826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12756);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 33771);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оберхаузен.
pub(crate) fn n133278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131918);
	s.is_empty()
}

/// Words ending -леверкузен.
pub(crate) fn n133279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131919);
	s.is_empty()
}

/// Words ending -ана-член.
pub(crate) fn n111132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109964);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 123649);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 131920);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 136801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очлен.
pub(crate) fn n34169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33842);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 61931);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 89112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 61932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 89113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элен.
pub(crate) fn n12877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12807);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -беллмен.
pub(crate) fn n90090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89133);
	s.is_empty()
}

/// Words ending -ммен.
pub(crate) fn n12888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12818);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 33871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нмен.
pub(crate) fn n12889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12819);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33872);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61962);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 33873);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61963);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 33874);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 61964);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омен.
pub(crate) fn n12890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12820);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 33875);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61965);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89137);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109971);
									s.is_empty()
								}
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
			super::push(out, 33876);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61966);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 61967);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 33877);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61968);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89139);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 109972);
									s.is_empty()
								}
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
			super::push(out, 33878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 33879);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 61970);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 61971);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 89140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33880);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 61972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмен.
pub(crate) fn n12891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12821);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33881);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 61973);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 61974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 33882);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61975);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89142);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 61976);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 61977);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 89143);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109974);
									s.is_empty()
								}
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
			super::push(out, 33883);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -смен.
pub(crate) fn n12892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12822);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 33885);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 61980);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89145);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 61981);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89146);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109976);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123650);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131921);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 131922);
													match s {
														[] => true,
														['и', s @ ..] => super::t47::n138215(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 33886);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 61982);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 89147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33887);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 33888);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61984);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109977);
									s.is_empty()
								}
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
			super::push(out, 33889);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 61985);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89149);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109978);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123651);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33890);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 61986);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89150);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109979);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123652);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33891);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61987);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89151);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 61988);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89152);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109981);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123653);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 33892);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33893);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 61990);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тмен.
pub(crate) fn n12893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12823);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33894);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33895);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 33896);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61992);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89153);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 109982);
									s.is_empty()
								}
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
			super::push(out, 33897);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 61993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умен.
pub(crate) fn n12894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12824);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33898);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 61994);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89154);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 109983);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123654);
											s.is_empty()
										}
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
					super::push(out, 61995);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89155);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 89156);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 89157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 61996);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89158);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109984);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123655);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 131923);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 33899);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 61997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 33900);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61998);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89159);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109985);
									s.is_empty()
								}
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
			super::push(out, 33901);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61999);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33902);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 62001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33903);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62002);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 89162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 62003);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 89163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33904);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 33905);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62005);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89164);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109986);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123656);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цмен.
pub(crate) fn n12895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12825);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33906);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шмен.
pub(crate) fn n12896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12826);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33907);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62007);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 89165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33908);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьмен.
pub(crate) fn n12897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12827);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33909);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62009);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89166);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 62010);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 62011);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89168);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109988);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 123657);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмен.
pub(crate) fn n12898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12828);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33910);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62012);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епидосирен.
pub(crate) fn n133284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131924);
	s.is_empty()
}

/// Words ending -стерботтен.
pub(crate) fn n133285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131925);
	s.is_empty()
}

/// The word манхэттен.
pub(crate) fn n133286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131926);
	s.is_empty()
}

/// Words ending -хохфильцен.
pub(crate) fn n133287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131927);
	s.is_empty()
}

/// Words ending -маульташен.
pub(crate) fn n133288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131928);
	s.is_empty()
}

/// Words ending -нон-фикшен.
pub(crate) fn n133289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131929);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новокрещен.
pub(crate) fn n133290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131930);
	s.is_empty()
}

/// Words ending -ацетамидин.
pub(crate) fn n133292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131932);
	s.is_empty()
}

/// Words ending -рфордин.
pub(crate) fn n90211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89254);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 110028);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 123678);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 131933);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 136805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урдин.
pub(crate) fn n34405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34076);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 62156);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 62157);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89255);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 110029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тдин.
pub(crate) fn n13022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12951);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 34077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удин.
pub(crate) fn n13023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12952);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34078);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 34079);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 34080);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62158);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89256);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110030);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123679);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34081);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 34082);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 34083);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62159);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34084);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62160);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 34085);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хдин.
pub(crate) fn n13024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12953);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 34086);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдин.
pub(crate) fn n13025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12954);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34087);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62162);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдин.
pub(crate) fn n13026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12955);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34088);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62163);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89260);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядин.
pub(crate) fn n13027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12956);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -алон-магазин.
pub(crate) fn n133294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131934);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136806);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ел-арлекин.
pub(crate) fn n133295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131935);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136807);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обзывалкин.
pub(crate) fn n133296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131936);
	s.is_empty()
}

/// Words ending -скин.
pub(crate) fn n13058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12987);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 34135);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62203);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 89289);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110046);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123684);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34136);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62204);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89290);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110047);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123685);
											s.is_empty()
										}
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
					super::push(out, 62205);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89291);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 110048);
									s.is_empty()
								}
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
			super::push(out, 34137);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62206);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34138);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 34139);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62207);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89293);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 110049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чкин.
pub(crate) fn n13060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12989);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34141);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62209);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89295);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110051);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123687);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкин.
pub(crate) fn n13061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12990);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 34142);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62210);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89296);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110052);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123688);
											s.is_empty()
										}
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
					super::push(out, 62211);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89297);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 62212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 34143);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькин.
pub(crate) fn n13062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12991);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34144);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62214);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идофилин.
pub(crate) fn n111229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110061);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 123693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 131937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клин.
pub(crate) fn n13068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12997);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34167);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 34168);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 62236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 62237);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 62238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34169);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62239);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89320);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110062);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 123694);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131938);
													match s {
														[] => true,
														['а', s @ ..] => super::t47::n138221(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 62240);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89321);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110063);
									s.is_empty()
								}
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
			super::push(out, 34170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62241);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89322);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 110064);
									s.is_empty()
								}
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
			super::push(out, 34171);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 34172);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 34173);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62243);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллин.
pub(crate) fn n13069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12998);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34174);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34175);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62245);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89325);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110066);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123695);
											s.is_empty()
										}
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
					super::push(out, 62246);
					match s {
						[] => true,
						['и', 'в', 'а', s @ ..] => {
							super::push(out, 89326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110067);
									s.is_empty()
								}
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
			super::push(out, 34176);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 62247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 34177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62248);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 62249);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89327);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 110068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123696);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млин.
pub(crate) fn n13070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12999);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34178);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62250);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110069);
									s.is_empty()
								}
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
			super::push(out, 34179);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62251);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олин.
pub(crate) fn n13071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13000);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34180);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 34181);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62253);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89330);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110071);
									s.is_empty()
								}
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
			super::push(out, 34182);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 34183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62255);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 62256);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89331);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110072);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 123697);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34184);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62257);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 62258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 34185);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62259);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34186);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 34187);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 34188);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 34189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плин.
pub(crate) fn n13072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13001);
	s.is_empty()
}

/// Words ending -рлин.
pub(crate) fn n13073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13002);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34190);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 62262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34191);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слин.
pub(crate) fn n13074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13003);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 34192);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62264);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлин.
pub(crate) fn n13075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13004);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34193);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 34194);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улин.
pub(crate) fn n13076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13005);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34195);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 34196);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62268);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89338);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word бенджамин.
pub(crate) fn n133299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131939);
	s.is_empty()
}

/// Words ending -нес-термин.
pub(crate) fn n133300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131940);
	s.is_empty()
}

/// Words ending -абдулмумин.
pub(crate) fn n133301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131941);
	s.is_empty()
}

/// Words ending -рховажанин.
pub(crate) fn n133302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131942);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136809);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 139719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хариджанин.
pub(crate) fn n133303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131943);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нианин.
pub(crate) fn n62953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62298);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 89360);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 110087);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 110088);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 123708);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131944);
									s.is_empty()
								}
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
			super::push(out, 89361);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 110089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тианин.
pub(crate) fn n62956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62301);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 89364);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 110090);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 123709);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 131945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фианин.
pub(crate) fn n62957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62302);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 89365);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 110091);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123710);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131946);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136811);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цианин.
pub(crate) fn n62958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62303);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 89366);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 110092);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123711);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 131947);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136812);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -канин.
pub(crate) fn n34548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34218);
	s.is_empty()
}

/// Words ending -ланин.
pub(crate) fn n34549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34219);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 62304);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89367);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 62305);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 89368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 62306);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 89369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 62307);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89370);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 110094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранин.
pub(crate) fn n34550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34220);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 62308);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89371);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танин.
pub(crate) fn n34551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34221);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 62309);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 62310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уанин.
pub(crate) fn n34552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34222);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 62311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 89372);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 89373);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 110096);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 123713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чанин.
pub(crate) fn n34553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34223);
	match s {
		[] => true,
		['в', 'о', 'м', 'е', 'т', 'р', 'а', '\u{1}', s @ ..] => {
			super::push(out, 62312);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 62313);
			match s {
				[] => true,
				['л', 'е', 'б', s @ ..] => {
					super::push(out, 89374);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110097);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 123714);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131948);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136813);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n141160(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шанин.
pub(crate) fn n34554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34224);
	s.is_empty()
}

/// Words ending -щанин.
pub(crate) fn n34555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34225);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 62314);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 89375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гнин.
pub(crate) fn n13088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13017);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34226);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34227);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62316);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89377);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110099);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123715);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енин.
pub(crate) fn n13089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13018);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 34229);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62318);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89378);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110100);
									s.is_empty()
								}
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
			super::push(out, 34230);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62320);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 62321);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 89379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 34232);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 62322);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89380);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 110101);
									s.is_empty()
								}
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
			super::push(out, 34233);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 62323);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89381);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110102);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инин.
pub(crate) fn n13090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13019);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 34234);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62324);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 34235);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 62325);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89383);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110103);
									s.is_empty()
								}
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
			super::push(out, 34236);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 34237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62327);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34238);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62328);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 62329);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 89386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34239);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62330);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89387);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 110105);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 123717);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 131949);
													match s {
														[] => true,
														['в', s @ ..] => super::t47::n138227(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 62331);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89388);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110106);
									s.is_empty()
								}
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
			super::push(out, 34240);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 34241);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 34242);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62333);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89389);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110107);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123718);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йнин.
pub(crate) fn n13091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13020);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34243);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 62334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мнин.
pub(crate) fn n13092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13021);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 34244);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62335);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ннин.
pub(crate) fn n13093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13022);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34245);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62336);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89391);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онин.
pub(crate) fn n13094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13023);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 34246);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62337);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34247);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 34248);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 34249);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62339);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 62340);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34250);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62341);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 89395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 62342);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 62343);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89397);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унин.
pub(crate) fn n13095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13024);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62344);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнин.
pub(crate) fn n13096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13025);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34252);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62345);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89398);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янин.
pub(crate) fn n13097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13026);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34253);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62346);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89399);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110112);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 110113);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123719);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131950);
													s.is_empty()
												}
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
							super::push(out, 89400);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110114);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123720);
											s.is_empty()
										}
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
					super::push(out, 62347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62348);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89401);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110115);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123721);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131951);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34254);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62350);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89402);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110116);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123722);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131952);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62351);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89403);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110117);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123723);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131953);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'а', 'з', 'а', 'н', '\u{1}', s @ ..] => {
					super::push(out, 62352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 34255);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 34256);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62353);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89404);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 110118);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123724);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 131954);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62354);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89405);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110119);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123725);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131955);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62355);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89406);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110120);
									s.is_empty()
								}
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
			super::push(out, 34257);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62356);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 62357);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 62358);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110121);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 110122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 62359);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110123);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123726);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 131956);
													match s {
														[] => true,
														['и', s @ ..] => super::t47::n138228(s, out),
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
									super::push(out, 110124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 89409);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110125);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 110126);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123727);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131957);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 34258);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62361);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t47::n34590(s, out),
		['р', s @ ..] => {
			super::push(out, 34260);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62368);
					s.is_empty()
				}
				['о', s @ ..] => super::t47::n63025(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t47::n34592(s, out),
		['ф', s @ ..] => super::t47::n34593(s, out),
		['ь', s @ ..] => super::t47::n34594(s, out),
		_ => false,
	}
}

/// Words ending -аника-воин.
pub(crate) fn n133324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131964);
	s.is_empty()
}

/// Words ending -гардемарин.
pub(crate) fn n133325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131965);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136818);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -грин.
pub(crate) fn n13113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13042);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34288);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62407);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89457);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110156);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123743);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62408);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дрин.
pub(crate) fn n13114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13043);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34290);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 62409);
					match s {
						[] => true,
						['э', 'о', s @ ..] => {
							super::push(out, 89458);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110157);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123744);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131966);
													match s {
														[] => true,
														['с', s @ ..] => super::t47::n138232(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерин.
pub(crate) fn n13115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13044);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34292);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 34293);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 62410);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34294);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 62411);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89460);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110158);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123745);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131967);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34296);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 34297);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирин.
pub(crate) fn n13116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13045);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 34298);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62414);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 34300);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62416);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 34301);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62417);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кейрин.
pub(crate) fn n63074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62418);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 89464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крин.
pub(crate) fn n13118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13047);
	s.is_empty()
}

/// Words ending -орин.
pub(crate) fn n13119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13048);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 34303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прин.
pub(crate) fn n13120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13049);
	s.is_empty()
}

/// Words ending -ррин.
pub(crate) fn n13121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13050);
	s.is_empty()
}

/// Words ending -трин.
pub(crate) fn n13122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13051);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34304);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34305);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62421);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 62422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34306);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 34307);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урин.
pub(crate) fn n13123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13052);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 34308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 34309);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 34310);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62426);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 89467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110160);
									s.is_empty()
								}
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
			super::push(out, 34311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62427);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрин.
pub(crate) fn n13124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13053);
	s.is_empty()
}

/// Words ending -юрин.
pub(crate) fn n13125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13054);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34312);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34313);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярин.
pub(crate) fn n13126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13055);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34314);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коломнитин.
pub(crate) fn n133328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131968);
	s.is_empty()
}

/// Words ending -антин.
pub(crate) fn n34679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34348);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 62462);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89488);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 62463);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 89489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 62464);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 89490);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 110175);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123754);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131969);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ентин.
pub(crate) fn n34680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34349);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 62465);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 89491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 62466);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 62467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89492);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 110177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отин.
pub(crate) fn n13144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13073);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62468);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 62469);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89493);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110178);
									s.is_empty()
								}
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
			super::push(out, 34351);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62470);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89494);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 89495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртин.
pub(crate) fn n13145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13074);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34353);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 62471);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89496);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110179);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123756);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131970);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89497);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 89498);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 110180);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123757);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131971);
													s.is_empty()
												}
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
							super::push(out, 89499);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110181);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 123758);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34354);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62473);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89500);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 62474);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34355);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62475);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стин.
pub(crate) fn n13146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13075);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34356);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 62477);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 62478);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 89505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34357);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62479);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34358);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62480);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89507);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 62481);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89508);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110184);
									s.is_empty()
								}
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
			super::push(out, 34359);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62482);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 62483);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89509);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110185);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123759);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34360);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 62484);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 89510);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110186);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123760);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 89511);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110187);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123761);
											s.is_empty()
										}
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
					super::push(out, 62485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еттин.
pub(crate) fn n34692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34361);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 62486);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89512);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110188);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 123762);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 131972);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утин.
pub(crate) fn n13148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13077);
	s.is_empty()
}

/// Words ending -хтин.
pub(crate) fn n13149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13078);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34362);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штин.
pub(crate) fn n13150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13079);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34363);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62488);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтин.
pub(crate) fn n13151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13080);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62489);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89515);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110190);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123763);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 89516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этин.
pub(crate) fn n13152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13081);
	s.is_empty()
}

/// Words ending -ютин.
pub(crate) fn n13153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13082);
	s.is_empty()
}

/// Words ending -ние-хозяин.
pub(crate) fn n133333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131973);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136820);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 139724);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141432);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изм-хозяин.
pub(crate) fn n133334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131974);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136821);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139725);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 141433);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дух-хозяин.
pub(crate) fn n133335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131975);
	s.is_empty()
}

/// Words ending -ногон.
pub(crate) fn n34798(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 62579);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 89575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 62580);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89576);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рогон.
pub(crate) fn n34800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34467);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 62582);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 89577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 62583);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 89578);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 110223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 89579);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 62584);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 89580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -согон.
pub(crate) fn n34801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34468);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 62585);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 62586);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 62587);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 89583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тогон.
pub(crate) fn n34802(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 62588);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89584);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 110225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 62589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 89585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргон.
pub(crate) fn n13244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13173);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34469);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 62590);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89586);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110226);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123777);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131977);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138236(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34470);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 62591);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 89587);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 110227);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123778);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131978);
													match s {
														[] => true,
														['б', s @ ..] => super::t47::n138237(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угон.
pub(crate) fn n13246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13175);
	s.is_empty()
}

/// Words ending -ыгон.
pub(crate) fn n13247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13176);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34471);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62592);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ропагандон.
pub(crate) fn n133339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131979);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одон.
pub(crate) fn n13260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13189);
	match s {
		[] => true,
		['г', 'и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62612);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 62613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 34494);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34495);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62615);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89604);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 62616);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89605);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110235);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123781);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131980);
													match s {
														[] => true,
														['л', 'п', '\u{1}', s @ ..] => super::t47::n138239(s, out),
														_ => false,
													}
												}
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
							super::push(out, 89606);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 110236);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 123782);
											s.is_empty()
										}
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
					super::push(out, 62617);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89607);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110237);
									s.is_empty()
								}
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
			super::push(out, 34496);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34497);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62619);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34498);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 62620);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word гаммадион.
pub(crate) fn n133341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131981);
	s.is_empty()
}

/// Words ending -акроподион.
pub(crate) fn n133342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131982);
	s.is_empty()
}

/// Words ending -ндоцеллион.
pub(crate) fn n133343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131983);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 136827);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дриллион.
pub(crate) fn n111420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110252);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 123792);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 131984);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 136828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиллион.
pub(crate) fn n90594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89635);
	s.is_empty()
}

/// Words ending -циллион.
pub(crate) fn n90595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89636);
	s.is_empty()
}

/// Words ending -мион.
pub(crate) fn n13290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13217);
	s.is_empty()
}

/// Words ending -нион.
pub(crate) fn n13291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13218);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34543);
			match s {
				[] => true,
				['б', 'и', s @ ..] => {
					super::push(out, 62659);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89637);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110254);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123793);
											s.is_empty()
										}
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
					super::push(out, 62660);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89638);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110255);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123794);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'р', s @ ..] => {
			super::push(out, 34544);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 62661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110256);
									s.is_empty()
								}
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
			super::push(out, 34545);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62662);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'е', s @ ..] => {
			super::push(out, 34546);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пион.
pub(crate) fn n13292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13219);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34547);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 34548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 34549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62665);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 62666);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 89643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34550);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 62667);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89644);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123795);
											s.is_empty()
										}
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
					super::push(out, 62668);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89645);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 110258);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123796);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 89646);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 110259);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 123797);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34551);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62669);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89647);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 110260);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123798);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131985);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34552);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 62670);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89648);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 110261);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123799);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131986);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138242(s, out),
														_ => false,
													}
												}
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
							super::push(out, 89649);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110262);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123800);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 131987);
													match s {
														[] => true,
														['у', s @ ..] => super::t47::n138243(s, out),
														_ => false,
													}
												}
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
							super::push(out, 89650);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110263);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123801);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131988);
													match s {
														[] => true,
														['м', s @ ..] => super::t47::n138244(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 62671);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89651);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 62672);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89652);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рион.
pub(crate) fn n13293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13220);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34553);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62673);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89653);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 110266);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 62674);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89654);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110267);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123802);
											s.is_empty()
										}
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
					super::push(out, 62675);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 62676);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89655);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 62677);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89656);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110269);
									s.is_empty()
								}
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
			super::push(out, 34554);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62678);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 89657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 34555);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34556);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62680);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 62681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34557);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 62682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34558);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62683);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 62684);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 62685);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89658);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 62686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89659);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 89660);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 110271);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123803);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 131989);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34559);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62687);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89661);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110272);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123804);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131990);
													match s {
														[] => true,
														['г', s @ ..] => super::t47::n138245(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34560);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62688);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 89662);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 110273);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123805);
											s.is_empty()
										}
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
						['е', s @ ..] => {
							super::push(out, 89663);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110274);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123806);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 89664);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110275);
									s.is_empty()
								}
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
			super::push(out, 34561);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62689);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 62690);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89666);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110276);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 123807);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34562);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сион.
pub(crate) fn n13294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13221);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 34563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тион.
pub(crate) fn n13295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13222);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34564);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 34565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62692);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89668);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110277);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123808);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131991);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62693);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хион.
pub(crate) fn n13296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13223);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34566);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цион.
pub(crate) fn n13297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13224);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енетлиакон.
pub(crate) fn n133352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131992);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 136833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -азин-салон.
pub(crate) fn n133353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131993);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 136834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139730);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 141438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерея-салон.
pub(crate) fn n133354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131994);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136835);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139731);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 141439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тье-баллон.
pub(crate) fn n133355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131995);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136836);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139732);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поллон.
pub(crate) fn n63389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62732);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 89694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эллон.
pub(crate) fn n34958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34623);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 62733);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 89695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олон.
pub(crate) fn n13323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13250);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34624);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62734);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89696);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110291);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123814);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34625);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 34626);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62735);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 62736);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89697);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110292);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123815);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131996);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62737);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89698);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 62738);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 89699);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123816);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34627);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 34629);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рлон.
pub(crate) fn n13324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13251);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34630);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62741);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слон.
pub(crate) fn n13325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13252);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34632);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 34633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тлон.
pub(crate) fn n13326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13253);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34634);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улон.
pub(crate) fn n13327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13254);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -флон.
pub(crate) fn n13328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13255);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34636);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 34637);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кибердемон.
pub(crate) fn n133357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131997);
	s.is_empty()
}

/// Words ending -нтелеймон.
pub(crate) fn n125103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123819);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 131998);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 136837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ммон.
pub(crate) fn n13336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34657);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62767);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омон.
pub(crate) fn n13337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13264);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 34658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62768);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 62769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89717);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 89718);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 110302);
									s.is_empty()
								}
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
			super::push(out, 34659);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 62770);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34660);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62771);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 62772);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 34661);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 62773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34662);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 62775);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 89723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рмон.
pub(crate) fn n13338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13265);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34664);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 62776);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 62777);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89724);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 62778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умон.
pub(crate) fn n13339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13266);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 34665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хмон.
pub(crate) fn n13340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13267);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34666);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шмон.
pub(crate) fn n13341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13268);
	s.is_empty()
}

/// Words ending -эмон.
pub(crate) fn n13342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13269);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34667);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62780);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word квартерон.
pub(crate) fn n133359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131999);
	s.is_empty()
}

/// The word хирон.
pub(crate) fn n63467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62810);
	s.is_empty()
}

/// Words ending -йрон.
pub(crate) fn n13361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13288);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34711);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62812);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 62813);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89744);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -крон.
pub(crate) fn n13362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13289);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34712);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34713);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орон.
pub(crate) fn n13363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13290);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62816);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 62817);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89745);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110316);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 123825);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132000);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62818);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89746);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110317);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123826);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34715);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62819);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 34716);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62820);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89748);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110318);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123827);
											s.is_empty()
										}
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
					super::push(out, 62821);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89749);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110319);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123828);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34717);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 34718);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прон.
pub(crate) fn n13364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13291);
	s.is_empty()
}

/// Words ending -ррон.
pub(crate) fn n13365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13292);
	s.is_empty()
}

/// Words ending -трон.
pub(crate) fn n13366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13293);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34719);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 62823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урон.
pub(crate) fn n13367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13294);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 34721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрон.
pub(crate) fn n13368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13295);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34722);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хрон.
pub(crate) fn n13369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13296);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34723);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 34724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эрон.
pub(crate) fn n13370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13297);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34725);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 34726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ричардсон.
pub(crate) fn n133361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132001);
	s.is_empty()
}

/// Words ending -бансон.
pub(crate) fn n63505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62848);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 89766);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 110327);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 123831);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инсон.
pub(crate) fn n35087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34748);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 62851);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 89767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110328);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 123832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132003);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онсон.
pub(crate) fn n35088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34749);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 62852);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 89768);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 110329);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 123833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осон.
pub(crate) fn n13380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13307);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 34750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34751);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62854);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110330);
									s.is_empty()
								}
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
			super::push(out, 34752);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62855);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псон.
pub(crate) fn n13381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13308);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34753);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсон.
pub(crate) fn n13382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13309);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34754);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 62857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34755);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62858);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89772);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110331);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 110332);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 123834);
											s.is_empty()
										}
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
					super::push(out, 62859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34756);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62860);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссон.
pub(crate) fn n13383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13310);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34757);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62861);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 62862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 34758);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 34759);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тсон.
pub(crate) fn n13384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13311);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34760);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62865);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усон.
pub(crate) fn n13385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13312);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34761);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62866);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 89776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34762);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62867);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34763);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62868);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьсон.
pub(crate) fn n13386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13313);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34764);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62869);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89779);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 62870);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясон.
pub(crate) fn n13387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13314);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гипербатон.
pub(crate) fn n133364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132004);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бедлингтон.
pub(crate) fn n133365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132005);
	s.is_empty()
}

/// Words ending -веллингтон.
pub(crate) fn n133366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132006);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -джералдтон.
pub(crate) fn n133367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132007);
	s.is_empty()
}

/// Words ending -цетон.
pub(crate) fn n35118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34779);
	s.is_empty()
}

/// Words ending -ьетон.
pub(crate) fn n35119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34780);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 62880);
			match s {
				[] => true,
				['е', 'ф', '-', s @ ..] => {
					super::push(out, 89795);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 110345);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123842);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 132008);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136840);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n141174(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итон.
pub(crate) fn n13394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13321);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34781);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 34782);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 34783);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62881);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 62882);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34784);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89797);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 89798);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 62883);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 62884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34785);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 34786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йтон.
pub(crate) fn n13395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13322);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34787);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62886);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктон.
pub(crate) fn n13396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13323);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34788);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62887);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 62888);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89800);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 110347);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123843);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132009);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34789);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62889);
					match s {
						[] => true,
						['л', 'п', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110348);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123844);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123845);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132010);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 123846);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132011);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138254(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 123847);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132012);
													s.is_empty()
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 123848);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132013);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 123849);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132014);
													match s {
														[] => true,
														['н', s @ ..] => super::t47::n138255(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 123850);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 132015);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 123851);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132016);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 123852);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132017);
													match s {
														[] => true,
														['ф', s @ ..] => super::t47::n138256(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтон.
pub(crate) fn n13397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13324);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34790);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 34791);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34792);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 62892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34793);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62893);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34794);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 34795);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 62895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 34796);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтон.
pub(crate) fn n13398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13325);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62897);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 62898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34798);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 62899);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 62900);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89802);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 62901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34799);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62902);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89804);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 110350);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123853);
											s.is_empty()
										}
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
					super::push(out, 62903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отон.
pub(crate) fn n13399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13326);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 34800);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62904);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89805);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110351);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123854);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132018);
													match s {
														[] => true,
														['а', s @ ..] => super::t47::n138257(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34801);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птон.
pub(crate) fn n13400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13327);
	match s {
		[] => true,
		['а', '\u{1}', s @ ..] => {
			super::push(out, 34802);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 34803);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 62907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 34804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62908);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89807);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 110352);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 123855);
											s.is_empty()
										}
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
					super::push(out, 62909);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртон.
pub(crate) fn n13401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13328);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34805);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 62910);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 62911);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89809);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110353);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 123856);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132019);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34806);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62912);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 62913);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 89810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 62914);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89811);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110355);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123857);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132020);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62915);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стон.
pub(crate) fn n13402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13329);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34807);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 62916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34808);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 34809);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 34810);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34811);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89813);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 110356);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123858);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34812);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89814);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 89815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110358);
									s.is_empty()
								}
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
			super::push(out, 34813);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62920);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 89816);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110359);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 123859);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132021);
													match s {
														[] => true,
														['а', s @ ..] => super::t47::n138258(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34814);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62921);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89817);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110360);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123860);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 132022);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34815);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 62922);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 89818);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттон.
pub(crate) fn n13403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13330);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34816);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 62923);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утон.
pub(crate) fn n13404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13331);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34817);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 34818);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 62924);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 62925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фтон.
pub(crate) fn n13405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13332);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34820);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62926);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

