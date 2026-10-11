use flexion::Path;

/// Words ending -ба.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4970);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4971);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 4972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20013);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 20014);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 45106);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 73729);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 98590);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 45107);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 73730);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 98591);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 45108);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 73731);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 98592);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 45109);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 73732);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 98593);
													s.is_empty()
												}
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
							super::push(out, 20015);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 20016);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 45110);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 73733);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 98594);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20019);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20020);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 45111);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 73734);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 98595);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4976);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20022);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20023);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 45112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4979);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20025);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 45113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4980);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20026);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 45114);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 73735);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 98596);
													s.is_empty()
												}
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
							super::push(out, 20027);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 45115);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 73736);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 98597);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20029);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 20030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4983);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t9::n605(s, out),
		['ж', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4993);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20043);
							match s {
								[] => true,
								['о', s @ ..] => super::t9::n45560(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t9::n5005(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t9::n607(s, out),
		['и', s @ ..] => super::t9::n608(s, out),
		['й', s @ ..] => super::t9::n609(s, out),
		['л', s @ ..] => super::t9::n610(s, out),
		['м', s @ ..] => super::t9::n611(s, out),
		['о', s @ ..] => super::t9::n612(s, out),
		['р', s @ ..] => super::t9::n613(s, out),
		['с', s @ ..] => super::t9::n614(s, out),
		['т', s @ ..] => super::t9::n615(s, out),
		['у', s @ ..] => super::t9::n616(s, out),
		['ч', s @ ..] => super::t9::n617(s, out),
		['ш', s @ ..] => super::t9::n618(s, out),
		['ы', s @ ..] => super::t9::n619(s, out),
		['ь', s @ ..] => super::t9::n620(s, out),
		['ю', s @ ..] => super::t9::n621(s, out),
		['я', s @ ..] => super::t9::n622(s, out),
		_ => false,
	}
}

/// Words ending -ва.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5053);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45214);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 45215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 20145);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 45216);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 73796);
											s.is_empty()
										}
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
					super::push(out, 5054);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 5055);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20146);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 45217);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 73797);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 98630);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5056);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20147);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 45218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5057);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20148);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20149);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 45219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 20150);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 45220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 20151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45221);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 45222);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 73798);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 98631);
													match s {
														[] => true,
														['э', s @ ..] => super::t9::n117990(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20152);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 45223);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 73799);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 98632);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 73800);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 98633);
													match s {
														[] => true,
														['б', s @ ..] => super::t9::n117991(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t9::n20259(s, out),
						['у', s @ ..] => super::t9::n20260(s, out),
						['х', s @ ..] => super::t9::n20261(s, out),
						['ц', s @ ..] => super::t9::n20262(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t9::n5070(s, out),
				['н', s @ ..] => super::t9::n5071(s, out),
				['п', s @ ..] => super::t9::n5072(s, out),
				['р', s @ ..] => super::t9::n5073(s, out),
				['с', s @ ..] => super::t9::n5074(s, out),
				['т', s @ ..] => super::t9::n5075(s, out),
				['у', s @ ..] => super::t9::n5076(s, out),
				['х', s @ ..] => {
					super::push(out, 5066);
					s.is_empty()
				}
				['ш', s @ ..] => super::t9::n5078(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t9::n624(s, out),
		['в', s @ ..] => super::t9::n625(s, out),
		['г', s @ ..] => super::t9::n626(s, out),
		['д', s @ ..] => super::t9::n627(s, out),
		['е', s @ ..] => super::t9::n628(s, out),
		['з', s @ ..] => super::t9::n629(s, out),
		['и', s @ ..] => super::t9::n630(s, out),
		['й', s @ ..] => super::t9::n631(s, out),
		['к', s @ ..] => super::t9::n632(s, out),
		['л', s @ ..] => super::t9::n633(s, out),
		['м', s @ ..] => super::t9::n634(s, out),
		['н', s @ ..] => super::t9::n635(s, out),
		['о', s @ ..] => super::t9::n636(s, out),
		['р', s @ ..] => super::t9::n637(s, out),
		['с', s @ ..] => super::t9::n638(s, out),
		['т', s @ ..] => super::t9::n639(s, out),
		['у', s @ ..] => super::t9::n640(s, out),
		['х', s @ ..] => super::t9::n641(s, out),
		['ч', s @ ..] => super::t9::n642(s, out),
		['ш', s @ ..] => super::t9::n643(s, out),
		['ы', s @ ..] => super::t9::n644(s, out),
		['ь', s @ ..] => super::t9::n645(s, out),
		['э', s @ ..] => super::t9::n646(s, out),
		['я', s @ ..] => super::t9::n647(s, out),
		_ => false,
	}
}

/// Words ending -га.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5158);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5159);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20302);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 45349);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 73867);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20303);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 45350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5161);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20305);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 45351);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 45352);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 73868);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 20306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45353);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 45354);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 73869);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 98665);
													match s {
														[] => true,
														['с', s @ ..] => super::t9::n118010(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20307);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 20308);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 45355);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 73870);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20309);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 45356);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 73871);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 98666);
													match s {
														[] => true,
														['г', s @ ..] => super::t9::n118011(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 73872);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 98667);
													match s {
														[] => true,
														['м', s @ ..] => super::t9::n118012(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20310);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 45357);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 73873);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 98668);
													s.is_empty()
												}
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
							super::push(out, 20311);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 45358);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 45359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5162);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20312);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 45360);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 73874);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 98669);
													match s {
														[] => true,
														['к', s @ ..] => super::t9::n118013(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20313);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20314);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 45361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 20315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 5163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5164);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 20317);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 20318);
							match s {
								[] => true,
								['л', s @ ..] => super::t9::n45797(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t9::n5176(s, out),
				['с', s @ ..] => {
					super::push(out, 5166);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5167);
					s.is_empty()
				}
				['ч', s @ ..] => super::t9::n5179(s, out),
				['щ', s @ ..] => super::t9::n5180(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t9::n649(s, out),
		['е', s @ ..] => super::t9::n650(s, out),
		['з', s @ ..] => super::t9::n651(s, out),
		['и', s @ ..] => super::t9::n652(s, out),
		['й', s @ ..] => super::t9::n653(s, out),
		['л', s @ ..] => super::t9::n654(s, out),
		['м', s @ ..] => super::t9::n655(s, out),
		['н', s @ ..] => super::t9::n656(s, out),
		['о', s @ ..] => super::t10::n657(s, out),
		['р', s @ ..] => super::t10::n658(s, out),
		['с', s @ ..] => super::t10::n659(s, out),
		['у', s @ ..] => super::t10::n660(s, out),
		['ы', s @ ..] => super::t10::n661(s, out),
		['ь', s @ ..] => super::t10::n662(s, out),
		['э', s @ ..] => super::t10::n663(s, out),
		['ю', s @ ..] => super::t10::n664(s, out),
		['я', s @ ..] => super::t10::n665(s, out),
		_ => false,
	}
}

/// Words ending -да.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5279);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20565);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 45596);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 74036);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 98755);
													match s {
														[] => true,
														['т', s @ ..] => super::t10::n118050(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20566);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20568);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 45598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5283);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20570);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 45599);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74037);
											s.is_empty()
										}
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
					super::push(out, 5284);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20571);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 45600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74038);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20572);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 45601);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 74039);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 98756);
													match s {
														[] => true,
														['ш', s @ ..] => super::t10::n118051(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5285);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20574);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 45603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74040);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 74041);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 98757);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n118052(s, out),
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
									super::push(out, 45604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5286);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20575);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 45605);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 45606);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74042);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20577);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20579);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 45608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74043);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20580);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5288);
					match s {
						[] => true,
						['а', s @ ..] => super::t10::n20691(s, out),
						['о', s @ ..] => {
							super::push(out, 20584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t10::n5301(s, out),
				['п', s @ ..] => super::t10::n5302(s, out),
				['р', s @ ..] => super::t10::n5303(s, out),
				['с', s @ ..] => super::t10::n5304(s, out),
				['т', s @ ..] => super::t10::n5305(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t10::n667(s, out),
		['в', s @ ..] => super::t10::n668(s, out),
		['г', s @ ..] => super::t10::n669(s, out),
		['д', s @ ..] => super::t10::n670(s, out),
		['е', s @ ..] => super::t10::n671(s, out),
		['ж', s @ ..] => super::t10::n672(s, out),
		['з', s @ ..] => super::t10::n673(s, out),
		['и', s @ ..] => super::t10::n674(s, out),
		['й', s @ ..] => super::t10::n675(s, out),
		['л', s @ ..] => super::t10::n676(s, out),
		['м', s @ ..] => super::t10::n677(s, out),
		['н', s @ ..] => super::t10::n678(s, out),
		['о', s @ ..] => super::t10::n679(s, out),
		['р', s @ ..] => super::t10::n680(s, out),
		['у', s @ ..] => super::t10::n681(s, out),
		['ы', s @ ..] => super::t10::n682(s, out),
		['ь', s @ ..] => super::t10::n683(s, out),
		['э', s @ ..] => super::t10::n684(s, out),
		['ю', s @ ..] => super::t10::n685(s, out),
		['я', s @ ..] => super::t10::n686(s, out),
		_ => false,
	}
}

/// Words ending -рокорнеа.
pub(crate) fn n99942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98870);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 116850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -г-жа.
pub(crate) fn n5420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5408);
	s.is_empty()
}

/// Words ending -ажа.
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5409);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20857);
					match s {
						[] => true,
						['р', 'п', '-', s @ ..] => {
							super::push(out, 45873);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74239);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98871);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 116851);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n129577(s, out),
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
									super::push(out, 74240);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98872);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116852);
													match s {
														[] => true,
														['у', s @ ..] => super::t10::n129578(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5410);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5411);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5412);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 20858);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45874);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74241);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98873);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116853);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бжа.
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 692);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -джа.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5414);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45875);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 45876);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74242);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98874);
											match s {
												[] => true,
												['м', '\u{1}', s @ ..] => {
													super::push(out, 116854);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 74243);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98875);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 116855);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 20862);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 45878);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74244);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98876);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 116856);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5415);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20864);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45880);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 45881);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74245);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98877);
											s.is_empty()
										}
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
					super::push(out, 20865);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 45882);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74246);
									s.is_empty()
								}
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
			super::push(out, 5417);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20866);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45883);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74247);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98878);
											s.is_empty()
										}
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
					super::push(out, 20867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5418);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20868);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20869);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ежа.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5419);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20870);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5420);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20871);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20872);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5422);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20875);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45888);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 45889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жжа.
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5423);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20877);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ижа.
pub(crate) fn n697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 696);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 5424);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5425);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20878);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74249);
									s.is_empty()
								}
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
			super::push(out, 5426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20879);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45891);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 74250);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98879);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116857);
													match s {
														[] => true,
														['п', s @ ..] => super::t10::n129579(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лжа.
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 697);
	s.is_empty()
}

/// Words ending -мжа.
pub(crate) fn n699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 698);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5428);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5429);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжа.
pub(crate) fn n700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5430);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5431);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5432);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20883);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожа.
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 5433);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5434);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5435);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 5436);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 20885);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45894);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74251);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 98880);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5437);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20886);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45895);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74252);
									s.is_empty()
								}
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
			super::push(out, 5438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ржа.
pub(crate) fn n702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5439);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20887);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20889);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5440);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужа.
pub(crate) fn n703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 702);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5441);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5442);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5443);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжа.
pub(crate) fn n704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5444);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5445);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20892);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьжа.
pub(crate) fn n705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 704);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 5446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20893);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжа.
pub(crate) fn n706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 705);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5447);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20894);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -за.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5448);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5449);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45899);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 45900);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 74254);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 98881);
													s.is_empty()
												}
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
							super::push(out, 20896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5450);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20897);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 45901);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 74255);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 98882);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n118117(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5451);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5452);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5453);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20899);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 45902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74256);
											s.is_empty()
										}
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
					super::push(out, 5454);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5455);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 45903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20901);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 20902);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 45904);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 74257);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 98883);
													match s {
														[] => true,
														['т', s @ ..] => super::t10::n118118(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20903);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 45905);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 74258);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 98884);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5456);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5457);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5458);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20905);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 45906);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 74259);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 20906);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20907);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 45907);
									match s {
										[] => true,
										['и', s @ ..] => super::t10::n75037(s, out),
										_ => false,
									}
								}
								['ф', s @ ..] => super::t10::n46347(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => super::t10::n21020(s, out),
						['ф', s @ ..] => {
							super::push(out, 20909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t10::n5471(s, out),
				['у', s @ ..] => super::t10::n5472(s, out),
				['ф', s @ ..] => super::t10::n5473(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t10::n708(s, out),
		['д', s @ ..] => super::t10::n709(s, out),
		['е', s @ ..] => super::t10::n710(s, out),
		['и', s @ ..] => super::t10::n711(s, out),
		['й', s @ ..] => super::t10::n712(s, out),
		['л', s @ ..] => super::t10::n713(s, out),
		['м', s @ ..] => super::t10::n714(s, out),
		['н', s @ ..] => super::t10::n715(s, out),
		['о', s @ ..] => super::t10::n716(s, out),
		['р', s @ ..] => super::t10::n717(s, out),
		['у', s @ ..] => super::t10::n718(s, out),
		['ы', s @ ..] => super::t10::n719(s, out),
		['ь', s @ ..] => super::t10::n720(s, out),
		['ю', s @ ..] => super::t10::n721(s, out),
		['я', s @ ..] => super::t10::n722(s, out),
		_ => false,
	}
}

/// Words ending -иа.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5534);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21022);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 46002);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 74315);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 98914);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n118131(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 723);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5535);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 21023);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 46003);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 74316);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 98915);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n118132(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ч', s @ ..] => {
											super::push(out, 74317);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 98916);
													match s {
														[] => true,
														['р', s @ ..] => super::t10::n118133(s, out),
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
									super::push(out, 46004);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 74318);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 98917);
													match s {
														[] => true,
														['л', s @ ..] => super::t10::n118134(s, out),
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
									super::push(out, 46005);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 74319);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 98918);
													match s {
														[] => true,
														['и', s @ ..] => super::t10::n118135(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 724);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5536);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21024);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 46006);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 74320);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 98919);
													s.is_empty()
												}
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
							super::push(out, 21025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ка.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5537);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21026);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 46007);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 74321);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 98920);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5538);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 46008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21028);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21029);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 46009);
									match s {
										[] => true,
										['о', 'н', 'и', 'в', 'с', '\u{1}', s @ ..] => {
											super::push(out, 74322);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 21030);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 46010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5539);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21031);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 46011);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 74323);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21032);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 46012);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 46013);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 74324);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21033);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 46014);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 74325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21034);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 46015);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 74326);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 98921);
													match s {
														[] => true,
														['н', s @ ..] => super::t10::n118136(s, out),
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
									super::push(out, 46016);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 74327);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 98922);
													match s {
														[] => true,
														['н', s @ ..] => super::t10::n118137(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5540);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21035);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 46017);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 74328);
											s.is_empty()
										}
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
					super::push(out, 5541);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21036);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21037);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 46018);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 74329);
											s.is_empty()
										}
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
					super::push(out, 5542);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21038);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 46019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5543);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21039);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 46020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 21040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5544);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21041);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 46021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t10::n5557(s, out),
				['о', s @ ..] => {
					super::push(out, 5546);
					match s {
						[] => true,
						['л', s @ ..] => super::t10::n21156(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t10::n5559(s, out),
				['р', s @ ..] => super::t10::n5560(s, out),
				['с', s @ ..] => super::t10::n5561(s, out),
				['т', s @ ..] => super::t10::n5562(s, out),
				['х', s @ ..] => super::t10::n5563(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t10::n727(s, out),
		['в', s @ ..] => super::t10::n728(s, out),
		['г', s @ ..] => super::t10::n729(s, out),
		['д', s @ ..] => super::t10::n730(s, out),
		['е', s @ ..] => super::t10::n731(s, out),
		['ж', s @ ..] => super::t10::n732(s, out),
		['з', s @ ..] => super::t10::n733(s, out),
		['и', s @ ..] => super::t10::n734(s, out),
		['й', s @ ..] => super::t10::n735(s, out),
		['к', s @ ..] => super::t10::n736(s, out),
		['л', s @ ..] => super::t10::n737(s, out),
		['м', s @ ..] => super::t10::n738(s, out),
		['н', s @ ..] => super::t10::n739(s, out),
		['о', s @ ..] => super::t10::n740(s, out),
		['п', s @ ..] => super::t11::n741(s, out),
		['р', s @ ..] => super::t11::n742(s, out),
		['с', s @ ..] => super::t11::n743(s, out),
		['т', s @ ..] => super::t11::n744(s, out),
		['у', s @ ..] => super::t11::n745(s, out),
		['ф', s @ ..] => super::t11::n746(s, out),
		['х', s @ ..] => super::t11::n747(s, out),
		['ц', s @ ..] => super::t11::n748(s, out),
		['ч', s @ ..] => super::t11::n749(s, out),
		['ш', s @ ..] => super::t11::n750(s, out),
		['ы', s @ ..] => super::t11::n751(s, out),
		['ь', s @ ..] => super::t11::n752(s, out),
		['э', s @ ..] => super::t11::n753(s, out),
		['ю', s @ ..] => super::t11::n754(s, out),
		['я', s @ ..] => super::t11::n755(s, out),
		_ => false,
	}
}

/// Words ending -ла.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5812);
					s.is_empty()
				}
				['б', s @ ..] => super::t11::n5828(s, out),
				['в', s @ ..] => super::t11::n5829(s, out),
				['г', s @ ..] => {
					super::push(out, 5815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22400);
							s.is_empty()
						}
						['а', s @ ..] => super::t11::n22533(s, out),
						['е', s @ ..] => super::t11::n22534(s, out),
						['и', s @ ..] => super::t11::n22535(s, out),
						['н', s @ ..] => super::t11::n22536(s, out),
						['о', s @ ..] => super::t11::n22537(s, out),
						_ => false,
					}
				}
				['д', s @ ..] => super::t11::n5831(s, out),
				['е', s @ ..] => super::t11::n5832(s, out),
				['ж', s @ ..] => super::t11::n5833(s, out),
				['з', s @ ..] => super::t11::n5834(s, out),
				['и', s @ ..] => super::t11::n5835(s, out),
				['к', s @ ..] => super::t11::n5836(s, out),
				['м', s @ ..] => super::t11::n5837(s, out),
				['н', s @ ..] => super::t11::n5838(s, out),
				['о', s @ ..] => super::t11::n5839(s, out),
				['п', s @ ..] => super::t11::n5840(s, out),
				['р', s @ ..] => super::t11::n5841(s, out),
				['с', s @ ..] => super::t11::n5842(s, out),
				['т', s @ ..] => super::t11::n5843(s, out),
				['у', s @ ..] => super::t11::n5844(s, out),
				['х', s @ ..] => {
					super::push(out, 5830);
					s.is_empty()
				}
				['ц', s @ ..] => super::t11::n5846(s, out),
				['ш', s @ ..] => super::t11::n5847(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t11::n757(s, out),
		['в', s @ ..] => super::t11::n758(s, out),
		['г', s @ ..] => super::t11::n759(s, out),
		['д', s @ ..] => super::t11::n760(s, out),
		['е', s @ ..] => super::t11::n761(s, out),
		['з', s @ ..] => super::t11::n762(s, out),
		['и', s @ ..] => super::t11::n763(s, out),
		['й', s @ ..] => super::t11::n764(s, out),
		['к', s @ ..] => super::t11::n765(s, out),
		['л', s @ ..] => super::t11::n766(s, out),
		['о', s @ ..] => super::t11::n767(s, out),
		['п', s @ ..] => super::t11::n768(s, out),
		['р', s @ ..] => super::t11::n769(s, out),
		['с', s @ ..] => super::t11::n770(s, out),
		['т', s @ ..] => super::t11::n771(s, out),
		['у', s @ ..] => super::t11::n772(s, out),
		['ы', s @ ..] => super::t11::n773(s, out),
		['э', s @ ..] => super::t11::n774(s, out),
		['ю', s @ ..] => super::t11::n775(s, out),
		['я', s @ ..] => super::t11::n776(s, out),
		_ => false,
	}
}

/// Words ending -ма.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5947);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22693);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 49513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 49514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 22695);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 49515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5949);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22696);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 49516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['а', s @ ..] => {
							super::push(out, 22697);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 49517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 22698);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 49518);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 78303);
											s.is_empty()
										}
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
					super::push(out, 5950);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5951);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22700);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 49519);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 78304);
											s.is_empty()
										}
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
					super::push(out, 5952);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22701);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 49520);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 78305);
											s.is_empty()
										}
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
					super::push(out, 5953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22702);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 22703);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 49521);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 78306);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 102044);
													match s {
														[] => true,
														['а', s @ ..] => super::t11::n119966(s, out),
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
									super::push(out, 49522);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 78307);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 102045);
													match s {
														[] => true,
														['н', s @ ..] => super::t11::n119967(s, out),
														_ => false,
													}
												}
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
							super::push(out, 22704);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 49523);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 78308);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 102046);
													match s {
														[] => true,
														['т', s @ ..] => super::t11::n119968(s, out),
														_ => false,
													}
												}
												['е', s @ ..] => {
													super::push(out, 102047);
													match s {
														[] => true,
														['т', s @ ..] => super::t11::n119969(s, out),
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
									super::push(out, 49524);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 78309);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 102048);
													match s {
														[] => true,
														['э', s @ ..] => super::t11::n119970(s, out),
														_ => false,
													}
												}
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
							super::push(out, 22705);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 49525);
									match s {
										[] => true,
										['р', s @ ..] => super::t11::n79137(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t11::n22840(s, out),
						['х', s @ ..] => {
							super::push(out, 22707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t11::n5969(s, out),
				['н', s @ ..] => super::t11::n5970(s, out),
				['р', s @ ..] => super::t11::n5971(s, out),
				['с', s @ ..] => super::t11::n5972(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t11::n778(s, out),
		['г', s @ ..] => super::t11::n779(s, out),
		['е', s @ ..] => super::t11::n780(s, out),
		['ж', s @ ..] => super::t11::n781(s, out),
		['з', s @ ..] => super::t11::n782(s, out),
		['и', s @ ..] => super::t11::n783(s, out),
		['й', s @ ..] => super::t11::n784(s, out),
		['к', s @ ..] => super::t11::n785(s, out),
		['л', s @ ..] => super::t11::n786(s, out),
		['м', s @ ..] => super::t11::n787(s, out),
		['н', s @ ..] => super::t11::n788(s, out),
		['о', s @ ..] => super::t11::n789(s, out),
		['р', s @ ..] => super::t11::n790(s, out),
		['с', s @ ..] => super::t11::n791(s, out),
		['т', s @ ..] => super::t11::n792(s, out),
		['у', s @ ..] => super::t11::n793(s, out),
		['ф', s @ ..] => super::t11::n794(s, out),
		['х', s @ ..] => super::t11::n795(s, out),
		['ч', s @ ..] => super::t11::n796(s, out),
		['ш', s @ ..] => super::t11::n797(s, out),
		['ь', s @ ..] => super::t11::n798(s, out),
		['э', s @ ..] => super::t11::n799(s, out),
		['я', s @ ..] => super::t11::n800(s, out),
		_ => false,
	}
}

/// Words ending -на.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6041);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6042);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22868);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22869);
							match s {
								[] => true,
								['к', s @ ..] => super::t11::n50166(s, out),
								_ => false,
							}
						}
						['и', s @ ..] => super::t11::n23004(s, out),
						['э', s @ ..] => super::t11::n23005(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t11::n6058(s, out),
				['г', s @ ..] => super::t11::n6059(s, out),
				['д', s @ ..] => super::t11::n6060(s, out),
				['ж', s @ ..] => super::t11::n6061(s, out),
				['з', s @ ..] => super::t11::n6062(s, out),
				['и', s @ ..] => super::t11::n6063(s, out),
				['й', s @ ..] => super::t11::n6064(s, out),
				['к', s @ ..] => super::t11::n6065(s, out),
				['л', s @ ..] => super::t11::n6066(s, out),
				['м', s @ ..] => super::t11::n6067(s, out),
				['о', s @ ..] => super::t11::n6068(s, out),
				['п', s @ ..] => super::t11::n6069(s, out),
				['р', s @ ..] => super::t11::n6070(s, out),
				['с', s @ ..] => super::t11::n6071(s, out),
				['т', s @ ..] => super::t11::n6072(s, out),
				['у', s @ ..] => super::t11::n6073(s, out),
				['ф', s @ ..] => super::t11::n6074(s, out),
				['х', s @ ..] => super::t11::n6075(s, out),
				['ц', s @ ..] => super::t11::n6076(s, out),
				['ш', s @ ..] => super::t11::n6077(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t11::n802(s, out),
		['в', s @ ..] => super::t11::n803(s, out),
		['г', s @ ..] => super::t11::n804(s, out),
		['д', s @ ..] => super::t11::n805(s, out),
		['е', s @ ..] => super::t11::n806(s, out),
		['ж', s @ ..] => super::t11::n807(s, out),
		['з', s @ ..] => super::t11::n808(s, out),
		['и', s @ ..] => super::t11::n809(s, out),
		['й', s @ ..] => super::t11::n810(s, out),
		['к', s @ ..] => super::t11::n811(s, out),
		['л', s @ ..] => super::t11::n812(s, out),
		['м', s @ ..] => super::t11::n813(s, out),
		['н', s @ ..] => super::t11::n814(s, out),
		['о', s @ ..] => super::t12::n815(s, out),
		['п', s @ ..] => super::t12::n816(s, out),
		['р', s @ ..] => super::t12::n817(s, out),
		['с', s @ ..] => super::t12::n818(s, out),
		['т', s @ ..] => super::t12::n819(s, out),
		['у', s @ ..] => super::t12::n820(s, out),
		['ф', s @ ..] => super::t12::n821(s, out),
		['х', s @ ..] => super::t12::n822(s, out),
		['ч', s @ ..] => super::t12::n823(s, out),
		['ш', s @ ..] => super::t12::n824(s, out),
		['ы', s @ ..] => super::t12::n825(s, out),
		['ь', s @ ..] => super::t12::n826(s, out),
		['ю', s @ ..] => super::t12::n827(s, out),
		['я', s @ ..] => super::t12::n828(s, out),
		_ => false,
	}
}

/// Words ending -оа.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6202);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6203);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 6204);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23388);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 50528);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 79305);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 829);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6205);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23389);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 50529);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 79306);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6206);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6207);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6208);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23391);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 50530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 23392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6209);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23393);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 50531);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79307);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 834);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 6210);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23394);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 50532);
									s.is_empty()
								}
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
			super::push(out, 835);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -па.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6211);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23395);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 50533);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 79308);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 102763);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6212);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6213);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 6214);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 50534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6215);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6216);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6217);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23397);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 50535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6218);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23398);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23399);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 50536);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79309);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 102764);
													match s {
														[] => true,
														['г', s @ ..] => super::t12::n120398(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6219);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23400);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23401);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 50537);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 79310);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 102765);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23402);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23403);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 50538);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79311);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 102766);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6221);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6223);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6224);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23404);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 50539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6225);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23405);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23406);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 50540);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79312);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 102767);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n120399(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6226);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23407);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 50541);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 79313);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 102768);
													s.is_empty()
												}
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
							super::push(out, 23408);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 23409);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 50542);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 79314);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 102769);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n120400(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6227);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 6228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6229);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6230);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6231);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23411);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 50543);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79315);
											s.is_empty()
										}
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
					super::push(out, 6232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t12::n6249(s, out),
				['щ', s @ ..] => super::t12::n6250(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t12::n841(s, out),
		['м', s @ ..] => super::t12::n842(s, out),
		['о', s @ ..] => super::t12::n843(s, out),
		['п', s @ ..] => super::t12::n844(s, out),
		['р', s @ ..] => super::t12::n845(s, out),
		['с', s @ ..] => super::t12::n846(s, out),
		['у', s @ ..] => super::t12::n847(s, out),
		['ы', s @ ..] => super::t12::n848(s, out),
		['ь', s @ ..] => super::t12::n849(s, out),
		['я', s @ ..] => super::t12::n850(s, out),
		_ => false,
	}
}

/// Words ending -ра.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6268);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23449);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 50572);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79332);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 851);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6269);
					s.is_empty()
				}
				['б', s @ ..] => super::t12::n6286(s, out),
				['в', s @ ..] => {
					super::push(out, 6271);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23456);
							match s {
								[] => true,
								['а', s @ ..] => super::t12::n51098(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23457);
							match s {
								[] => true,
								['у', s @ ..] => super::t12::n51099(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t12::n6288(s, out),
				['д', s @ ..] => super::t12::n6289(s, out),
				['ж', s @ ..] => super::t12::n6290(s, out),
				['з', s @ ..] => super::t12::n6291(s, out),
				['и', s @ ..] => super::t12::n6292(s, out),
				['к', s @ ..] => super::t12::n6293(s, out),
				['л', s @ ..] => super::t12::n6294(s, out),
				['м', s @ ..] => super::t12::n6295(s, out),
				['н', s @ ..] => super::t12::n6296(s, out),
				['о', s @ ..] => super::t12::n6297(s, out),
				['п', s @ ..] => super::t12::n6298(s, out),
				['р', s @ ..] => super::t12::n6299(s, out),
				['с', s @ ..] => super::t12::n6300(s, out),
				['т', s @ ..] => super::t12::n6301(s, out),
				['у', s @ ..] => super::t12::n6302(s, out),
				['ф', s @ ..] => super::t12::n6303(s, out),
				['х', s @ ..] => super::t12::n6304(s, out),
				['ц', s @ ..] => super::t12::n6305(s, out),
				['ч', s @ ..] => super::t12::n6306(s, out),
				['ш', s @ ..] => super::t12::n6307(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t12::n853(s, out),
		['в', s @ ..] => super::t12::n854(s, out),
		['г', s @ ..] => super::t12::n855(s, out),
		['д', s @ ..] => super::t12::n856(s, out),
		['е', s @ ..] => super::t12::n857(s, out),
		['ж', s @ ..] => super::t12::n858(s, out),
		['з', s @ ..] => super::t12::n859(s, out),
		['и', s @ ..] => super::t12::n860(s, out),
		['й', s @ ..] => super::t12::n861(s, out),
		['к', s @ ..] => super::t12::n862(s, out),
		['м', s @ ..] => super::t12::n863(s, out),
		['о', s @ ..] => super::t12::n864(s, out),
		['п', s @ ..] => super::t12::n865(s, out),
		['р', s @ ..] => super::t12::n866(s, out),
		['т', s @ ..] => super::t12::n867(s, out),
		['у', s @ ..] => super::t12::n868(s, out),
		['ф', s @ ..] => super::t12::n869(s, out),
		['х', s @ ..] => super::t12::n870(s, out),
		['ы', s @ ..] => super::t12::n871(s, out),
		['э', s @ ..] => super::t12::n872(s, out),
		['ю', s @ ..] => super::t12::n873(s, out),
		['я', s @ ..] => super::t12::n874(s, out),
		_ => false,
	}
}

/// Words ending -са.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6453);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23840);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 50944);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 79583);
											s.is_empty()
										}
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
					super::push(out, 6454);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23841);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 50945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6455);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 50946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6457);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 23844);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 50947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6458);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 6459);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23845);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 50948);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 79584);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 102905);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6460);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23846);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 50949);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 79585);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 102906);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 875);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6461);
					match s {
						[] => true,
						['е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 23847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6462);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23848);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 50950);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 50951);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79586);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 102907);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n120473(s, out),
														_ => false,
													}
												}
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
							super::push(out, 23849);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 50952);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 79587);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 50953);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79588);
											s.is_empty()
										}
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
					super::push(out, 6463);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23850);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 50954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6464);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23851);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 50955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23852);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 23853);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 50956);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 79589);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 102908);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n120474(s, out),
														_ => false,
													}
												}
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
							super::push(out, 23854);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 50957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6466);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23855);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 50958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6467);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23856);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 50959);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 79590);
											s.is_empty()
										}
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
					super::push(out, 6468);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23857);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 50960);
									match s {
										[] => true,
										['т', s @ ..] => super::t12::n80424(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t12::n877(s, out),
		['й', s @ ..] => super::t12::n878(s, out),
		['к', s @ ..] => super::t12::n879(s, out),
		['м', s @ ..] => super::t12::n880(s, out),
		['н', s @ ..] => super::t12::n881(s, out),
		['о', s @ ..] => super::t12::n882(s, out),
		['п', s @ ..] => super::t12::n883(s, out),
		['р', s @ ..] => super::t12::n884(s, out),
		['с', s @ ..] => super::t12::n885(s, out),
		['т', s @ ..] => super::t12::n886(s, out),
		['у', s @ ..] => super::t12::n887(s, out),
		['ы', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['р', s @ ..] => super::t12::n6534(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t12::n889(s, out),
		['я', s @ ..] => super::t12::n890(s, out),
		_ => false,
	}
}

/// Words ending -та.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 6521);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23941);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 51034);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 79641);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 102942);
													match s {
														[] => true,
														['и', s @ ..] => super::t12::n120489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23942);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23943);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 51035);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 79642);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 51036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6523);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23945);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 51037);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 79643);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 102943);
													s.is_empty()
												}
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
							super::push(out, 23946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 6524);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23947);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 51038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6525);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6526);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23948);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 51039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 6527);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23949);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 51040);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 79644);
											match s {
												[] => true,
												['а', s @ ..] => super::t12::n104034(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => super::t12::n24096(s, out),
						['я', s @ ..] => super::t12::n24097(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6528);
					s.is_empty()
				}
				['к', s @ ..] => super::t12::n6545(s, out),
				['л', s @ ..] => super::t12::n6546(s, out),
				['м', s @ ..] => super::t12::n6547(s, out),
				['н', s @ ..] => super::t12::n6548(s, out),
				['о', s @ ..] => super::t12::n6549(s, out),
				['п', s @ ..] => super::t12::n6550(s, out),
				['р', s @ ..] => super::t12::n6551(s, out),
				['т', s @ ..] => super::t12::n6552(s, out),
				['у', s @ ..] => super::t12::n6553(s, out),
				['ф', s @ ..] => super::t12::n6554(s, out),
				['х', s @ ..] => super::t12::n6555(s, out),
				['ч', s @ ..] => super::t12::n6556(s, out),
				['ш', s @ ..] => super::t12::n6557(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t12::n892(s, out),
		['и', s @ ..] => super::t12::n893(s, out),
		['й', s @ ..] => super::t12::n894(s, out),
		['к', s @ ..] => super::t12::n895(s, out),
		['н', s @ ..] => super::t12::n896(s, out),
		['о', s @ ..] => super::t12::n897(s, out),
		['п', s @ ..] => super::t12::n898(s, out),
		['р', s @ ..] => super::t12::n899(s, out),
		['с', s @ ..] => super::t12::n900(s, out),
		['т', s @ ..] => super::t12::n901(s, out),
		['у', s @ ..] => super::t12::n902(s, out),
		['ф', s @ ..] => super::t12::n903(s, out),
		['х', s @ ..] => super::t12::n904(s, out),
		['ч', s @ ..] => super::t12::n905(s, out),
		['ш', s @ ..] => super::t12::n906(s, out),
		['ы', s @ ..] => super::t12::n907(s, out),
		['ь', s @ ..] => super::t12::n908(s, out),
		['э', s @ ..] => super::t12::n909(s, out),
		['ю', s @ ..] => super::t12::n910(s, out),
		['я', s @ ..] => super::t12::n911(s, out),
		_ => false,
	}
}

/// Words ending -уа.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6664);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24297);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 51388);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79871);
											s.is_empty()
										}
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
					super::push(out, 6665);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24298);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 51389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6667);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6668);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24300);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51390);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 79872);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24301);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51391);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 79873);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24302);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51392);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 79874);
											s.is_empty()
										}
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
					super::push(out, 6669);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6670);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24304);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 51393);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79875);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 914);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6671);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 24305);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 51394);
									s.is_empty()
								}
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
			super::push(out, 915);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6672);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 24306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6673);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24307);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 51395);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 79876);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 103053);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 917);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24308);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 51396);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 79877);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 103054);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6675);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24309);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51397);
									s.is_empty()
								}
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
			super::push(out, 918);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6676);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24310);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51398);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 79878);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 919);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6677);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24311);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 51399);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 79879);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 103055);
													match s {
														[] => true,
														['а', s @ ..] => super::t12::n120527(s, out),
														_ => false,
													}
												}
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
							super::push(out, 24312);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 51400);
									s.is_empty()
								}
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
			super::push(out, 920);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24313);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 51401);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 79880);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 921);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 6679);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24314);
							match s {
								[] => true,
								['б', s @ ..] => super::t12::n51920(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t12::n6698(s, out),
				['о', s @ ..] => super::t12::n6699(s, out),
				['ь', s @ ..] => super::t12::n6700(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t12::n924(s, out),
		_ => false,
	}
}

/// Words ending -фа.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6684);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24321);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 24322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 51406);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 51407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24323);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 51408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6687);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24324);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 51409);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 79884);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 103059);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 925);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6688);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24325);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 51410);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79885);
											s.is_empty()
										}
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
					super::push(out, 6689);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6690);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6691);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24327);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 51411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6692);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24328);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 51412);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79886);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 103060);
													s.is_empty()
												}
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
							super::push(out, 24329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6693);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24330);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 51413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 24331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6694);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24332);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 51414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6695);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24333);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51415);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79887);
											s.is_empty()
										}
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
					super::push(out, 6696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6697);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 24334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6698);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6699);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24336);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 51416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24337);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 51417);
									s.is_empty()
								}
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
			super::push(out, 930);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24338);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 24339);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 51418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6701);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6702);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24341);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 51419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6703);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 51420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t12::n24495(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6705);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 24345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t12::n934(s, out),
		['у', s @ ..] => super::t12::n935(s, out),
		['ф', s @ ..] => super::t12::n936(s, out),
		['ь', s @ ..] => super::t12::n937(s, out),
		['э', s @ ..] => super::t12::n938(s, out),
		['ю', s @ ..] => super::t12::n939(s, out),
		['я', s @ ..] => super::t12::n940(s, out),
		_ => false,
	}
}

/// Words ending -ха.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 940);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6718);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 24354);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 51425);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 79892);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 103063);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 941);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6719);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24355);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 51426);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 79893);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 103064);
													s.is_empty()
												}
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
							super::push(out, 24356);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 51427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24357);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 51428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 51429);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 51430);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 79894);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24358);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 51431);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 79895);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 103065);
													s.is_empty()
												}
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
							super::push(out, 24359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24360);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24361);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 51432);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 79896);
											s.is_empty()
										}
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
					super::push(out, 6721);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24362);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 51433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6722);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24363);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 51434);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79897);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24364);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 51435);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 24365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24366);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24367);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 51436);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79898);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24368);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 51437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24369);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 51438);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 51439);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 79899);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 103066);
													match s {
														[] => true,
														['а', s @ ..] => super::t12::n120531(s, out),
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
										['о', s @ ..] => super::t12::n80733(s, out),
										['с', s @ ..] => super::t12::n80734(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t12::n6743(s, out),
				['п', s @ ..] => super::t12::n6744(s, out),
				['р', s @ ..] => super::t12::n6745(s, out),
				['т', s @ ..] => super::t12::n6746(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t12::n943(s, out),
		['е', s @ ..] => super::t12::n944(s, out),
		['и', s @ ..] => super::t12::n945(s, out),
		['й', s @ ..] => super::t12::n946(s, out),
		['м', s @ ..] => super::t12::n947(s, out),
		['н', s @ ..] => super::t12::n948(s, out),
		['о', s @ ..] => super::t13::n949(s, out),
		['р', s @ ..] => super::t13::n950(s, out),
		['с', s @ ..] => super::t13::n951(s, out),
		['т', s @ ..] => super::t13::n952(s, out),
		['у', s @ ..] => super::t13::n953(s, out),
		['ы', s @ ..] => super::t13::n954(s, out),
		['ь', s @ ..] => super::t13::n955(s, out),
		['э', s @ ..] => super::t13::n956(s, out),
		['ю', s @ ..] => super::t13::n957(s, out),
		['я', s @ ..] => super::t13::n958(s, out),
		_ => false,
	}
}

/// Words ending -ца.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6818);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 24617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6819);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 51683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 6820);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24619);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 51684);
									s.is_empty()
								}
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
			super::push(out, 959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24620);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 51685);
									s.is_empty()
								}
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
			super::push(out, 960);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6822);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24621);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 51686);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 80085);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24622);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 51687);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 80086);
											s.is_empty()
										}
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
					super::push(out, 6823);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 24623);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 51688);
									s.is_empty()
								}
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
			super::push(out, 961);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6824);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24624);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 51689);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 80087);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 103187);
													match s {
														[] => true,
														['л', s @ ..] => super::t13::n120579(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6825);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6826);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24626);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 51690);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 80088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24627);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 51691);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 80089);
											s.is_empty()
										}
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
					super::push(out, 6827);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24628);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 51692);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 80090);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24629);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 51693);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 80091);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24630);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 51694);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 80092);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 962);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 6828);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24631);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 51695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t13::n964(s, out),
		['й', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6847);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24791);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 80768);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 103957);
													match s {
														[] => true,
														['л', s @ ..] => super::t13::n121187(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 80769);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 80770);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 103958);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 103959);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6848);
					match s {
						[] => true,
						['п', s @ ..] => super::t13::n24947(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t13::n6868(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t13::n966(s, out),
		['п', s @ ..] => super::t13::n967(s, out),
		['р', s @ ..] => super::t13::n968(s, out),
		['с', s @ ..] => super::t13::n969(s, out),
		['т', s @ ..] => super::t13::n970(s, out),
		['у', s @ ..] => super::t13::n971(s, out),
		['ц', s @ ..] => super::t13::n972(s, out),
		['ш', s @ ..] => super::t13::n973(s, out),
		['ы', s @ ..] => super::t13::n974(s, out),
		['ь', s @ ..] => super::t13::n975(s, out),
		['я', s @ ..] => super::t13::n976(s, out),
		_ => false,
	}
}

/// Words ending -ча.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6875);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 24829);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 52120);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 80791);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 103968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 977);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6876);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24830);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24831);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24832);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24833);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 52121);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 80792);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 80793);
											s.is_empty()
										}
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
					super::push(out, 6877);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24834);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 52122);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 80794);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 103969);
													match s {
														[] => true,
														['г', s @ ..] => super::t13::n121192(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6878);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24835);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 52123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 80795);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 978);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6879);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24836);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52124);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 52125);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 80796);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 103970);
													match s {
														[] => true,
														['е', s @ ..] => super::t13::n121193(s, out),
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
									super::push(out, 52126);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 80797);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 103971);
													match s {
														[] => true,
														['п', s @ ..] => super::t13::n121194(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6880);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6881);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24837);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24838);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52127);
									match s {
										[] => true,
										['в', '-', s @ ..] => {
											super::push(out, 80798);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 103972);
													match s {
														[] => true,
														['н', s @ ..] => super::t13::n121195(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 103973);
													match s {
														[] => true,
														['е', s @ ..] => super::t13::n121196(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 103974);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6882);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6883);
					match s {
						[] => true,
						['д', s @ ..] => super::t13::n24994(s, out),
						['о', s @ ..] => super::t13::n24995(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t13::n980(s, out),
		['к', s @ ..] => super::t13::n981(s, out),
		['л', s @ ..] => super::t13::n982(s, out),
		['м', s @ ..] => super::t13::n983(s, out),
		['н', s @ ..] => super::t13::n984(s, out),
		['о', s @ ..] => super::t13::n985(s, out),
		['р', s @ ..] => super::t13::n986(s, out),
		['т', s @ ..] => super::t13::n987(s, out),
		['у', s @ ..] => super::t13::n988(s, out),
		['х', s @ ..] => super::t13::n989(s, out),
		['ы', s @ ..] => super::t13::n990(s, out),
		['ь', s @ ..] => super::t13::n991(s, out),
		['я', s @ ..] => super::t13::n992(s, out),
		_ => false,
	}
}

/// Words ending -ша.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6917);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52155);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 52156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 24880);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6919);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24881);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52158);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 80815);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 103980);
													match s {
														[] => true,
														['о', s @ ..] => super::t13::n121199(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6920);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6921);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24882);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24883);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52160);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 80816);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 103981);
													match s {
														[] => true,
														['ш', s @ ..] => super::t13::n121200(s, out),
														_ => false,
													}
												}
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
							super::push(out, 24884);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6923);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24885);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52162);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 80817);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 52163);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 52164);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 52165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52166);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 52167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24887);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24888);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24889);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52169);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 80818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24890);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 52170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6926);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24891);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24892);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52172);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 80819);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 103982);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6927);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24893);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6929);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6930);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24895);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52174);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 80820);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6931);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24896);
							s.is_empty()
						}
						['р', s @ ..] => super::t13::n25052(s, out),
						['т', s @ ..] => super::t13::n25053(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t13::n6951(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t13::n995(s, out),
		['д', s @ ..] => super::t13::n996(s, out),
		['е', s @ ..] => super::t13::n997(s, out),
		['з', s @ ..] => super::t13::n998(s, out),
		['и', s @ ..] => super::t13::n999(s, out),
		['й', s @ ..] => super::t13::n1000(s, out),
		['к', s @ ..] => super::t13::n1001(s, out),
		['м', s @ ..] => super::t13::n1002(s, out),
		['н', s @ ..] => super::t13::n1003(s, out),
		['о', s @ ..] => super::t13::n1004(s, out),
		['п', s @ ..] => super::t13::n1005(s, out),
		['р', s @ ..] => super::t13::n1006(s, out),
		['т', s @ ..] => super::t13::n1007(s, out),
		['у', s @ ..] => super::t13::n1008(s, out),
		['х', s @ ..] => super::t13::n1009(s, out),
		['ы', s @ ..] => super::t13::n1010(s, out),
		['ь', s @ ..] => super::t13::n1011(s, out),
		['ю', s @ ..] => super::t13::n1012(s, out),
		['я', s @ ..] => super::t13::n1013(s, out),
		_ => false,
	}
}

/// Words ending -ща.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7016);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 7017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7018);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25065);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7020);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25066);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 52377);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 80975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25067);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 52378);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 80976);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 52379);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 80977);
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
									super::push(out, 52380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 80978);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 52381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25068);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52382);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 80979);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 52383);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 80980);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25069);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 52384);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 80981);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 104073);
													s.is_empty()
												}
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
							super::push(out, 25070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25071);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52385);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 80982);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 104074);
													match s {
														[] => true,
														['т', s @ ..] => super::t13::n121236(s, out),
														_ => false,
													}
												}
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
							super::push(out, 25072);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 25073);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52386);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 80983);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104075);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t13::n7042(s, out),
				['з', s @ ..] => {
					super::push(out, 7023);
					s.is_empty()
				}
				['л', s @ ..] => super::t13::n7044(s, out),
				['м', s @ ..] => {
					super::push(out, 7025);
					match s {
						[] => true,
						['й', s @ ..] => super::t13::n25246(s, out),
						['я', s @ ..] => super::t13::n25247(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t13::n7046(s, out),
				['п', s @ ..] => super::t13::n7047(s, out),
				['р', s @ ..] => super::t13::n7048(s, out),
				['т', s @ ..] => super::t13::n7049(s, out),
				['ч', s @ ..] => super::t13::n7050(s, out),
				['щ', s @ ..] => super::t13::n7051(s, out),
				['ь', s @ ..] => super::t13::n7052(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t13::n1017(s, out),
		['о', s @ ..] => super::t13::n1018(s, out),
		['у', s @ ..] => super::t13::n1019(s, out),
		['ы', s @ ..] => super::t13::n1020(s, out),
		_ => false,
	}
}

/// Words ending -ъа.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7042);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эа.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раб.
pub(crate) fn n1032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1030);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7058);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 7059);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25110);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52421);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81001);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104084);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7060);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52422);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 52423);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81003);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104085);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7061);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25111);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52424);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81004);
									s.is_empty()
								}
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
			super::push(out, 7062);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25112);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7063);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7064);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25114);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 25115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7065);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25116);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52426);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81005);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104086);
											s.is_empty()
										}
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
					super::push(out, 25117);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 52427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7066);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25118);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 52428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7067);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7068);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25120);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 7069);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -штаб.
pub(crate) fn n7093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7073);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25122);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 25123);
			match s {
				[] => true,
				['е', 'р', '\u{1}', s @ ..] => {
					super::push(out, 52431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 25124);
			match s {
				[] => true,
				['е', 'р', '\u{1}', s @ ..] => {
					super::push(out, 52432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 25125);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25126);
			match s {
				[] => true,
				['е', 'п', 'о', '\u{1}', s @ ..] => {
					super::push(out, 52434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25127);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52435);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хаб.
pub(crate) fn n1035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1033);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 7074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25128);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81008);
									s.is_empty()
								}
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
			super::push(out, 7075);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бб.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7076);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еб.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7077);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7080);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 25131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7081);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25132);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7082);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 7083);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 7084);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7086);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7087);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7088);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7089);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7090);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25139);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25140);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 52439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 25141);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 25142);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52441);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81009);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7091);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25144);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25145);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25146);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7092);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7093);
					s.is_empty()
				}
				['р', 'п', s @ ..] => {
					super::push(out, 7094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7095);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25148);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 52444);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25149);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25150);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 52446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7096);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 25151);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52447);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 52448);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81011);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104087);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7097);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25152);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52449);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 81012);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104088);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 52450);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81013);
											s.is_empty()
										}
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
					super::push(out, 7098);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25153);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52451);
									match s {
										[] => true,
										['р', s @ ..] => super::t13::n81860(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t13::n25321(s, out),
						_ => false,
					}
				}
				['э', s @ ..] => super::t13::n7119(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t13::n1045(s, out),
		['ф', s @ ..] => super::t13::n1046(s, out),
		['х', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['ъ', s @ ..] => super::t13::n1048(s, out),
		_ => false,
	}
}

/// Words ending -зб.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иб.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7104);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7107);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25157);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 25158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7108);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 25160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7109);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7110);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7111);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25164);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25165);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52457);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81016);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 104090);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n121241(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7112);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7113);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7114);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25167);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52458);
									s.is_empty()
								}
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
			super::push(out, 1051);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7116);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25168);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7117);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 25169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7118);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7119);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7120);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25172);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 52461);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81017);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 104091);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7121);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25173);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7122);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25174);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52463);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81018);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1054);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7123);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7124);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 7125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25176);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25177);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7126);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7127);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 25179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25180);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52466);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81019);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25181);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25182);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52468);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81020);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104092);
													s.is_empty()
												}
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
							super::push(out, 25183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t13::n1058(s, out),
		['х', s @ ..] => super::t13::n1059(s, out),
		['ш', s @ ..] => super::t13::n1060(s, out),
		_ => false,
	}
}

/// Words ending -йб.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7136);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1060);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лб.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7139);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25191);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52470);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81021);
											s.is_empty()
										}
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
					super::push(out, 7140);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мб.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1063);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1064);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -об.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7141);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7142);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7143);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52471);
									s.is_empty()
								}
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
			super::push(out, 1068);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7144);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7146);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7147);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 25196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 7148);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25197);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 52473);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81022);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104093);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25198);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7151);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7152);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7153);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 25200);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52475);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 81023);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 104094);
													match s {
														[] => true,
														['а', s @ ..] => super::t13::n121242(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7155);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7156);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52476);
									s.is_empty()
								}
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
			super::push(out, 1072);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 7157);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7158);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 7159);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 25203);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52477);
									s.is_empty()
								}
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
			super::push(out, 1073);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7160);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7161);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25206);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 25207);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7164);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25208);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52479);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81024);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 104095);
													s.is_empty()
												}
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
							super::push(out, 25209);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52480);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81025);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 104096);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7165);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25210);
							match s {
								[] => true,
								['м', s @ ..] => super::t13::n53010(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t13::n7186(s, out),
				['п', s @ ..] => {
					super::push(out, 7167);
					match s {
						[] => true,
						['а', s @ ..] => super::t13::n25381(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t13::n7188(s, out),
				['э', s @ ..] => super::t13::n7189(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t13::n1077(s, out),
		['ф', s @ ..] => super::t13::n1078(s, out),
		_ => false,
	}
}

/// Words ending -рб.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7174);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1078);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7175);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25226);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7177);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7178);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 7180);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1079);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7182);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уб.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1080);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7183);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25232);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 52497);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81036);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1081);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1082);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7185);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25233);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 52498);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81037);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7186);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7187);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7188);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25235);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52500);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81038);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104104);
													match s {
														[] => true,
														['х', s @ ..] => super::t13::n121248(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7189);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7190);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25236);
							s.is_empty()
						}
						['н', 'и', s @ ..] => {
							super::push(out, 25237);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52501);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81039);
											s.is_empty()
										}
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
					super::push(out, 7191);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1085);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7193);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7194);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7195);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25240);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7196);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25241);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7197);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25242);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52504);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81040);
											s.is_empty()
										}
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
					super::push(out, 7198);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25243);
							match s {
								[] => true,
								['г', 'а', '\u{1}', s @ ..] => {
									super::push(out, 52505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7199);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25244);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52506);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81041);
											s.is_empty()
										}
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
					super::push(out, 7200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7201);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7202);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25247);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 25248);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52508);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81042);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 104105);
													match s {
														[] => true,
														['а', s @ ..] => super::t13::n121249(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t13::n1091(s, out),
		['ч', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7212);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тыб.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7213);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 25261);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52520);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81049);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьб.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эб.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['к', 'с', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1095);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юб.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['е', 'н', s @ ..] => {
					super::push(out, 7216);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25262);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 52521);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81050);
											s.is_empty()
										}
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
							super::push(out, 25263);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25264);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 25265);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52522);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81051);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 104108);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n121250(s, out),
														_ => false,
													}
												}
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
							super::push(out, 25266);
							s.is_empty()
						}
						['т', 'е', s @ ..] => {
							super::push(out, 25267);
							match s {
								[] => true,
								['в', 'с', s @ ..] => {
									super::push(out, 52523);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81052);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 25268);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 52524);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81053);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104109);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7218);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25269);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 52525);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81054);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яб.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-волкодав.
pub(crate) fn n105211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104110);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 119987);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 129860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -говнодав.
pub(crate) fn n105212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104111);
	s.is_empty()
}

/// Words ending -лав.
pub(crate) fn n1109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1107);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7227);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7228);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25275);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 52530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7229);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7230);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25277);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25278);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7232);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					super::push(out, 25281);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52531);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81057);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104112);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 119988);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25282);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25283);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81058);
									s.is_empty()
								}
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
			super::push(out, 7233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25284);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52533);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 81059);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104113);
											s.is_empty()
										}
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
					super::push(out, 25285);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 52534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25286);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							super::push(out, 52535);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 52536);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81061);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 104114);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 52537);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 52538);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81063);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104115);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119989);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25287);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52539);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 52540);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 52541);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81064);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104116);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119990);
													match s {
														[] => true,
														['т', s @ ..] => super::t13::n131213(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'а', s @ ..] => {
							super::push(out, 52542);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81065);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104117);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119991);
													s.is_empty()
												}
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
							super::push(out, 52543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 7234);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25288);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81066);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25289);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52545);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81067);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104118);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рав.
pub(crate) fn n1111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7237);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7238);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25293);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7239);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7240);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7241);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25295);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 52547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7242);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25296);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 52548);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81068);
									s.is_empty()
								}
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
			super::push(out, 7243);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тав.
pub(crate) fn n1112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 7244);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25298);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52549);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81069);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104119);
											s.is_empty()
										}
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
					super::push(out, 25299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52550);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 52551);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 81070);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104120);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 119992);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n131214(s, out),
														_ => false,
													}
												}
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
							super::push(out, 52552);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 81071);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104121);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119993);
													s.is_empty()
												}
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
							super::push(out, 52553);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81072);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 104122);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119994);
													s.is_empty()
												}
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
							super::push(out, 52554);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 52555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25300);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чав.
pub(crate) fn n1115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1113);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7247);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гв.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7248);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25302);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52556);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ев.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7249);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25303);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 52557);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81075);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 104123);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7250);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25304);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 52558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7251);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25305);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 52559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25306);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 52560);
									s.is_empty()
								}
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
			super::push(out, 1116);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1117);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7252);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25307);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52561);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 81076);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1118);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 7253);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25308);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52562);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 81077);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 104124);
													match s {
														[] => true,
														['т', s @ ..] => super::t13::n121259(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1120);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 7254);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25309);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52563);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81078);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104125);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7256);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25311);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52564);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 81079);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104126);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7257);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7258);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 7259);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7260);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7261);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25314);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 52565);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81080);
											s.is_empty()
										}
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
					super::push(out, 7262);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25315);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52566);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81081);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 104127);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 52567);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 81082);
											s.is_empty()
										}
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
					super::push(out, 7263);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7264);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25317);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 52568);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81083);
											s.is_empty()
										}
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
					super::push(out, 7265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7266);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7267);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25320);
							match s {
								[] => true,
								['б', s @ ..] => super::t13::n53100(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t13::n7289(s, out),
				['е', s @ ..] => super::t13::n7290(s, out),
				['ж', s @ ..] => super::t13::n7291(s, out),
				['и', s @ ..] => super::t13::n7292(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t13::n1125(s, out),
		['р', s @ ..] => super::t13::n1126(s, out),
		['с', s @ ..] => super::t13::n1127(s, out),
		['т', s @ ..] => super::t13::n1128(s, out),
		['у', s @ ..] => super::t13::n1129(s, out),
		['ц', s @ ..] => super::t13::n1130(s, out),
		['ч', s @ ..] => super::t13::n1131(s, out),
		['ш', s @ ..] => super::t13::n1132(s, out),
		['щ', s @ ..] => super::t13::n1133(s, out),
		['ь', s @ ..] => super::t13::n1134(s, out),
		_ => false,
	}
}

/// Words ending -ив.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7290);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 7291);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25345);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52591);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81094);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104134);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7292);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7293);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 25347);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52593);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 81095);
											s.is_empty()
										}
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
					super::push(out, 7294);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25348);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 52594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7295);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 7296);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25349);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52595);
									s.is_empty()
								}
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
			super::push(out, 1134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 7298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7299);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7300);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25351);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25352);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81096);
											s.is_empty()
										}
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
					super::push(out, 7301);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25353);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52597);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81097);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104135);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7302);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25354);
							match s {
								[] => true,
								['н', 'р', 'е', 'ч', s @ ..] => {
									super::push(out, 52598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7303);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52599);
									s.is_empty()
								}
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
			super::push(out, 1137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7304);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7305);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7306);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25356);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25357);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 52600);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 52601);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81099);
											match s {
												[] => true,
												['о', 'р', 'п', '\u{1}', s @ ..] => {
													super::push(out, 104136);
													s.is_empty()
												}
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
							super::push(out, 25358);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52602);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81100);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104137);
													match s {
														[] => true,
														['с', s @ ..] => super::t13::n121261(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1138);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7307);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25359);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52603);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81101);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104138);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t13::n25527(s, out),
						['р', s @ ..] => {
							super::push(out, 25361);
							s.is_empty()
						}
						['т', s @ ..] => super::t13::n25529(s, out),
						_ => false,
					}
				}
				['и', 'с', s @ ..] => super::t13::n7329(s, out),
				['к', s @ ..] => super::t13::n7330(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t13::n1141(s, out),
		_ => false,
	}
}

/// Words ending -йв.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1141);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7312);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

