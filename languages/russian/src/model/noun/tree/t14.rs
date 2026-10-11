use flexion::Path;

/// Words ending -дид.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7864);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7865);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26118);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53284);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81586);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104417);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120147);
													match s {
														[] => true,
														['ц', s @ ..] => super::t33::n131308(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7866);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26120);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теид.
pub(crate) fn n7890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7868);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26122);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 26123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жид.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1323);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7869);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7870);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26124);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26125);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 53288);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81587);
									s.is_empty()
								}
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
			super::push(out, 7871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26126);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26127);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зид.
pub(crate) fn n1327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1324);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7872);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26128);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 53290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26129);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 26130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53292);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 26131);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 53293);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81589);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104418);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26132);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 26133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26134);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53294);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81590);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 104419);
											s.is_empty()
										}
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
					super::push(out, 26135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7874);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7875);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26136);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейид.
pub(crate) fn n7899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7876);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 26137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кид.
pub(crate) fn n1329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1326);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 7877);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26138);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53296);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 81591);
									s.is_empty()
								}
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
			super::push(out, 7878);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7879);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7880);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лид.
pub(crate) fn n1330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1327);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7882);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26140);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53297);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81592);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 104420);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 120148);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131309(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7883);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26142);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7884);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26143);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26144);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7885);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26145);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 53301);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81594);
									s.is_empty()
								}
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
			super::push(out, 7886);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26146);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 53302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7887);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26147);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 26148);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 53303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26149);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53304);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 53305);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7888);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мид.
pub(crate) fn n1331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1328);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7889);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53307);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 53308);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81596);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104421);
											s.is_empty()
										}
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
					super::push(out, 26153);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26154);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7890);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7891);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7892);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26157);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53311);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81597);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26158);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нид.
pub(crate) fn n1332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1329);
	match s {
		[] => true,
		['а', 'с', s @ ..] => {
			super::push(out, 7893);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26159);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81598);
									s.is_empty()
								}
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
			super::push(out, 7894);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26160);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 26161);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 26162);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53314);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 81599);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104422);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 53315);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81600);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 104423);
											s.is_empty()
										}
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
					super::push(out, 26163);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'м', 'о', s @ ..] => {
			super::push(out, 7895);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7896);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26165);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 53318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7897);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7898);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26168);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53320);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оид.
pub(crate) fn n1333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1330);
	match s {
		[] => true,
		['б', 'е', 'г', '\u{1}', s @ ..] => {
			super::push(out, 7899);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7900);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7901);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26169);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53321);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81602);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 104424);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'п', s @ ..] => {
			super::push(out, 7902);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26170);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53322);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81603);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104425);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7903);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26171);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53323);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 53324);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26172);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 53325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7904);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 7905);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26173);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 26174);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53326);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81605);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104426);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7906);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', 'р', s @ ..] => {
							super::push(out, 53327);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81606);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 104427);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120149);
													s.is_empty()
												}
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
							super::push(out, 53328);
							s.is_empty()
						}
						['р', 'т', s @ ..] => {
							super::push(out, 53329);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81607);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104428);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120150);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 53331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26175);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26176);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53332);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81609);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104429);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 53333);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81610);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 104430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 53334);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 53335);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81612);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104431);
											s.is_empty()
										}
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
					super::push(out, 26177);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53336);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 81613);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104432);
											s.is_empty()
										}
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
					super::push(out, 26178);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53338);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 53339);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81614);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104433);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 120151);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['а', s @ ..] => super::t33::n53877(s, out),
						['и', s @ ..] => super::t33::n53878(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t33::n7930(s, out),
		['н', s @ ..] => super::t33::n7931(s, out),
		['п', s @ ..] => super::t33::n7932(s, out),
		['р', s @ ..] => super::t33::n7933(s, out),
		['с', s @ ..] => super::t33::n7934(s, out),
		['т', s @ ..] => super::t33::n7935(s, out),
		['х', s @ ..] => super::t33::n7936(s, out),
		['ц', s @ ..] => super::t33::n7937(s, out),
		_ => false,
	}
}

/// Words ending -пид.
pub(crate) fn n1334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1331);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7914);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7915);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7916);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26198);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26199);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53364);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104458);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7917);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26200);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53365);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81651);
									s.is_empty()
								}
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
			super::push(out, 7918);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26201);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53366);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81652);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рид.
pub(crate) fn n1335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1332);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7919);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 26202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7920);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26203);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81653);
									s.is_empty()
								}
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
			super::push(out, 7921);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26204);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26205);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 53368);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81654);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104459);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120165);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7922);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26206);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53369);
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
				['ф', s @ ..] => {
					super::push(out, 26207);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53370);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104460);
											s.is_empty()
										}
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
					super::push(out, 26208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7923);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7924);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26209);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 53371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 26210);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53372);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81656);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104461);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120166);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131315(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7925);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26211);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53373);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81657);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 81658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26212);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26213);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81659);
									s.is_empty()
								}
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
			super::push(out, 7926);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26214);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 7927);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26217);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53379);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81660);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104462);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7928);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сид.
pub(crate) fn n1336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1333);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7929);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7930);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 26219);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7931);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26220);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53382);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81661);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 104463);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7932);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7933);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26222);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53383);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 53384);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81662);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104464);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120167);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тид.
pub(crate) fn n1337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1334);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7934);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7935);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7936);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26224);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26225);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53386);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81663);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104465);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7937);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26226);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 53387);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81664);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 104466);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7938);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26227);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53388);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81665);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уид.
pub(crate) fn n1338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1335);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7939);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фид.
pub(crate) fn n1339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1336);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 7940);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хид.
pub(crate) fn n1340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1337);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7941);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26230);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7942);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цид.
pub(crate) fn n1341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1338);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7943);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7944);
			match s {
				[] => true,
				['б', 'е', 'м', 'а', '\u{1}', s @ ..] => {
					super::push(out, 26232);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шид.
pub(crate) fn n1342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7945);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7946);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26235);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 53391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7948);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щид.
pub(crate) fn n1343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7949);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юид.
pub(crate) fn n1344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1341);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7950);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 26238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ддерсфилд.
pub(crate) fn n121435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 129964);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word зальбанд.
pub(crate) fn n121436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120171);
	s.is_empty()
}

/// Words ending -ермунганд.
pub(crate) fn n121437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120172);
	s.is_empty()
}

/// Words ending -од-айланд.
pub(crate) fn n121438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120173);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 129965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фолькланд.
pub(crate) fn n121439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120174);
	s.is_empty()
}

/// Words ending -елдерланд.
pub(crate) fn n121440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120175);
	s.is_empty()
}

/// Words ending -нанд.
pub(crate) fn n7999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7973);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26264);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 53412);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81686);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104480);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 120176);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранд.
pub(crate) fn n8000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7974);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 26265);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26266);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 53414);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерсанд.
pub(crate) fn n53957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53415);
	s.is_empty()
}

/// Words ending -фанд.
pub(crate) fn n8002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7976);
	s.is_empty()
}

/// Words ending -ханд.
pub(crate) fn n8003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7977);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 26268);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53416);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 53417);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81689);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 104481);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 120177);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енд.
pub(crate) fn n1353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1350);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7978);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 26269);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26270);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53418);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81690);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104482);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 120178);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131318(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7979);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7980);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 26271);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7981);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26272);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53420);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81691);
									s.is_empty()
								}
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
			super::push(out, 7982);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 26273);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26274);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53422);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 81692);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 104483);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 120179);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26275);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 26276);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53423);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81693);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 81694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 53424);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81695);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104484);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 120180);
													match s {
														[] => true,
														['ю', s @ ..] => super::t33::n131319(s, out),
														_ => false,
													}
												}
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
							super::push(out, 53425);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26277);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 53426);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81697);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104485);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120181);
													s.is_empty()
												}
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
							super::push(out, 53427);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 81698);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 53428);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81699);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 104487);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 120182);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26278);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53429);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 81700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 53430);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81701);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104488);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 120183);
													match s {
														[] => true,
														['д', s @ ..] => super::t33::n131320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26279);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26280);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 53432);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81702);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 104489);
											s.is_empty()
										}
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
					super::push(out, 26281);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53433);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81703);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104490);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120184);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t33::n26460(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t33::n8010(s, out),
		['т', s @ ..] => {
			super::push(out, 7985);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => super::t33::n8012(s, out),
		_ => false,
	}
}

/// Words ending -инд.
pub(crate) fn n1354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7987);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26287);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 26288);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53441);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81709);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104493);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120187);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7988);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26289);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53442);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81710);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104494);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 120188);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7989);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26290);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26291);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53444);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81711);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йнд.
pub(crate) fn n1355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7990);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онд.
pub(crate) fn n1356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1353);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26292);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53445);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81712);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104496);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120189);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7992);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 26293);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53446);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81713);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104497);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120190);
													match s {
														[] => true,
														['а', s @ ..] => super::t33::n131323(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'е', s @ ..] => {
			super::push(out, 7993);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7994);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 7995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унд.
pub(crate) fn n1357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7996);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26296);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26297);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26298);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26299);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 53449);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81714);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 53450);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81715);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 53451);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81716);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104500);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 120191);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7997);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7998);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26300);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 53452);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81717);
									s.is_empty()
								}
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
			super::push(out, 7999);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 8000);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26301);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53453);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26302);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 53454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26303);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53455);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81719);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 104501);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120192);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8001);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26304);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26305);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8002);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26306);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26307);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53457);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81720);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104502);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 120193);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -энд.
pub(crate) fn n1358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 8003);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26308);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 53458);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81721);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104503);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8004);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 26309);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 53459);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81722);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104504);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120194);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -евод.
pub(crate) fn n8037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8011);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26314);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 53461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 26315);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 53462);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26316);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 26317);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 53463);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 81724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 53464);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 53465);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81725);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 104505);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 120195);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129972);
													match s {
														[] => true,
														['в', s @ ..] => super::t33::n137149(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 26318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53466);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 53467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 26319);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53468);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 53469);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 81727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53470);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81728);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 53471);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 81729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53472);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 53473);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81731);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -звод.
pub(crate) fn n8038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8012);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 53474);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 53475);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81732);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104508);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120196);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ивод.
pub(crate) fn n8039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8013);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26321);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 53476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81733);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 81734);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 104509);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 104510);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 120197);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 120198);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 120199);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129973);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n137150(s, out),
														_ => false,
													}
												}
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
							super::push(out, 81735);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 104511);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120200);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овод.
pub(crate) fn n8040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26322);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 26323);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53477);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 53478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 26324);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53479);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 26325);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 53480);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53481);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 53482);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 53483);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81739);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 81740);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 81741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 26327);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53484);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 53485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 26328);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53486);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 53487);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 53488);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 53489);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 53490);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 53491);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81744);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 53492);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 53493);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 81745);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 53494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 26329);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53495);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 81746);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 53496);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 81747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 53497);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81748);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 104517);
									s.is_empty()
								}
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
			super::push(out, 26330);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53498);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 81749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 26331);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 53499);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81750);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 104518);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120201);
											s.is_empty()
										}
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
					super::push(out, 53500);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 53501);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81752);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 104519);
									s.is_empty()
								}
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
			super::push(out, 26332);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53502);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 53503);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81753);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 104520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 53504);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 81754);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104521);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 120202);
											s.is_empty()
										}
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
					super::push(out, 53505);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 81755);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104522);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120203);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 129974);
													match s {
														[] => true,
														['т', s @ ..] => super::t33::n137151(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 53506);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81756);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 104523);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t33::n26514(s, out),
		['с', s @ ..] => {
			super::push(out, 26334);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53511);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 53512);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 53513);
					match s {
						[] => true,
						['у', s @ ..] => super::t33::n82610(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 53514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t33::n26516(s, out),
		['х', s @ ..] => super::t33::n26517(s, out),
		_ => false,
	}
}

/// Words ending -твод.
pub(crate) fn n8042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8016);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26337);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53518);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хвод.
pub(crate) fn n8044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8018);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26338);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 53519);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывод.
pub(crate) fn n8045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8019);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26339);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 53520);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 81767);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104534);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 120212);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129979);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 53521);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 53522);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -год.
pub(crate) fn n1361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8020);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 8021);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26340);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53523);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 81770);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104535);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120213);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8022);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26341);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 53524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8023);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26342);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53525);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26343);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53526);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дод.
pub(crate) fn n1362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1359);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8025);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26345);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еод.
pub(crate) fn n1363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1360);
	s.is_empty()
}

/// Words ending -зод.
pub(crate) fn n1364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1361);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8026);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26346);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иод.
pub(crate) fn n1365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1362);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8027);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8028);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26348);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йод.
pub(crate) fn n1366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1363);
	s.is_empty()
}

/// Words ending -код.
pub(crate) fn n1367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1364);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 8029);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8030);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26350);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53528);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 81773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104536);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8031);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лод.
pub(crate) fn n1368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1365);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26351);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26352);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53529);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81774);
									s.is_empty()
								}
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
			super::push(out, 8033);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8034);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53530);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81775);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104537);
											s.is_empty()
										}
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
					super::push(out, 26355);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 26356);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26358);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26359);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26360);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26361);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мод.
pub(crate) fn n1369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1366);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нод.
pub(crate) fn n1370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1367);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -под.
pub(crate) fn n1371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1368);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8038);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8039);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26362);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53534);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26363);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53535);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26364);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53536);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26365);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53537);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81780);
									s.is_empty()
								}
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
			super::push(out, 8040);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26366);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26367);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53539);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81781);
									s.is_empty()
								}
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
			super::push(out, 8041);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26368);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8043);
			match s {
				[] => true,
				['з', 'и', 'р', s @ ..] => {
					super::push(out, 26370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26371);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26372);
					match s {
						[] => true,
						['о', 'м', '\u{1}', s @ ..] => {
							super::push(out, 53542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26373);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 26374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8044);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 8045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -род.
pub(crate) fn n1372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1369);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8046);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 8047);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 8048);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26378);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8049);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26381);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 53545);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81782);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104538);
											s.is_empty()
										}
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
					super::push(out, 26382);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53546);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104539);
											s.is_empty()
										}
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
					super::push(out, 26383);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53547);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81784);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104540);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 53548);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81785);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104541);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 120214);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26384);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53549);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81786);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 104542);
											s.is_empty()
										}
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
					super::push(out, 26385);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53550);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 53551);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81787);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 104543);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120215);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 8050);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26387);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81788);
									s.is_empty()
								}
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
			super::push(out, 8051);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26388);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8052);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26389);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26390);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 26391);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53555);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81789);
									s.is_empty()
								}
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
			super::push(out, 8054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26392);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 53556);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81790);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104544);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8055);
			match s {
				[] => true,
				['г', s @ ..] => super::t33::n26575(s, out),
				['д', s @ ..] => {
					super::push(out, 26394);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53566);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 26395);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53567);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81806);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104556);
											s.is_empty()
										}
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
					super::push(out, 26396);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81807);
									s.is_empty()
								}
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
			super::push(out, 8056);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 26397);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53569);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81808);
									s.is_empty()
								}
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
			super::push(out, 8057);
			match s {
				[] => true,
				['к', s @ ..] => super::t33::n26580(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t33::n8084(s, out),
		['ф', s @ ..] => {
			super::push(out, 8059);
			s.is_empty()
		}
		['ю', s @ ..] => super::t33::n8086(s, out),
		_ => false,
	}
}

/// Words ending -сод.
pub(crate) fn n1373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1370);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тод.
pub(crate) fn n1374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8062);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8063);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8064);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26403);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8065);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26404);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8066);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26405);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 26406);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фод.
pub(crate) fn n1375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1372);
	s.is_empty()
}

/// Words ending -ход.
pub(crate) fn n1376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8067);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 8068);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26407);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53574);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81811);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104557);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120222);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8069);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8070);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8071);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8072);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8073);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26409);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53575);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 53576);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104558);
											s.is_empty()
										}
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
					super::push(out, 26410);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53577);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81814);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104559);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8074);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26411);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8075);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26412);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 26413);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 26414);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53579);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81815);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104560);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120223);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53580);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 53581);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 81816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26416);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 26417);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53582);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81817);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104561);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120224);
													s.is_empty()
												}
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
							super::push(out, 53583);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 81818);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104562);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 53584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26418);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 26419);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26420);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53585);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81819);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104563);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120225);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26421);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26422);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53586);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81820);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104564);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8076);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8077);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8078);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53587);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 53588);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -форвард.
pub(crate) fn n82674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81826);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 104566);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 104567);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120226);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 129980);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 135748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гард.
pub(crate) fn n8107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8081);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26430);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 26431);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53596);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 53597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26432);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 53598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26433);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 53599);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81829);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104568);
									s.is_empty()
								}
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
			super::push(out, 26434);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53600);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 26435);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53601);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26436);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 53602);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81832);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104569);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120227);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26437);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53603);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81833);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104570);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 120228);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129981);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n137156(s, out),
														_ => false,
													}
												}
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
							super::push(out, 81834);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104571);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120229);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26438);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26439);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53605);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 53606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 26440);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 53607);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иард.
pub(crate) fn n8108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8082);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26441);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 53608);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81839);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104572);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 104573);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 104574);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 104575);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120230);
											s.is_empty()
										}
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
					super::push(out, 53609);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81840);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 53610);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 81841);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104577);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 104578);
									s.is_empty()
								}
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
			super::push(out, 26442);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 53611);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кард.
pub(crate) fn n8109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8083);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 26443);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 53612);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81842);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 104579);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120231);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 129982);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 53613);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81843);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104580);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120232);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26444);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53614);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 26445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26446);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лард.
pub(crate) fn n8110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8084);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26447);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53616);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нард.
pub(crate) fn n8111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8085);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26448);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53617);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 26449);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53618);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81847);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пард.
pub(crate) fn n8112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8086);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26450);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 53619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26451);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53620);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 26452);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53621);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81849);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тард.
pub(crate) fn n8114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8088);
	match s {
		[] => true,
		['е', 'р', '\u{1}', s @ ..] => {
			super::push(out, 26454);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26455);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53622);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 53623);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81851);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104583);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120233);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26456);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53624);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уард.
pub(crate) fn n8115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8089);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 26457);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 53625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фард.
pub(crate) fn n8116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8090);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26458);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 53626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хард.
pub(crate) fn n8117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8091);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26459);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 53627);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81853);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104584);
									s.is_empty()
								}
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
			super::push(out, 26460);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 26461);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 53629);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81854);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 53630);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81855);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104586);
									s.is_empty()
								}
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
			super::push(out, 26462);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53631);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 53632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чард.
pub(crate) fn n8118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8092);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26463);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шард.
pub(crate) fn n8119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8093);
	s.is_empty()
}

/// Words ending -юард.
pub(crate) fn n8120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8094);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 26464);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 53634);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 81859);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерд.
pub(crate) fn n1378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1375);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26465);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26466);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26467);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81861);
									s.is_empty()
								}
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
			super::push(out, 8096);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26468);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53637);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81862);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104588);
											s.is_empty()
										}
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
					super::push(out, 26469);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53638);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81863);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104589);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8097);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 8099);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 26472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирд.
pub(crate) fn n1379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8100);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8101);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орд.
pub(crate) fn n1380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1377);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8102);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26474);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8103);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8104);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 8105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26476);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 26477);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53641);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81865);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104590);
											s.is_empty()
										}
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
					super::push(out, 26478);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 26479);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53643);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81866);
									s.is_empty()
								}
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
			super::push(out, 8106);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26480);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53644);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81867);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 104591);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8107);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26481);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53645);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81868);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104592);
											s.is_empty()
										}
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
					super::push(out, 26482);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 53646);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81869);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104593);
											s.is_empty()
										}
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
					super::push(out, 26483);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53647);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81870);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104594);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120234);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n131335(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26484);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53648);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26485);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53649);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 26486);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53650);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81873);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104595);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120235);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8108);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урд.
pub(crate) fn n1381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1378);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8109);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8111);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярд.
pub(crate) fn n1382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1379);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8112);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word дрифтвуд.
pub(crate) fn n121501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120236);
	s.is_empty()
}

/// Words ending -руд.
pub(crate) fn n1394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8133);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 8134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26508);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53666);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81883);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104598);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120237);
													match s {
														[] => true,
														['т', s @ ..] => super::t33::n131336(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8135);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 26509);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 53667);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81884);
									s.is_empty()
								}
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
			super::push(out, 8136);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 8137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26511);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 26512);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53668);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26513);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53669);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26514);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53670);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26515);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53671);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81888);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104599);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суд.
pub(crate) fn n1395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8139);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8140);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8141);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26517);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26518);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26519);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81889);
									s.is_empty()
								}
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
			super::push(out, 8144);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26520);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53676);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8145);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26522);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26523);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 8147);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26524);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53679);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тфуд.
pub(crate) fn n8178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8152);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26528);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -худ.
pub(crate) fn n1398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8153);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26530);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -билефельд.
pub(crate) fn n121503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120238);
	s.is_empty()
}

/// Words ending -озенфельд.
pub(crate) fn n121504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120239);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 129985);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word альфельд.
pub(crate) fn n121505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120240);
	s.is_empty()
}

/// Words ending -эгершельд.
pub(crate) fn n121506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120241);
	s.is_empty()
}

/// Words ending -ольд.
pub(crate) fn n8188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8162);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 26544);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53698);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 81901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 26545);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 53699);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81902);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 53700);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81903);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104608);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 81904);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 104609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 53701);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81905);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104610);
									s.is_empty()
								}
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
			super::push(out, 26546);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 53702);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 26547);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53703);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 26548);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53704);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81908);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104611);
									s.is_empty()
								}
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
			super::push(out, 26549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53705);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81909);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 81910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 53706);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26550);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53707);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 53708);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81913);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104612);
									s.is_empty()
								}
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
			super::push(out, 26551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ет-снаряд.
pub(crate) fn n121508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120243);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 129986);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 135751);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 139120);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 141069);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 142209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word аудиоряд.
pub(crate) fn n121509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120244);
	s.is_empty()
}

/// Words ending -або-верде.
pub(crate) fn n121511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120246);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 129987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иглокожее.
pub(crate) fn n121512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120247);
	s.is_empty()
}

/// Words ending -ростейшее.
pub(crate) fn n121513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120248);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 129988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щее.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1443);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8236);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26622);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53761);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81945);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104625);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120249);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26623);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 53762);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81946);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104626);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 120250);
													match s {
														[] => true,
														['ч', s @ ..] => super::t33::n131341(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8238);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26625);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 53763);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 53764);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81948);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104627);
											s.is_empty()
										}
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
					super::push(out, 26626);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 53765);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26628);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53766);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81950);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104628);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120251);
													match s {
														[] => true,
														['б', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => super::t33::n131342(s, out),
														_ => false,
													}
												}
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
							super::push(out, 53767);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81951);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104629);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120252);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131343(s, out),
														_ => false,
													}
												}
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
							super::push(out, 53768);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81952);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104630);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120253);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131344(s, out),
														_ => false,
													}
												}
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
							super::push(out, 53769);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81953);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104631);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 120254);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131345(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26629);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53770);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81954);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104632);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120255);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26630);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53771);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81955);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104633);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 120256);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131346(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t33::n8266(s, out),
		_ => false,
	}
}

/// Words ending --камикадзе.
pub(crate) fn n121525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120260);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 129996);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 135756);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 139121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жоникидзе.
pub(crate) fn n121526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120261);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 129997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -болоньезе.
pub(crate) fn n121527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120262);
	s.is_empty()
}

/// Words ending -тельгейзе.
pub(crate) fn n121528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120263);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 129998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -авославие.
pub(crate) fn n121529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120264);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 129999);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 135757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -течествие.
pub(crate) fn n121530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120265);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 135758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-странствие.
pub(crate) fn n121531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120266);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130001);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 135759);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139122);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рюхоногие.
pub(crate) fn n121532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120267);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 130002);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word меджидие.
pub(crate) fn n121533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120268);
	s.is_empty()
}

/// Words ending -жие.
pub(crate) fn n1466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1462);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8273);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8274);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26659);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 53800);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 53801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8275);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26660);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26661);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53802);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81975);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104649);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120269);
													s.is_empty()
												}
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
							super::push(out, 53803);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81976);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 104650);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 120270);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зие.
pub(crate) fn n1467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1463);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8276);
			match s {
				[] => true,
				['л', 'г', 'о', s @ ..] => {
					super::push(out, 26662);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26663);
					match s {
						[] => true,
						['б', 'о', 'о', 'н', 'з', s @ ..] => {
							super::push(out, 53805);
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 81977);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104651);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 120271);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131355(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кие.
pub(crate) fn n1468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1464);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 8278);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26664);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53806);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81978);
									s.is_empty()
								}
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
			super::push(out, 8279);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 26665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53807);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 53808);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81980);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104652);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120272);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26666);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53809);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81981);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104653);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120273);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26667);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53810);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81982);
									s.is_empty()
								}
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
			super::push(out, 8280);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26668);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53811);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81983);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104654);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лие.
pub(crate) fn n1469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1465);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8281);
			match s {
				[] => true,
				['ч', 'а', 'н', 'о', 'н', s @ ..] => {
					super::push(out, 26669);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53812);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81984);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 104655);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 120274);
													match s {
														[] => true,
														['в', s @ ..] => super::t33::n131356(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8282);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26670);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53813);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81985);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104656);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120275);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8283);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26672);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 53814);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81986);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104657);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8284);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 8285);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8286);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 26673);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81987);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104658);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 120276);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 81988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мие.
pub(crate) fn n1470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1466);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8287);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8288);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26674);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81989);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104659);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 81990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26675);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ние.
pub(crate) fn n1471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1467);
	match s {
		[] => true,
		['а', s @ ..] => super::t33::n8315(s, out),
		['в', s @ ..] => {
			super::push(out, 8290);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26696);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53930);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 82198);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104980);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t33::n8317(s, out),
		['и', s @ ..] => {
			super::push(out, 8292);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8293);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26717);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8294);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26718);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 54020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8295);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26719);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26720);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 54022);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82345);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105151);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8297);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26721);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 54023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8298);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26722);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 54024);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 54025);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 54026);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82348);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 82349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82350);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105152);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 82351);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105153);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120757);
													s.is_empty()
												}
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
							super::push(out, 54027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 54028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 54029);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82354);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105154);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 54030);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82355);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105155);
											s.is_empty()
										}
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
					super::push(out, 26723);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54031);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 54032);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 105156);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 105157);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 105158);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 105159);
											s.is_empty()
										}
										['р', 'а', s @ ..] => {
											super::push(out, 105160);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 120758);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n131720(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 105161);
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t33::n122030(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => super::t33::n54588(s, out),
						['м', s @ ..] => super::t33::n54589(s, out),
						['п', s @ ..] => super::t33::n54590(s, out),
						['р', s @ ..] => super::t33::n54591(s, out),
						['с', s @ ..] => super::t33::n54592(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t33::n26908(s, out),
				['л', s @ ..] => super::t33::n26909(s, out),
				['н', s @ ..] => super::t33::n26910(s, out),
				['о', s @ ..] => super::t33::n26911(s, out),
				['р', s @ ..] => super::t33::n26912(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епие.
pub(crate) fn n8325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8299);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26729);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -опие.
pub(crate) fn n8326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8300);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 26730);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рие.
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1469);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8301);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8302);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26731);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82365);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105166);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120761);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n131721(s, out),
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
									super::push(out, 82366);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 82367);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105167);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120762);
													match s {
														[] => true,
														['р', s @ ..] => super::t33::n131722(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'е', s @ ..] => {
					super::push(out, 26732);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 54044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8303);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26733);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54045);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82368);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 82369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26734);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54046);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сие.
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1470);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8304);
			match s {
				[] => true,
				['л', 'г', 'о', 'с', s @ ..] => {
					super::push(out, 26735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8305);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8306);
			match s {
				[] => true,
				['л', 'о', 'г', 'х', 'е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 54047);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 54048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тие.
pub(crate) fn n1475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1471);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8307);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8308);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54049);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 54050);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 54051);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105168);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 120763);
													match s {
														[] => true,
														['ь', s @ ..] => super::t33::n131723(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', 'е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 120764);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 120765);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 82372);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 82373);
									s.is_empty()
								}
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
			super::push(out, 8309);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26736);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 26737);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 26738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54052);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 54053);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 82374);
									match s {
										[] => true,
										['б', 'о', '-', s @ ..] => {
											super::push(out, 105169);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 120766);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131724(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 26739);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 26740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54055);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 54056);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82375);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105170);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8310);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26741);
					s.is_empty()
				}
				['р', 'е', 'п', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 26742);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26743);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54057);
							match s {
								[] => true,
								['о', 'к', 'с', s @ ..] => {
									super::push(out, 82376);
									s.is_empty()
								}
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
			super::push(out, 8311);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26744);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54058);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82377);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 105171);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8312);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54059);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 54060);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 82378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82379);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105172);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120767);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 82380);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105173);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t33::n54619(s, out),
						['т', s @ ..] => {
							super::push(out, 54061);
							match s {
								[] => true,
								['о', s @ ..] => super::t33::n83257(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t33::n26932(s, out),
				['м', s @ ..] => super::t33::n26933(s, out),
				['р', s @ ..] => super::t33::n26934(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t33::n8339(s, out),
		_ => false,
	}
}

/// Words ending -чие.
pub(crate) fn n1476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1472);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8314);
			match s {
				[] => true,
				['р', 'б', 'з', s @ ..] => {
					super::push(out, 26750);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8315);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8316);
			match s {
				[] => true,
				['л', 'т', s @ ..] => {
					super::push(out, 26751);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54067);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82386);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105176);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 120768);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131725(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8317);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8318);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шие.
pub(crate) fn n1477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1473);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8320);
			match s {
				[] => true,
				['т', 'с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 26752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8321);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54068);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82387);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105177);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120769);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'в', s @ ..] => {
							super::push(out, 54069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щие.
pub(crate) fn n1478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1474);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8322);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 26753);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 54070);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 82388);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 105178);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 120770);
													match s {
														[] => true,
														['д', s @ ..] => super::t33::n131726(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8323);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26754);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54071);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82389);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 105179);
											s.is_empty()
										}
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
					super::push(out, 26755);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54072);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 82390);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 105180);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120771);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8324);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26756);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 54073);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 82391);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105181);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120772);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131727(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54074);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82392);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105182);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120773);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131728(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26757);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 54075);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82393);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105183);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 120774);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n131729(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26758);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 54076);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82394);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оразбийке.
pub(crate) fn n122046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120775);
	s.is_empty()
}

/// Words ending -генацвале.
pub(crate) fn n122047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120776);
	s.is_empty()
}

/// Words ending -а-де-кале.
pub(crate) fn n122048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120777);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 130375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тритикале.
pub(crate) fn n122049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120778);
	s.is_empty()
}

/// Words ending -убспинале.
pub(crate) fn n122050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120779);
	s.is_empty()
}

/// Words ending -темпорале.
pub(crate) fn n122051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120780);
	s.is_empty()
}

/// Words ending -о-мортале.
pub(crate) fn n122052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120781);
	s.is_empty()
}

/// Words ending -абеле.
pub(crate) fn n26970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26783);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 54092);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82409);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 105196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -желе.
pub(crate) fn n8380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8354);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -леле.
pub(crate) fn n8381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8355);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 26785);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54093);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пеле.
pub(crate) fn n8382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реле.
pub(crate) fn n8383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8357);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26787);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26788);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 54095);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теле.
pub(crate) fn n8384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8358);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 26789);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54096);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 54097);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82413);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105197);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120782);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130376);
													match s {
														[] => true,
														['д', s @ ..] => super::t33::n137404(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -целе.
pub(crate) fn n8385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8359);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26790);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 54098);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54099);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82415);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 82416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54100);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82417);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105198);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120783);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зле.
pub(crate) fn n1495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1491);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 8360);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иле.
pub(crate) fn n1496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1492);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8361);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26792);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 54101);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 82418);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105199);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 120784);
													match s {
														[] => true,
														['у', s @ ..] => super::t33::n131732(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8362);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26793);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54102);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 82419);
									s.is_empty()
								}
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
			super::push(out, 8363);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26794);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54103);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82420);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105200);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 120785);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8364);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26795);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54104);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82421);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105201);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 120786);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8365);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26796);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54105);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82422);
									s.is_empty()
								}
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
			super::push(out, 8366);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26797);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54106);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82423);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105202);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120787);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131733(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8367);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26798);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кле.
pub(crate) fn n1497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1493);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8368);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26799);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 54107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8369);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8370);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26801);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лле.
pub(crate) fn n1498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1494);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8371);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26802);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 26803);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54108);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82424);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 105203);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8372);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26804);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 54109);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 82425);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105204);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120788);
													match s {
														[] => true,
														['т', s @ ..] => super::t33::n131734(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8373);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26805);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54110);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82426);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105205);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8374);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26806);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 54111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оле.
pub(crate) fn n1499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1495);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 8375);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26807);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 54112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8376);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26808);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 54113);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82427);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 105206);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 120789);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8377);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 26809);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54114);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82428);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105207);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120790);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131735(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54115);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 82429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8378);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26811);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 54116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26812);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54117);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 82430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сле.
pub(crate) fn n1500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1496);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8379);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26813);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тле.
pub(crate) fn n1501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1497);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8380);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26814);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54118);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 82431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уле.
pub(crate) fn n1502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1498);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8381);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8382);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8383);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26815);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54119);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82432);
									s.is_empty()
								}
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
			super::push(out, 8384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фле.
pub(crate) fn n1503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1499);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8385);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдеиформе.
pub(crate) fn n122062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120791);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сатверсме.
pub(crate) fn n122063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120792);
	s.is_empty()
}

/// Words ending -а-мурмане.
pub(crate) fn n122064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120793);
	match s {
		[] => true,
		['н', '-', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 135995);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139278);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141180);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142275);
											s.is_empty()
										}
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
					super::push(out, 135996);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 139279);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141181);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142276);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -колеогине.
pub(crate) fn n122065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120794);
	s.is_empty()
}

/// Words ending -арине.
pub(crate) fn n27029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26842);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 54143);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 82447);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 105215);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120795);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130382);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135997);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тине.
pub(crate) fn n8440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8414);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26843);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 54145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уине.
pub(crate) fn n8441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8415);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 26844);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 54146);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82448);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йне.
pub(crate) fn n1518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1514);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8416);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26845);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26846);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 54148);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82449);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 105217);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 120796);
													match s {
														[] => true,
														['т', s @ ..] => super::t33::n131739(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8417);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8418);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26848);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кне.
pub(crate) fn n1519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1515);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8419);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26850);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 54150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нне.
pub(crate) fn n1520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1516);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8421);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8422);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26852);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54151);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 82450);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105218);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120797);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оне.
pub(crate) fn n1521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1517);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8423);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26853);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54152);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 82451);
									s.is_empty()
								}
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
			super::push(out, 8424);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 8425);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26855);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54153);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 82452);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105219);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120798);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26856);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 54154);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 82453);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105220);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8427);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26857);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54155);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82454);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105221);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120799);
													match s {
														[] => true,
														['а', s @ ..] => super::t33::n131740(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8428);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26858);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54156);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82455);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105222);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26859);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54157);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 82456);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105223);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120800);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131741(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8430);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26860);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54158);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82457);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105224);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120801);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n131742(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8431);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26861);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 54159);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82458);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 105225);
											s.is_empty()
										}
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
					super::push(out, 26862);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 54160);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82459);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105226);
											s.is_empty()
										}
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
					super::push(out, 26863);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54161);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82460);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 105227);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 120802);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n131743(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8432);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26864);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54162);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 82461);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105228);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120803);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8433);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 26865);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54163);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82462);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рне.
pub(crate) fn n1522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1518);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8434);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26866);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54164);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82463);
									s.is_empty()
								}
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
			super::push(out, 8435);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сне.
pub(crate) fn n1523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1519);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8436);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26868);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 54165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шне.
pub(crate) fn n1524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1520);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8437);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26869);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эне.
pub(crate) fn n1525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1521);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 8438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аведливое.
pub(crate) fn n122075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120804);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 130388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тыльковое.
pub(crate) fn n122076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120805);
	s.is_empty()
}

/// Words ending -лицеровое.
pub(crate) fn n122077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120806);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 130389);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 135999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -голосовое.
pub(crate) fn n122078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120807);
	s.is_empty()
}

/// Words ending -текстовое.
pub(crate) fn n122079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120808);
	s.is_empty()
}

/// Words ending -оногое.
pub(crate) fn n54735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54176);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 82473);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 105238);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 120809);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 130390);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136000);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 139280);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ругое.
pub(crate) fn n27068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26881);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 54177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 82474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жое.
pub(crate) fn n1528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1524);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8444);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 26882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кое.
pub(crate) fn n1529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1525);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8445);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 8446);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26884);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8447);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26885);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54181);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 82475);
									s.is_empty()
								}
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
			super::push(out, 8448);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26886);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 54182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8449);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26887);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54183);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82476);
									s.is_empty()
								}
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
			super::push(out, 8450);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26888);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 54184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8451);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26889);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54185);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82477);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 105239);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120810);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26890);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54186);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 54187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26891);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 54188);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82479);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 105240);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120811);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131747(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26892);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54190);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 82480);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105241);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 120812);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n131748(s, out),
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
									super::push(out, 82481);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 54191);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 82482);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105243);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120813);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 120814);
													match s {
														[] => true,
														['з', s @ ..] => super::t33::n131749(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54192);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 82483);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 105244);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 120815);
													match s {
														[] => true,
														['с', s @ ..] => super::t33::n131750(s, out),
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
									super::push(out, 82484);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105245);
											s.is_empty()
										}
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
					super::push(out, 26894);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54193);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 82485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26895);
					match s {
						[] => true,
						['а', s @ ..] => super::t33::n54753(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => super::t33::n8478(s, out),
		['ь', s @ ..] => super::t33::n8479(s, out),
		_ => false,
	}
}

/// Words ending -лое.
pub(crate) fn n1530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1526);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8454);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26899);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54198);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 82489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26900);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 26901);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54199);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82490);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105250);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8455);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26902);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 26903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8456);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26904);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54200);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 82491);
									s.is_empty()
								}
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
			super::push(out, 8457);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26906);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 54202);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82492);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105251);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 82493);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 105252);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мое.
pub(crate) fn n1531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1527);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26907);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 54203);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 54204);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82494);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 105253);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120818);
													s.is_empty()
												}
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
							super::push(out, 54205);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82495);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105254);
											match s {
												[] => true,
												['с', 'и', '\u{1}', s @ ..] => {
													super::push(out, 120819);
													s.is_empty()
												}
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
							super::push(out, 54206);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 54207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26908);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 54208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8459);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26909);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54209);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 54210);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 82497);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 105255);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120820);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131752(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26910);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54211);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 82498);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 54212);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82499);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105257);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120821);
													match s {
														[] => true,
														['с', s @ ..] => super::t33::n131753(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26911);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54213);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 82500);
									s.is_empty()
								}
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
			super::push(out, 8460);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26912);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54214);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 82501);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105258);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 120822);
													s.is_empty()
												}
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
							super::push(out, 54215);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ное.
pub(crate) fn n1532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1528);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8461);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26913);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54216);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82503);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105259);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8462);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26914);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54217);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 82504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26915);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54218);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82505);
									match s {
										[] => true,
										['ъ', s @ ..] => {
											super::push(out, 105260);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120823);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 82506);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105261);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8463);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26916);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54219);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 82507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26917);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54220);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82508);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 105262);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 54221);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82509);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105263);
											s.is_empty()
										}
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
					super::push(out, 26918);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 54222);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82510);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105264);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 120824);
													s.is_empty()
												}
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
							super::push(out, 54223);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 82511);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105265);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8464);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54224);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82512);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105266);
											s.is_empty()
										}
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
					super::push(out, 26920);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26921);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54226);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82513);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 82514);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 105267);
											match s {
												[] => true,
												['м', 'е', 'з', '\u{1}', s @ ..] => {
													super::push(out, 120825);
													s.is_empty()
												}
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
							super::push(out, 54227);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 54228);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 82516);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 105268);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120826);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26922);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54229);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82517);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 105269);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 82518);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 105270);
											s.is_empty()
										}
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
					super::push(out, 26923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54230);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 82519);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105271);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120827);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n131754(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t33::n8491(s, out),
		['ж', s @ ..] => super::t33::n8492(s, out),
		['з', s @ ..] => super::t33::n8493(s, out),
		['м', s @ ..] => {
			super::push(out, 8468);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26934);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54243);
							match s {
								[] => true,
								['о', s @ ..] => super::t33::n83406(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t33::n8495(s, out),
		['п', s @ ..] => super::t33::n8496(s, out),
		['р', s @ ..] => super::t33::n8497(s, out),
		['с', s @ ..] => super::t33::n8498(s, out),
		['т', s @ ..] => super::t33::n8499(s, out),
		['ч', s @ ..] => super::t33::n8500(s, out),
		['ш', s @ ..] => super::t33::n8501(s, out),
		['щ', s @ ..] => super::t33::n8502(s, out),
		['ь', s @ ..] => super::t33::n8503(s, out),
		_ => false,
	}
}

/// Words ending -пое.
pub(crate) fn n1533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1529);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8478);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26958);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54283);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 82571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рое.
pub(crate) fn n1534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1530);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8479);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26959);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8480);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26960);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54285);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82572);
									s.is_empty()
								}
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
			super::push(out, 8481);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26961);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тое.
pub(crate) fn n1535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1531);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8482);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 26962);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54287);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82573);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105309);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 120851);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131771(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8483);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26963);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54288);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82574);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105310);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120852);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8484);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26964);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54289);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82575);
									s.is_empty()
								}
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
			super::push(out, 8485);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26965);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54290);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 82576);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105311);
											s.is_empty()
										}
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
					super::push(out, 26966);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54291);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 82577);
									s.is_empty()
								}
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
			super::push(out, 8486);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26967);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 54292);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82578);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 105312);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120853);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131772(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хое.
pub(crate) fn n1536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1532);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8487);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26968);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 54293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шое.
pub(crate) fn n1537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1533);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 8488);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26969);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54294);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 82579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкаф-купе.
pub(crate) fn n122125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120854);
	s.is_empty()
}

/// Words ending -лькредере.
pub(crate) fn n122126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120855);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 130417);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 136021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рре.
pub(crate) fn n1553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1549);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8522);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8523);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тре.
pub(crate) fn n1554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1550);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8524);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27004);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уре.
pub(crate) fn n1555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1551);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 8525);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27005);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 54318);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82591);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 105321);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 120856);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131774(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фре.
pub(crate) fn n1556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1552);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8526);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эре.
pub(crate) fn n1557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1553);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8527);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юре.
pub(crate) fn n1558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1554);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8528);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 8529);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 27007);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 54319);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 82592);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105322);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8530);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27008);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -айгеноссе.
pub(crate) fn n122128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120857);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 130419);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черк-эссе.
pub(crate) fn n122129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120858);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130420);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оман-эссе.
pub(crate) fn n122130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120859);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 130421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -екуритате.
pub(crate) fn n122131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120860);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 130422);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дюплисите.
pub(crate) fn n122132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120861);
	s.is_empty()
}

/// Words ending -данте.
pub(crate) fn n27226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27039);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 54341);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82608);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 105330);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120862);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130423);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иканте.
pub(crate) fn n54901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54342);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82609);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енте.
pub(crate) fn n8586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8560);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27041);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54343);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82610);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105332);
									s.is_empty()
								}
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
			super::push(out, 27042);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82611);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онте.
pub(crate) fn n8587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8561);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 27043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оте.
pub(crate) fn n1570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8562);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27044);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54345);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82612);
									s.is_empty()
								}
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
			super::push(out, 8563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рте.
pub(crate) fn n1571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1567);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8564);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27045);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 54346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27046);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8565);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 27047);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54348);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сте.
pub(crate) fn n1572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1568);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8566);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27048);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54349);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 82614);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105334);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 120863);
													match s {
														[] => true,
														['ь', s @ ..] => super::t33::n131780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тте.
pub(crate) fn n1573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1569);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8567);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8568);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 27050);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уте.
pub(crate) fn n1574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1570);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 8569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ште.
pub(crate) fn n1575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1571);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8570);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27051);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54350);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 82615);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105335);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 120864);
													match s {
														[] => true,
														['т', s @ ..] => super::t33::n131781(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27052);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54351);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82616);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105336);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 120865);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьте.
pub(crate) fn n1576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1572);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8571);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27053);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 54352);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82617);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эте.
pub(crate) fn n1577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1573);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8572);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 27054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юте.
pub(crate) fn n1578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1574);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8573);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27055);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чная-кафе.
pub(crate) fn n122137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120866);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130426);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136023);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 139292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ки-галифе.
pub(crate) fn n122138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120867);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 130427);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136024);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --тенерифе.
pub(crate) fn n122139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120868);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 130428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льтр-кофе.
pub(crate) fn n122140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120869);
	s.is_empty()
}

/// Words ending -ваффе.
pub(crate) fn n27252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27065);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 54365);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82625);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 105343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 54366);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 82626);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 105344);
							match s {
								[] => true,
								['л', 'о', s @ ..] => {
									super::push(out, 120870);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130429);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 136025);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n140733(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word кружевце.
pub(crate) fn n122142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120871);
	s.is_empty()
}

/// The word катовице.
pub(crate) fn n122143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120872);
	s.is_empty()
}

/// Words ending -надобьице.
pub(crate) fn n122144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120873);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 130430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 136026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лованьице.
pub(crate) fn n122145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120874);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130431);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 136027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озданьице.
pub(crate) fn n122146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120875);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 130432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенце.
pub(crate) fn n27268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27081);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 54378);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82635);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнце.
pub(crate) fn n8624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8598);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27082);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 54379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онце.
pub(crate) fn n8625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8599);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54380);
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					super::push(out, 54381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 27084);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54382);
					match s {
						[] => true,
						['л', 'о', 'т', s @ ..] => {
							super::push(out, 82636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 54383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рце.
pub(crate) fn n1594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1590);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8600);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27085);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54384);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 82637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тце.
pub(crate) fn n1595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1591);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8601);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 27086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8602);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8603);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27088);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54385);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 82638);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 82639);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 82640);
									s.is_empty()
								}
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
			super::push(out, 8604);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27089);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54386);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьце.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1592);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8605);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27090);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 54387);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 54388);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 82643);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105352);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120876);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131789(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54389);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82644);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 105353);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120877);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131790(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82645);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 82646);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 105354);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 120878);
													s.is_empty()
												}
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
							super::push(out, 54391);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 54392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82648);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 82649);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105355);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 120879);
													s.is_empty()
												}
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
							super::push(out, 54393);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 82650);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 105356);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120880);
													s.is_empty()
												}
												['о', 'г', 'о', 'н', '\u{1}', s @ ..] => {
													super::push(out, 120881);
													s.is_empty()
												}
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
							super::push(out, 54394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82651);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 54395);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 82652);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105358);
											s.is_empty()
										}
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
					super::push(out, 27091);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 54397);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 54398);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82655);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 105359);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 105360);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120882);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n131791(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27092);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54400);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82656);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 105361);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120883);
													s.is_empty()
												}
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
							super::push(out, 54401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27093);
					match s {
						[] => true,
						['к', 'е', 'т', 'с', '\u{1}', s @ ..] => {
							super::push(out, 54402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27094);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54403);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t33::n54963(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t33::n27282(s, out),
				['я', s @ ..] => super::t33::n27283(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ище.
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8629);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27114);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54420);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 82666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 27115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8630);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27116);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54422);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 82667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 27117);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54423);
							match s {
								[] => true,
								['б', 'л', 'о', 'д', '\u{1}', s @ ..] => {
									super::push(out, 82668);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 82669);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105365);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120884);
													match s {
														[] => true,
														['д', s @ ..] => super::t33::n131792(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54424);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 54425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 27118);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27119);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54427);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82671);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105366);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120885);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27120);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27121);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 54429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82672);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 54430);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 82673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 54431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 27122);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 54432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8631);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27123);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 54433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27124);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54434);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82674);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105367);
											s.is_empty()
										}
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
					super::push(out, 27125);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 54435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27126);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 54436);
							match s {
								[] => true,
								['о', s @ ..] => super::t33::n83549(s, out),
								_ => false,
							}
						}
						['д', s @ ..] => super::t33::n54996(s, out),
						['к', s @ ..] => super::t33::n54997(s, out),
						['л', s @ ..] => super::t33::n54998(s, out),
						['м', s @ ..] => super::t33::n54999(s, out),
						['н', s @ ..] => super::t33::n55000(s, out),
						['п', s @ ..] => {
							super::push(out, 54442);
							match s {
								[] => true,
								['р', s @ ..] => super::t33::n83560(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t33::n55002(s, out),
						['с', s @ ..] => super::t33::n55003(s, out),
						['т', s @ ..] => super::t33::n55004(s, out),
						['х', s @ ..] => super::t33::n55005(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t33::n8658(s, out),
		['ж', s @ ..] => super::t33::n8659(s, out),
		['з', s @ ..] => super::t33::n8660(s, out),
		['к', s @ ..] => super::t33::n8661(s, out),
		['л', s @ ..] => super::t33::n8662(s, out),
		['м', s @ ..] => super::t33::n8663(s, out),
		['н', s @ ..] => super::t33::n8664(s, out),
		['о', s @ ..] => super::t33::n8665(s, out),
		['п', s @ ..] => super::t33::n8666(s, out),
		['р', s @ ..] => super::t33::n8667(s, out),
		['с', s @ ..] => super::t33::n8668(s, out),
		['т', s @ ..] => super::t33::n8669(s, out),
		['у', s @ ..] => super::t33::n8670(s, out),
		['ч', s @ ..] => super::t33::n8671(s, out),
		['ш', s @ ..] => super::t33::n8672(s, out),
		['ь', s @ ..] => super::t33::n8673(s, out),
		['я', s @ ..] => super::t33::n8674(s, out),
		_ => false,
	}
}

/// Words ending -сьмизубые.
pub(crate) fn n122212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120941);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олнозубые.
pub(crate) fn n122213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120942);
	s.is_empty()
}

