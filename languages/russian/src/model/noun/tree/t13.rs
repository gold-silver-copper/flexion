use flexion::Path;

/// Words ending -оха.
pub(crate) fn n949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6759);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24463);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51537);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79983);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 103126);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 79984);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103127);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 79985);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 103128);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6760);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24464);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 24465);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 51538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 24466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51539);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79986);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 103129);
											s.is_empty()
										}
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
					super::push(out, 24467);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51540);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79987);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 103130);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 119302);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24468);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 51541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6761);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24469);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51542);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79988);
									s.is_empty()
								}
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
			super::push(out, 6762);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24470);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24471);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24472);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24473);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24474);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6764);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24475);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6765);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 24477);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 51549);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79989);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 103131);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 119303);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130805(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6766);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24478);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 51550);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 24479);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24480);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 51551);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79991);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 103132);
											s.is_empty()
										}
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
					super::push(out, 24481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51552);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79992);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103133);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24482);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24483);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51553);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24484);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51554);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79994);
									s.is_empty()
								}
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
			super::push(out, 6768);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24485);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24486);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 51555);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 79995);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103134);
											s.is_empty()
										}
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
					super::push(out, 24487);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24488);
					match s {
						[] => true,
						['а', s @ ..] => super::t32::n52077(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t32::n24643(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t32::n6788(s, out),
		['ш', s @ ..] => super::t32::n6789(s, out),
		_ => false,
	}
}

/// Words ending -рха.
pub(crate) fn n950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6771);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24492);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51559);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79997);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103135);
											s.is_empty()
										}
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
					super::push(out, 24493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6772);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24494);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51561);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79998);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 103136);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6773);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сха.
pub(crate) fn n951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6774);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6775);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6776);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 24498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тха.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6777);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24499);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51563);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79999);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 103137);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119304);
													match s {
														[] => true,
														['ж', s @ ..] => super::t32::n130806(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уха.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 952);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6778);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6779);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6780);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24501);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24502);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24503);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51564);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 80000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 51565);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80001);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103138);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 51566);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 80002);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103139);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119305);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24504);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24505);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24506);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51567);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 80003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 24507);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 24508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6781);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					super::push(out, 24509);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51568);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 80004);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103140);
											s.is_empty()
										}
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
					super::push(out, 24510);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 24511);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51569);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80005);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103141);
											s.is_empty()
										}
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
					super::push(out, 24512);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24513);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 80006);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 80007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 51570);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80008);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 103143);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119306);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 80009);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103144);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119307);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6782);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24514);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51571);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 80010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 24515);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24516);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51574);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80011);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 103145);
											s.is_empty()
										}
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
					super::push(out, 24518);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51575);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80012);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103146);
											s.is_empty()
										}
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
					super::push(out, 24519);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51576);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 80013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103147);
											s.is_empty()
										}
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
					super::push(out, 24520);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80014);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 80015);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103148);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6784);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 6785);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 24521);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 51577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24522);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51578);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 51579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24523);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t32::n6806(s, out),
		['л', s @ ..] => super::t32::n6807(s, out),
		['м', s @ ..] => super::t32::n6808(s, out),
		['н', s @ ..] => super::t32::n6809(s, out),
		['п', s @ ..] => super::t32::n6810(s, out),
		['р', s @ ..] => super::t32::n6811(s, out),
		['с', s @ ..] => super::t32::n6812(s, out),
		['т', s @ ..] => super::t32::n6813(s, out),
		['ц', s @ ..] => super::t32::n6814(s, out),
		['ч', s @ ..] => super::t32::n6815(s, out),
		['ш', s @ ..] => super::t32::n6816(s, out),
		['щ', s @ ..] => super::t32::n6817(s, out),
		_ => false,
	}
}

/// Words ending -ыха.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6799);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24585);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51662);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 80074);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 103182);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6800);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24586);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51663);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 80075);
									s.is_empty()
								}
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
			super::push(out, 6801);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24587);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51664);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 80076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьха.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6802);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эха.
pub(crate) fn n956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 955);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юха.
pub(crate) fn n957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6804);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6805);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24589);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6806);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6807);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24590);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24591);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 51666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24592);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24593);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24594);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24595);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24596);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 24597);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51670);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 80077);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103183);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 119314);
													match s {
														[] => true,
														['ж', s @ ..] => super::t32::n130810(s, out),
														_ => false,
													}
												}
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
							super::push(out, 51671);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 80078);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 103184);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119315);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6808);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24598);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24599);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51673);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 80079);
									s.is_empty()
								}
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
			super::push(out, 6809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24601);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24602);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 51674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24603);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51675);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24604);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 51676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24605);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24606);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24607);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51678);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 80081);
									s.is_empty()
								}
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
			super::push(out, 6811);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24608);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яха.
pub(crate) fn n958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 957);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6812);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6813);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6814);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6815);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6816);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24613);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24614);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51679);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 51680);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 80082);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103185);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119316);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24615);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51681);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 80083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 51682);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80084);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 103186);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -леспедеца.
pub(crate) fn n120579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119317);
	s.is_empty()
}

/// Words ending -ица.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 963);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6829);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24632);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51696);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24633);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51697);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 80094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 51698);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 24634);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 51699);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 80096);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103188);
											s.is_empty()
										}
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
					super::push(out, 24635);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 51700);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 80097);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 103189);
											s.is_empty()
										}
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
					super::push(out, 24636);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24637);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51701);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 80098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24638);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51702);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80099);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 103190);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119318);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n130811(s, out),
														_ => false,
													}
												}
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
							super::push(out, 51703);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80100);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 103191);
											s.is_empty()
										}
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
					super::push(out, 24639);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51704);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 80101);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 103192);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 119319);
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
										['л', s @ ..] => {
											super::push(out, 103193);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119320);
													match s {
														[] => true,
														['б', s @ ..] => super::t32::n130812(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 103194);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 119321);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n130813(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24640);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51705);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80102);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103195);
											s.is_empty()
										}
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
					super::push(out, 24641);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51706);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t32::n6849(s, out),
		['д', s @ ..] => super::t32::n6850(s, out),
		['е', s @ ..] => {
			super::push(out, 6832);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6833);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24662);
					match s {
						[] => true,
						['б', s @ ..] => super::t32::n52281(s, out),
						['н', s @ ..] => {
							super::push(out, 51759);
							match s {
								[] => true,
								['с', s @ ..] => super::t32::n81003(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t32::n24818(s, out),
				['л', s @ ..] => {
					super::push(out, 24664);
					s.is_empty()
				}
				['о', s @ ..] => super::t32::n24820(s, out),
				['р', s @ ..] => super::t32::n24821(s, out),
				['у', s @ ..] => super::t32::n24822(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t32::n6853(s, out),
		['л', s @ ..] => super::t32::n6854(s, out),
		['м', s @ ..] => super::t32::n6855(s, out),
		['н', s @ ..] => super::t32::n6856(s, out),
		['о', s @ ..] => super::t32::n6857(s, out),
		['п', s @ ..] => super::t32::n6858(s, out),
		['р', s @ ..] => super::t32::n6859(s, out),
		['с', s @ ..] => super::t32::n6860(s, out),
		['т', s @ ..] => super::t32::n6861(s, out),
		['ч', s @ ..] => super::t32::n6862(s, out),
		['ш', s @ ..] => super::t32::n6863(s, out),
		['щ', s @ ..] => super::t32::n6864(s, out),
		['ь', s @ ..] => super::t32::n6865(s, out),
		_ => false,
	}
}

/// Words ending -ла-убийца.
pub(crate) fn n121187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119923);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 129840);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 135669);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 139075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пойца.
pub(crate) fn n24947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24792);
	s.is_empty()
}

/// Words ending -уйца.
pub(crate) fn n6868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6849);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 24793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нца.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6850);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24794);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 52088);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80771);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 103960);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6851);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24795);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52089);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 24796);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52090);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24797);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52091);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 80774);
									s.is_empty()
								}
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
			super::push(out, 6852);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 24798);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52092);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 80775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пца.
pub(crate) fn n967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 966);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6853);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24799);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рца.
pub(crate) fn n968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 967);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6854);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 24800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6855);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24801);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52093);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 52094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52095);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 80777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24803);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52096);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80778);
									s.is_empty()
								}
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
			super::push(out, 6856);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24804);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 52097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6857);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24805);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52098);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 80779);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 103961);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119924);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n131193(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6858);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сца.
pub(crate) fn n969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 968);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 6859);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24807);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6860);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24808);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тца.
pub(crate) fn n970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 969);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6861);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24809);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6862);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24810);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24811);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52102);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 80780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24812);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 52103);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 80781);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 103962);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6863);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24813);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52104);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80782);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 103963);
											s.is_empty()
										}
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
					super::push(out, 24814);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52105);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80783);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103964);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119925);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24815);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52106);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уца.
pub(crate) fn n971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 970);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6864);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24816);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6865);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24817);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6866);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24818);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 6867);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 6868);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24819);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цца.
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 971);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6869);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шца.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 972);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 6870);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыца.
pub(crate) fn n974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 973);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6871);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24822);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьца.
pub(crate) fn n975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 974);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6872);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24823);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52111);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 80785);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103965);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119926);
													s.is_empty()
												}
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
							super::push(out, 52112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24824);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52113);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 52114);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80787);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 103966);
											s.is_empty()
										}
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
					super::push(out, 24825);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 52116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 24826);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яца.
pub(crate) fn n976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 975);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6873);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24827);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6874);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24828);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52119);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 80790);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 103967);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 119927);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гуашамача.
pub(crate) fn n121192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119928);
	s.is_empty()
}

/// Words ending -етр-свеча.
pub(crate) fn n121193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119929);
	s.is_empty()
}

/// Words ending -пиросвеча.
pub(crate) fn n121194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119930);
	s.is_empty()
}

/// Words ending -нг-встреча.
pub(crate) fn n121195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119931);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 129842);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 135670);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 139076);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 141043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ер-встреча.
pub(crate) fn n121196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119932);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 129843);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 135671);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дтеча.
pub(crate) fn n24994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24839);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 52128);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80799);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103975);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отеча.
pub(crate) fn n24995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24840);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 52129);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80800);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ича.
pub(crate) fn n980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 979);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 6884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24841);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кча.
pub(crate) fn n981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 980);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6885);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6886);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 24843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лча.
pub(crate) fn n982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 981);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6887);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 24844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6888);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мча.
pub(crate) fn n983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 982);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6889);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6890);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 24847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нча.
pub(crate) fn n984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 983);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6892);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52132);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24849);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52133);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24850);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52134);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 52135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24851);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80803);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 80804);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 80805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6893);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24853);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52137);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80806);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 103977);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6894);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6895);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24855);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52138);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 80807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оча.
pub(crate) fn n985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6897);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24857);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52139);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 80808);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 103978);
											s.is_empty()
										}
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
					super::push(out, 24858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6898);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24859);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80809);
									s.is_empty()
								}
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
			super::push(out, 6899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6900);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24861);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рча.
pub(crate) fn n986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 985);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6901);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24862);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24863);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6902);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24865);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52142);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80810);
									s.is_empty()
								}
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
			super::push(out, 6903);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6904);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тча.
pub(crate) fn n987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 986);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6905);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6906);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24870);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уча.
pub(crate) fn n988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 987);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6907);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6908);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24871);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 52147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', '\u{1}', s @ ..] => {
			super::push(out, 6909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хча.
pub(crate) fn n989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6910);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыча.
pub(crate) fn n990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 989);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6911);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24873);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52148);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24874);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 52149);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80812);
									s.is_empty()
								}
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
			super::push(out, 6913);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьча.
pub(crate) fn n991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 990);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6914);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24876);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52151);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80813);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103979);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119934);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n131196(s, out),
														_ => false,
													}
												}
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
							super::push(out, 52152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яча.
pub(crate) fn n992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 991);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6915);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6916);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 24878);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остокваша.
pub(crate) fn n121199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119935);
	s.is_empty()
}

/// Words ending -ша-малаша.
pub(crate) fn n121200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 129845);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 135672);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -равша.
pub(crate) fn n25052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24897);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 52175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тавша.
pub(crate) fn n25053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24898);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 52176);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103983);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евша.
pub(crate) fn n6951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6932);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 24899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гша.
pub(crate) fn n995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 994);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24900);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52177);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 80822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дша.
pub(crate) fn n996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 995);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6934);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6935);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24902);
					match s {
						[] => true,
						['р', 'ф', 'л', 'р', 'е', 'г', '\u{1}', s @ ..] => {
							super::push(out, 52178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24903);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24904);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52180);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80823);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 103984);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еша.
pub(crate) fn n997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 996);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 6937);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24905);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 52181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6938);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24906);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6939);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6940);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24908);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6941);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24909);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зша.
pub(crate) fn n998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6942);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24911);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52186);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иша.
pub(crate) fn n999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 998);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6943);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6944);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24913);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24914);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52188);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80825);
									s.is_empty()
								}
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
			super::push(out, 6946);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24915);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6947);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6948);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24916);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 24917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6949);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24918);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52191);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80826);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 103985);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йша.
pub(crate) fn n1000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 999);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6950);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24920);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24921);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 24923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6951);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24924);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кша.
pub(crate) fn n1001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1000);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6952);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24926);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 24927);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24929);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6953);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6954);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24932);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52196);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 52197);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80827);
									s.is_empty()
								}
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
			super::push(out, 6955);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6956);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24935);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24936);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52199);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80828);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103986);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119938);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24937);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52200);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 80829);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103987);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мша.
pub(crate) fn n1002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6957);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6958);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24939);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52201);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 80830);
									s.is_empty()
								}
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
			super::push(out, 6959);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24940);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52202);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80831);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 103988);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119939);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нша.
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1002);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6960);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24942);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24943);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52204);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 80832);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 103989);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 52205);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80833);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103990);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 52206);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80834);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 103991);
											s.is_empty()
										}
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
					super::push(out, 24944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6961);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52207);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80835);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 103992);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 52208);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 52209);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 80837);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103993);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119940);
													match s {
														[] => true,
														['н', s @ ..] => super::t32::n131198(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24945);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52210);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 80838);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 103994);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оша.
pub(crate) fn n1004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1003);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6964);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6965);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24947);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6966);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24948);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52212);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 80839);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 80840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24949);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52213);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 80841);
									s.is_empty()
								}
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
			super::push(out, 6967);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24950);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52214);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 52215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24951);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24952);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52216);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 80842);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 103995);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119941);
													s.is_empty()
												}
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
							super::push(out, 52217);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 80843);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103996);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 119942);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n131199(s, out),
														_ => false,
													}
												}
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
							super::push(out, 52218);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80844);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 103997);
											s.is_empty()
										}
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
					super::push(out, 24953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24954);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24955);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6970);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24957);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52221);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 80845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 24958);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 24959);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52223);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 80846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пша.
pub(crate) fn n1005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1004);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6971);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6972);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24961);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52224);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80847);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 103998);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119943);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24962);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52225);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80848);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103999);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119944);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рша.
pub(crate) fn n1006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1005);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6973);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24963);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 24964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52226);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 52227);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52229);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 52230);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 52231);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 80850);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104000);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119945);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n131200(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24966);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52232);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80851);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104001);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119946);
													s.is_empty()
												}
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
							super::push(out, 52233);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t32::n6993(s, out),
		['и', s @ ..] => {
			super::push(out, 6975);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24983);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24984);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24985);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24987);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24988);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52295);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80913);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104041);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119961);
													match s {
														[] => true,
														['л', s @ ..] => super::t32::n131204(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24989);
					match s {
						[] => true,
						['и', 'в', 'е', s @ ..] => {
							super::push(out, 52296);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80914);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104042);
											s.is_empty()
										}
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
					super::push(out, 24990);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52297);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 80915);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104043);
											s.is_empty()
										}
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
					super::push(out, 24991);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52298);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80916);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104044);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 119962);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24992);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 52299);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80917);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104045);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119963);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n131205(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24993);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24994);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52300);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 52301);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 80918);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104046);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119964);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n131206(s, out),
														_ => false,
													}
												}
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
							super::push(out, 52302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t32::n25157(s, out),
				['ц', s @ ..] => super::t32::n25158(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t32::n6996(s, out),
		['у', s @ ..] => super::t32::n6997(s, out),
		['ы', s @ ..] => super::t32::n6998(s, out),
		['э', s @ ..] => super::t32::n6999(s, out),
		['ю', s @ ..] => super::t32::n7000(s, out),
		['я', s @ ..] => super::t32::n7001(s, out),
		_ => false,
	}
}

/// Words ending -тша.
pub(crate) fn n1007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6983);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 6984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25005);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25006);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52313);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80923);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104050);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6985);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25007);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52314);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80924);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уша.
pub(crate) fn n1008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6986);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25008);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25009);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52316);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80925);
									s.is_empty()
								}
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
			super::push(out, 6987);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25010);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52317);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80926);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104052);
											s.is_empty()
										}
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
					super::push(out, 25011);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52318);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80927);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 80928);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104054);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6988);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25012);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52319);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 80929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25013);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52320);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80930);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104055);
											s.is_empty()
										}
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
					super::push(out, 25014);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52321);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 80931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25015);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52322);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80932);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104056);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6989);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6990);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52323);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 52324);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 80934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 52325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 52326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80936);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 80937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 25017);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 52327);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25018);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 52328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25019);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52329);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 52330);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 52331);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25020);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52332);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80942);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104057);
											s.is_empty()
										}
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
					super::push(out, 25021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52333);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80943);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 80944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25022);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52334);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80945);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104058);
											s.is_empty()
										}
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
					super::push(out, 25023);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52335);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 80946);
									s.is_empty()
								}
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
			super::push(out, 6991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25024);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52336);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80947);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104059);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t32::n25188(s, out),
				['и', s @ ..] => {
					super::push(out, 25026);
					match s {
						[] => true,
						['м', s @ ..] => super::t32::n52867(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t32::n25190(s, out),
				['о', s @ ..] => super::t32::n25191(s, out),
				['х', s @ ..] => super::t32::n25192(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t32::n7011(s, out),
		['н', s @ ..] => super::t32::n7012(s, out),
		['о', s @ ..] => super::t32::n7013(s, out),
		['п', s @ ..] => super::t32::n7014(s, out),
		['р', s @ ..] => super::t32::n7015(s, out),
		['с', s @ ..] => super::t32::n7016(s, out),
		['т', s @ ..] => super::t32::n7017(s, out),
		['х', s @ ..] => super::t32::n7018(s, out),
		['ш', s @ ..] => super::t32::n7019(s, out),
		_ => false,
	}
}

/// Words ending -хша.
pub(crate) fn n1009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1008);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7001);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25046);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52356);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 80961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыша.
pub(crate) fn n1010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1009);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7002);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25047);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7003);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7004);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25048);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьша.
pub(crate) fn n1011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1010);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7005);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25049);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52359);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 80962);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104065);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 80963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 52360);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 80964);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104066);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 119969);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25050);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52361);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80965);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104067);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119970);
													match s {
														[] => true,
														['д', s @ ..] => super::t32::n131207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 104068);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 80966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25051);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 80967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 52363);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 80968);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104069);
											s.is_empty()
										}
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
						['а', s @ ..] => {
							super::push(out, 52364);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 80969);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104070);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 52365);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 80970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юша.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1011);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25052);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7007);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25053);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7008);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25054);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52368);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 80971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25055);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52369);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 80972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25056);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7009);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25058);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яша.
pub(crate) fn n1013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1012);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7011);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25059);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 52372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7012);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 7013);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 7014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25060);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25061);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25062);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52374);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 80973);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104071);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7015);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25063);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52375);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 80974);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104072);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119971);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тетрадища.
pub(crate) fn n121236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119972);
	s.is_empty()
}

/// Words ending -жища.
pub(crate) fn n7042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7022);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25074);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 52387);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80984);
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
				['н', s @ ..] => {
					super::push(out, 52388);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 52389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25075);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52390);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 52391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 25076);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 52392);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80985);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лища.
pub(crate) fn n7044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7024);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25077);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 52393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 52394);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80987);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 104077);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119973);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129856);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n137083(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25078);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 25079);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 52396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ймища.
pub(crate) fn n25246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25080);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 52397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ямища.
pub(crate) fn n25247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25081);
	s.is_empty()
}

/// Words ending -нища.
pub(crate) fn n7046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7026);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25082);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 52398);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80988);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 52399);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80989);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104079);
									s.is_empty()
								}
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
			super::push(out, 25083);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25084);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52401);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пища.
pub(crate) fn n7047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7027);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25085);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52402);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 52403);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 25086);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25087);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рища.
pub(crate) fn n7048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7028);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25088);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 52404);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 52405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25089);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52406);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 80992);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104080);
									s.is_empty()
								}
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
			super::push(out, 25090);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 52407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25091);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 52408);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 25092);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 52409);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тища.
pub(crate) fn n7049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7029);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25093);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 52410);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80994);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119974);
											s.is_empty()
										}
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
					super::push(out, 52411);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 52412);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80995);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 104082);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чища.
pub(crate) fn n7050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7030);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25094);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 52413);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80996);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104083);
									s.is_empty()
								}
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
			super::push(out, 25095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щища.
pub(crate) fn n7051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7031);
	s.is_empty()
}

/// Words ending -ьища.
pub(crate) fn n7052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7032);
	s.is_empty()
}

/// Words ending -лща.
pub(crate) fn n1017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1016);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7033);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25096);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оща.
pub(crate) fn n1018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1017);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7034);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 7035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уща.
pub(crate) fn n1019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1018);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7037);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 7038);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыща.
pub(crate) fn n1020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1019);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7040);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25098);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7041);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25099);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпотреб.
pub(crate) fn n81860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81014);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 104089);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 119976);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 129857);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135677);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 139080);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 141045);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стреб.
pub(crate) fn n25321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25154);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 52452);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81015);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эреб.
pub(crate) fn n7119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7099);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теб.
pub(crate) fn n1045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7100);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -феб.
pub(crate) fn n1046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1044);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 7102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъеб.
pub(crate) fn n1048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 7103);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word листогиб.
pub(crate) fn n121241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119977);
	s.is_empty()
}

/// Words ending -сиб.
pub(crate) fn n1058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1056);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7129);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хиб.
pub(crate) fn n1059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1057);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7130);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7131);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 25186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиб.
pub(crate) fn n1060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1058);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7132);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25187);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 25188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7135);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аэрожелоб.
pub(crate) fn n121242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119978);
	s.is_empty()
}

/// Words ending -микроб.
pub(crate) fn n53010(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 81026);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 81027);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 104097);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119979);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 129858);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 135678);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ороб.
pub(crate) fn n7186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7166);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 25211);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52481);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81028);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 104098);
									s.is_empty()
								}
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
			super::push(out, 25212);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 52482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 25213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52483);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 52484);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81030);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 104099);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119980);
											s.is_empty()
										}
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
					super::push(out, 52485);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81031);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104100);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 119981);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -апроб.
pub(crate) fn n25381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25214);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 52486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -троб.
pub(crate) fn n7188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7168);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 25215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эроб.
pub(crate) fn n7189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7169);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25216);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 52487);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соб.
pub(crate) fn n1077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7170);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25217);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52488);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81033);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104101);
											s.is_empty()
										}
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
					super::push(out, 25218);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7171);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25219);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фоб.
pub(crate) fn n1078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1076);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7172);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 25220);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 52491);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81034);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104102);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119982);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7173);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25221);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'е', s @ ..] => {
					super::push(out, 25222);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25223);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25224);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52494);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81035);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104103);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119983);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -холстодуб.
pub(crate) fn n121248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119984);
	s.is_empty()
}

/// Words ending -агон-клуб.
pub(crate) fn n121249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119985);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 129859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 135679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руб.
pub(crate) fn n1091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1089);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7206);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7207);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25250);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52509);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81043);
									s.is_empty()
								}
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
			super::push(out, 7208);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25251);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52510);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81044);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104106);
											s.is_empty()
										}
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
					super::push(out, 25252);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52511);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 25253);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 52512);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81046);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104107);
											s.is_empty()
										}
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
					super::push(out, 25254);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52513);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25256);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25257);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52516);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81048);
									s.is_empty()
								}
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
			super::push(out, 7209);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 7210);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25258);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25259);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7211);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word песколюб.
pub(crate) fn n121250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119986);
	s.is_empty()
}

/// Words ending -теринослав.
pub(crate) fn n131213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135681);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word трубостав.
pub(crate) fn n131214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129862);
	s.is_empty()
}

/// Words ending -тимирязев.
pub(crate) fn n121259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119995);
	s.is_empty()
}

/// Words ending -бигнев.
pub(crate) fn n53100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52570);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 81084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 104128);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -днев.
pub(crate) fn n7289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7268);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25321);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 52571);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енев.
pub(crate) fn n7290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7269);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жнев.
pub(crate) fn n7291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7270);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25323);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52572);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инев.
pub(crate) fn n7292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7271);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 25324);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52573);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пев.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1123);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рев.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1124);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7273);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7274);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7275);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25327);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52575);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сев.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1125);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7276);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7277);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7278);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тев.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1126);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 7279);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25330);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уев.
pub(crate) fn n1129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1127);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7280);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25331);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 52579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7282);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25333);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 52580);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81087);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104129);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цев.
pub(crate) fn n1130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1128);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7283);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25334);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52581);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81088);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104130);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 119996);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n131215(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чев.
pub(crate) fn n1131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1129);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7284);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25335);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52582);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81089);
									s.is_empty()
								}
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
			super::push(out, 7285);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25336);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25337);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25338);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шев.
pub(crate) fn n1132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1130);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 7286);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25339);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 52586);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81090);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щев.
pub(crate) fn n1133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1131);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7287);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25340);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52587);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьев.
pub(crate) fn n1134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1132);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 7288);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25341);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52588);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81092);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104132);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7289);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25342);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81093);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104133);
											s.is_empty()
										}
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
					super::push(out, 25343);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 25344);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сабмиссив.
pub(crate) fn n121261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119997);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 129864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -катив.
pub(crate) fn n25527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25360);
	match s {
		[] => true,
		['и', 'д', s @ ..] => {
			super::push(out, 52604);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 81102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -татив.
pub(crate) fn n25529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25362);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 52605);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81103);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ситив.
pub(crate) fn n7329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7308);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 25363);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52606);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктив.
pub(crate) fn n7330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7309);
	match s {
		[] => true,
		['а', '\u{1}', s @ ..] => {
			super::push(out, 25364);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52607);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 52608);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81105);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104140);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119998);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129865);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 52609);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81106);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104141);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119999);
											s.is_empty()
										}
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
					super::push(out, 52610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шив.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1139);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7310);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25365);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52611);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81107);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104142);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олобродов.
pub(crate) fn n121264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120000);
	s.is_empty()
}

/// Words ending -вка-вызов.
pub(crate) fn n121265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 129866);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 135682);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 139081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ков.
pub(crate) fn n1150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1148);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7328);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25381);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 52622);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81114);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104147);
											s.is_empty()
										}
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
					super::push(out, 25382);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25383);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25384);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52624);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81115);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104148);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7329);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 25385);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 52625);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81116);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104149);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120002);
													match s {
														[] => true,
														['н', s @ ..] => super::t32::n131219(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7330);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25386);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52626);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81117);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104150);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 52627);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81118);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104151);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120003);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n131220(s, out),
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
									super::push(out, 81119);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104152);
											s.is_empty()
										}
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
					super::push(out, 25387);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52628);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 81120);
									s.is_empty()
								}
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
			super::push(out, 7331);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25388);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7332);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25389);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25390);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7333);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25391);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52632);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25392);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7334);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25394);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 25395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7335);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 7336);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25397);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25398);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52635);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 81122);
									s.is_empty()
								}
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
			super::push(out, 7337);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лов.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1149);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7338);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7339);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25400);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52636);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104153);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7340);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7341);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25402);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52637);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25403);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52638);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81125);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104154);
											s.is_empty()
										}
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
					super::push(out, 25404);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52639);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81126);
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
							super::push(out, 52640);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81127);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 52641);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81128);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104156);
											s.is_empty()
										}
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
					super::push(out, 25405);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 52642);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81129);
									s.is_empty()
								}
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
			super::push(out, 7342);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25406);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7343);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25407);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52644);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104157);
											s.is_empty()
										}
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
					super::push(out, 25408);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 25409);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52646);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81131);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104158);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 120004);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7344);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25410);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52647);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81132);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104159);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7345);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 25411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7346);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25412);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52648);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81133);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104160);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7347);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25413);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52649);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 52650);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81135);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 104161);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 120005);
													match s {
														[] => true,
														['н', s @ ..] => super::t32::n131221(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t32::n25583(s, out),
				['д', 'е', s @ ..] => super::t32::n25584(s, out),
				['к', s @ ..] => super::t32::n25585(s, out),
				['м', s @ ..] => super::t32::n25586(s, out),
				['р', 'п', s @ ..] => super::t32::n25587(s, out),
				['с', s @ ..] => super::t32::n25588(s, out),
				['т', s @ ..] => super::t32::n25589(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t32::n7369(s, out),
		['с', s @ ..] => super::t32::n7370(s, out),
		['т', s @ ..] => super::t32::n7371(s, out),
		['у', s @ ..] => super::t32::n7372(s, out),
		['ш', s @ ..] => super::t32::n7373(s, out),
		['ы', s @ ..] => super::t32::n7374(s, out),
		_ => false,
	}
}

/// Words ending -лимов.
pub(crate) fn n25599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25430);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 52672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нимов.
pub(crate) fn n25600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25431);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 52673);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 81155);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 104174);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120011);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129873);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135686);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -симов.
pub(crate) fn n25601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25432);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52674);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 81156);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нов.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1151);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7357);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25433);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 52675);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 25434);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52676);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81158);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104175);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120012);
													match s {
														[] => true,
														['ф', s @ ..] => super::t32::n131226(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25435);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 52677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 25436);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52678);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81159);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104176);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 120013);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25437);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52679);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81160);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104177);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 120014);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25438);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52680);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81161);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104178);
											s.is_empty()
										}
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
					super::push(out, 25439);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52681);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81162);
									s.is_empty()
								}
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
			super::push(out, 7358);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25440);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52682);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81163);
									s.is_empty()
								}
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
			super::push(out, 7359);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25441);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52683);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81164);
									s.is_empty()
								}
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
			super::push(out, 7360);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25442);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 52684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7361);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25443);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52685);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81165);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104179);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пов.
pub(crate) fn n1154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1152);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7362);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25444);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52686);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 81166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ров.
pub(crate) fn n1155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1153);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7363);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7364);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7365);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25446);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7366);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25447);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7367);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52689);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81167);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104180);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120015);
													match s {
														[] => true,
														['л', s @ ..] => super::t32::n131227(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7368);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7369);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25450);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 25451);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 25452);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52691);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81168);
									s.is_empty()
								}
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
			super::push(out, 7370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25453);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7371);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25454);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 25455);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 52693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25457);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 25458);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7373);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25459);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52695);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81170);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 81171);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104181);
											s.is_empty()
										}
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
					super::push(out, 25460);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52696);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81172);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 104182);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120016);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сов.
pub(crate) fn n1156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1154);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7374);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7375);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25462);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7376);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52698);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81173);
									s.is_empty()
								}
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
			super::push(out, 7377);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25465);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52700);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81174);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104183);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 120017);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25466);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тов.
pub(crate) fn n1157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1155);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7378);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25467);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52702);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81175);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104184);
											s.is_empty()
										}
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
					super::push(out, 25468);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52703);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25469);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52704);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 81177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25470);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52705);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25471);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52706);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81179);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104185);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7379);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25472);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52707);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81180);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104186);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7380);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 7381);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25474);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7382);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25475);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7383);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25476);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52710);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81181);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 104187);
											s.is_empty()
										}
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
					super::push(out, 25477);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52711);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81182);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104188);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 120018);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25478);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52712);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81183);
									s.is_empty()
								}
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
			super::push(out, 7384);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25479);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52713);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81184);
									s.is_empty()
								}
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
			super::push(out, 7385);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25480);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52714);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 81185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25481);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81186);
									s.is_empty()
								}
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
			super::push(out, 7386);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25482);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 52716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25483);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52717);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81187);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104189);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120019);
													s.is_empty()
												}
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
							super::push(out, 52718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25484);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7387);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25485);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 7388);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25486);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52721);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81189);
									s.is_empty()
								}
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
			super::push(out, 7389);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25487);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52722);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81190);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104190);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120020);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хов.
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1156);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7390);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7391);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25489);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7392);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25491);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52724);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 52725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7393);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25492);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7394);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25493);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25494);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52728);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81191);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104191);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7395);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 7396);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25496);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52729);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цов.
pub(crate) fn n1159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1157);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7397);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25497);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 52730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шов.
pub(crate) fn n1160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1158);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7398);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7399);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25498);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52731);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25499);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7400);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вет-резерв.
pub(crate) fn n121285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120021);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 129876);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 135688);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пшеводув.
pub(crate) fn n121286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120022);
	s.is_empty()
}

/// Words ending -стеклодув.
pub(crate) fn n121287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120023);
	s.is_empty()
}

/// Words ending -оздуходув.
pub(crate) fn n121288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120024);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 129877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тво-архипелаг.
pub(crate) fn n131230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129878);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 135689);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -актериофаг.
pub(crate) fn n131231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129879);
	s.is_empty()
}

/// Words ending -савчег.
pub(crate) fn n53330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52797);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 81233);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 104212);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 120029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овчег.
pub(crate) fn n25749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25579);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 52798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хистратиг.
pub(crate) fn n121294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120030);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 129880);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фиг.
pub(crate) fn n1218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1215);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7517);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 25611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7518);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25612);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -циг.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1216);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 7519);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25613);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52817);
							s.is_empty()
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
				['й', s @ ..] => {
					super::push(out, 25614);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чиг.
pub(crate) fn n1220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1217);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щиг.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1218);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7522);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25615);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вольфганг.
pub(crate) fn n121295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120031);
	s.is_empty()
}

/// Words ending -миннезанг.
pub(crate) fn n121296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120032);
	s.is_empty()
}

/// Words ending -танг.
pub(crate) fn n7561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7539);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25636);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 52835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25637);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25638);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 52837);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25639);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25640);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 52839);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81259);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104225);
									s.is_empty()
								}
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
			super::push(out, 25641);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52840);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нцанг.
pub(crate) fn n25813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25642);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 52841);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 81261);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енг.
pub(crate) fn n1226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1223);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7542);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25644);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7543);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25646);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52843);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81262);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104227);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инг.
pub(crate) fn n1227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1224);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7544);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 7545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25647);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 25648);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 25649);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25650);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52845);
							match s {
								[] => true,
								['л', 'о', 'ф', 'с', s @ ..] => {
									super::push(out, 81263);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104228);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120033);
													match s {
														[] => true,
														['м', s @ ..] => super::t32::n131233(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25651);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52846);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81264);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104229);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 120034);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7546);
			match s {
				[] => true,
				['г', 'о', s @ ..] => {
					super::push(out, 25652);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25653);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 52847);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 52848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25654);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52849);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81266);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104230);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120035);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n131234(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25655);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52850);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 81267);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104231);
											s.is_empty()
										}
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
					super::push(out, 25656);
					match s {
						[] => true,
						['м', 'н', 'е', 'к', 'к', 'е', 'ь', 'к', '\u{1}', s @ ..] => {
							super::push(out, 52851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25657);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 25658);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 25659);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52852);
							match s {
								[] => true,
								['б', 'и', s @ ..] => {
									super::push(out, 81268);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104232);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 120036);
													match s {
														[] => true,
														['б', s @ ..] => super::t32::n131235(s, out),
														_ => false,
													}
												}
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
							super::push(out, 52853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25660);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52854);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81269);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104233);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120037);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n131236(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52855);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 104234);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 104235);
											s.is_empty()
										}
										['к', s @ ..] => super::t32::n105337(s, out),
										['т', s @ ..] => super::t32::n105338(s, out),
										['у', s @ ..] => super::t32::n105339(s, out),
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
					super::push(out, 25662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 7548);
			s.is_empty()
		}
		['з', s @ ..] => super::t32::n7571(s, out),
		['и', s @ ..] => super::t32::n7572(s, out),
		['к', s @ ..] => super::t32::n7573(s, out),
		['л', s @ ..] => super::t33::n7574(s, out),
		['м', s @ ..] => super::t33::n7575(s, out),
		['н', s @ ..] => super::t33::n7576(s, out),
		['о', s @ ..] => super::t33::n7577(s, out),
		['п', s @ ..] => super::t33::n7578(s, out),
		['р', s @ ..] => super::t33::n7579(s, out),
		['с', s @ ..] => super::t33::n7580(s, out),
		['т', s @ ..] => super::t33::n7581(s, out),
		['у', s @ ..] => super::t33::n7582(s, out),
		['ф', s @ ..] => super::t33::n7583(s, out),
		['ч', s @ ..] => super::t33::n7584(s, out),
		['ш', s @ ..] => super::t33::n7585(s, out),
		_ => false,
	}
}

/// Words ending -онг.
pub(crate) fn n1228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1225);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7564);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7565);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7566);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 7567);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7568);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25735);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52941);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81349);
									s.is_empty()
								}
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
			super::push(out, 7569);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25736);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7570);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 25738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7571);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25739);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52943);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81351);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 104298);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 120077);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7572);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 25740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7573);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25741);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унг.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1226);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7574);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7575);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25743);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52945);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 25744);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52946);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81353);
									s.is_empty()
								}
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
			super::push(out, 7576);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25745);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7577);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25746);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52948);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25747);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52949);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81355);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104299);
											s.is_empty()
										}
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
					super::push(out, 25748);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52950);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81356);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104300);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7578);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25749);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7579);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25750);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7580);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 25751);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -й-бульдог.
pub(crate) fn n121342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120078);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 129913);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 135707);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139091);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word сучкожог.
pub(crate) fn n121343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120079);
	s.is_empty()
}

/// Words ending -лог.
pub(crate) fn n1235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1232);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7593);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 7594);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25765);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52966);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81365);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104305);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120080);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131266(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7595);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25766);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52967);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81366);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104306);
											s.is_empty()
										}
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
					super::push(out, 25767);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52968);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81367);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104307);
											s.is_empty()
										}
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
					super::push(out, 25768);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 25769);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 52969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 25770);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 25771);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52970);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25772);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52971);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 52972);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81370);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 104308);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 120081);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25773);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52973);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81371);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 104309);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 120082);
													match s {
														[] => true,
														['й', s @ ..] => super::t33::n131267(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7596);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 25774);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7597);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7598);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7599);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25776);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52974);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25777);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7600);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25778);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52976);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 52977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25779);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 52978);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81373);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104310);
											s.is_empty()
										}
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
					super::push(out, 25780);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52979);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81374);
									s.is_empty()
								}
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
			super::push(out, 7601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25781);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7602);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25783);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52982);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25784);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 52983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7603);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 25785);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 52984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t33::n7626(s, out),
		['п', s @ ..] => {
			super::push(out, 7605);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7606);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25800);
					match s {
						[] => true,
						['д', s @ ..] => super::t33::n53550(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t33::n25972(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7607);
			s.is_empty()
		}
		['т', s @ ..] => super::t33::n7630(s, out),
		_ => false,
	}
}

/// Words ending -сьминог.
pub(crate) fn n82254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81406);
	s.is_empty()
}

/// Words ending -оног.
pub(crate) fn n7634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7612);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 25805);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 25806);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 53018);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81407);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 104331);
									s.is_empty()
								}
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
			super::push(out, 25807);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 25808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пог.
pub(crate) fn n1238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1235);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7613);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рог.
pub(crate) fn n1239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1236);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7614);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7615);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7617);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25812);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7618);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25813);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53021);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81408);
									s.is_empty()
								}
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
			super::push(out, 7619);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7620);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25815);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53023);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81409);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104332);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7621);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53024);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 53025);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25817);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53026);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25818);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53027);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81412);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 53028);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81413);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104334);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 120100);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25819);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25820);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53029);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81414);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 104335);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120101);
													match s {
														[] => true,
														['ц', s @ ..] => super::t33::n131280(s, out),
														_ => false,
													}
												}
												['к', s @ ..] => {
													super::push(out, 120102);
													match s {
														[] => true,
														['у', s @ ..] => super::t33::n131281(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7622);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 25821);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 53030);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81415);
									s.is_empty()
								}
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
			super::push(out, 7623);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25822);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25823);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тог.
pub(crate) fn n1240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1237);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7624);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7625);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7626);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7627);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25825);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 53032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7629);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53033);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цог.
pub(crate) fn n1241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1238);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7630);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25828);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чог.
pub(crate) fn n1242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1239);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вюртемберг.
pub(crate) fn n121367(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 129930);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 129931);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 135720);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 139102);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 141060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ауфенберг.
pub(crate) fn n121368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120103);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 129932);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 135721);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ауберг.
pub(crate) fn n53583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53046);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 81429);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьберг.
pub(crate) fn n26009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25838);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 53047);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 81430);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -верг.
pub(crate) fn n7657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7635);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 25839);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25840);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53049);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 81431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дерг.
pub(crate) fn n7658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7636);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 25841);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53050);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перг.
pub(crate) fn n7659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7637);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 25842);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 53051);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орг.
pub(crate) fn n1245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1242);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7638);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 7639);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25843);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53052);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81434);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104342);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 120104);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25844);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 53053);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81435);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104343);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120105);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25845);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53054);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 25846);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7640);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25847);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 53056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7641);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25848);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53057);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81437);
									s.is_empty()
								}
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
			super::push(out, 7642);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7643);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25850);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 53058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 7644);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 7645);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 7646);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25851);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7647);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25852);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53060);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81438);
									s.is_empty()
								}
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
			super::push(out, 7648);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53061);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81439);
									s.is_empty()
								}
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
			super::push(out, 7649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25854);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 25855);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53062);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25856);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 53063);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81441);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104344);
											s.is_empty()
										}
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
					super::push(out, 25857);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25858);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53065);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81442);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104345);
											s.is_empty()
										}
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
					super::push(out, 25859);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25860);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53067);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25861);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53068);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 25862);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 25863);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53070);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81445);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104346);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7650);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25864);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53071);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ург.
pub(crate) fn n1246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1243);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7651);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25865);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53072);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81447);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104347);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 120106);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25866);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53073);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81448);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 104348);
											s.is_empty()
										}
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
					super::push(out, 25867);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53074);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 53075);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81450);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104349);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 120107);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131286(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25868);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53076);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81451);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104350);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120108);
													match s {
														[] => true,
														['р', s @ ..] => super::t33::n131287(s, out),
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
									super::push(out, 81452);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104351);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120109);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n131288(s, out),
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
									super::push(out, 81453);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104352);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 53077);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81454);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 104353);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 81455);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104354);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 53078);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81456);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104355);
											s.is_empty()
										}
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
					super::push(out, 25869);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 53079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25870);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53080);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81457);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104356);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120110);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25871);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53081);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81458);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 53082);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81459);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104358);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120111);
													match s {
														[] => true,
														['х', s @ ..] => super::t33::n131289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 25872);
					match s {
						[] => true,
						['р', s @ ..] => super::t33::n53620(s, out),
						_ => false,
					}
				}
				['ц', s @ ..] => super::t33::n26044(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7652);
			match s {
				[] => true,
				['т', s @ ..] => super::t33::n26045(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t33::n7675(s, out),
		['л', s @ ..] => super::t33::n7676(s, out),
		['р', s @ ..] => super::t33::n7677(s, out),
		['т', s @ ..] => super::t33::n7678(s, out),
		_ => false,
	}
}

/// Words ending -эрг.
pub(crate) fn n1247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1244);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7657);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25880);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53092);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81469);
									s.is_empty()
								}
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
			super::push(out, 7658);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25881);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word полукруг.
pub(crate) fn n121380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120115);
	s.is_empty()
}

/// Words ending -бцуг.
pub(crate) fn n7699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53109);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нцуг.
pub(crate) fn n7700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7678);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25902);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 53110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сцуг.
pub(crate) fn n7701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7679);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 25903);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53111);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81482);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104368);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 120116);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129940);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n137133(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьцуг.
pub(crate) fn n7702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7680);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 25904);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53112);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 81483);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104369);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 120117);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129941);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуг.
pub(crate) fn n1257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1254);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25905);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25906);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 53114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81484);
									s.is_empty()
								}
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
			super::push(out, 7682);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25907);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53115);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81485);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104370);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120118);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7683);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7684);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуг.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трубоклад.
pub(crate) fn n121384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120119);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 129942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --склад.
pub(crate) fn n53673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53136);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 81495);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 104373);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120120);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129943);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 135727);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139105);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n142506(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -завсклад.
pub(crate) fn n105476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 120121);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -исклад.
pub(crate) fn n53675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53138);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 81497);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 104375);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осклад.
pub(crate) fn n53676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53139);
	s.is_empty()
}

/// Words ending -тсклад.
pub(crate) fn n53677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53140);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 81498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олад.
pub(crate) fn n7748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7726);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 25943);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53141);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							super::push(out, 81499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слад.
pub(crate) fn n7749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7727);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улад.
pub(crate) fn n7750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7728);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 25945);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлад.
pub(crate) fn n7751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7729);
	s.is_empty()
}

/// Words ending -мад.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1283);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7730);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25946);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 53143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7731);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7732);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25948);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 25949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -над.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1284);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7733);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25950);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53144);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									super::push(out, 81500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 25951);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53145);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81501);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104376);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7734);
			match s {
				[] => true,
				['б', 'р', 'а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 25952);
					s.is_empty()
				}
				['м', 'и', 'л', '\u{1}', s @ ..] => {
					super::push(out, 25953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пад.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1285);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7735);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25954);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 53146);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81502);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104377);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 120123);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 104378);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120124);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7736);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7737);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7738);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7740);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рад.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1286);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7741);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25958);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 25959);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25960);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 25961);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53149);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25962);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53150);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 53151);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 81504);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104379);
											s.is_empty()
										}
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
					super::push(out, 25963);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53152);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 53153);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81505);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 104380);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120125);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 81506);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 81507);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104381);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 120126);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131296(s, out),
														_ => false,
													}
												}
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
							super::push(out, 53154);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81508);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104382);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 120127);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 120128);
													s.is_empty()
												}
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
							super::push(out, 53155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 25964);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53156);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81509);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 104383);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7743);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25965);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25966);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53158);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81510);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 104384);
											s.is_empty()
										}
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
					super::push(out, 25967);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 53159);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 53160);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81512);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104385);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 53161);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81513);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104386);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120129);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7745);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25971);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81514);
									s.is_empty()
								}
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
			super::push(out, 7746);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25972);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53165);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81515);
									s.is_empty()
								}
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
			super::push(out, 7747);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53166);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 53167);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81516);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 104387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сад.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7748);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 7749);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25974);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53168);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81517);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104388);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 120130);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25975);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 25976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7751);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25977);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25978);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7752);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7753);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25981);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 53170);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25982);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 53171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81520);
									s.is_empty()
								}
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
			super::push(out, 7754);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25983);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25984);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 53172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25985);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25986);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53173);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81522);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104389);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 120131);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7755);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25987);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 53174);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81523);
									s.is_empty()
								}
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
			super::push(out, 7756);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7757);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25989);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тад.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1288);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 7758);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25990);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53176);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81525);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104390);
											s.is_empty()
										}
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
					super::push(out, 25991);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 53177);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81526);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104391);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 120132);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25992);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53178);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81527);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104392);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120133);
													match s {
														[] => true,
														['и', s @ ..] => super::t33::n131297(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уад.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1289);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 7759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хад.
pub(crate) fn n1293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1290);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7760);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7761);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 25995);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25996);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 53181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7762);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25997);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 53182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7763);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7764);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чад.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1291);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шад.
pub(crate) fn n1295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1292);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7766);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7767);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -к-антиковед.
pub(crate) fn n121399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120134);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 129946);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 135729);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139107);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 141062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ук-птицеед.
pub(crate) fn n121400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120135);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 129947);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 135730);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ортплед.
pub(crate) fn n82384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81536);
	s.is_empty()
}

/// Words ending -след.
pub(crate) fn n7811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26018);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26019);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 53199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мед.
pub(crate) fn n1307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7790);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7791);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 26021);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53201);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81537);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 53202);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81538);
									s.is_empty()
								}
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
			super::push(out, 7792);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7793);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26023);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 53203);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81539);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104397);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7794);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26024);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оед.
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7795);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 26025);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53205);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81540);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104398);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120136);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7796);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7797);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26026);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53206);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81541);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104399);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7798);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26027);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53207);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81542);
									s.is_empty()
								}
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
			super::push(out, 7799);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 7800);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 7801);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 7802);
			match s {
				[] => true,
				['в', 'о', 'г', '\u{1}', s @ ..] => {
					super::push(out, 26028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7803);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26029);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53208);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 81543);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104400);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 120137);
													match s {
														[] => true,
														['ж', s @ ..] => super::t33::n131300(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26030);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81544);
									s.is_empty()
								}
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
			super::push(out, 7804);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 26031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7805);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26032);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 26033);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53210);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81545);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104401);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 104402);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120138);
													match s {
														[] => true,
														['у', s @ ..] => super::t33::n131301(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пед.
pub(crate) fn n1309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7807);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7808);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26034);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26035);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7809);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26036);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53213);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81546);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104403);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 120139);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131302(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26037);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53214);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81547);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104404);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7810);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26038);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53215);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26039);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 26040);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53216);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81549);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 104405);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 120140);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131303(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7811);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26041);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ред.
pub(crate) fn n1310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 7812);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 26042);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53218);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 81550);
									s.is_empty()
								}
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
			super::push(out, 7813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26043);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 26044);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26045);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 26046);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53221);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81551);
									s.is_empty()
								}
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
			super::push(out, 7814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26047);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26048);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53222);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81552);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104406);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 120141);
													match s {
														[] => true,
														['с', s @ ..] => super::t33::n131304(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 26049);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53223);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81553);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104407);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26050);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26051);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53224);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26052);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53225);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81555);
									s.is_empty()
								}
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
			super::push(out, 7816);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 26055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7817);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26056);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7818);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53229);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81556);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104408);
											s.is_empty()
										}
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
					super::push(out, 26058);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53230);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26059);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53231);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 81558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26060);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53233);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 53234);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26062);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7819);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26063);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26064);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 53236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 7820);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26065);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53237);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81560);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 104409);
											s.is_empty()
										}
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
					super::push(out, 26066);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 53238);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81561);
									s.is_empty()
								}
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
			super::push(out, 7821);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 26067);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 53239);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81562);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104410);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120142);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сед.
pub(crate) fn n1311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1308);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7822);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7823);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7824);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26069);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 53241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7825);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26070);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53242);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 81563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26071);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 53243);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 81564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7826);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 53245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тед.
pub(crate) fn n1312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1309);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 7827);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26074);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53246);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 81565);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104411);
											s.is_empty()
										}
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
					super::push(out, 26075);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 53247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фед.
pub(crate) fn n1313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7828);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26076);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 53248);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хед.
pub(crate) fn n1314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1311);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7829);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26077);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 53249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7830);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26078);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 53250);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81567);
									s.is_empty()
								}
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
			super::push(out, 7831);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26079);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53251);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81568);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104412);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 120143);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26080);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 53252);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81569);
									s.is_empty()
								}
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
			super::push(out, 7832);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26081);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 53253);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 81570);
									s.is_empty()
								}
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
			super::push(out, 7833);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26082);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 53254);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 81571);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104413);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шед.
pub(crate) fn n1315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1312);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьед.
pub(crate) fn n1316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1313);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7835);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26083);
					match s {
						[] => true,
						['р', 'у', 'м', '-', s @ ..] => {
							super::push(out, 53255);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 81572);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 104414);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120144);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n131306(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порт-саид.
pub(crate) fn n121411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120146);
	s.is_empty()
}

