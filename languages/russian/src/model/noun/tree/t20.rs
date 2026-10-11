use flexion::Path;

/// Words ending -иан.
pub(crate) fn n2725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2715);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61433);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 61434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33350);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 12554);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33351);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 61437);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88829);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 61438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12555);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33352);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 33353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 12556);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12557);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33356);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33357);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33358);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61441);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 61442);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33359);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 61443);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109824);
											s.is_empty()
										}
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
					super::push(out, 33360);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 33361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33362);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61444);
							s.is_empty()
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
				['а', s @ ..] => {
					super::push(out, 33363);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33364);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61446);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88833);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 109825);
											s.is_empty()
										}
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
					super::push(out, 33365);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12560);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33366);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61448);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 61449);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88834);
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
							super::push(out, 61450);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88835);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109826);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123597);
													s.is_empty()
												}
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
							super::push(out, 61451);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88836);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109827);
											s.is_empty()
										}
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
					super::push(out, 33367);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61452);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88837);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109828);
											s.is_empty()
										}
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
					super::push(out, 33368);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 33369);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61454);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88838);
									s.is_empty()
								}
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
			super::push(out, 12561);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 12562);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33370);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61455);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88839);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109829);
											s.is_empty()
										}
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
					super::push(out, 33371);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61456);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88840);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109830);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 109831);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йан.
pub(crate) fn n2726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2716);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12564);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кан.
pub(crate) fn n2727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2717);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33374);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 33375);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61459);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 61460);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61461);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33377);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12566);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 33379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61464);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 61465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 33380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 33381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61467);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 88843);
									s.is_empty()
								}
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
			super::push(out, 12567);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33382);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61468);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33383);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61469);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 61470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 61471);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88847);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109832);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123598);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133256(s, out),
														['о', s @ ..] => super::t36::n133257(s, out),
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
									super::push(out, 88848);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109833);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123599);
													match s {
														[] => true,
														['б', s @ ..] => super::t36::n133258(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33384);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61472);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 61473);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 88850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33385);
					s.is_empty()
				}
				['р', s @ ..] => super::t36::n33695(s, out),
				['т', s @ ..] => {
					super::push(out, 33387);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61477);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t36::n62132(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t36::n33697(s, out),
				['щ', s @ ..] => super::t36::n33698(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t36::n12633(s, out),
		['к', s @ ..] => super::t36::n12634(s, out),
		['л', s @ ..] => super::t36::n12635(s, out),
		['н', s @ ..] => super::t36::n12636(s, out),
		['о', s @ ..] => super::t36::n12637(s, out),
		['п', s @ ..] => super::t36::n12638(s, out),
		['р', s @ ..] => super::t36::n12639(s, out),
		['с', s @ ..] => super::t36::n12640(s, out),
		['у', s @ ..] => super::t36::n12641(s, out),
		['ц', s @ ..] => super::t36::n12642(s, out),
		['ш', s @ ..] => super::t36::n12643(s, out),
		['ь', s @ ..] => super::t36::n12644(s, out),
		_ => false,
	}
}

/// Words ending -лан.
pub(crate) fn n2728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33419);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 33420);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 33421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33422);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33423);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61507);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88865);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 109839);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123601);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12581);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61508);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 61509);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88866);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109840);
											s.is_empty()
										}
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
					super::push(out, 33426);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61510);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88867);
									s.is_empty()
								}
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
			super::push(out, 12582);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 12583);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33428);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88868);
									s.is_empty()
								}
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
			super::push(out, 12584);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 12585);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33429);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 61512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88869);
									s.is_empty()
								}
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
				['а', s @ ..] => {
					super::push(out, 33430);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33431);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 61514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33432);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 33433);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61515);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88870);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109841);
											s.is_empty()
										}
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
					super::push(out, 33434);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61516);
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
					super::push(out, 33435);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 33436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61517);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88871);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109842);
											s.is_empty()
										}
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
					super::push(out, 33437);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61518);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88872);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109843);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12587);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					super::push(out, 33438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12588);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 61521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33440);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33441);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33442);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'б', '\u{1}', s @ ..] => {
					super::push(out, 33443);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 33444);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61526);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 88874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 61527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61528);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 61529);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88875);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109844);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12590);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33445);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 33446);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61530);
							match s {
								[] => true,
								['й', s @ ..] => super::t36::n89832(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 33447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t36::n33760(s, out),
				['т', s @ ..] => super::t36::n33761(s, out),
				['ч', s @ ..] => super::t36::n33762(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t36::n12658(s, out),
		_ => false,
	}
}

/// Words ending -ман.
pub(crate) fn n2729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12592);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33451);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 33452);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61535);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 61536);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88878);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109846);
											s.is_empty()
										}
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
					super::push(out, 33453);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 33454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61538);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 61539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33455);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33456);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61540);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 61541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 33457);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 33458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12594);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33459);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61542);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88880);
									s.is_empty()
								}
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
			super::push(out, 12595);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33460);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61543);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 88881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109847);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12596);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33461);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 61544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 33462);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61545);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88882);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 109848);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123603);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33463);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61546);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88883);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'е', s @ ..] => {
					super::push(out, 33464);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 61549);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 33465);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61550);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88886);
									s.is_empty()
								}
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
			super::push(out, 12597);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33466);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33467);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61551);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 61552);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33468);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12598);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33469);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12599);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 61556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33471);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33472);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 33473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12600);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33474);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61557);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 61558);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 61559);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12601);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t36::n33793(s, out),
				['г', s @ ..] => super::t36::n33794(s, out),
				['д', s @ ..] => super::t36::n33795(s, out),
				['з', '\u{1}', s @ ..] => super::t36::n33796(s, out),
				['к', s @ ..] => super::t36::n33797(s, out),
				['л', 'о', s @ ..] => super::t36::n33798(s, out),
				['н', s @ ..] => super::t36::n33799(s, out),
				['р', s @ ..] => super::t36::n33800(s, out),
				['с', s @ ..] => super::t36::n33801(s, out),
				['т', s @ ..] => super::t36::n33802(s, out),
				['ф', s @ ..] => super::t36::n33803(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t36::n12669(s, out),
		['р', s @ ..] => super::t36::n12670(s, out),
		['с', s @ ..] => super::t36::n12671(s, out),
		['т', s @ ..] => super::t36::n12672(s, out),
		['у', s @ ..] => super::t36::n12673(s, out),
		['х', s @ ..] => super::t36::n12674(s, out),
		['ц', s @ ..] => super::t36::n12675(s, out),
		['ч', s @ ..] => super::t36::n12676(s, out),
		['ш', s @ ..] => super::t36::n12677(s, out),
		['ь', s @ ..] => super::t36::n12678(s, out),
		_ => false,
	}
}

/// Words ending -нан.
pub(crate) fn n2730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2720);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12612);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12613);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12614);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12615);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33510);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33511);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оан.
pub(crate) fn n2731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2721);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 12616);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12617);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 33513);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 61618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пан.
pub(crate) fn n2732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2722);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12618);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12619);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33514);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33515);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 33518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12620);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33519);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33520);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61622);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 33521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12621);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 33522);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12622);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12623);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33524);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33525);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12624);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33526);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61626);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88938);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109876);
											s.is_empty()
										}
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
					super::push(out, 33527);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 61627);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88939);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109877);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123614);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33528);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61628);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88940);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109878);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12625);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33529);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 61629);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88941);
									s.is_empty()
								}
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
			super::push(out, 12626);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 61631);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33530);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12627);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 33531);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 12628);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33533);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 33534);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 33535);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 61634);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 61635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12629);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33536);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ран.
pub(crate) fn n2733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2723);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12630);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12631);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 33538);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 33539);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61638);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 61639);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88945);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109879);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 61640);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88946);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109880);
											s.is_empty()
										}
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
					super::push(out, 33541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61641);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 61642);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88947);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109881);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123615);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133264(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', '\u{1}', s @ ..] => {
			super::push(out, 12632);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 12633);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33543);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88948);
									s.is_empty()
								}
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
			super::push(out, 12634);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33544);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12635);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33545);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61645);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 33546);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61646);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33547);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61647);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88951);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109882);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123616);
													match s {
														[] => true,
														['в', s @ ..] => super::t36::n133265(s, out),
														_ => false,
													}
												}
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
							super::push(out, 61648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12636);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33548);
					s.is_empty()
				}
				['м', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61649);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 61650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33549);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33550);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 61651);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88952);
									s.is_empty()
								}
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
			super::push(out, 12637);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33551);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 33552);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 61652);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12638);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 33554);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 61654);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88954);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109883);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123617);
													match s {
														[] => true,
														['о', s @ ..] => super::t36::n133266(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12639);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61655);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 88955);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109884);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n12707(s, out),
		['п', s @ ..] => {
			super::push(out, 12641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33563);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61661);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t36::n89915(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t36::n12709(s, out),
		['с', s @ ..] => super::t36::n12710(s, out),
		['т', s @ ..] => super::t36::n12711(s, out),
		['у', s @ ..] => super::t36::n12712(s, out),
		['ф', s @ ..] => super::t36::n12713(s, out),
		['х', s @ ..] => super::t36::n12714(s, out),
		_ => false,
	}
}

/// Words ending -сан.
pub(crate) fn n2734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2724);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12648);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12649);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33574);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 33575);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61670);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88963);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109889);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123620);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 12650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12651);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12652);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 33579);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88964);
									s.is_empty()
								}
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
			super::push(out, 12653);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33580);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12654);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33582);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33583);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33584);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61675);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 88965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33585);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33586);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61677);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88966);
									s.is_empty()
								}
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
			super::push(out, 12657);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61678);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88967);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 61679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33589);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33590);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12658);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33591);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тан.
pub(crate) fn n2735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12659);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33592);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 33593);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 33594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 33595);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 61683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33596);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 61684);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 61685);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 61686);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88970);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109891);
											s.is_empty()
										}
										['ш', s @ ..] => {
											super::push(out, 109892);
											s.is_empty()
										}
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
					super::push(out, 33597);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 33598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61687);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 88971);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109893);
											s.is_empty()
										}
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
					super::push(out, 33599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33600);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 33601);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12661);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33602);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12662);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33604);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 33605);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61691);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33606);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 61692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88973);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 109894);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 123621);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12663);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33607);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61693);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 61694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12664);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12665);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61695);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 61696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33609);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 61698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61699);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 61700);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 61701);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88976);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109895);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12666);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33610);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 61702);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88977);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109896);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 123622);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133269(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33611);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61703);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 61704);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88979);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109897);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123623);
													match s {
														[] => true,
														['ь', s @ ..] => super::t36::n133270(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n12735(s, out),
		['п', s @ ..] => {
			super::push(out, 12668);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12669);
			s.is_empty()
		}
		['с', s @ ..] => super::t36::n12738(s, out),
		['т', s @ ..] => super::t36::n12739(s, out),
		['у', s @ ..] => super::t36::n12740(s, out),
		['ф', s @ ..] => {
			super::push(out, 12673);
			match s {
				[] => true,
				['а', s @ ..] => super::t36::n33956(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t36::n12742(s, out),
		['ш', s @ ..] => super::t36::n12743(s, out),
		['э', 'а', s @ ..] => super::t36::n12744(s, out),
		_ => false,
	}
}

/// Words ending -уан.
pub(crate) fn n2736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2726);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 12677);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33635);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12678);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33636);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61724);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88990);
									s.is_empty()
								}
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
			super::push(out, 12679);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33637);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 12680);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33638);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61726);
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
					super::push(out, 33639);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33640);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12682);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 12683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фан.
pub(crate) fn n2737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2727);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12684);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 12685);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61728);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88991);
									s.is_empty()
								}
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
			super::push(out, 12686);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33644);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12687);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33645);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61731);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33646);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61732);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 61733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88994);
									s.is_empty()
								}
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
			super::push(out, 12688);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33647);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12689);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 33648);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61735);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88995);
									s.is_empty()
								}
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
			super::push(out, 12690);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33649);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 61736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 33650);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61737);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88996);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109907);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123630);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33651);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33652);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61739);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33653);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61740);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88998);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109908);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 61741);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88999);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109909);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 61742);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 61743);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89000);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109910);
											s.is_empty()
										}
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
					super::push(out, 33654);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 33655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12691);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33656);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12692);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33657);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 12693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33658);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61748);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89001);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109911);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12694);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33659);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хан.
pub(crate) fn n2738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2728);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12695);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 12696);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33660);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61749);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33661);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61750);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89003);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109912);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33662);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 33663);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 33664);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33665);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33667);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 33668);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12698);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33669);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61756);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89005);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109913);
											s.is_empty()
										}
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
					super::push(out, 33670);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 33671);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12699);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33672);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33673);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 12700);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33675);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 61762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 12701);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33676);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12702);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33677);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 33678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61765);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 89006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33679);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61766);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33680);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89008);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 89009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 61768);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 89010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33681);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61769);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89011);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109914);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 89012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 61770);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 89013);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109915);
											s.is_empty()
										}
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
					super::push(out, 33682);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 33683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12703);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33684);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12704);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33685);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12705);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33686);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61775);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89014);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109916);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 61776);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89015);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109917);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12706);
			match s {
				[] => true,
				['а', s @ ..] => super::t36::n34012(s, out),
				['и', s @ ..] => super::t36::n34013(s, out),
				['ы', s @ ..] => {
					super::push(out, 33689);
					match s {
						[] => true,
						['р', s @ ..] => super::t36::n62436(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t36::n12776(s, out),
		['о', s @ ..] => super::t36::n12777(s, out),
		['п', s @ ..] => super::t36::n12778(s, out),
		['р', s @ ..] => super::t36::n12779(s, out),
		['с', s @ ..] => super::t36::n12780(s, out),
		['т', s @ ..] => super::t36::n12781(s, out),
		['у', s @ ..] => super::t36::n12782(s, out),
		['ч', s @ ..] => super::t36::n12783(s, out),
		['ш', s @ ..] => super::t36::n12784(s, out),
		['щ', s @ ..] => super::t36::n12785(s, out),
		['ы', s @ ..] => super::t36::n12786(s, out),
		['ь', s @ ..] => super::t36::n12787(s, out),
		['ю', s @ ..] => super::t36::n12788(s, out),
		_ => false,
	}
}

/// Words ending -цан.
pub(crate) fn n2739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2729);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12720);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33721);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 33722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чан.
pub(crate) fn n2740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2730);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12721);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12722);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33723);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 33724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12723);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33725);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61826);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89052);
									s.is_empty()
								}
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
			super::push(out, 12724);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33726);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12725);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12726);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33728);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шан.
pub(crate) fn n2741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2731);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12727);
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['е', '\u{1}', s @ ..] => {
					super::push(out, 33729);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33730);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12728);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 12729);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12730);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33733);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61831);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ер-штевен.
pub(crate) fn n124921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123637);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 131913);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 136800);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ген.
pub(crate) fn n2746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2736);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12749);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33751);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61844);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 61845);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 61846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 33752);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61847);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89060);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109945);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123638);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33753);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33754);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 61849);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 61850);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89062);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109946);
											s.is_empty()
										}
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
						['к', s @ ..] => {
							super::push(out, 61851);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 61852);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33755);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33756);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61854);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 33757);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12750);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 12751);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33758);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61856);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89066);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109947);
											s.is_empty()
										}
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
					super::push(out, 33759);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12752);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33760);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61858);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12753);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33762);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 33763);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61860);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89068);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109948);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123639);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33764);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61861);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89069);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109949);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123640);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133274(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'ф', s @ ..] => {
							super::push(out, 61862);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89070);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109950);
											s.is_empty()
										}
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
					super::push(out, 33765);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61863);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89071);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109951);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123641);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133275(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33766);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12754);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33768);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61864);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 89072);
									match s {
										[] => true,
										['и', s @ ..] => super::t36::n111120(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => super::t36::n62520(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t36::n34096(s, out),
				['ю', s @ ..] => {
					super::push(out, 33770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t36::n62522(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t36::n12825(s, out),
		['т', s @ ..] => super::t36::n12826(s, out),
		_ => false,
	}
}

/// The word аден.
pub(crate) fn n34099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33772);
	s.is_empty()
}

/// Words ending -баден.
pub(crate) fn n34100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33773);
	s.is_empty()
}

/// Words ending -ладен.
pub(crate) fn n34101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33774);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 61869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бден.
pub(crate) fn n12828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12758);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 33775);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61870);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89075);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зден.
pub(crate) fn n12829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12759);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33776);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 61871);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иден.
pub(crate) fn n12830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12760);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 33777);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 61872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йден.
pub(crate) fn n12831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12761);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33778);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 61873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 33779);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кден.
pub(crate) fn n12832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12762);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 33780);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61875);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нден.
pub(crate) fn n12833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12763);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 33781);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 61876);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оден.
pub(crate) fn n12834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12764);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рден.
pub(crate) fn n12835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12765);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33783);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61877);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 61878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 33784);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьден.
pub(crate) fn n12836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12766);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 61881);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 89078);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109955);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123643);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131917);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 61882);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 61883);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эден.
pub(crate) fn n12837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12767);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жен.
pub(crate) fn n2748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2738);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12768);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12769);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33789);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 33790);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61884);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89081);
									s.is_empty()
								}
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
			super::push(out, 12770);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 12771);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 33791);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 61885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зен.
pub(crate) fn n2749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2739);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12772);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 12773);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12774);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33793);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61887);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89083);
									s.is_empty()
								}
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
			super::push(out, 12775);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33794);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12776);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33795);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61889);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89084);
									s.is_empty()
								}
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
			super::push(out, 12777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33796);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 61890);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89085);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109956);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123644);
													match s {
														[] => true,
														['о', s @ ..] => super::t36::n133278(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33797);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61891);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89086);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109957);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123645);
													match s {
														[] => true,
														['л', s @ ..] => super::t36::n133279(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12778);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 33798);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 33799);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61892);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иен.
pub(crate) fn n2750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2740);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12780);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12781);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33802);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 61894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йен.
pub(crate) fn n2751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2741);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12782);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33803);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 33804);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кен.
pub(crate) fn n2752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2742);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12783);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33805);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61896);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33806);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12784);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33807);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 33808);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61898);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 61899);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109958);
											s.is_empty()
										}
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
					super::push(out, 33809);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61900);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 61901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89093);
									s.is_empty()
								}
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
			super::push(out, 12785);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33810);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 61902);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89094);
									s.is_empty()
								}
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
			super::push(out, 12786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12787);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33812);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61903);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89095);
									s.is_empty()
								}
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
			super::push(out, 12788);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33813);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61904);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12789);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33815);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33816);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12791);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33817);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61908);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лен.
pub(crate) fn n2753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2743);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12792);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33818);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 33819);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 33820);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61910);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89098);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109959);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123646);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12794);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33822);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61911);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89099);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 61912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12795);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33823);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61913);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33824);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 33825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12796);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 33827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61916);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 61917);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89101);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109960);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123647);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33828);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61918);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 89102);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109961);
											s.is_empty()
										}
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
					super::push(out, 33829);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33831);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61921);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89103);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 89104);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109962);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123648);
													s.is_empty()
												}
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
							super::push(out, 61922);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 61923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12797);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33832);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61924);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 89105);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109963);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12798);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33833);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61925);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33834);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61926);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89107);
									s.is_empty()
								}
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
			super::push(out, 12799);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33835);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 33836);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12800);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12801);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33837);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89108);
									s.is_empty()
								}
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
			super::push(out, 12802);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12803);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 12804);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 12805);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33839);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 61929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89109);
									s.is_empty()
								}
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
			super::push(out, 12806);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33840);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 33841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61930);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89110);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 89111);
									match s {
										[] => true,
										['а', s @ ..] => super::t36::n111132(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t36::n34169(s, out),
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 33843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => super::t36::n12877(s, out),
		_ => false,
	}
}

/// Words ending -мен.
pub(crate) fn n2754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2744);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12808);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33845);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 61933);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89114);
									s.is_empty()
								}
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
			super::push(out, 12809);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33846);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61934);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89115);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109965);
											s.is_empty()
										}
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
					super::push(out, 33847);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61935);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 89116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33848);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61936);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89117);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 109966);
											s.is_empty()
										}
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
					super::push(out, 33849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33850);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61938);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89118);
									s.is_empty()
								}
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
			super::push(out, 12810);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33851);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33852);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61940);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 33853);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61941);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89120);
									s.is_empty()
								}
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
			super::push(out, 12811);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33854);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61942);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89121);
									s.is_empty()
								}
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
			super::push(out, 12812);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33855);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61943);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109967);
											s.is_empty()
										}
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
					super::push(out, 33856);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61944);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 61945);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33857);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12813);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 33858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 12814);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33860);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 61948);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 61949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33861);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 33862);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61951);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89126);
									s.is_empty()
								}
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
			super::push(out, 12815);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33863);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 33864);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61952);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89127);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109968);
											s.is_empty()
										}
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
					super::push(out, 33865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12816);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33866);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61954);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89128);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109969);
											s.is_empty()
										}
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
					super::push(out, 33867);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61955);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 61956);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33868);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61957);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109970);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 61958);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33869);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61959);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89132);
									s.is_empty()
								}
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
			super::push(out, 12817);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33870);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61960);
							match s {
								[] => true,
								['б', s @ ..] => super::t36::n90090(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t36::n12888(s, out),
		['н', s @ ..] => super::t36::n12889(s, out),
		['о', s @ ..] => super::t36::n12890(s, out),
		['р', s @ ..] => super::t36::n12891(s, out),
		['с', s @ ..] => super::t36::n12892(s, out),
		['т', s @ ..] => super::t36::n12893(s, out),
		['у', s @ ..] => super::t36::n12894(s, out),
		['ц', s @ ..] => super::t36::n12895(s, out),
		['ш', s @ ..] => super::t36::n12896(s, out),
		['ь', s @ ..] => super::t36::n12897(s, out),
		['ю', s @ ..] => super::t36::n12898(s, out),
		_ => false,
	}
}

/// Words ending -нен.
pub(crate) fn n2755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2745);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 12829);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33911);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12830);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33912);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62014);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89170);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109989);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12831);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33913);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оен.
pub(crate) fn n2756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2746);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12833);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33915);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пен.
pub(crate) fn n2757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2747);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12834);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33916);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12835);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 33917);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12837);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33919);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 62019);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89171);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109990);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123658);
													s.is_empty()
												}
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
							super::push(out, 62020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рен.
pub(crate) fn n2758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2748);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12838);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12839);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33921);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62022);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 89172);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109991);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12840);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33922);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62023);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 62024);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 33923);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 33924);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62025);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 62026);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33925);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62027);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 89176);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 109992);
											s.is_empty()
										}
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
					super::push(out, 33926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12841);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33927);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 33928);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62029);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62030);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 62031);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89178);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109993);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123659);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133284(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12842);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12843);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33930);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 33931);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12844);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12845);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33933);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62034);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89179);
									s.is_empty()
								}
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
			super::push(out, 12846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33934);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62035);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89180);
									s.is_empty()
								}
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
			super::push(out, 12847);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33935);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 33936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12848);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33937);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 12849);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33939);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12850);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 12851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сен.
pub(crate) fn n2759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12852);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 12853);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33941);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62039);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89181);
									s.is_empty()
								}
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
			super::push(out, 12854);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 33942);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12855);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33943);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33944);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33945);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62043);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89182);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 62044);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 62045);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33946);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62046);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89185);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109995);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 89186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33947);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62047);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89187);
									s.is_empty()
								}
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
			super::push(out, 12856);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33948);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62048);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33949);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12858);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33951);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 33952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12859);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 33953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тен.
pub(crate) fn n2760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2750);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12860);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33954);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 33955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12861);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33956);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62056);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33957);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 62057);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89190);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109996);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 123660);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12862);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 12863);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33958);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12864);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33959);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62059);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89191);
									s.is_empty()
								}
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
			super::push(out, 12865);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12866);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33960);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62060);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89192);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109997);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123661);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 89193);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109998);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123662);
													s.is_empty()
												}
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
							super::push(out, 62061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12867);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33961);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33962);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33963);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62064);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89194);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 62065);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89195);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110000);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12868);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33964);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62066);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89196);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110001);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123663);
													match s {
														[] => true,
														['с', s @ ..] => super::t36::n133285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33965);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62067);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89197);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110002);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123664);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133286(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12869);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33966);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33967);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 62068);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89198);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110003);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 123665);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12870);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 12871);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 12872);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 12873);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33968);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word руен.
pub(crate) fn n2761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2751);
	s.is_empty()
}

/// Words ending -фен.
pub(crate) fn n2762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2752);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12874);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12875);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12876);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33970);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62070);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89199);
									s.is_empty()
								}
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
			super::push(out, 12877);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33971);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62071);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89200);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110004);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12878);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33972);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62072);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 89201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 62073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хен.
pub(crate) fn n2763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2753);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12879);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12880);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33973);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62075);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33976);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62076);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89203);
									s.is_empty()
								}
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
			super::push(out, 12881);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12882);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 33978);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цен.
pub(crate) fn n2764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2754);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12883);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12884);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33979);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62078);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89204);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110005);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123666);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12885);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33980);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62079);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89205);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110006);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12886);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33981);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62080);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 89206);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110007);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123667);
													match s {
														[] => true,
														['х', s @ ..] => super::t36::n133287(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чен.
pub(crate) fn n2765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2755);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12887);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33982);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12888);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 33983);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шен.
pub(crate) fn n2766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2756);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12889);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33984);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 62082);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89207);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 110008);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123668);
													match s {
														[] => true,
														['м', s @ ..] => super::t36::n133288(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12890);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33985);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12891);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33986);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33987);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 62085);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89208);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 110009);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123669);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12892);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12893);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 12894);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33989);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62086);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89209);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110010);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123670);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щен.
pub(crate) fn n2767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2757);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12895);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12896);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33990);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62087);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89210);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110011);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123671);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133290(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьен.
pub(crate) fn n2768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2758);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12897);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33991);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 62088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89211);
									s.is_empty()
								}
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
			super::push(out, 12898);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33992);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62089);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89212);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110012);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123672);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33993);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 62090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12899);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12900);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62091);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89213);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110013);
											s.is_empty()
										}
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
					super::push(out, 33996);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89214);
									s.is_empty()
								}
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
			super::push(out, 12901);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 33997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нька-каин.
pub(crate) fn n124957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123673);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 131931);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 136804);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хунвейбин.
pub(crate) fn n124958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123674);
	s.is_empty()
}

/// Words ending -хунвэйбин.
pub(crate) fn n124959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123675);
	s.is_empty()
}

/// Words ending -вин.
pub(crate) fn n2774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2764);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12919);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34017);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 62106);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89225);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110018);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 110019);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12920);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34018);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 12921);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34019);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62108);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 62109);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89227);
									s.is_empty()
								}
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
			super::push(out, 12922);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 34020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12923);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 12924);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12925);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34023);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12926);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 34024);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62113);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89228);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110020);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 123676);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 89229);
									s.is_empty()
								}
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
			super::push(out, 12927);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34025);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89230);
									s.is_empty()
								}
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
			super::push(out, 12928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34026);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34027);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62115);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34028);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62116);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89232);
									s.is_empty()
								}
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
			super::push(out, 12929);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34029);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 34030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12930);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 12931);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34031);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 34032);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12932);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34033);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89233);
									s.is_empty()
								}
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
			super::push(out, 12933);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34034);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62121);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 62122);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гин.
pub(crate) fn n2775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2765);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12934);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62123);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 62124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12935);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34037);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62125);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89236);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110021);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12936);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12937);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12938);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34039);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62127);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 34040);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34041);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34042);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62129);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89238);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110022);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12939);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 12940);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 34044);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62130);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89239);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 110023);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12941);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34045);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дин.
pub(crate) fn n2776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2766);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12942);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34046);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34047);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34048);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34049);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62134);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 62135);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89240);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110024);
											s.is_empty()
										}
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
					super::push(out, 34050);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34051);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62136);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89241);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110025);
											s.is_empty()
										}
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
					super::push(out, 34052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12943);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12944);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34054);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34055);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62138);
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
				['м', s @ ..] => {
					super::push(out, 34056);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34057);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62139);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 89242);
									s.is_empty()
								}
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
			super::push(out, 12945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34058);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62140);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89243);
									s.is_empty()
								}
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
			super::push(out, 12946);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34060);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34061);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62142);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89244);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110026);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 123677);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133292(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34063);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34064);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34066);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62145);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89246);
									s.is_empty()
								}
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
			super::push(out, 12948);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34067);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62146);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 89247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34068);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62147);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34069);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62148);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89249);
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
							super::push(out, 62149);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 62150);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89251);
									s.is_empty()
								}
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
			super::push(out, 12949);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34070);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34071);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62151);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89252);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 110027);
											s.is_empty()
										}
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
					super::push(out, 34072);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12950);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34073);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34074);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34075);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62154);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 62155);
							match s {
								[] => true,
								['р', s @ ..] => super::t36::n90211(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t36::n34405(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t36::n13022(s, out),
		['у', s @ ..] => super::t36::n13023(s, out),
		['х', s @ ..] => super::t36::n13024(s, out),
		['ь', s @ ..] => super::t36::n13025(s, out),
		['ю', s @ ..] => super::t36::n13026(s, out),
		['я', s @ ..] => super::t36::n13027(s, out),
		_ => false,
	}
}

/// Words ending -феин.
pub(crate) fn n13031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12960);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34090);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62164);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89261);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жин.
pub(crate) fn n2778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2768);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12961);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34091);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12962);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12963);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34094);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62165);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 62166);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89262);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110033);
											s.is_empty()
										}
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
					super::push(out, 34095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12964);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12965);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34097);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62168);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34098);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62169);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89264);
									s.is_empty()
								}
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
			super::push(out, 12966);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34099);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62170);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34100);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62171);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89266);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110034);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12967);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зин.
pub(crate) fn n2779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2769);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12968);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34102);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62173);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 34103);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62174);
							match s {
								[] => true,
								['м', '-', 'н', s @ ..] => {
									super::push(out, 89268);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110035);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123680);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133294(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34104);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 34105);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89269);
									s.is_empty()
								}
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
			super::push(out, 12969);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 34106);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62176);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89270);
									s.is_empty()
								}
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
			super::push(out, 12970);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 12971);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62177);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110036);
											s.is_empty()
										}
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
					super::push(out, 34108);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62178);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89272);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110037);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12972);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12973);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34110);
					match s {
						[] => true,
						['б', 'а', s @ ..] => {
							super::push(out, 62179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12974);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34111);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62180);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89273);
									s.is_empty()
								}
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
			super::push(out, 12975);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34112);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34113);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34114);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62181);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89274);
									s.is_empty()
								}
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
			super::push(out, 12976);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34115);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62182);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89275);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110038);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ийин.
pub(crate) fn n13048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12977);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 34116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кин.
pub(crate) fn n2781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2771);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12978);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34117);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62183);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89276);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110039);
											s.is_empty()
										}
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
					super::push(out, 34118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62184);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34119);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12979);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34120);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62186);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89278);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110040);
											s.is_empty()
										}
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
					super::push(out, 34121);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12980);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34122);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62188);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89279);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 110041);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123681);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133295(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34123);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62189);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12981);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12982);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34126);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62190);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89281);
									s.is_empty()
								}
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
			super::push(out, 12983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62191);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 62192);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 89282);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 110042);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123682);
													match s {
														[] => true,
														['о', s @ ..] => super::t36::n133296(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62193);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89283);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110043);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123683);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34128);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 62194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89284);
									s.is_empty()
								}
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
			super::push(out, 12984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34129);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62195);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89285);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110044);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 62196);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 62197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34130);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62198);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89287);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110045);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12985);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 34131);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62199);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 89288);
									s.is_empty()
								}
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
			super::push(out, 12986);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34132);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 34134);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 62202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t36::n13058(s, out),
		['т', s @ ..] => {
			super::push(out, 12988);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34140);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62208);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89294);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110050);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123686);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t36::n13060(s, out),
		['ш', s @ ..] => super::t36::n13061(s, out),
		['ь', s @ ..] => super::t36::n13062(s, out),
		_ => false,
	}
}

/// Words ending -лин.
pub(crate) fn n2782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2772);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12992);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34145);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62215);
							match s {
								[] => true,
								['р', 'д', 'а', 'р', s @ ..] => {
									super::push(out, 89300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34146);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62216);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89301);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 89302);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110054);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123689);
													s.is_empty()
												}
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
							super::push(out, 62217);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89303);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 110055);
											s.is_empty()
										}
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
					super::push(out, 34147);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62218);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89304);
									s.is_empty()
								}
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
			super::push(out, 12993);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34148);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34149);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62219);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 62220);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89306);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110056);
											s.is_empty()
										}
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
					super::push(out, 34150);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 34151);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62222);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89308);
									s.is_empty()
								}
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
			super::push(out, 12994);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34152);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12995);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34154);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 62225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 34155);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34156);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34157);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34159);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62227);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89310);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110057);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123690);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12996);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34161);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62228);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89311);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110058);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123691);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34162);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62229);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34163);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 62230);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89313);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110059);
											s.is_empty()
										}
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
					super::push(out, 34164);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62231);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 89314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34165);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62232);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89315);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110060);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													super::push(out, 123692);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62233);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 62234);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 62235);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89317);
									match s {
										[] => true,
										['и', s @ ..] => super::t36::n111229(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t36::n13068(s, out),
		['л', s @ ..] => super::t36::n13069(s, out),
		['м', s @ ..] => super::t36::n13070(s, out),
		['о', s @ ..] => super::t36::n13071(s, out),
		['п', s @ ..] => super::t36::n13072(s, out),
		['р', s @ ..] => super::t36::n13073(s, out),
		['с', s @ ..] => super::t36::n13074(s, out),
		['т', s @ ..] => super::t36::n13075(s, out),
		['у', s @ ..] => super::t36::n13076(s, out),
		_ => false,
	}
}

/// Words ending -мин.
pub(crate) fn n2783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2773);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13006);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13007);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 34198);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62270);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89339);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110074);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123698);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133299(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
							super::push(out, 62271);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 62272);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89340);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110075);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123699);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62273);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 62274);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 62275);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34200);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62276);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34201);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62277);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89343);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 89344);
									s.is_empty()
								}
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
			super::push(out, 13008);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34202);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62278);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89345);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110076);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13009);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 13010);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 34203);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 62279);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110077);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13011);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34204);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13012);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34206);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34207);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62281);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 62282);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 62283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89347);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 89348);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110078);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123700);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133300(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13013);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34208);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 62284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13014);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34209);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 62285);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34210);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62286);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89350);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 110079);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123701);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34211);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62287);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 89351);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110080);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123702);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133301(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13015);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 34212);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62288);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89352);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110081);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нин.
pub(crate) fn n2784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2774);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13016);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34213);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62289);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89353);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110082);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123703);
													s.is_empty()
												}
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
							super::push(out, 62290);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89354);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110083);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123704);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 89355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 34214);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62291);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 62292);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89356);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110084);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 123705);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34215);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62293);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 34216);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62294);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89358);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110085);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 123706);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133302(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62295);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89359);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110086);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123707);
													match s {
														[] => true,
														['х', s @ ..] => super::t36::n133303(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34217);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62297);
							s.is_empty()
						}
						['н', s @ ..] => super::t36::n62953(s, out),
						['р', s @ ..] => {
							super::push(out, 62299);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 62300);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t36::n62956(s, out),
						['ф', s @ ..] => super::t36::n62957(s, out),
						['ц', s @ ..] => super::t36::n62958(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t36::n34548(s, out),
				['л', s @ ..] => super::t36::n34549(s, out),
				['р', s @ ..] => super::t36::n34550(s, out),
				['т', s @ ..] => super::t36::n34551(s, out),
				['у', s @ ..] => super::t36::n34552(s, out),
				['ч', s @ ..] => super::t36::n34553(s, out),
				['ш', s @ ..] => super::t36::n34554(s, out),
				['щ', s @ ..] => super::t36::n34555(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t36::n13088(s, out),
		['е', s @ ..] => super::t36::n13089(s, out),
		['и', s @ ..] => super::t36::n13090(s, out),
		['й', s @ ..] => super::t36::n13091(s, out),
		['м', s @ ..] => super::t36::n13092(s, out),
		['н', s @ ..] => super::t36::n13093(s, out),
		['о', s @ ..] => super::t36::n13094(s, out),
		['у', s @ ..] => super::t36::n13095(s, out),
		['х', s @ ..] => super::t36::n13096(s, out),
		['я', s @ ..] => super::t36::n13097(s, out),
		_ => false,
	}
}

/// Words ending -оин.
pub(crate) fn n2785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2775);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13027);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 34264);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62376);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89431);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110144);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123738);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133324(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13028);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34265);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34266);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13029);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34267);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 34268);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62379);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34269);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13030);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34270);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62381);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пин.
pub(crate) fn n2786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2776);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13031);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13032);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13033);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13034);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34272);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62383);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89434);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 89435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 34273);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89436);
									s.is_empty()
								}
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
			super::push(out, 13035);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 13036);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13037);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 13038);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34274);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62385);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 89437);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 110145);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13039);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рин.
pub(crate) fn n2787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2777);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13040);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 34277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62388);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 62389);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89439);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110147);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 62390);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89440);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110148);
											s.is_empty()
										}
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
					super::push(out, 34278);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62391);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89441);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110149);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 89442);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110150);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123740);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34279);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62392);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 34280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62393);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89444);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 62394);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 62395);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 62396);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89445);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110152);
											s.is_empty()
										}
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
					super::push(out, 34281);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 62397);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89446);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 110153);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 123741);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34282);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62398);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89447);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 89448);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 89449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 62399);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89450);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110154);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123742);
													match s {
														[] => true,
														['г', s @ ..] => super::t36::n133325(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62400);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 62401);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34283);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62402);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34284);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62403);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 62404);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 34285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62405);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89455);
									s.is_empty()
								}
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
			super::push(out, 13041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34286);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34287);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t36::n13113(s, out),
		['д', s @ ..] => super::t36::n13114(s, out),
		['е', s @ ..] => super::t36::n13115(s, out),
		['и', s @ ..] => super::t36::n13116(s, out),
		['й', s @ ..] => {
			super::push(out, 13046);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34302);
					match s {
						[] => true,
						['к', s @ ..] => super::t36::n63074(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t36::n13118(s, out),
		['о', s @ ..] => super::t36::n13119(s, out),
		['п', s @ ..] => super::t36::n13120(s, out),
		['р', s @ ..] => super::t36::n13121(s, out),
		['т', s @ ..] => super::t36::n13122(s, out),
		['у', s @ ..] => super::t36::n13123(s, out),
		['ф', s @ ..] => super::t36::n13124(s, out),
		['ю', s @ ..] => super::t36::n13125(s, out),
		['я', s @ ..] => super::t36::n13126(s, out),
		_ => false,
	}
}

/// Words ending -син.
pub(crate) fn n2788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2778);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13056);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62431);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 62432);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34316);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62433);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89469);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110161);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13057);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13058);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34317);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 62434);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89470);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110162);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123746);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13059);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34318);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62435);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89471);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110163);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123747);
													s.is_empty()
												}
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
							super::push(out, 62436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89472);
									s.is_empty()
								}
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
			super::push(out, 13060);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34319);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62437);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 89473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34320);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 34321);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 62438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13061);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34322);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62439);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89474);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110164);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123748);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13062);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34323);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13063);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 13064);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34325);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62441);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 62442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13065);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 34327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13066);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тин.
pub(crate) fn n2789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2779);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13067);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13068);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34329);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62445);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 62446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34331);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62447);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89475);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110165);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123749);
													s.is_empty()
												}
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
							super::push(out, 62448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34332);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62449);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89476);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110166);
											s.is_empty()
										}
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
					super::push(out, 34333);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34334);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13069);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34336);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34337);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62451);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89477);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110167);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 62452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89478);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 89479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34339);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62453);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 62454);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89481);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 89482);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 110168);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123750);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13070);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34341);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62455);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89483);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110169);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123751);
													s.is_empty()
												}
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
							super::push(out, 62456);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89484);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110170);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123752);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34342);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62457);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89485);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110171);
											s.is_empty()
										}
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
					super::push(out, 34343);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 62458);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 62459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34344);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62460);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89487);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110173);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123753);
													match s {
														[] => true,
														['к', s @ ..] => super::t36::n133328(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34345);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 34346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13071);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13072);
			match s {
				[] => true,
				['а', s @ ..] => super::t36::n34679(s, out),
				['е', s @ ..] => super::t36::n34680(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n13144(s, out),
		['р', s @ ..] => super::t36::n13145(s, out),
		['с', s @ ..] => super::t36::n13146(s, out),
		['т', s @ ..] => {
			super::push(out, 13076);
			match s {
				[] => true,
				['е', s @ ..] => super::t36::n34692(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t36::n13148(s, out),
		['х', s @ ..] => super::t36::n13149(s, out),
		['ш', s @ ..] => super::t36::n13150(s, out),
		['ь', s @ ..] => super::t36::n13151(s, out),
		['э', s @ ..] => super::t36::n13152(s, out),
		['ю', s @ ..] => super::t36::n13153(s, out),
		_ => false,
	}
}

/// Words ending -уин.
pub(crate) fn n2790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2780);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34365);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 13084);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34366);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13085);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34367);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 13086);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34368);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62493);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89517);
									s.is_empty()
								}
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
			super::push(out, 13087);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34369);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62494);
							match s {
								[] => true,
								['е', 'х', '\u{1}', s @ ..] => {
									super::push(out, 89518);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 89519);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110191);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123764);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фин.
pub(crate) fn n2791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2781);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13088);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34370);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62495);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 89520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 62496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13089);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34371);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62497);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 89521);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110192);
											s.is_empty()
										}
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
					super::push(out, 34372);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13090);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34373);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13091);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13092);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34375);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13093);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 13094);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34377);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 34378);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89522);
									s.is_empty()
								}
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
			super::push(out, 13095);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34379);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89523);
									s.is_empty()
								}
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
			super::push(out, 13096);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34380);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62504);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хин.
pub(crate) fn n2792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2782);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13097);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13098);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13099);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цин.
pub(crate) fn n2793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2783);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13101);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34381);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34382);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34383);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62506);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 62507);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 62508);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13102);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 34385);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13103);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13104);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13105);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13106);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13107);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34388);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34389);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чин.
pub(crate) fn n2794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2784);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13108);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13109);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34390);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34393);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34394);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62513);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89527);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13110);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34395);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62514);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89528);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110194);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13111);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13112);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 34397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13113);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 34398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шин.
pub(crate) fn n2795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2785);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13114);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13115);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 34400);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13116);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34401);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 62519);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89529);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110195);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 62520);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89530);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110196);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13117);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34402);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13118);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34403);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89531);
									s.is_empty()
								}
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
			super::push(out, 13119);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34404);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34405);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89532);
									s.is_empty()
								}
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
			super::push(out, 13120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13121);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34407);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62525);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яин.
pub(crate) fn n2796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2786);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 13122);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34408);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62526);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89534);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110197);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123765);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 110198);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 123766);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133334(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 110199);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123767);
													match s {
														[] => true,
														['д', s @ ..] => super::t36::n133335(s, out),
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
									super::push(out, 89535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штурмманн.
pub(crate) fn n125055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123771);
	s.is_empty()
}

/// Words ending -стиансанн.
pub(crate) fn n125056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123772);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 131976);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -долбозвон.
pub(crate) fn n125057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123773);
	s.is_empty()
}

/// Words ending -гон.
pub(crate) fn n2812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13166);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34449);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34450);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62560);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 62561);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89559);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 110214);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123774);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34452);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62562);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34453);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62563);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89561);
									s.is_empty()
								}
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
			super::push(out, 13167);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13168);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13169);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34455);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89562);
									s.is_empty()
								}
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
			super::push(out, 13171);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34456);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62565);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34457);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 62566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'т', s @ ..] => {
					super::push(out, 34458);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62567);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89565);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110215);
											s.is_empty()
										}
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
					super::push(out, 34459);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62568);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89566);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110216);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13172);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34460);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 34461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62569);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 62570);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 62571);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 89568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 34462);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34463);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62573);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 62574);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 89570);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110217);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123775);
													s.is_empty()
												}
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
							super::push(out, 62575);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89571);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110218);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123776);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34464);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62576);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89572);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 62577);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 89573);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110220);
											s.is_empty()
										}
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
					super::push(out, 34465);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62578);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89574);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t36::n34798(s, out),
				['п', s @ ..] => {
					super::push(out, 34466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t36::n34800(s, out),
				['с', s @ ..] => super::t36::n34801(s, out),
				['т', s @ ..] => super::t36::n34802(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t36::n13244(s, out),
		['с', s @ ..] => {
			super::push(out, 13174);
			s.is_empty()
		}
		['у', s @ ..] => super::t36::n13246(s, out),
		['ы', s @ ..] => super::t36::n13247(s, out),
		_ => false,
	}
}

/// Words ending -дон.
pub(crate) fn n2813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2803);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13177);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 13178);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13179);
			match s {
				[] => true,
				['л', 'е', 'с', '\u{1}', s @ ..] => {
					super::push(out, 34472);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34473);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62593);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34474);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34475);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62594);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89589);
									s.is_empty()
								}
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
			super::push(out, 13180);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13181);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13182);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34478);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13183);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34479);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62598);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89590);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 89591);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110228);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123779);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34480);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62599);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89592);
									s.is_empty()
								}
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
			super::push(out, 13184);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34481);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13185);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34482);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34483);
					s.is_empty()
				}
				['п', 'у', s @ ..] => {
					super::push(out, 34484);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62601);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'и', s @ ..] => {
					super::push(out, 34485);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62602);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', 'у', 'к', 'с', '\u{1}', s @ ..] => {
					super::push(out, 34486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13186);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34487);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62603);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89595);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110230);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13187);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34488);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 62604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34489);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 62605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34490);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13188);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34491);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62607);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 62608);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89597);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 89598);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110231);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123780);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133339(s, out),
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
									super::push(out, 89599);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110232);
											s.is_empty()
										}
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
					super::push(out, 34492);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62609);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110233);
											s.is_empty()
										}
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
					super::push(out, 34493);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62610);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 62611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n13260(s, out),
		['р', s @ ..] => {
			super::push(out, 13190);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34499);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34500);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89610);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 62622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 34501);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13191);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 13192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -амелеон.
pub(crate) fn n90571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89612);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 110238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -клеон.
pub(crate) fn n34842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34507);
	s.is_empty()
}

/// Words ending -меон.
pub(crate) fn n13266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13194);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34508);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неон.
pub(crate) fn n13267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13195);
	s.is_empty()
}

/// Words ending -пеон.
pub(crate) fn n13268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13196);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 34509);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 62628);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89614);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110239);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123783);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реон.
pub(crate) fn n13269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13197);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34510);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 62629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 34511);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 34512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теон.
pub(crate) fn n13270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13198);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 34513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жон.
pub(crate) fn n2815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2805);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13199);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34514);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 34515);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62630);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89616);
									s.is_empty()
								}
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
					super::push(out, 34516);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13200);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34518);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13201);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34519);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13202);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34520);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62634);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89617);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110240);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13203);
			s.is_empty()
		}
		_ => false,
	}
}

