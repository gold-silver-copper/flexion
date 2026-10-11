use flexion::Path;

/// Words ending -кируешь.
pub(crate) fn n4596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4433);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10381);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17479);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24000);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17481);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10382);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10383);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17483);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							super::push(out, 24003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17484);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24004);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 24005);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28798);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31735);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10385);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17485);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17486);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28799);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28800);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31736);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 28801);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17487);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10386);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17489);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 24006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10387);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17490);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28803);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28804);
									s.is_empty()
								}
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
			super::push(out, 10388);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лируешь.
pub(crate) fn n4597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4434);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10389);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17492);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17493);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17494);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 17495);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 24008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17496);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28805);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28806);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31737);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10390);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17497);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17498);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17499);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17500);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28807);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28808);
									s.is_empty()
								}
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
			super::push(out, 10391);
			match s {
				[] => true,
				['н', 'о', 'ж', 'о', s @ ..] => {
					super::push(out, 17501);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10392);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10393);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17502);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24012);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31738);
											s.is_empty()
										}
										['ш', s @ ..] => {
											super::push(out, 31739);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24013);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 17503);
					match s {
						[] => true,
						['и', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28810);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28811);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17504);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28813);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28814);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17505);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24014);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28816);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31740);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 31741);
											s.is_empty()
										}
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
					super::push(out, 17506);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10394);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17508);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 17509);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17510);
					match s {
						[] => true,
						['м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28817);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28818);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 24015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17511);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17512);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['н', 'е', 'в', 'о', s @ ..] => {
							super::push(out, 24016);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24018);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24019);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31742);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 17514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10395);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10396);
			match s {
				[] => true,
				['е', 'п', s @ ..] => {
					super::push(out, 17516);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 24021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ц', s @ ..] => {
					super::push(out, 17517);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t10::n10963(s, out),
		['п', s @ ..] => {
			super::push(out, 10398);
			match s {
				[] => true,
				['м', 'е', 'с', 'а', s @ ..] => {
					super::push(out, 17526);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10399);
			match s {
				[] => true,
				['н', 'а', 'р', 'т', 'т', s @ ..] => {
					super::push(out, 17527);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t10::n10966(s, out),
		['ф', s @ ..] => super::t10::n10967(s, out),
		_ => false,
	}
}

/// Words ending -мируешь.
pub(crate) fn n4598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4435);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10402);
			match s {
				[] => true,
				['з', 'ь', 'л', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24055);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'к', 'е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28847);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28848);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 31758);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31759);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32975);
													s.is_empty()
												}
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
								['\u{1}', s @ ..] => {
									super::push(out, 28849);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 28850);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28851);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28852);
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
						['о', s @ ..] => {
							super::push(out, 24057);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 24058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10403);
			s.is_empty()
		}
		['г', 'а', 'р', 'ф', 'а', 'и', 'д', 'а', s @ ..] => {
			super::push(out, 10404);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10405);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10406);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17541);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 17542);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17543);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17544);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24060);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28853);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 24061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10407);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17545);
					match s {
						[] => true,
						['р', 'г', 'о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28855);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28856);
									s.is_empty()
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
							super::push(out, 24062);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28858);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28859);
									s.is_empty()
								}
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
			super::push(out, 10408);
			match s {
				[] => true,
				['р', 'х', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24063);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24065);
							match s {
								[] => true,
								['н', 'а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31760);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 31761);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10409);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24067);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28860);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31762);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28861);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28862);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31763);
											s.is_empty()
										}
										['р', s @ ..] => super::t10::n33673(s, out),
										_ => false,
									}
								}
								['н', s @ ..] => super::t10::n30609(s, out),
								['о', s @ ..] => super::t10::n30610(s, out),
								['с', s @ ..] => super::t10::n30611(s, out),
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
			super::push(out, 10410);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 10411);
			match s {
				[] => true,
				['и', s @ ..] => super::t10::n18536(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t10::n10978(s, out),
		_ => false,
	}
}

/// Words ending -нируешь.
pub(crate) fn n4599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4436);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10413);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17547);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 24068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17548);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24069);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24070);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24071);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28866);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28867);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17549);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28869);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28870);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28871);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31769);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31770);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 24072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28873);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17551);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24074);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24075);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 24076);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31771);
											s.is_empty()
										}
										['к', 'а', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32982);
													match s {
														[] => true,
														['р', s @ ..] => super::t10::n35419(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 32983);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17552);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17553);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24077);
							match s {
								[] => true,
								['о', 'ф', '\u{1}', s @ ..] => {
									super::push(out, 28876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17554);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10414);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10415);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17555);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17556);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 17557);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17558);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17559);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24079);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24080);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28878);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28879);
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31772);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31773);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28880);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28881);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28882);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28883);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31774);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 24081);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28884);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31775);
											s.is_empty()
										}
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
					super::push(out, 17560);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 17561);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24082);
							match s {
								[] => true,
								['н', 'а', 'з', 'и', 'м', 'о', s @ ..] => {
									super::push(out, 28885);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31776);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n10982(s, out),
		['й', s @ ..] => {
			super::push(out, 10417);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10418);
			s.is_empty()
		}
		['о', s @ ..] => super::t10::n10985(s, out),
		['р', s @ ..] => {
			super::push(out, 10420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17585);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17586);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оируешь.
pub(crate) fn n4600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4437);
	match s {
		[] => true,
		['в', 'н', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17588);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пируешь.
pub(crate) fn n4601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10421);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10422);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17590);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17591);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17592);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10423);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10424);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 10425);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10426);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10427);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17593);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24115);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24116);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28919);
									s.is_empty()
								}
								['р', 'е', 'с', 'к', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31790);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32991);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 31791);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32992);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24117);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28921);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 31792);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24118);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17594);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24119);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 28923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17595);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17596);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17597);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10428);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17598);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17599);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28924);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28925);
									s.is_empty()
								}
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
			super::push(out, 10429);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10430);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10431);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 10432);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24122);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рируешь.
pub(crate) fn n4602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4439);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10434);
			match s {
				[] => true,
				['л', 'к', 'е', 'д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24124);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									super::push(out, 28927);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 24126);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17601);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10435);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24128);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28929);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31793);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 24129);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28930);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 31794);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 28931);
									s.is_empty()
								}
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
			super::push(out, 10436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17603);
					match s {
						[] => true,
						['т', 'с', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28932);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17604);
					match s {
						[] => true,
						['н', 'а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28934);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28935);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28936);
									s.is_empty()
								}
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
			super::push(out, 10437);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17605);
					match s {
						[] => true,
						['т', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28937);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28938);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31795);
											s.is_empty()
										}
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
					super::push(out, 17606);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['м', 'и', 'е', s @ ..] => {
									super::push(out, 28939);
									s.is_empty()
								}
								['э', 'е', s @ ..] => {
									super::push(out, 28940);
									s.is_empty()
								}
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
			super::push(out, 10438);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17607);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28941);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17608);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10439);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17610);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17611);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17612);
					match s {
						[] => true,
						['е', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28943);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31796);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17613);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24130);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28945);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28946);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 31797);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31798);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'и', 'л', 'б', s @ ..] => {
					super::push(out, 17614);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17615);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24134);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31799);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31800);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 31801);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24135);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 28947);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 28948);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31802);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24136);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10440);
			match s {
				[] => true,
				['п', 'с', 'н', 'о', s @ ..] => super::t10::n18610(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t10::n11008(s, out),
		['р', s @ ..] => {
			super::push(out, 10442);
			s.is_empty()
		}
		['т', s @ ..] => super::t11::n11010(s, out),
		['у', s @ ..] => super::t11::n11011(s, out),
		['ф', s @ ..] => super::t11::n11012(s, out),
		['э', s @ ..] => {
			super::push(out, 10446);
			s.is_empty()
		}
		['ю', s @ ..] => super::t11::n11014(s, out),
		_ => false,
	}
}

/// Words ending -сируешь.
pub(crate) fn n4603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4440);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10448);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17628);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10449);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10450);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17630);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28956);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 28957);
									s.is_empty()
								}
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
			super::push(out, 10451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17631);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17632);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24156);
							match s {
								[] => true,
								['н', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31807);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 24157);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28958);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31809);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31810);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 31811);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32994);
													match s {
														[] => true,
														['о', s @ ..] => super::t11::n35424(s, out),
														_ => false,
													}
												}
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
							super::push(out, 24158);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17633);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28960);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17634);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24159);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24160);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17635);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28963);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28964);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 28965);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 28966);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28967);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28968);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28969);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31812);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10452);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17636);
					match s {
						[] => true,
						['в', 'а', 'а', s @ ..] => {
							super::push(out, 24161);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24162);
							s.is_empty()
						}
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28971);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31813);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 28972);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28973);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31814);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28974);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31815);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24163);
							match s {
								[] => true,
								['и', 'ф', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31816);
											s.is_empty()
										}
										['о', s @ ..] => super::t11::n33730(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t11::n18635(s, out),
				['о', s @ ..] => super::t11::n18636(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t11::n11020(s, out),
		['р', s @ ..] => super::t11::n11021(s, out),
		['с', s @ ..] => super::t11::n11022(s, out),
		['у', s @ ..] => super::t11::n11023(s, out),
		['ь', s @ ..] => super::t11::n11024(s, out),
		['ю', s @ ..] => super::t11::n11025(s, out),
		_ => false,
	}
}

/// Words ending -тируешь.
pub(crate) fn n4604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4441);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10459);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10460);
			match s {
				[] => true,
				['б', 'е', 'д', 'о', s @ ..] => {
					super::push(out, 17649);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', 'е', 'р', s @ ..] => {
					super::push(out, 17650);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'а', s @ ..] => {
					super::push(out, 17651);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17652);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17653);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 24185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17654);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24186);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24187);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24188);
							match s {
								[] => true,
								['о', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31827);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31828);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 24189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17655);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17656);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10461);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17657);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17658);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17659);
					s.is_empty()
				}
				['ж', 'д', 'ю', 'б', 'а', s @ ..] => {
					super::push(out, 17660);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24191);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									super::push(out, 28995);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 28996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28997);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 28998);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'а', 'о', s @ ..] => {
							super::push(out, 24192);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24193);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 24194);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 24195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17661);
					s.is_empty()
				}
				['м', 'о', 'р', 'п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24196);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17662);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29000);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17663);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24198);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24199);
							match s {
								[] => true,
								['р', 'е', 'т', 'н', 'и', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', 'е', s @ ..] => {
											super::push(out, 31829);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31830);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17664);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 17665);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10462);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17667);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29002);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29003);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 31831);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17668);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29004);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'е', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24202);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17669);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24204);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17670);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 17671);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24206);
							s.is_empty()
						}
						['и', s @ ..] => super::t11::n25648(s, out),
						['ы', s @ ..] => super::t11::n25649(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17672);
					s.is_empty()
				}
				['р', s @ ..] => super::t11::n18676(s, out),
				['ф', s @ ..] => super::t11::n18677(s, out),
				['ц', s @ ..] => super::t11::n18678(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t11::n11030(s, out),
		['к', s @ ..] => super::t11::n11031(s, out),
		['н', s @ ..] => super::t11::n11032(s, out),
		['о', s @ ..] => super::t11::n11033(s, out),
		['п', s @ ..] => super::t11::n11034(s, out),
		['р', s @ ..] => super::t11::n11035(s, out),
		['с', s @ ..] => super::t11::n11036(s, out),
		['т', s @ ..] => super::t11::n11037(s, out),
		['у', s @ ..] => super::t11::n11038(s, out),
		['ь', s @ ..] => super::t11::n11039(s, out),
		['ю', s @ ..] => super::t11::n11040(s, out),
		_ => false,
	}
}

/// Words ending -уируешь.
pub(crate) fn n4605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4442);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10474);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10475);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10476);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10477);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17702);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10479);
			match s {
				[] => true,
				['т', 'с', 'н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24273);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24274);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31876);
											s.is_empty()
										}
										['е', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33013);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 33014);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 33015);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 31877);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10480);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17704);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29068);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29069);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31878);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29070);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31879);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29071);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31880);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29072);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31881);
											s.is_empty()
										}
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
					super::push(out, 17705);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17706);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фируешь.
pub(crate) fn n4606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4443);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10481);
			match s {
				[] => true,
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24276);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 29073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29074);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 29075);
									s.is_empty()
								}
								['л', 'о', 'г', 'с', s @ ..] => {
									super::push(out, 29076);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 29077);
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31882);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 33016);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31883);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 29078);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31884);
											s.is_empty()
										}
										['и', 'л', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33017);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 33018);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 33019);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 31885);
											s.is_empty()
										}
										['о', 'ф', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33020);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 33021);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33022);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 31886);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10482);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10483);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10484);
			match s {
				[] => true,
				['о', 'м', 'о', s @ ..] => {
					super::push(out, 17707);
					match s {
						[] => true,
						['л', 'л', 'а', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 24277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цируешь.
pub(crate) fn n4607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4444);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10485);
			match s {
				[] => true,
				['о', 'р', 'п', 'с', s @ ..] => {
					super::push(out, 17708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10486);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17709);
					match s {
						[] => true,
						['у', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29079);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29080);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31887);
											s.is_empty()
										}
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
					super::push(out, 17710);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17711);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 24278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'т', 'а', 'м', 'а', s @ ..] => {
					super::push(out, 17712);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17713);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['л', 'а', 'в', 'к', 'е', s @ ..] => {
									super::push(out, 29081);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31888);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'а', s @ ..] => {
									super::push(out, 29082);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 31889);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33023);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29083);
									match s {
										[] => true,
										['с', 'а', 'л', 'к', 'с', s @ ..] => {
											super::push(out, 31890);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33024);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', 'р', 'е', s @ ..] => {
											super::push(out, 31891);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33025);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ь', 'л', 'а', 'ф', 'с', s @ ..] => {
											super::push(out, 31892);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 29084);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 29085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'и', 'з', s @ ..] => {
							super::push(out, 24280);
							match s {
								[] => true,
								['е', 'д', 'о', s @ ..] => {
									super::push(out, 29086);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31893);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10487);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17714);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17715);
					match s {
						[] => true,
						['р', 'е', 'ф', 'ф', 'и', 'д', '\u{1}', s @ ..] => {
							super::push(out, 24281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10488);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17716);
					match s {
						[] => true,
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29087);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10489);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24282);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24283);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24284);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29090);
									s.is_empty()
								}
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
			super::push(out, 10490);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -амульчируешь.
pub(crate) fn n4608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4445);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 10491);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шируешь.
pub(crate) fn n4609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4446);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10492);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 17718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10493);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17719);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							super::push(out, 24285);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17720);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17721);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 17722);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							super::push(out, 24287);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29092);
									s.is_empty()
								}
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
			super::push(out, 10494);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10495);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17724);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24288);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29093);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 24289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29095);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29096);
									s.is_empty()
								}
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
			super::push(out, 10496);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17725);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29097);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29098);
									s.is_empty()
								}
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
			super::push(out, 10497);
			match s {
				[] => true,
				['к', 'а', s @ ..] => {
					super::push(out, 17726);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьируешь.
pub(crate) fn n4610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4447);
	match s {
		[] => true,
		['р', 'а', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17728);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оинтервьюируешь.
pub(crate) fn n4611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4448);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 10498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -поразмышляешь.
pub(crate) fn n35010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33026);
	s.is_empty()
}

/// Words ending -напромышляешь.
pub(crate) fn n35011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33027);
	s.is_empty()
}

/// Words ending -попромышляешь.
pub(crate) fn n35012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33028);
	s.is_empty()
}

/// The word заблагорассудишь.
pub(crate) fn n35015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33031);
	s.is_empty()
}

/// Words ending -ваешься.
pub(crate) fn n7700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7356);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13511);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19524);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19525);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19526);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 19527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13512);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19528);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19529);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24904);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24905);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29274);
									s.is_empty()
								}
								['и', 'о', s @ ..] => {
									super::push(out, 29275);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31923);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29276);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31924);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24906);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29277);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 29278);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19530);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24907);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 29280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19531);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29281);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29282);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31925);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 24908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19532);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19533);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19534);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24909);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29283);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 29284);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 24910);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29286);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33034);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33035);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33036);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 29287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19535);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19536);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24911);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24912);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29288);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31926);
											s.is_empty()
										}
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
					super::push(out, 19537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n14286(s, out),
		['о', s @ ..] => {
			super::push(out, 13514);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19553);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24978);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24979);
							match s {
								[] => true,
								['д', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31983);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 31984);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31985);
											s.is_empty()
										}
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
					super::push(out, 19554);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 19555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24980);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24981);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13515);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t11::n14289(s, out),
		_ => false,
	}
}

/// Words ending -гаешься.
pub(crate) fn n7701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7357);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13517);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19568);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 19569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25040);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13518);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25042);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25043);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 25044);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25045);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 25046);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 29463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', s @ ..] => {
							super::push(out, 25047);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25048);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25049);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19571);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25051);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25052);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 25053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19573);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25054);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29466);
									s.is_empty()
								}
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
			super::push(out, 13519);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 19574);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19575);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29467);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29468);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32009);
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
								['б', s @ ..] => {
									super::push(out, 29469);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32010);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32011);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32012);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13520);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19576);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29470);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29471);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 29472);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29473);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32013);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 19577);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19578);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25056);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19579);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13521);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19581);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19582);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25057);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25058);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29477);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29478);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32014);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29479);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32015);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32016);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33076);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 33077);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32017);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 32018);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33078);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t11::n14295(s, out),
		['у', s @ ..] => super::t11::n14296(s, out),
		['ы', s @ ..] => super::t11::n14297(s, out),
		['ю', s @ ..] => {
			super::push(out, 13525);
			match s {
				[] => true,
				['т', 'а', 'м', s @ ..] => super::t11::n20688(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t11::n14299(s, out),
		_ => false,
	}
}

/// Words ending -даешься.
pub(crate) fn n7702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7358);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13527);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19595);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19596);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19597);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25085);
							s.is_empty()
						}
						['с', 'а', s @ ..] => {
							super::push(out, 25086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19599);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29498);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32023);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29499);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32024);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13528);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19601);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13530);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19603);
					match s {
						[] => true,
						['н', 'о', s @ ..] => {
							super::push(out, 25088);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 19604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25089);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25090);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25091);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 29501);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32025);
											s.is_empty()
										}
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
					super::push(out, 19605);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19606);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19607);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19608);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19609);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25092);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25093);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19610);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25095);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25096);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19611);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 19612);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 19613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 13532);
			match s {
				[] => true,
				['а', 'л', 'с', s @ ..] => {
					super::push(out, 19614);
					match s {
						[] => true,
						['а', 'н', 'о', s @ ..] => {
							super::push(out, 25098);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19615);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25099);
							match s {
								[] => true,
								['о', 'р', 'г', 'а', s @ ..] => {
									super::push(out, 29505);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32026);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25101);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29506);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32027);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 25102);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29507);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32028);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19616);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25103);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29508);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32029);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32030);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 32031);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 33079);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 32032);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33080);
													s.is_empty()
												}
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
							super::push(out, 25104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19617);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19618);
					match s {
						[] => true,
						['с', 'е', s @ ..] => {
							super::push(out, 25105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19619);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n14307(s, out),
		['н', s @ ..] => {
			super::push(out, 13535);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19625);
					match s {
						[] => true,
						['л', 'а', s @ ..] => {
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 29510);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 29511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t11::n14309(s, out),
		['с', s @ ..] => super::t11::n14310(s, out),
		['т', s @ ..] => super::t11::n14311(s, out),
		['у', s @ ..] => super::t11::n14312(s, out),
		['ы', s @ ..] => super::t11::n14313(s, out),
		['ю', s @ ..] => super::t11::n14314(s, out),
		['я', s @ ..] => super::t11::n14315(s, out),
		_ => false,
	}
}

/// Words ending -адолжаешься.
pub(crate) fn n14321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13547);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 19645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ожаешься.
pub(crate) fn n14322(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 19646);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19647);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25137);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25138);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19648);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ужаешься.
pub(crate) fn n14323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13548);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 19649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25140);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25141);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25142);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19650);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжаешься.
pub(crate) fn n14324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13549);
	match s {
		[] => true,
		['р', 'а', s @ ..] => {
			super::push(out, 19652);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25144);
					s.is_empty()
				}
				['н', 'а', s @ ..] => {
					super::push(out, 25145);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заешься.
pub(crate) fn n7704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7360);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13550);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19653);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13551);
			match s {
				[] => true,
				['р', 'а', s @ ..] => {
					super::push(out, 19654);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13552);
			match s {
				[] => true,
				['н', 'с', s @ ..] => {
					super::push(out, 19655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13553);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25148);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25149);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29522);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13554);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13555);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25151);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25152);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25153);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29523);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29524);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29525);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 25154);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29527);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 29528);
									s.is_empty()
								}
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
			super::push(out, 13556);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19656);
					match s {
						[] => true,
						['о', 'л', 'б', s @ ..] => {
							super::push(out, 25155);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13557);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 13558);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 13559);
			match s {
				[] => true,
				['т', 'с', 'о', 'с', 'о', s @ ..] => {
					super::push(out, 19659);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каешься.
pub(crate) fn n7705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7361);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19660);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 19661);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19662);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 25158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19663);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25159);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25160);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25162);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29531);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32038);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 32039);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 29532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19666);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19667);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25165);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19668);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 13561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19669);
					match s {
						[] => true,
						['р', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29534);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29535);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29536);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32040);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13562);
			match s {
				[] => true,
				['а', 'г', 'з', s @ ..] => {
					super::push(out, 19670);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'о', 'д', 'з', 'о', s @ ..] => {
					super::push(out, 19671);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 19672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19673);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19674);
					match s {
						[] => true,
						['в', 'з', s @ ..] => {
							super::push(out, 25169);
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 29537);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32041);
											s.is_empty()
										}
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
					super::push(out, 19675);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25170);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19676);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19677);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25171);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29539);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32042);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33081);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'п', s @ ..] => {
											super::push(out, 32043);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33082);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13564);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19679);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25172);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29540);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32044);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25173);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29541);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32045);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 29542);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29543);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29544);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32046);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29545);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29546);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25174);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29548);
									match s {
										[] => true,
										['а', s @ ..] => super::t11::n33969(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t11::n20785(s, out),
				['р', s @ ..] => super::t11::n20786(s, out),
				['с', s @ ..] => super::t11::n20787(s, out),
				['х', s @ ..] => super::t11::n20788(s, out),
				['ч', s @ ..] => super::t11::n20789(s, out),
				['ш', s @ ..] => super::t11::n20790(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t11::n14340(s, out),
		['м', s @ ..] => super::t11::n14341(s, out),
		['н', s @ ..] => super::t11::n14342(s, out),
		['о', s @ ..] => super::t11::n14343(s, out),
		['п', s @ ..] => super::t11::n14344(s, out),
		['р', s @ ..] => super::t11::n14345(s, out),
		['с', s @ ..] => super::t11::n14346(s, out),
		['т', s @ ..] => super::t11::n14347(s, out),
		['у', s @ ..] => super::t11::n14348(s, out),
		['ц', s @ ..] => super::t11::n14349(s, out),
		['ч', s @ ..] => super::t11::n14350(s, out),
		['ш', s @ ..] => super::t11::n14351(s, out),
		['ы', s @ ..] => super::t11::n14352(s, out),
		['ь', s @ ..] => super::t11::n14353(s, out),
		['ю', s @ ..] => super::t11::n14354(s, out),
		['я', s @ ..] => super::t11::n14355(s, out),
		_ => false,
	}
}

/// Words ending -лаешься.
pub(crate) fn n7706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7362);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13581);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13582);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19747);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 25268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13583);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25269);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25270);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25271);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 19749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25272);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25273);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19750);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13584);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13585);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 19751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13586);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 13587);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13588);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13590);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19755);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13591);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маешься.
pub(crate) fn n7707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7363);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13592);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13593);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19758);
					s.is_empty()
				}
				['р', 'т', 'с', '\u{1}', s @ ..] => {
					super::push(out, 19759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13594);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13595);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 19760);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25276);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29640);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32089);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32090);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 29641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 25277);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32091);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32092);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 25278);
							s.is_empty()
						}
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29642);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32093);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 29643);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25279);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29645);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32094);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'п', s @ ..] => {
									super::push(out, 29646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19762);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13596);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13597);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25282);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25283);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25284);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25285);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29647);
									s.is_empty()
								}
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
			super::push(out, 13598);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 13599);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13600);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19765);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25286);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25287);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25288);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25289);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29648);
									s.is_empty()
								}
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
			super::push(out, 13601);
			match s {
				[] => true,
				['д', 'о', 'п', 'о', s @ ..] => {
					super::push(out, 19766);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19767);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наешься.
pub(crate) fn n7708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7364);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13602);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13603);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13604);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13605);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19770);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19771);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 25291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 19772);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19773);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 25292);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25293);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25294);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 29650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25295);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25296);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19774);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19775);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13606);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 19776);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25299);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29652);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32095);
											s.is_empty()
										}
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
					super::push(out, 19777);
					match s {
						[] => true,
						['о', 'п', 'с', s @ ..] => {
							super::push(out, 25300);
							match s {
								[] => true,
								['в', 'а', s @ ..] => {
									super::push(out, 29653);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32096);
											s.is_empty()
										}
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
					super::push(out, 19778);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25301);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25302);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29654);
									s.is_empty()
								}
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
			super::push(out, 13607);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25303);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19780);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13608);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13609);
			match s {
				[] => true,
				['я', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25305);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25306);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13610);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паешься.
pub(crate) fn n7709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7365);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13611);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25308);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25309);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							super::push(out, 25310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19785);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19786);
					match s {
						[] => true,
						['а', 'ц', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29655);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29656);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29657);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29658);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29659);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32097);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', '\u{1}', s @ ..] => {
					super::push(out, 19787);
					s.is_empty()
				}
				['ц', '\u{1}', s @ ..] => {
					super::push(out, 19788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13612);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25311);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29660);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32098);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 25312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29661);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29662);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29663);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 25313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29664);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32101);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 32102);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 29665);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29666);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19789);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 19790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25314);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13613);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19791);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19792);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 19793);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25317);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25318);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29668);
									s.is_empty()
								}
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
			super::push(out, 13614);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19794);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25320);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25321);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25322);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25323);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25324);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25325);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25326);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 25327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19797);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19798);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29672);
									s.is_empty()
								}
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
			super::push(out, 13615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19799);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19800);
					match s {
						[] => true,
						['ч', s @ ..] => super::t11::n26834(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t11::n14391(s, out),
		['ы', s @ ..] => super::t11::n14392(s, out),
		['ю', s @ ..] => {
			super::push(out, 13618);
			match s {
				[] => true,
				['л', s @ ..] => super::t11::n20917(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t11::n14394(s, out),
		_ => false,
	}
}

/// Words ending -раешься.
pub(crate) fn n7710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7366);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13620);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19812);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19813);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 25353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25354);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25355);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25356);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25357);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29699);
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
						['\u{1}', s @ ..] => {
							super::push(out, 25358);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25359);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25360);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 13621);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25362);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25363);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25364);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25365);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19815);
					match s {
						[] => true,
						['з', 'а', s @ ..] => {
							super::push(out, 25366);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 29701);
									s.is_empty()
								}
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
			super::push(out, 13622);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29702);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32120);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 29703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29704);
									match s {
										[] => true,
										['р', 'о', s @ ..] => {
											super::push(out, 32121);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33093);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 29705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25367);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29706);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29707);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29708);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29709);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32122);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32123);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32124);
											s.is_empty()
										}
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
					super::push(out, 19816);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19817);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 19818);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19819);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19820);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25368);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29710);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32125);
											s.is_empty()
										}
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
					super::push(out, 19821);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25369);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25370);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25371);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25372);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32126);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 32127);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 32128);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 29711);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29712);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 29713);
									s.is_empty()
								}
								['о', s @ ..] => super::t11::n31510(s, out),
								['с', s @ ..] => {
									super::push(out, 29714);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29715);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32133);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => super::t11::n31513(s, out),
								['ы', s @ ..] => super::t11::n31514(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => super::t11::n26881(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t11::n14398(s, out),
		['у', s @ ..] => super::t11::n14399(s, out),
		_ => false,
	}
}

/// Words ending -саешься.
pub(crate) fn n7711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7367);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13624);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19824);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19825);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19826);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25378);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29718);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32135);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13625);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19827);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19828);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19829);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25382);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25383);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25384);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13626);
			match s {
				[] => true,
				['о', 'р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25386);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13627);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25388);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25389);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13628);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25391);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19831);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таешься.
pub(crate) fn n7712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7368);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13630);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19833);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29719);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 25395);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29721);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25396);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25397);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25398);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25399);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25400);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25401);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19836);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25403);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19837);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25404);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32136);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32137);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 32138);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33094);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25405);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25406);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25407);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29725);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25408);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25409);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29726);
									s.is_empty()
								}
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
			super::push(out, 13631);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19839);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 25411);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 25412);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29728);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 25413);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25414);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25415);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29731);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32140);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33095);
													s.is_empty()
												}
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
							super::push(out, 25416);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25417);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19840);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 25418);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25419);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25420);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29735);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29736);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19841);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19842);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 19843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13632);
			match s {
				[] => true,
				['в', 'к', s @ ..] => super::t11::n20961(s, out),
				['к', 'с', s @ ..] => super::t11::n20962(s, out),
				['п', s @ ..] => super::t11::n20963(s, out),
				['ч', s @ ..] => super::t11::n20964(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t11::n14409(s, out),
		['о', s @ ..] => super::t11::n14410(s, out),
		['п', s @ ..] => super::t11::n14411(s, out),
		['р', s @ ..] => super::t11::n14412(s, out),
		['с', s @ ..] => super::t11::n14413(s, out),
		['у', s @ ..] => super::t11::n14414(s, out),
		['х', s @ ..] => super::t11::n14415(s, out),
		['ч', s @ ..] => super::t11::n14416(s, out),
		['ы', s @ ..] => super::t11::n14417(s, out),
		['я', s @ ..] => super::t11::n14418(s, out),
		_ => false,
	}
}

/// Words ending -хаешься.
pub(crate) fn n7713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7369);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13643);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13644);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19867);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 19868);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 19869);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19870);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25491);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25492);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 25493);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25494);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25495);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25496);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25497);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25498);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19873);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25500);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29776);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29777);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29778);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13645);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19874);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19875);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19876);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13646);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19877);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19879);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25508);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29780);
									s.is_empty()
								}
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
			super::push(out, 13647);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19880);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19881);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 19882);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', 'у', s @ ..] => {
							super::push(out, 25511);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13648);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19883);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19884);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13649);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19885);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19886);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29781);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29782);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32170);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13651);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25515);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25516);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25517);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19889);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19890);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25520);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25521);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13652);
			match s {
				[] => true,
				['д', s @ ..] => super::t11::n21017(s, out),
				['м', s @ ..] => {
					super::push(out, 19891);
					s.is_empty()
				}
				['п', s @ ..] => super::t11::n21019(s, out),
				['с', s @ ..] => {
					super::push(out, 19893);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t11::n21021(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t11::n14429(s, out),
		['я', s @ ..] => super::t11::n14430(s, out),
		_ => false,
	}
}

/// Words ending -цаешься.
pub(crate) fn n7714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7370);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13655);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чаешься.
pub(crate) fn n7715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7371);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13657);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25540);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25541);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25542);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25543);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25544);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19900);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19901);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19902);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19903);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13658);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 13659);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 19905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13660);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19906);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19907);
					match s {
						[] => true,
						['т', 'с', 'в', 'о', s @ ..] => {
							super::push(out, 25549);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29797);
									s.is_empty()
								}
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
			super::push(out, 13661);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19908);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25550);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25551);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 25552);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32175);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32176);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 25553);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19909);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25555);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25556);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32177);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 33102);
													match s {
														[] => true,
														['а', s @ ..] => super::t11::n35442(s, out),
														['ъ', s @ ..] => super::t11::n35443(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 32178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 25557);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25558);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 29799);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 32179);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 25559);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25560);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 29800);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 25561);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									super::push(out, 29801);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 32181);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25562);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29802);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29803);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32182);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29804);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32183);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 29805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25563);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 29806);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25564);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['к', 'о', 'к', s @ ..] => {
											match s {
												[] => true,
												['а', 'з', s @ ..] => {
													super::push(out, 33103);
													s.is_empty()
												}
												['с', 'и', s @ ..] => {
													super::push(out, 33104);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => super::t11::n34108(s, out),
										_ => false,
									}
								}
								['с', s @ ..] => super::t11::n31616(s, out),
								_ => false,
							}
						}
						['ь', s @ ..] => super::t11::n27078(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t11::n14438(s, out),
		['н', s @ ..] => super::t11::n14439(s, out),
		['о', s @ ..] => super::t11::n14440(s, out),
		['р', s @ ..] => super::t11::n14441(s, out),
		['т', s @ ..] => super::t11::n14442(s, out),
		['у', s @ ..] => super::t11::n14443(s, out),
		['ь', s @ ..] => super::t11::n14444(s, out),
		['ю', s @ ..] => super::t11::n14445(s, out),
		['я', s @ ..] => super::t11::n14446(s, out),
		_ => false,
	}
}

/// Words ending -шаешься.
pub(crate) fn n7716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13671);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13672);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13673);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25577);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25578);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25579);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25581);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25582);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25583);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25584);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25585);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25586);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25587);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19927);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13674);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 13675);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 19929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13676);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19930);
					s.is_empty()
				}
				['р', 'о', s @ ..] => {
					super::push(out, 19931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13677);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13678);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19933);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19934);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19935);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25590);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25591);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25592);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25593);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29819);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'и', s @ ..] => {
							super::push(out, 25594);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19936);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25596);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29821);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29822);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29823);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29824);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 29825);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32188);
											s.is_empty()
										}
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
					super::push(out, 19937);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25598);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 25599);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29826);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19938);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19939);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25600);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25601);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 25602);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32189);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33105);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32190);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33106);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19940);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 13679);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13680);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13681);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щаешься.
pub(crate) fn n7717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7373);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13682);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 25603);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29828);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32191);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25604);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32192);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 32193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13683);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25605);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25606);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29829);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29830);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32194);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29831);
									s.is_empty()
								}
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
				['б', s @ ..] => {
					super::push(out, 19944);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 19945);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25608);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25609);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25610);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 29832);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29833);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32195);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32196);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32197);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33107);
													s.is_empty()
												}
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
							super::push(out, 25611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25612);
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 29834);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 32198);
											s.is_empty()
										}
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
					super::push(out, 19947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13684);
			match s {
				[] => true,
				['х', 'с', 'о', s @ ..] => {
					super::push(out, 19948);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 25613);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19949);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 19950);
					match s {
						[] => true,
						['а', 'з', 'т', s @ ..] => {
							super::push(out, 25614);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29836);
									s.is_empty()
								}
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
			super::push(out, 13685);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19951);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19952);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19953);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19954);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25615);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29837);
									s.is_empty()
								}
								['о', 'п', s @ ..] => {
									super::push(out, 29838);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29839);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29840);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 29841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', 'и', s @ ..] => {
					super::push(out, 19955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13686);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19956);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19957);
					match s {
						[] => true,
						['з', 'о', 'в', 'о', s @ ..] => {
							super::push(out, 25617);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', s @ ..] => {
							super::push(out, 25618);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29843);
									s.is_empty()
								}
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
			super::push(out, 13687);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 13688);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word сеешься.
pub(crate) fn n14526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13740);
	s.is_empty()
}

/// Words ending -осеешься.
pub(crate) fn n14527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13741);
	s.is_empty()
}

/// Words ending -ысеешься.
pub(crate) fn n14528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13742);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теешься.
pub(crate) fn n7743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13743);
			s.is_empty()
		}
		['л', 'е', 'ж', '\u{1}', s @ ..] => {
			super::push(out, 13744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вяжешься.
pub(crate) fn n14545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13759);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20059);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 20060);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 20061);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ряжешься.
pub(crate) fn n14547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13761);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 20063);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 25662);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -восторгнешься.
pub(crate) fn n35092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33108);
	s.is_empty()
}

/// Words ending -енешься.
pub(crate) fn n7777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7433);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13820);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 13821);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знешься.
pub(crate) fn n7779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7435);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13822);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13823);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25700);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 25701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13824);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13825);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25702);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29882);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 25703);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29883);
									s.is_empty()
								}
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
			super::push(out, 13826);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инешься.
pub(crate) fn n7780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7436);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13827);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13828);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25704);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25705);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25706);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29884);
									s.is_empty()
								}
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
			super::push(out, 13829);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20116);
					match s {
						[] => true,
						['р', 'п', 'ы', s @ ..] => {
							super::push(out, 25707);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20117);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20118);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13830);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кнешься.
pub(crate) fn n7781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7437);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13832);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20120);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 20121);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20122);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20123);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13833);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20125);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20127);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20128);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20129);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20130);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 25711);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29886);
									s.is_empty()
								}
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
			super::push(out, 13835);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20131);
					match s {
						[] => true,
						['щ', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29887);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29888);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29889);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32210);
											s.is_empty()
										}
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
					super::push(out, 20132);
					match s {
						[] => true,
						['т', 'ы', s @ ..] => {
							super::push(out, 25712);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29890);
									s.is_empty()
								}
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
			super::push(out, 13836);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13837);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20133);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13838);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20135);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25714);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29891);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29892);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32211);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 25715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20136);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25716);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 25717);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29894);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32212);
											s.is_empty()
										}
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
					super::push(out, 20137);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 25718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20138);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25719);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 32213);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 32214);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33109);
													s.is_empty()
												}
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
							super::push(out, 25720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20139);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20140);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25722);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 29895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25723);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 25724);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29896);
									s.is_empty()
								}
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
			super::push(out, 13839);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20141);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20142);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13840);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 13841);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13842);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20144);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20145);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25726);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25727);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20146);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20147);
					s.is_empty()
				}
				['т', 'о', s @ ..] => {
					super::push(out, 20148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13843);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 13844);
			match s {
				[] => true,
				['т', 'а', s @ ..] => {
					super::push(out, 20149);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мнешься.
pub(crate) fn n7782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13846);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13847);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13848);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13849);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онешься.
pub(crate) fn n7783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7439);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13850);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13851);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25731);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29897);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 25732);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25733);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пнешься.
pub(crate) fn n7784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7440);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20152);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20153);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 20155);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 20156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20157);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20158);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 20159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20160);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20161);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 20162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13852);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20163);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13853);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20165);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29898);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29899);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32215);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13854);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13855);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20166);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20167);
					match s {
						[] => true,
						['о', 'к', 'ы', s @ ..] => {
							super::push(out, 25737);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29900);
									s.is_empty()
								}
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
			super::push(out, 13856);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 13857);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 13858);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнешься.
pub(crate) fn n7785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7441);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13859);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20168);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20169);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25738);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20170);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25739);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29902);
									s.is_empty()
								}
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
			super::push(out, 13860);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13861);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20171);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20172);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20173);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13862);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13863);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25742);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 29903);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25743);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 29904);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 25744);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29905);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снешься.
pub(crate) fn n7786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7442);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13864);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20175);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 20176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13865);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20177);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 25745);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20178);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13866);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20179);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25747);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29907);
									s.is_empty()
								}
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
			super::push(out, 13867);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13868);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13869);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13870);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20180);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 25748);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29908);
									s.is_empty()
								}
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
			super::push(out, 13871);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тнешься.
pub(crate) fn n7787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7443);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13873);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13874);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 20182);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13875);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20183);
					match s {
						[] => true,
						['л', 'х', 'ы', s @ ..] => {
							super::push(out, 25750);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20184);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25751);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20185);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унешься.
pub(crate) fn n7788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7444);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 13876);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13877);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20187);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20188);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20189);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20190);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20191);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20192);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20193);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20194);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25756);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29911);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фнешься.
pub(crate) fn n7789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7445);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 13878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хнешься.
pub(crate) fn n7790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7446);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13879);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20195);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 20196);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20197);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20198);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13880);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20200);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 20201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13881);
			match s {
				[] => true,
				['в', 'ы', s @ ..] => {
					super::push(out, 20202);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'ы', s @ ..] => {
					super::push(out, 20203);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20204);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25760);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29912);
									s.is_empty()
								}
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
			super::push(out, 13882);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20205);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25761);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 25762);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25763);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20206);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20207);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25765);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25766);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20208);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13883);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13884);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20209);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20210);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13885);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20211);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20212);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20213);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25769);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29916);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29917);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32220);
											s.is_empty()
										}
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
					super::push(out, 20214);
					match s {
						[] => true,
						['л', 'у', s @ ..] => {
							super::push(out, 25770);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29918);
									s.is_empty()
								}
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
			super::push(out, 13886);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20215);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13887);
			match s {
				[] => true,
				['р', 'т', 'ы', s @ ..] => {
					super::push(out, 20217);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цнешься.
pub(crate) fn n7791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7447);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чнешься.
pub(crate) fn n7792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7448);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13889);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щнешься.
pub(crate) fn n7793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7449);
	s.is_empty()
}

/// Words ending -ынешься.
pub(crate) fn n7794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7450);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьнешься.
pub(crate) fn n7795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7451);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юнешься.
pub(crate) fn n7796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7452);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13892);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25773);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25774);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25775);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29920);
									s.is_empty()
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
						['с', s @ ..] => {
							super::push(out, 25776);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25777);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25778);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янешься.
pub(crate) fn n7797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7453);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13893);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25779);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25780);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25781);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25782);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25783);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29923);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32221);
											s.is_empty()
										}
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
					super::push(out, 20219);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25784);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25785);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 20220);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25787);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 32222);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33110);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 32223);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33111);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13894);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13895);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20223);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20224);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25788);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29924);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 25789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20225);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20226);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20227);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20228);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20229);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жертвуешься.
pub(crate) fn n21444(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25811);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 25812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ствуешься.
pub(crate) fn n21445(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25813);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 25814);
			match s {
				[] => true,
				['а', 'р', 'д', 'з', '\u{1}', s @ ..] => {
					super::push(out, 29933);
					s.is_empty()
				}
				['у', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32224);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 32225);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 32226);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32227);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25815);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32229);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 32230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29934);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25816);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29936);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32232);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 33113);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 33114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 29937);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 29938);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32234);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 29939);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32235);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32236);
							match s {
								[] => true,
								['т', 'у', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33444);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 33445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 32237);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33117);
									s.is_empty()
								}
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
			super::push(out, 25817);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['и', 'а', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32238);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 25818);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29941);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32240);
							s.is_empty()
						}
						['ш', 'р', 'е', 'в', 'о', 'с', 'у', s @ ..] => {
							super::push(out, 32241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 29942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 25819);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25820);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 25821);
			match s {
				[] => true,
				['е', 'в', 'и', 'р', 'п', 'о', s @ ..] => {
					super::push(out, 29943);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 25822);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 25823);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32243);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33118);
									match s {
										[] => true,
										['е', 'д', 'и', 'в', 'с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33575);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33576);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33577);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 33446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32244);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33119);
									match s {
										[] => true,
										['о', 'д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33578);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33579);
													s.is_empty()
												}
												['у', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гуешься.
pub(crate) fn n7824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7480);
	match s {
		[] => true,
		['з', 'о', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20298);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['п', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25824);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25825);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25827);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13952);
			match s {
				[] => true,
				['е', 'л', 'е', 'п', 'а', s @ ..] => {
					super::push(out, 20300);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['р', 'р', 'а', 'п', 'с', 'о', s @ ..] => {
							super::push(out, 25830);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'и', s @ ..] => {
							super::push(out, 25831);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29946);
									s.is_empty()
								}
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
			super::push(out, 13953);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13954);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25832);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20301);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 13955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дуешься.
pub(crate) fn n7825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7481);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13956);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13957);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20303);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20304);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25834);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25835);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25836);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25837);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25838);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25839);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 13958);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13959);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20308);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25840);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20309);
					match s {
						[] => true,
						['о', 'п', 'с', 'и', 'о', s @ ..] => {
							super::push(out, 25841);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20310);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', 'у', s @ ..] => {
									super::push(out, 29950);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 29951);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 29952);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32246);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29953);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20311);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25842);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 29955);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32247);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33120);
													s.is_empty()
												}
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
							super::push(out, 25843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20312);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13960);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20313);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20314);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20315);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13961);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13962);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13963);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20318);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25846);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 25847);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 29958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20319);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25848);
							match s {
								[] => true,
								['о', 'к', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32248);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32249);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'а', '\u{1}', s @ ..] => {
							super::push(out, 25849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13964);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20320);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25850);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 25851);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25852);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29959);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 29960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 20321);
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29961);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29962);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29964);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29965);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 29966);
									s.is_empty()
								}
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
			super::push(out, 13965);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20322);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20323);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29967);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n21477(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13966);
			s.is_empty()
		}
		['т', s @ ..] => super::t11::n14768(s, out),
		['у', s @ ..] => super::t11::n14769(s, out),
		['ы', s @ ..] => super::t11::n14770(s, out),
		_ => false,
	}
}

/// Words ending -жуешься.
pub(crate) fn n7826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7482);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13970);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13971);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 13972);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13973);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20329);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29974);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25854);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25855);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 13974);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13975);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13976);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13977);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13978);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13979);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13980);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 13981);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13982);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13983);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зуешься.
pub(crate) fn n7827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7483);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13984);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20340);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20341);
					match s {
						[] => true,
						['б', 'о', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 25857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13985);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20342);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25858);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['б', 'р', s @ ..] => {
											super::push(out, 32251);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33121);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', 'е', s @ ..] => {
											super::push(out, 32252);
											match s {
												[] => true,
												['т', 'а', 'м', 'е', s @ ..] => {
													super::push(out, 33122);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 29977);
									s.is_empty()
								}
								['м', 'р', 'о', 'ф', 'а', s @ ..] => {
									super::push(out, 29978);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32253);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['и', 'м', 'и', 'р', 'к', 'е', s @ ..] => {
											super::push(out, 32254);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33123);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'и', 'ц', 'а', 'н', s @ ..] => {
											super::push(out, 32255);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32256);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 32257);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33124);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 32258);
											s.is_empty()
										}
										['у', 'т', 'к', s @ ..] => {
											super::push(out, 32259);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 33125);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', 'т', s @ ..] => {
									super::push(out, 29979);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 32260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'б', 'о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29980);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29981);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32261);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 25859);
							match s {
								[] => true,
								['а', 'т', 'с', 'и', 'р', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32262);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 32263);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33126);
													s.is_empty()
												}
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
							super::push(out, 25860);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29982);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29983);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 29984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'т', 'и', 'р', 'о', s @ ..] => {
					super::push(out, 20344);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25862);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29985);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 29986);
									match s {
										[] => true,
										['р', 'о', s @ ..] => super::t11::n34187(s, out),
										_ => false,
									}
								}
								['и', s @ ..] => super::t11::n31812(s, out),
								['к', s @ ..] => super::t11::n31813(s, out),
								['м', s @ ..] => super::t11::n31814(s, out),
								['р', s @ ..] => super::t11::n31815(s, out),
								_ => false,
							}
						}
						['о', 'л', 'о', 'к', 'е', s @ ..] => super::t11::n27399(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t11::n21499(s, out),
				['т', s @ ..] => super::t11::n21500(s, out),
				['ф', s @ ..] => super::t11::n21501(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t11::n14787(s, out),
		['ь', s @ ..] => super::t11::n14788(s, out),
		_ => false,
	}
}

/// Words ending -куешься.
pub(crate) fn n7828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7484);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13988);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13989);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20348);
					s.is_empty()
				}
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25870);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25871);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25872);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20349);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20350);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25874);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25875);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25876);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25877);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20351);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30007);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30008);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30009);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30010);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32272);
											s.is_empty()
										}
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
					super::push(out, 20352);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13990);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13991);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20353);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13992);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13993);
			match s {
				[] => true,
				['л', 'б', 'у', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25881);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20354);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20355);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'к', '\u{1}', s @ ..] => {
							super::push(out, 25884);
							s.is_empty()
						}
						['к', 'а', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30011);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20356);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13994);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20357);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25887);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25888);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13995);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13996);
			match s {
				[] => true,
				['е', 'з', 'з', s @ ..] => {
					super::push(out, 20359);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ш', 'а', s @ ..] => {
					super::push(out, 20360);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13997);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13998);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25892);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13999);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20362);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20364);
					s.is_empty()
				}
				['с', 'е', 'в', 'з', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25894);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25895);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25896);
							s.is_empty()
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
				['к', s @ ..] => {
					super::push(out, 20365);
					s.is_empty()
				}
				['т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25897);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25898);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14000);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25900);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25901);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20366);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							super::push(out, 25902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14001);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20367);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20368);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 25903);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 25904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20369);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -луешься.
pub(crate) fn n7829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7485);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14002);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25905);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25906);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 20372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25908);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25909);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25910);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 25911);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25912);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 20373);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 20374);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25914);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32273);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32274);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14003);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 20375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25915);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14004);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25917);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25918);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25919);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14005);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14006);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25921);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14007);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14008);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -муешься.
pub(crate) fn n7830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7486);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14009);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20378);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25923);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 25924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', 'и', 'р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20380);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нуешься.
pub(crate) fn n7831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7487);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14011);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25925);
							s.is_empty()
						}
						['о', 'с', 'о', s @ ..] => {
							super::push(out, 25926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14012);
			match s {
				[] => true,
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25927);
							s.is_empty()
						}
						['с', 'а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30018);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32275);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30019);
									s.is_empty()
								}
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
			super::push(out, 14013);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20382);
					match s {
						[] => true,
						['а', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30020);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30021);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30022);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32276);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14014);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20383);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30023);
									match s {
										[] => true,
										['к', 'э', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33129);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33130);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32277);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 32278);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33131);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32279);
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
								['\u{1}', s @ ..] => {
									super::push(out, 30024);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30025);
									s.is_empty()
								}
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
			super::push(out, 14015);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20384);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20385);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20386);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20387);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20388);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30026);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30027);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30028);
									s.is_empty()
								}
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
			super::push(out, 14016);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25929);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25930);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14017);
			match s {
				[] => true,
				['п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25932);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25933);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30029);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 25935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14018);
			match s {
				[] => true,
				['е', 'д', 'а', s @ ..] => {
					super::push(out, 20389);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14019);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20391);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20392);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30031);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30032);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30033);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20393);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пуешься.
pub(crate) fn n7832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7488);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14020);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 14021);
			match s {
				[] => true,
				['а', 'т', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 20394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руешься.
pub(crate) fn n7833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14022);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20395);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25937);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20396);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25939);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 20397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20398);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20399);
					match s {
						[] => true,
						['л', 'а', 'к', 'т', s @ ..] => {
							super::push(out, 25941);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30034);
									s.is_empty()
								}
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
			super::push(out, 14024);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20400);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14025);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20401);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20402);
					match s {
						[] => true,
						['о', 'к', 'а', s @ ..] => {
							super::push(out, 25943);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'у', s @ ..] => {
					super::push(out, 20403);
					match s {
						[] => true,
						['н', 'о', s @ ..] => {
							super::push(out, 25944);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20404);
					match s {
						[] => true,
						['а', 'ф', 'т', s @ ..] => {
							super::push(out, 25945);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n14831(s, out),
		['о', s @ ..] => {
			super::push(out, 14027);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20425);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30214);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30215);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32486);
											s.is_empty()
										}
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
					super::push(out, 20426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14028);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20427);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26059);
							s.is_empty()
						}
						['т', 'т', s @ ..] => {
							super::push(out, 26060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20428);
					match s {
						[] => true,
						['е', 'ц', '\u{1}', s @ ..] => {
							super::push(out, 26061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20429);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26062);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26063);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 30217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 20430);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26064);
							match s {
								[] => true,
								['м', 'ы', s @ ..] => {
									super::push(out, 30218);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32487);
											s.is_empty()
										}
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
					super::push(out, 20431);
					match s {
						[] => true,
						['л', 'и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30219);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30220);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30221);
									s.is_empty()
								}
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
			super::push(out, 14029);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20432);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26065);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26066);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 26067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20434);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							super::push(out, 26068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 20435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14030);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20436);
					match s {
						[] => true,
						['ц', 'о', s @ ..] => {
							super::push(out, 26069);
							s.is_empty()
						}
						['ш', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30222);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30223);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30224);
									s.is_empty()
								}
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
			super::push(out, 14031);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20437);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

