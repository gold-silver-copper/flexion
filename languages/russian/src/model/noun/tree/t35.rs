use flexion::Path;

/// Words ending -чник.
pub(crate) fn n10982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10949);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30735);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58489);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86361);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 108072);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122560);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 86362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 58490);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58491);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 58492);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 86364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 58493);
					match s {
						[] => true,
						['з', 'о', 'н', 'д', 'о', 'к', 'а', 'т', 'е', 'с', 'в', 'е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 86365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58494);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86366);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108073);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122561);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131364);
													s.is_empty()
												}
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
							super::push(out, 86367);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30736);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58496);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86368);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108075);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 122562);
											s.is_empty()
										}
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
					super::push(out, 58497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86369);
							s.is_empty()
						}
						['с', s @ ..] => super::t47::n87292(s, out),
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58498);
					match s {
						[] => true,
						['е', s @ ..] => super::t47::n87293(s, out),
						['р', s @ ..] => super::t47::n87294(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t47::n59111(s, out),
				['ж', s @ ..] => super::t47::n59112(s, out),
				['л', s @ ..] => super::t47::n59113(s, out),
				['м', s @ ..] => super::t47::n59114(s, out),
				['н', s @ ..] => super::t47::n59115(s, out),
				['о', s @ ..] => super::t47::n59116(s, out),
				['п', s @ ..] => super::t47::n59117(s, out),
				['р', s @ ..] => super::t47::n59118(s, out),
				['с', s @ ..] => super::t47::n59119(s, out),
				['т', s @ ..] => super::t47::n59120(s, out),
				['ч', s @ ..] => super::t47::n59121(s, out),
				['ш', s @ ..] => super::t47::n59122(s, out),
				['щ', s @ ..] => super::t47::n59123(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t47::n30959(s, out),
		['л', s @ ..] => super::t47::n30960(s, out),
		['о', s @ ..] => super::t47::n30961(s, out),
		['у', s @ ..] => super::t47::n30962(s, out),
		['ы', s @ ..] => super::t47::n30963(s, out),
		['ь', s @ ..] => super::t47::n30964(s, out),
		['ю', s @ ..] => super::t47::n30965(s, out),
		['я', s @ ..] => super::t47::n30966(s, out),
		_ => false,
	}
}

/// Words ending -шник.
pub(crate) fn n10983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10950);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30745);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58555);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86531);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 108284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 58556);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58557);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86533);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108285);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122712);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131445);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58558);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86534);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 86535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58559);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86536);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 108286);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 108287);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122713);
											s.is_empty()
										}
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
					super::push(out, 58560);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86537);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58561);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86538);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 108289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108290);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 122714);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 108291);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122715);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 86539);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58562);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86540);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58563);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86541);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 86542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 58564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 30746);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58565);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58566);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 86544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 30747);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58567);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 86545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30748);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 86546);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 86547);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86548);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 108295);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 86549);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 86550);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'и', s @ ..] => {
					super::push(out, 58568);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 86551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58569);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86552);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 58570);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t47::n59185(s, out),
				['п', s @ ..] => super::t47::n59186(s, out),
				['р', s @ ..] => {
					super::push(out, 58572);
					match s {
						[] => true,
						['г', s @ ..] => super::t47::n87487(s, out),
						['е', s @ ..] => super::t47::n87488(s, out),
						['о', s @ ..] => {
							super::push(out, 86564);
							s.is_empty()
						}
						['т', s @ ..] => super::t47::n87490(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t47::n59188(s, out),
				['у', s @ ..] => super::t47::n59189(s, out),
				['ч', s @ ..] => super::t47::n59190(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t47::n30971(s, out),
		['о', s @ ..] => super::t47::n30972(s, out),
		['р', 'е', 'в', 'а', s @ ..] => super::t47::n30973(s, out),
		['у', s @ ..] => super::t47::n30974(s, out),
		['ы', s @ ..] => super::t47::n30975(s, out),
		['э', s @ ..] => super::t47::n30976(s, out),
		['ю', s @ ..] => super::t47::n30977(s, out),
		['я', s @ ..] => super::t47::n30978(s, out),
		_ => false,
	}
}

/// Words ending -щник.
pub(crate) fn n10984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10951);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30756);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 30757);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30758);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30759);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58631);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58632);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьник.
pub(crate) fn n10985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10952);
	match s {
		[] => true,
		['л', s @ ..] => super::t47::n30983(s, out),
		_ => false,
	}
}

/// Words ending -юник.
pub(crate) fn n10986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10953);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30761);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58642);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30762);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58643);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яник.
pub(crate) fn n10987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10954);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30763);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58644);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86714);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 58645);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 58646);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86716);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122835);
											s.is_empty()
										}
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
					super::push(out, 58647);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86717);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108454);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122836);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30764);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58648);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 86718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108455);
									s.is_empty()
								}
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
			super::push(out, 30765);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58649);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86719);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108456);
									s.is_empty()
								}
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
			super::push(out, 30766);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58650);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 58651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30767);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58652);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86721);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108457);
									s.is_empty()
								}
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
			super::push(out, 30768);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58653);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86722);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108458);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122837);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131509);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58654);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86723);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58655);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30769);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58657);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30770);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 58658);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 58659);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86726);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108460);
									s.is_empty()
								}
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
			super::push(out, 30771);
			match s {
				[] => true,
				['р', 'о', 'т', '\u{1}', s @ ..] => {
					super::push(out, 58660);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ав-фонарик.
pub(crate) fn n132868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131510);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136583);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 139576);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поребрик.
pub(crate) fn n109630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108475);
	s.is_empty()
}

/// Words ending -ибрик.
pub(crate) fn n31043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30817);
	s.is_empty()
}

/// Words ending -мбрик.
pub(crate) fn n31044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30818);
	s.is_empty()
}

/// Words ending -обрик.
pub(crate) fn n31045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30819);
	s.is_empty()
}

/// Words ending -убрик.
pub(crate) fn n31046(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 58705);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 58706);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86755);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -врик.
pub(crate) fn n11005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10972);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30820);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30821);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30822);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 30823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -грик.
pub(crate) fn n11006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10973);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30824);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58709);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86756);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108476);
									s.is_empty()
								}
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
			super::push(out, 30825);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 58710);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дрик.
pub(crate) fn n11007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10974);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30826);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58711);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30827);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58713);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 58714);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 86758);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30828);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86759);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 58717);
					s.is_empty()
				}
				['и', 'л', s @ ..] => {
					super::push(out, 58718);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86760);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 108479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерик.
pub(crate) fn n11008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10975);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30829);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 30830);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58719);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58720);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86763);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 108480);
									s.is_empty()
								}
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
			super::push(out, 30831);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 58722);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86764);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 108481);
									s.is_empty()
								}
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
			super::push(out, 30832);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58723);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 58724);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 86765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30833);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58725);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 58726);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 58727);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86768);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108483);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122840);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 122841);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30834);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58728);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 86769);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 86770);
							s.is_empty()
						}
						['ж', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108484);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 108485);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122842);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58729);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 58730);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 86771);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108486);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122843);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131511);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 58731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30836);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58732);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 58733);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 86773);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 108487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58734);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86774);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 108488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 86775);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86776);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 58735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 30837);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 58736);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 86777);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирик.
pub(crate) fn n11009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10976);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 30838);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58737);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86778);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108492);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122844);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30839);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 30840);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 30841);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 30842);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30843);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58739);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 30844);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 30845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крик.
pub(crate) fn n11010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10977);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 30846);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 58740);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108493);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122845);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30847);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 58741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30848);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30849);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 30850);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58744);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нрик.
pub(crate) fn n11011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10978);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орик.
pub(crate) fn n11012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10979);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30852);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58745);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 58746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 30853);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 30854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58748);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 58749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30855);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 58750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 30856);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 30857);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58751);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30858);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58752);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 86782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 30859);
			match s {
				[] => true,
				['о', 'т', '\u{1}', s @ ..] => {
					super::push(out, 58753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30860);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 58754);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ррик.
pub(crate) fn n11013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10980);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30861);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30862);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30863);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 58757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трик.
pub(crate) fn n11014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10981);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 30864);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 58758);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86784);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 108494);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122846);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58759);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30865);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 30866);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58762);
					match s {
						[] => true,
						['л', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108495);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 108496);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122847);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131512);
													match s {
														[] => true,
														['н', s @ ..] => super::t47::n137996(s, out),
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
									super::push(out, 108497);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122848);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'т', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 122849);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131513);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 122850);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131514);
													match s {
														[] => true,
														['г', s @ ..] => super::t47::n137997(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30867);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58763);
					match s {
						[] => true,
						['ц', 'о', s @ ..] => {
							super::push(out, 86786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 58764);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30868);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58765);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86788);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 86789);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 86790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 58766);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86791);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 86792);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108499);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122851);
											s.is_empty()
										}
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
					super::push(out, 58767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 58768);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 86794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урик.
pub(crate) fn n11015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10982);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 30869);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 30870);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30871);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 58770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30872);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58771);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 86795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30874);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 30875);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 58773);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86796);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 108501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрик.
pub(crate) fn n11016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10983);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30876);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86797);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108502);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 122852);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30877);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58775);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86798);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108503);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122853);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 131515);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырик.
pub(crate) fn n11017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10984);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 30878);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 30879);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 30880);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрик.
pub(crate) fn n11018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10985);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30881);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58776);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 58777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30882);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярик.
pub(crate) fn n11019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10986);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58778);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -долгоносик.
pub(crate) fn n132874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131516);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 136586);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 139578);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 141337);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 142352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роносик.
pub(crate) fn n87740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86811);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 108511);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 122858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -росик.
pub(crate) fn n31132(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 58797);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86812);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'а', s @ ..] => {
			super::push(out, 58798);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 86813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уесосик.
pub(crate) fn n31133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30899);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 58799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86814);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тосик.
pub(crate) fn n31134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30900);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 58800);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86815);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108514);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псик.
pub(crate) fn n11026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10993);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30901);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 58801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30902);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58802);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 58803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсик.
pub(crate) fn n11027(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30903);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30904);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58805);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссик.
pub(crate) fn n11028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10994);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30905);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58806);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 86816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30906);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58808);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 86818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усик.
pub(crate) fn n11029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10995);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30907);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58809);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86819);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108515);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122860);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30908);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 30909);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58810);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 86820);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108516);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122861);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30910);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58811);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86821);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108517);
									s.is_empty()
								}
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
			super::push(out, 30911);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 58812);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58813);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 58814);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 86823);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58815);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30913);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 58817);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мысик.
pub(crate) fn n31148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30914);
	s.is_empty()
}

/// Words ending -ьсик.
pub(crate) fn n11031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10997);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юсик.
pub(crate) fn n11032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10998);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30916);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -п-солдатик.
pub(crate) fn n132875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131517);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136587);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139579);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 141338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word синематик.
pub(crate) fn n132876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131518);
	s.is_empty()
}

/// Words ending -ратик.
pub(crate) fn n31158(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58833);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86841);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108526);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122865);
									s.is_empty()
								}
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
			super::push(out, 58834);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 58835);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86842);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108527);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122866);
									s.is_empty()
								}
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
			super::push(out, 58836);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86843);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108528);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122867);
									s.is_empty()
								}
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
			super::push(out, 58837);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 58838);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -асатик.
pub(crate) fn n59458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58839);
	s.is_empty()
}

/// Words ending -осатик.
pub(crate) fn n59459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58840);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 86845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -татик.
pub(crate) fn n31160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30925);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 58841);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86846);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 86847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 58842);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86848);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фатик.
pub(crate) fn n31161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30926);
	s.is_empty()
}

/// Words ending -чатик.
pub(crate) fn n31162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30927);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 58843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щатик.
pub(crate) fn n31163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30928);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 58844);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86849);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -втик.
pub(crate) fn n11034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11000);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30929);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 30930);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 58845);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86850);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122868);
											s.is_empty()
										}
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
					super::push(out, 58846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етик.
pub(crate) fn n11035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11001);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58847);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86851);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108531);
									s.is_empty()
								}
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
			super::push(out, 30932);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 58848);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 58850);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86854);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108532);
									match s {
										[] => true,
										['е', 'с', '-', s @ ..] => {
											super::push(out, 122869);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 131519);
													match s {
														[] => true,
														['и', s @ ..] => super::t47::n138000(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30933);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58851);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86855);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122870);
											s.is_empty()
										}
										['э', 'о', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131520);
													s.is_empty()
												}
												['к', s @ ..] => {
													super::push(out, 131521);
													match s {
														[] => true,
														['л', s @ ..] => super::t47::n138001(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58852);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86856);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108533);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122871);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30934);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58853);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 58854);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 86858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30935);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 30936);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58856);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 58857);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86860);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 58858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 30937);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30938);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 58860);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 58861);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 58862);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86862);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108535);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122872);
											s.is_empty()
										}
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
					super::push(out, 58863);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86863);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108536);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122873);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30939);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58864);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86864);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 58865);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 86866);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108538);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 122874);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', 'е', 'л', 'е', s @ ..] => {
							super::push(out, 86867);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108539);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t47::n59486(s, out),
				['ю', s @ ..] => {
					super::push(out, 58867);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86869);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t47::n31175(s, out),
		['т', s @ ..] => super::t47::n31176(s, out),
		_ => false,
	}
}

/// Words ending -итик.
pub(crate) fn n11036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11002);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30942);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 30943);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58874);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86878);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108551);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 122882);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 131525);
													match s {
														[] => true,
														['с', s @ ..] => super::t47::n138003(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 122883);
											s.is_empty()
										}
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
					super::push(out, 58875);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 86879);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 108552);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122884);
											s.is_empty()
										}
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
					super::push(out, 58876);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86880);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 86881);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 108554);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 122885);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30944);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58877);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 58878);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86882);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108555);
									s.is_empty()
								}
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
			super::push(out, 30945);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58879);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 86883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 58880);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86884);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108556);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122886);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131526);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30946);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 58881);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 58882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 30947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 30948);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ктик.
pub(crate) fn n11037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11003);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30949);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86885);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108557);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122887);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131527);
													s.is_empty()
												}
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
							super::push(out, 86886);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 108558);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 122888);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131528);
													match s {
														[] => true,
														['и', s @ ..] => super::t47::n138004(s, out),
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
									super::push(out, 108559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58884);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 58885);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30950);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58886);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 58887);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86888);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108560);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122889);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30951);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58888);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 86889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30952);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 58889);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86890);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108561);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтик.
pub(crate) fn n11038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11004);
	s.is_empty()
}

/// Words ending -мтик.
pub(crate) fn n11039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11005);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30953);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтик.
pub(crate) fn n11040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11006);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58891);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 58892);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 58893);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86891);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 58894);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86892);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108563);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122891);
											s.is_empty()
										}
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
					super::push(out, 58895);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86893);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'ы', s @ ..] => {
					super::push(out, 58896);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 86894);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 108565);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 122892);
											s.is_empty()
										}
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
					super::push(out, 58897);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86895);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58898);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86896);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108567);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122893);
											s.is_empty()
										}
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
					super::push(out, 58899);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86897);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108568);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 122894);
											s.is_empty()
										}
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
					super::push(out, 58900);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 58901);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58902);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 86899);
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 108569);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122895);
											s.is_empty()
										}
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
					super::push(out, 58903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30955);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58904);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86900);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108570);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122896);
											s.is_empty()
										}
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
					super::push(out, 58905);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86901);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108571);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 122897);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131529);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138005(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58906);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86902);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108572);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122898);
											s.is_empty()
										}
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
					super::push(out, 58907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86903);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 108573);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122899);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131530);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138006(s, out),
														_ => false,
													}
												}
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
							super::push(out, 86904);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 86905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 58908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30956);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30957);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 58910);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 58911);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 86906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'л', s @ ..] => {
			super::push(out, 30958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отик.
pub(crate) fn n11041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11007);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58913);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108575);
									s.is_empty()
								}
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
			super::push(out, 30960);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58914);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 58915);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86908);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58916);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86909);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108577);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122900);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30961);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58917);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 58918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 30962);
			s.is_empty()
		}
		['м', 'е', s @ ..] => {
			super::push(out, 30963);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58919);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86911);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108578);
									s.is_empty()
								}
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
			super::push(out, 30964);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 30965);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 30966);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58920);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86912);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 58921);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 58922);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86913);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108580);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122901);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'т', s @ ..] => {
					super::push(out, 58923);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 86914);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108581);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 122902);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птик.
pub(crate) fn n11042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11008);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58924);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 86915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108582);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122903);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 108583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 86916);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108584);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 122904);
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
								['к', s @ ..] => {
									super::push(out, 108585);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 108586);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 122905);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131531);
													match s {
														[] => true,
														['н', s @ ..] => super::t47::n138007(s, out),
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
									super::push(out, 108587);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122906);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131532);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138008(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 58925);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86917);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108588);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 122907);
											s.is_empty()
										}
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
					super::push(out, 58926);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86918);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108589);
									s.is_empty()
								}
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
			super::push(out, 30967);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 58927);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртик.
pub(crate) fn n11043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11009);
	match s {
		[] => true,
		['е', 'ч', s @ ..] => {
			super::push(out, 30968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30969);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 58929);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 58930);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86920);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108590);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122908);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стик.
pub(crate) fn n11044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11010);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 30970);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58931);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86921);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108591);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122909);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30971);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 58932);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86922);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 108592);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122910);
											s.is_empty()
										}
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
					super::push(out, 58933);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86923);
							match s {
								[] => true,
								['л', 'о', 'г', '\u{1}', s @ ..] => {
									super::push(out, 108593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 58934);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86924);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108594);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 122911);
											s.is_empty()
										}
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
					super::push(out, 58935);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86925);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 108595);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122912);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131533);
													s.is_empty()
												}
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
							super::push(out, 86926);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 108596);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122913);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 131534);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 108597);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 58936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 30972);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 58937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86927);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108598);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122914);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30973);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30974);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 58939);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86928);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 58940);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 58941);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86929);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108600);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 108601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 86930);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 108602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 86931);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108603);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122915);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58942);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 58943);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 58944);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58945);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 58946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 30975);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 58947);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 86933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 30976);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 30977);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30978);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 58948);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 86934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 58949);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 58950);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 86936);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 58951);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 58952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 30979);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 30980);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 58953);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 58954);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86938);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108606);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122916);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'х', s @ ..] => {
					super::push(out, 58955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 58956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 30981);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 30982);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 30983);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 58957);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 86940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттик.
pub(crate) fn n11045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11011);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утик.
pub(crate) fn n11046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11012);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 58958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30986);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58959);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86941);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108607);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122917);
											s.is_empty()
										}
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
					super::push(out, 58960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтик.
pub(crate) fn n11047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11013);
	s.is_empty()
}

/// Words ending -хтик.
pub(crate) fn n11048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11014);
	s.is_empty()
}

/// Words ending -штик.
pub(crate) fn n11049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11015);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 30987);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 58961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30988);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 58962);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 86942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытик.
pub(crate) fn n11050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11016);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 30989);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мультик.
pub(crate) fn n11051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11017);
	s.is_empty()
}

/// Words ending -этик.
pub(crate) fn n11052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11018);
	s.is_empty()
}

/// Words ending -ютик.
pub(crate) fn n11053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11019);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30990);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятик.
pub(crate) fn n11054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11020);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 30991);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 30992);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 58963);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ю-брансуик.
pub(crate) fn n132893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131535);
	s.is_empty()
}

/// Words ending -лан-график.
pub(crate) fn n132894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131536);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -счастливчик.
pub(crate) fn n132895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131537);
	s.is_empty()
}

/// Words ending -дчик.
pub(crate) fn n11070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11033);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31019);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59007);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 59008);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86966);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', 'о', s @ ..] => {
									super::push(out, 108622);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 122923);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131538);
													match s {
														[] => true,
														['с', s @ ..] => super::t47::n138010(s, out),
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
									super::push(out, 108623);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 108624);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 122924);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 122925);
											match s {
												[] => true,
												['п', 'о', s @ ..] => {
													super::push(out, 131539);
													match s {
														[] => true,
														['с', s @ ..] => super::t47::n138011(s, out),
														['т', s @ ..] => super::t47::n138012(s, out),
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
									super::push(out, 108625);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 108626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122926);
											s.is_empty()
										}
										['е', 'л', s @ ..] => {
											super::push(out, 122927);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131540);
													match s {
														[] => true,
														['б', s @ ..] => super::t47::n138013(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 122928);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 131541);
													match s {
														[] => true,
														['у', s @ ..] => super::t47::n138014(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 131542);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 131543);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31020);
			match s {
				[] => true,
				['в', s @ ..] => super::t47::n59634(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 31021);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31022);
			s.is_empty()
		}
		['о', s @ ..] => super::t47::n31266(s, out),
		['у', s @ ..] => {
			super::push(out, 31024);
			s.is_empty()
		}
		['я', s @ ..] => super::t47::n31268(s, out),
		_ => false,
	}
}

/// Words ending -глечик.
pub(crate) fn n59640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59016);
	s.is_empty()
}

/// Words ending -плечик.
pub(crate) fn n59641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59017);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86974);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 108637);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нечик.
pub(crate) fn n31270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31026);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 59018);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 86975);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -речик.
pub(crate) fn n31271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31027);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 59019);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 86976);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жчик.
pub(crate) fn n11072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11034);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31028);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 31029);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 59021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зчик.
pub(crate) fn n11073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11035);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31030);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59022);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86977);
							match s {
								[] => true,
								['з', '-', s @ ..] => {
									super::push(out, 108639);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 122939);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131551);
													match s {
														[] => true,
														['ц', s @ ..] => super::t47::n138022(s, out),
														_ => false,
													}
												}
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
							super::push(out, 86978);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108640);
									match s {
										[] => true,
										['а', 'р', '-', s @ ..] => {
											super::push(out, 122940);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131552);
													match s {
														[] => true,
														['о', s @ ..] => super::t47::n138023(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59023);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 59024);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 86979);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108641);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122941);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31031);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59025);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86980);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 86981);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108642);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122942);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31032);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59026);
					match s {
						[] => true,
						['е', 'р', 'е', 'п', 'а', s @ ..] => {
							super::push(out, 86982);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 108643);
									match s {
										[] => true,
										['в', 'а', '\u{1}', s @ ..] => {
											super::push(out, 122943);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31033);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59027);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108644);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 108645);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 122944);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131553);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 131554);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 108646);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122945);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131555);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 108647);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122946);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131556);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138024(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 131557);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => super::t47::n109806(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t47::n31278(s, out),
		_ => false,
	}
}

/// Words ending -ичик.
pub(crate) fn n11074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11036);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31035);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31036);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59030);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 86983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йчик.
pub(crate) fn n11075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11037);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31037);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59031);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86984);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108649);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122948);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86985);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 86986);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108651);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 122949);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131559);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59032);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 59033);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 86988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59034);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86989);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108652);
									s.is_empty()
								}
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
			super::push(out, 31038);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59035);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 59036);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59037);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 59038);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 86990);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108653);
									s.is_empty()
								}
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
				['б', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 86991);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108654);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 108655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 86992);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108656);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 122950);
											s.is_empty()
										}
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
					super::push(out, 59039);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86993);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 59040);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 86994);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59041);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86995);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 108659);
									s.is_empty()
								}
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
				['ж', s @ ..] => {
					super::push(out, 59042);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 86996);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 108660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 59043);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 86997);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 108661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59044);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 86998);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108662);
									s.is_empty()
								}
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
			super::push(out, 31039);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59045);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 86999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мчик.
pub(crate) fn n11076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11038);
	match s {
		[] => true,
		['а', 'с', s @ ..] => {
			super::push(out, 31040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31041);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 59047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 59048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31042);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59049);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 87001);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 59050);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 87002);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108664);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 122951);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 131560);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31043);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 31044);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59052);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87003);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 108665);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122952);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31045);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 31046);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчик.
pub(crate) fn n11077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11039);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31047);
			match s {
				[] => true,
				['б', 'а', 'к', s @ ..] => {
					super::push(out, 59054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'л', s @ ..] => {
					super::push(out, 59055);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87005);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108666);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122953);
											s.is_empty()
										}
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
					super::push(out, 59056);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59057);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 59058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87006);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 108667);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122954);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87007);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108668);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122955);
											s.is_empty()
										}
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
					super::push(out, 59059);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87008);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108669);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122956);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131561);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59060);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108670);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122957);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 131562);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 108671);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122958);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 87009);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108672);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122959);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 131563);
													s.is_empty()
												}
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
							super::push(out, 87010);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 108673);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 122960);
											s.is_empty()
										}
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
					super::push(out, 59061);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 59062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 31048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59063);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31049);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59064);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 59065);
					s.is_empty()
				}
				['д', 'а', s @ ..] => {
					super::push(out, 59066);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87012);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 108674);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122961);
											s.is_empty()
										}
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
					super::push(out, 59067);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 87013);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 108675);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 122962);
											s.is_empty()
										}
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
					super::push(out, 59068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108676);
									s.is_empty()
								}
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
			super::push(out, 31050);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59069);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87015);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108677);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 122963);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131564);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n138025(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59070);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87016);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108678);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 122964);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 131565);
													s.is_empty()
												}
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
							super::push(out, 87017);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108679);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 122965);
											match s {
												[] => true,
												['о', s @ ..] => super::t47::n132924(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t47::n59697(s, out),
				['м', 'д', s @ ..] => super::t47::n59698(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t47::n31297(s, out),
		['у', s @ ..] => super::t47::n31298(s, out),
		['я', 'р', s @ ..] => super::t47::n31299(s, out),
		_ => false,
	}
}

/// Words ending -пчик.
pub(crate) fn n11078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11040);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59092);
					s.is_empty()
				}
				['р', 'а', '\u{1}', s @ ..] => {
					super::push(out, 59093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31054);
			s.is_empty()
		}
		['и', 'т', '\u{1}', s @ ..] => {
			super::push(out, 31055);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59094);
					s.is_empty()
				}
				['л', 'х', s @ ..] => {
					super::push(out, 59095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87044);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 87045);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 31056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рчик.
pub(crate) fn n11079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11041);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59099);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59100);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 87046);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 108702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59101);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87047);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108703);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 122979);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131571);
													s.is_empty()
												}
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
							super::push(out, 87048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108704);
									s.is_empty()
								}
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
				['с', s @ ..] => {
					super::push(out, 59102);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 87049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59103);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87050);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108706);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 122980);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131572);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138028(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['к', s @ ..] => {
					super::push(out, 59104);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87051);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108707);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 122981);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131573);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59105);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87052);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108708);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 122982);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131574);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31057);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59107);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 59108);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 87053);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108709);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 122983);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131575);
													match s {
														[] => true,
														['у', s @ ..] => super::t47::n138029(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'м', s @ ..] => {
			super::push(out, 31058);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', '\u{1}', s @ ..] => {
			super::push(out, 31059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -счик.
pub(crate) fn n11080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11042);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 31060);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31061);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 59111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тчик.
pub(crate) fn n11081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11043);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31062);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59112);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 59113);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 87056);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108710);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 122984);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 108711);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 122985);
											match s {
												[] => true,
												['е', 'п', '-', s @ ..] => {
													super::push(out, 131576);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138030(s, out),
														['ь', s @ ..] => super::t47::n138031(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59114);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 87057);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 108712);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 108713);
									match s {
										[] => true,
										['и', s @ ..] => super::t47::n124267(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => super::t47::n87995(s, out),
						['з', s @ ..] => super::t47::n87996(s, out),
						['о', s @ ..] => {
							super::push(out, 87060);
							match s {
								[] => true,
								['м', s @ ..] => super::t47::n109875(s, out),
								['р', s @ ..] => super::t47::n109876(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t47::n87998(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t47::n87999(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t47::n59743(s, out),
				['м', s @ ..] => super::t47::n59744(s, out),
				['п', s @ ..] => super::t47::n59745(s, out),
				['р', s @ ..] => super::t47::n59746(s, out),
				['т', s @ ..] => super::t47::n59747(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t47::n31315(s, out),
		['и', s @ ..] => super::t47::n31316(s, out),
		['о', s @ ..] => super::t47::n31317(s, out),
		['у', s @ ..] => super::t47::n31318(s, out),
		['ы', s @ ..] => super::t47::n31319(s, out),
		['я', 'с', s @ ..] => super::t47::n31320(s, out),
		_ => false,
	}
}

/// Words ending -учик.
pub(crate) fn n11082(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31069);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 31070);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59137);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фчик.
pub(crate) fn n11083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11044);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59138);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 87087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59139);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108739);
									s.is_empty()
								}
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
			super::push(out, 31071);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 59140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычик.
pub(crate) fn n11084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11045);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 31072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьчик.
pub(crate) fn n11085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11046);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31073);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59141);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87089);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 87090);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 87091);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 87092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108740);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 108741);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123010);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 131592);
													match s {
														[] => true,
														['ь', s @ ..] => super::t47::n138043(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['й', s @ ..] => {
											super::push(out, 123011);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131593);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138044(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 123012);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131594);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138045(s, out),
														_ => false,
													}
												}
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
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 108742);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123013);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131595);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138046(s, out),
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
									super::push(out, 108743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 87093);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 108744);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123014);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131596);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n138047(s, out),
														_ => false,
													}
												}
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
							super::push(out, 87094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87095);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 87096);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108745);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123015);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131597);
													s.is_empty()
												}
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
							super::push(out, 87097);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'д', s @ ..] => {
					super::push(out, 59143);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87098);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108747);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123016);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131598);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n138048(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59144);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87099);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 87100);
							match s {
								[] => true,
								['о', s @ ..] => super::t47::n109908(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючик.
pub(crate) fn n11086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11047);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31074);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59146);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячик.
pub(crate) fn n11087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11048);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 31075);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word погребщик.
pub(crate) fn n132959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131600);
	s.is_empty()
}

/// The word опалубщик.
pub(crate) fn n132960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131601);
	s.is_empty()
}

/// Words ending -вщик.
pub(crate) fn n11098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11059);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31093);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59164);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59165);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87112);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108757);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123025);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131602);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59166);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59167);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87114);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108758);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 123026);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131603);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 131604);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138050(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 131605);
													match s {
														[] => true,
														['в', s @ ..] => super::t47::n138051(s, out),
														['л', s @ ..] => super::t47::n138052(s, out),
														['т', s @ ..] => super::t47::n138053(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 123027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 108759);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123028);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 108760);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 108761);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123029);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131606);
													s.is_empty()
												}
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
							super::push(out, 87115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59168);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87116);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108762);
									match s {
										[] => true,
										['д', s @ ..] => super::t47::n124311(s, out),
										['п', s @ ..] => super::t47::n124312(s, out),
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 108763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t47::n31348(s, out),
		['и', s @ ..] => super::t47::n31349(s, out),
		['о', s @ ..] => super::t47::n31350(s, out),
		['у', s @ ..] => super::t47::n31351(s, out),
		['ы', s @ ..] => super::t47::n31352(s, out),
		['я', s @ ..] => super::t47::n31353(s, out),
		_ => false,
	}
}

/// Words ending -т-забойщик.
pub(crate) fn n133008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131649);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136666);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 139639);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141383);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мщик.
pub(crate) fn n11104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11065);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31106);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31107);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 31108);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59197);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87173);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108847);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123104);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131650);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31109);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 59199);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 59200);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 59201);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87174);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108848);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 108849);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123105);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131651);
													match s {
														[] => true,
														['т', s @ ..] => super::t47::n138080(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31110);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59202);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87175);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108850);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123106);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131652);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n138081(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31111);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 59203);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87176);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108851);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123107);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 87177);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59204);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87178);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108853);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123108);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131653);
													match s {
														[] => true,
														['л', s @ ..] => super::t47::n138082(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59205);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87179);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 108854);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123109);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131654);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n138083(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31112);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59206);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нщик.
pub(crate) fn n11105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11066);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31113);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59207);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87180);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108855);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123110);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131655);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n138084(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31114);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59209);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87181);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108856);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123111);
											s.is_empty()
										}
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
					super::push(out, 59210);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 59211);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87182);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 87183);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59213);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 59214);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87184);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108858);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123112);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131656);
													s.is_empty()
												}
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
							super::push(out, 87185);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108859);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123113);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131657);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59215);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87186);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 108860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123114);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 87187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 59216);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87188);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 87189);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 108862);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123115);
											s.is_empty()
										}
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
					super::push(out, 59217);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87190);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108863);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123116);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131658);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138085(s, out),
														_ => false,
													}
												}
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
							super::push(out, 87191);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 87192);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => super::t47::n88136(s, out),
						['з', s @ ..] => super::t47::n88137(s, out),
						['о', s @ ..] => {
							super::push(out, 87194);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 108869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 87195);
							s.is_empty()
						}
						['у', s @ ..] => super::t47::n88140(s, out),
						['ь', s @ ..] => super::t47::n88141(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t47::n59846(s, out),
				['т', s @ ..] => super::t47::n59847(s, out),
				['ц', s @ ..] => super::t47::n59848(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t47::n31369(s, out),
		['о', s @ ..] => super::t47::n31370(s, out),
		['у', s @ ..] => super::t47::n31371(s, out),
		['я', s @ ..] => super::t47::n31372(s, out),
		_ => false,
	}
}

/// Words ending -пщик.
pub(crate) fn n11106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11067);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31119);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31120);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 31121);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59227);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87207);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 87208);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108877);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87209);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 87210);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 87211);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 87212);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108880);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123124);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31122);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 59228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рщик.
pub(crate) fn n11107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11068);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31123);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59229);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 59230);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 87213);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123125);
											s.is_empty()
										}
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
					super::push(out, 59231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31124);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59232);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87214);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108882);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 59233);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 87216);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 108883);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123127);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 131665);
													match s {
														[] => true,
														['з', s @ ..] => super::t47::n138089(s, out),
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
									super::push(out, 108884);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123128);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 131666);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59234);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 108885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 59235);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87218);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 108886);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 123129);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131667);
													match s {
														[] => true,
														['р', s @ ..] => super::t47::n138090(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59236);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87219);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108887);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123130);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131668);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 59237);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 59238);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 59239);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87221);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 108888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59240);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87222);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 87223);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 108890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 108891);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123131);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 108892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87224);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 87225);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 87226);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 108893);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123132);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131669);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', 'л', s @ ..] => {
					super::push(out, 59241);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87227);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 108894);
									s.is_empty()
								}
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
			super::push(out, 31125);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 59242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t47::n31380(s, out),
		['т', s @ ..] => {
			super::push(out, 31127);
			match s {
				[] => true,
				['о', s @ ..] => super::t47::n59883(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 31128);
			match s {
				[] => true,
				['т', s @ ..] => super::t47::n59884(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тщик.
pub(crate) fn n11108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11069);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 31129);
			match s {
				[] => true,
				['е', 'м', 'а', s @ ..] => {
					super::push(out, 59257);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87249);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 108915);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 123150);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 131678);
													s.is_empty()
												}
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
							super::push(out, 87250);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 108916);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыщик.
pub(crate) fn n11111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11072);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31131);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьщик.
pub(crate) fn n11112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11073);
	match s {
		[] => true,
		['л', s @ ..] => super::t47::n31387(s, out),
		_ => false,
	}
}

/// Words ending -ющик.
pub(crate) fn n11113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11074);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31134);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящик.
pub(crate) fn n11114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11075);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31135);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31136);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59268);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87303);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 108984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 59269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31137);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 59270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -медия-лубок.
pub(crate) fn n138122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136709);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139666);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141403);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -подмалевок.
pub(crate) fn n133083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131724);
	s.is_empty()
}

/// Words ending -кровок.
pub(crate) fn n59980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59351);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 87363);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 109021);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 123227);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оровок.
pub(crate) fn n59981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59352);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 87364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 109022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тровок.
pub(crate) fn n59982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59353);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 87365);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 109023);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совок.
pub(crate) fn n31471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31217);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 59354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывок.
pub(crate) fn n11164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11125);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31218);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59355);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 87366);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109024);
									s.is_empty()
								}
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
			super::push(out, 31219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59356);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 59357);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59358);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перволедок.
pub(crate) fn n133085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131726);
	s.is_empty()
}

/// Words ending -здок.
pub(crate) fn n11171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11132);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31236);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59384);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87384);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109031);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123230);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131727);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59385);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59386);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 87385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 59387);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87386);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109032);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123231);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131728);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31238);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59388);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87387);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109033);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123232);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31239);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59389);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 31240);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59390);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87389);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109034);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123233);
											s.is_empty()
										}
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
					super::push(out, 59391);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109035);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 109036);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123234);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131729);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одок.
pub(crate) fn n11174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11135);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31245);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 31246);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59396);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 87393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 87394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59397);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59398);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87396);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 87397);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59399);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 59400);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87399);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109041);
									s.is_empty()
								}
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
			super::push(out, 31247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59401);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59402);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87400);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 87401);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 87402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59403);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87403);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109044);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123235);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31248);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59404);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87404);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59405);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87406);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 87407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 59406);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87408);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109048);
									s.is_empty()
								}
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
			super::push(out, 31249);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59407);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87409);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109049);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123236);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131730);
													s.is_empty()
												}
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
							super::push(out, 87410);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 109050);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 109051);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123237);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131731);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n138123(s, out),
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
									super::push(out, 109052);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 109053);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123238);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131732);
													s.is_empty()
												}
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
							super::push(out, 87411);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109054);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123239);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131733);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 109055);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 123240);
											s.is_empty()
										}
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
					super::push(out, 59408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 31250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59409);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тдок.
pub(crate) fn n11175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11136);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 31251);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59410);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удок.
pub(crate) fn n11176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11137);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31252);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59411);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 31253);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 31254);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59412);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87415);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109056);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123241);
											s.is_empty()
										}
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
					super::push(out, 59413);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 87416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 31255);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59414);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31256);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59415);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 59416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59417);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59418);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87420);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59419);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87421);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щдок.
pub(crate) fn n11177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11138);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31258);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдок.
pub(crate) fn n11178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11139);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31259);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59421);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87422);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109059);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123242);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131734);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдок.
pub(crate) fn n11179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11140);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31260);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59422);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 87423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядок.
pub(crate) fn n11180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11141);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31261);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59423);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 87424);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123243);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59424);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59425);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ок-пирожок.
pub(crate) fn n133094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131735);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 136711);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 139667);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141404);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 142409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отморозок.
pub(crate) fn n133095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131736);
	s.is_empty()
}

/// Words ending -леш-брелок.
pub(crate) fn n133096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131737);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 136712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жерелок.
pub(crate) fn n88409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87463);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 109079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 123249);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трелок.
pub(crate) fn n60105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59476);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 87464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -селок.
pub(crate) fn n31572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31317);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 59477);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 87465);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 109080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 59478);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87466);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 87467);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 109081);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123250);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131738);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136713);
													match s {
														[] => true,
														['с', s @ ..] => super::t47::n141108(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 87468);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 109082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 59479);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 59480);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 87470);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телок.
pub(crate) fn n31573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31318);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 59481);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 59482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87471);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 109083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 59483);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 87472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 59484);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87473);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 109084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щелок.
pub(crate) fn n31574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31319);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 59485);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 59486);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 87474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илок.
pub(crate) fn n11211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11172);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59487);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 59488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87475);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59489);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 31321);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59490);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59491);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 31322);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 31323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31324);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59493);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59494);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59495);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 59496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31325);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59497);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 59498);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йлок.
pub(crate) fn n11212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11173);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31326);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клок.
pub(crate) fn n11213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11174);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31327);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 31328);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59500);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 87480);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 109087);
									s.is_empty()
								}
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
			super::push(out, 31329);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31330);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллок.
pub(crate) fn n11214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11175);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31331);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59503);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олок.
pub(crate) fn n11215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11176);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31332);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 31333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59505);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59506);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 59507);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87482);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59508);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87483);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109089);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123251);
											s.is_empty()
										}
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
					super::push(out, 59509);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87484);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 87485);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59510);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87486);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 109091);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123252);
											s.is_empty()
										}
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
					super::push(out, 59511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 31334);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31335);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59513);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87487);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 87488);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59515);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 31336);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 59516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87490);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109093);
									s.is_empty()
								}
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
			super::push(out, 31337);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59517);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87491);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59518);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31338);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59519);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 31339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59520);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59521);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 87494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плок.
pub(crate) fn n11216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11177);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 31340);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлок.
pub(crate) fn n11217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11178);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59523);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59524);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87496);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 59525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31342);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 59526);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 87498);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109096);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123253);
											s.is_empty()
										}
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
					super::push(out, 59527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улок.
pub(crate) fn n11218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11179);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31343);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31344);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59529);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87500);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109097);
									s.is_empty()
								}
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
			super::push(out, 31345);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87501);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59531);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 31346);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59532);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 59533);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87503);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 109099);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123254);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131739);
													match s {
														[] => true,
														['л', s @ ..] => super::t47::n138127(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылок.
pub(crate) fn n11219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11180);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31347);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 59534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59535);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 87504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -припаромок.
pub(crate) fn n133099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131740);
	s.is_empty()
}

/// Words ending -едоумок.
pub(crate) fn n88467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87521);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 109108);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ждоумок.
pub(crate) fn n88468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87522);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 109109);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 123257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -румок.
pub(crate) fn n31624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31369);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 59556);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 87523);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сумок.
pub(crate) fn n31625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31370);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 59557);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87524);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 109110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шумок.
pub(crate) fn n31626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31371);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 59558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чмок.
pub(crate) fn n11228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11189);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31372);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 31373);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59559);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ымок.
pub(crate) fn n11229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11190);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 31374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59560);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59561);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87526);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямок.
pub(crate) fn n11230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11191);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 31375);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59562);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трофуганок.
pub(crate) fn n133100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131741);
	s.is_empty()
}

/// Words ending -олуфуганок.
pub(crate) fn n133101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131742);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136715);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -полустанок.
pub(crate) fn n133102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131743);
	s.is_empty()
}

/// Words ending -енок.
pub(crate) fn n11233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11194);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31385);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59577);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109121);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 109122);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 123261);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 109123);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 109124);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123262);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131744);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138129(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31386);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59578);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59579);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 59580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 31387);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59581);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 59582);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 87545);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87546);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109127);
									s.is_empty()
								}
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
			super::push(out, 31388);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 31389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59584);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59585);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87548);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87549);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 59586);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 59587);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 59588);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87551);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109130);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123263);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131745);
													match s {
														[] => true,
														['п', s @ ..] => super::t47::n138130(s, out),
														_ => false,
													}
												}
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
							super::push(out, 87552);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 87553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 59589);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 59590);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87554);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 59591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87555);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 87556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59592);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87557);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 87558);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109132);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123264);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131746);
													s.is_empty()
												}
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
							super::push(out, 87559);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 87560);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 59593);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 59594);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87561);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109134);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59596);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87563);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109135);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 123266);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131747);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31390);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 59597);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87564);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109136);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123267);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131748);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t47::n31646(s, out),
		['п', s @ ..] => super::t47::n31647(s, out),
		['р', s @ ..] => super::t47::n31648(s, out),
		['с', s @ ..] => super::t47::n31649(s, out),
		['т', s @ ..] => super::t47::n31650(s, out),
		['ф', s @ ..] => {
			super::push(out, 31396);
			s.is_empty()
		}
		['ц', s @ ..] => super::t47::n31652(s, out),
		['ч', s @ ..] => super::t47::n31653(s, out),
		['щ', s @ ..] => super::t47::n31654(s, out),
		['ь', s @ ..] => {
			super::push(out, 31400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инок.
pub(crate) fn n11234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11195);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31401);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 31402);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59627);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87590);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59628);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87591);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109157);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123276);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31403);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59629);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87592);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109158);
									s.is_empty()
								}
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
			super::push(out, 31404);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59630);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31405);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 59633);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 87595);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 59634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87596);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 87597);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109160);
									s.is_empty()
								}
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
			super::push(out, 31406);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59635);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31408);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59637);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59638);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31409);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59639);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 31410);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59640);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 31411);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онок.
pub(crate) fn n11236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11197);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31413);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59643);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 31414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 31415);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 59645);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87605);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109161);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123277);
											s.is_empty()
										}
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
					super::push(out, 59646);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87606);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109162);
									s.is_empty()
								}
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
			super::push(out, 31416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59647);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87607);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109163);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123278);
											s.is_empty()
										}
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
					super::push(out, 59648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 31417);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59649);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87608);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109164);
									s.is_empty()
								}
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
			super::push(out, 31418);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59650);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31419);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59651);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87610);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109165);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 123279);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31420);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59652);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87611);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109166);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123280);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31421);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 31422);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87613);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 109167);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123281);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131753);
													s.is_empty()
												}
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
							super::push(out, 87614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 59654);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87615);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109168);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123282);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131754);
													match s {
														[] => true,
														['м', s @ ..] => super::t47::n138132(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59655);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59656);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87616);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59657);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 59658);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 59659);
					match s {
						[] => true,
						['л', 'а', s @ ..] => {
							super::push(out, 87617);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109170);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123283);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 109171);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123284);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31423);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59660);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унок.
pub(crate) fn n11238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11199);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31425);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59664);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 59665);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 31426);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59666);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 59667);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87622);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 109174);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123285);
											s.is_empty()
										}
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
					super::push(out, 59668);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 87623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 31427);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59669);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 87624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 31428);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59670);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 59671);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59672);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87627);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 59673);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87628);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 59674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87629);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 109177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59675);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87630);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109178);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123286);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131755);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n138133(s, out),
														_ => false,
													}
												}
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
							super::push(out, 87631);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109179);
									s.is_empty()
								}
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
			super::push(out, 31430);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59676);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 59677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31431);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59678);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59679);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87634);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109180);
									s.is_empty()
								}
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
			super::push(out, 31432);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59680);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 87635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59681);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87636);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109181);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123287);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 131756);
													match s {
														[] => true,
														['к', s @ ..] => super::t47::n138134(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59682);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109182);
									s.is_empty()
								}
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
			super::push(out, 31433);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59683);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87638);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109183);
									s.is_empty()
								}
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
			super::push(out, 31434);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59684);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87639);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынок.
pub(crate) fn n11239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11200);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31435);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59685);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109185);
									s.is_empty()
								}
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
			super::push(out, 31436);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 59686);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59687);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87641);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109186);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 123288);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 131757);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n138135(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59688);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59689);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнок.
pub(crate) fn n11240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11201);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31439);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59691);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31440);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59692);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 31441);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59693);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перестарок.
pub(crate) fn n133117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131758);
	s.is_empty()
}

/// Words ending -ги-пемброк.
pub(crate) fn n133118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131759);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 136723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дерок.
pub(crate) fn n31733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31478);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59728);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 87668);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 109199);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123293);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131760);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мерок.
pub(crate) fn n31734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31479);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 59729);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 87669);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 109200);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 123294);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131761);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 59730);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 59731);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 87671);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 109202);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 123295);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131762);
											s.is_empty()
										}
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
					super::push(out, 87672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 59732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стерок.
pub(crate) fn n60364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59734);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черок.
pub(crate) fn n31736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31481);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирок.
pub(crate) fn n11254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11215);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 31482);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 31483);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 31484);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 31485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59737);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орок.
pub(crate) fn n11255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11216);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31486);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 31487);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 31488);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59738);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 59739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87674);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 87675);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 87676);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 87677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59740);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109203);
									s.is_empty()
								}
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
			super::push(out, 31489);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 59741);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 59742);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87680);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59743);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 31490);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59744);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31491);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 31492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59746);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 59747);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 59748);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 59749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31493);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59750);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87686);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 59751);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 87687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59752);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87688);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109206);
									s.is_empty()
								}
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
			super::push(out, 31494);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 59754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 59755);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87691);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109207);
									s.is_empty()
								}
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
			super::push(out, 31495);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59756);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87692);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 109208);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123296);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31496);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59757);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 59758);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 87693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31497);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59759);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 87694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 31498);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59760);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87695);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109209);
									s.is_empty()
								}
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
			super::push(out, 31499);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59761);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прок.
pub(crate) fn n11256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11217);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31500);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 31501);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31502);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59763);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -срок.
pub(crate) fn n11257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11218);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31503);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31504);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59764);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87698);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трок.
pub(crate) fn n11258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11219);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31506);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 31507);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 59767);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87699);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109212);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123297);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урок.
pub(crate) fn n11259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11220);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31508);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 31509);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59768);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 87700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 31510);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 59769);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87701);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 59770);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87702);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109214);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123298);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59771);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59772);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87703);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109215);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123299);
											s.is_empty()
										}
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
					super::push(out, 59773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31512);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 59774);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 59775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59777);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59778);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87705);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109216);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123300);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 131763);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрок.
pub(crate) fn n11260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31515);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 59779);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 87706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырок.
pub(crate) fn n11261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11222);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 31516);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 31517);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59781);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 59782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31518);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 31519);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59783);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87709);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109217);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123301);
											s.is_empty()
										}
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
					super::push(out, 59784);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109218);
									s.is_empty()
								}
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
			super::push(out, 31520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 31521);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 31522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрок.
pub(crate) fn n11262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11223);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31523);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 31524);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 31525);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 59788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярок.
pub(crate) fn n11263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11224);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31526);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 59789);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87712);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109219);
									s.is_empty()
								}
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
			super::push(out, 31527);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 59790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ахар-песок.
pub(crate) fn n133123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131764);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осок.
pub(crate) fn n11268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11229);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 31541);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59813);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 59814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31542);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87728);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 87729);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109230);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123307);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 131765);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 109231);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123308);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 87730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59815);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 59816);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 59817);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87732);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59818);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87733);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109233);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123309);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31544);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59819);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 87734);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 87735);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 87736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59820);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 59821);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 59822);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87738);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 59823);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87739);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 59824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ииссок.
pub(crate) fn n60457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59826);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 87741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 109237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усок.
pub(crate) fn n11271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11232);
	match s {
		[] => true,
		['к', 'б', s @ ..] => {
			super::push(out, 31548);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 31549);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 59828);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысок.
pub(crate) fn n11272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11233);
	match s {
		[] => true,
		['м', 'б', s @ ..] => {
			super::push(out, 31550);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59829);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясок.
pub(crate) fn n11273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11234);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31551);
			match s {
				[] => true,
				['п', 'о', s @ ..] => {
					super::push(out, 59830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word трехлеток.
pub(crate) fn n133125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131766);
	s.is_empty()
}

/// Words ending -тырехлеток.
pub(crate) fn n133126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131767);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136725);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 139672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -меток.
pub(crate) fn n31824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31569);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 59854);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 59855);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87760);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неток.
pub(crate) fn n31825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31570);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 59856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87761);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реток.
pub(crate) fn n31826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31571);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59857);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 87762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

