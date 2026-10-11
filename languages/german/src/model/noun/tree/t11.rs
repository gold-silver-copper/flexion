use flexion::Path;

/// Words ending -pat.
pub(crate) fn n2961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2957);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10359);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 21056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10360);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30587);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30588);
							match s {
								[] => true,
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 37123);
									s.is_empty()
								}
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
			super::push(out, 10361);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21057);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 21058);
					match s {
						[] => true,
						['l', 'e', 'f', 'n', s @ ..] => {
							super::push(out, 30589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21059);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 30590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21060);
					match s {
						[] => true,
						['e', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 30591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21061);
					s.is_empty()
				}
				['t', 'u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 21062);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rat.
pub(crate) fn n2963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2959);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10362);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10363);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21063);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 21064);
					match s {
						[] => true,
						['c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 21065);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21066);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30595);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 37124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 21067);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30596);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 37125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 30597);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 't', 'o', 'n', 's', s @ ..] => {
											super::push(out, 41154);
											match s {
												[] => true,
												['s', 'u', 'f', '\u{1}', s @ ..] => {
													super::push(out, 43357);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'i', 'd', s @ ..] => {
											super::push(out, 41155);
											s.is_empty()
										}
										['s', 'g', 'n', 'u', 'r', s @ ..] => {
											super::push(out, 41156);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 30598);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 37126);
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 43358);
													match s {
														[] => true,
														['b', 'm', 'o', 'k', '\u{1}', s @ ..] => super::t14::n47333(s, out),
														_ => false,
													}
												}
												['n', 'i', 'h', s @ ..] => {
													super::push(out, 43359);
													match s {
														[] => true,
														['p', 'r', 'o', 'm', '\u{1}', s @ ..] => super::t14::n47334(s, out),
														_ => false,
													}
												}
												['s', s @ ..] => {
													super::push(out, 43360);
													match s {
														[] => true,
														['n', 'o', 'i', 't', 'a', 'n', 'i', 'b', 'm', 'o', 'k', '\u{1}', s @ ..] => super::t14::n47335(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10364);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21068);
					match s {
						[] => true,
						['e', 'r', 'h', 'c', 's', 't', 'h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 30599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10365);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21069);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37127);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21070);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30600);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 37129);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 37130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 21071);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37131);
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 41157);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 41158);
											match s {
												[] => true,
												['e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43361);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 37132);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 37133);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 41159);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 41160);
											match s {
												[] => true,
												['l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 43362);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10366);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21072);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30601);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 30602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 21073);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21074);
					match s {
						[] => true,
						['o', 'l', 'g', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37134);
									s.is_empty()
								}
								['n', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41161);
											s.is_empty()
										}
										['s', 'n', 'e', 'm', 'h', 'e', 'n', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 41162);
											s.is_empty()
										}
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
					super::push(out, 21075);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21076);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37135);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21077);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30603);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 30604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t14::n10490(s, out),
		['g', s @ ..] => super::t14::n10491(s, out),
		['h', s @ ..] => super::t14::n10492(s, out),
		['i', s @ ..] => super::t14::n10493(s, out),
		['k', s @ ..] => super::t14::n10494(s, out),
		['l', s @ ..] => super::t14::n10495(s, out),
		['m', s @ ..] => {
			super::push(out, 10373);
			s.is_empty()
		}
		['n', s @ ..] => super::t14::n10497(s, out),
		['o', s @ ..] => super::t14::n10498(s, out),
		['r', s @ ..] => super::t14::n10499(s, out),
		['s', s @ ..] => super::t14::n10500(s, out),
		['t', s @ ..] => super::t14::n10501(s, out),
		['u', s @ ..] => super::t14::n10502(s, out),
		['v', s @ ..] => super::t14::n10503(s, out),
		['y', s @ ..] => super::t14::n10504(s, out),
		_ => false,
	}
}

/// Words ending -sat.
pub(crate) fn n2964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2960);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10382);
			match s {
				[] => true,
				['l', 'a', 't', s @ ..] => {
					super::push(out, 21120);
					match s {
						[] => true,
						['i', 'g', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 30650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'k', 'l', 'u', 'v', '\u{1}', s @ ..] => {
					super::push(out, 21121);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10383);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10384);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10385);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 21124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 21125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tat.
pub(crate) fn n2965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2961);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10386);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 10387);
			match s {
				[] => true,
				['a', 'l', 'm', s @ ..] => {
					super::push(out, 21126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10388);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21127);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 21128);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'y', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 41178);
											s.is_empty()
										}
										['t', 'u', s @ ..] => {
											super::push(out, 41179);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 37174);
									s.is_empty()
								}
								['r', 'o', s @ ..] => {
									super::push(out, 37175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 21129);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 30652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 21130);
					match s {
						[] => true,
						['c', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 30653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21131);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21132);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 30654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21133);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21134);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 30655);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30656);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 21135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10390);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10391);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21137);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21138);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 30658);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 30659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 21139);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 21140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10392);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21141);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 30660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21142);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37176);
									s.is_empty()
								}
								['r', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 37177);
									s.is_empty()
								}
								['s', 'n', 'e', 'd', 'e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 37178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21143);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10393);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21144);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21145);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10394);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21147);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37179);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37180);
									match s {
										[] => true,
										['n', 'a', 'r', 't', 't', 'u', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 41180);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30662);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 37181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 30663);
							match s {
								[] => true,
								['f', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 37182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 30664);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 37183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 30665);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 37184);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 41181);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 37185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 21148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10395);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30666);
							s.is_empty()
						}
						['m', 'u', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 30667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10396);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21149);
					s.is_empty()
				}
				['e', 'i', 'n', 'o', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 21150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10397);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21151);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 21152);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 21153);
					match s {
						[] => true,
						['e', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 30668);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30669);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 30670);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37186);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37187);
									match s {
										[] => true,
										['e', 'h', 't', '\u{1}', s @ ..] => {
											super::push(out, 41182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 30671);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 37188);
									s.is_empty()
								}
								['y', 'g', '\u{1}', s @ ..] => {
									super::push(out, 37189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 30672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10398);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10399);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 21154);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 30673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 21155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10400);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 10401);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 10402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uat.
pub(crate) fn n2966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2962);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 10403);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21156);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vat.
pub(crate) fn n2967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2963);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10404);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10405);
			match s {
				[] => true,
				['e', 's', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 21158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', 'y', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ebiet.
pub(crate) fn n21795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21183);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 30689);
			match s {
				[] => true,
				['d', 'l', 'i', s @ ..] => {
					super::push(out, 37203);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 37204);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							super::push(out, 41186);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					super::push(out, 37205);
					match s {
						[] => true,
						['b', 'e', 'n', s @ ..] => {
							super::push(out, 41187);
							s.is_empty()
						}
						['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 41188);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 41189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 37206);
					match s {
						[] => true,
						['g', 'n', 'u', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 43368);
									match s {
										[] => true,
										['n', 'e', 'w', 'n', 'a', 't', 'p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 44354);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 43369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 37207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -riet.
pub(crate) fn n10568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10445);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 21186);
			match s {
				[] => true,
				['s', 'g', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 30691);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jet.
pub(crate) fn n2986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2981);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10446);
			match s {
				[] => true,
				['l', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 21187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10447);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 21188);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ket.
pub(crate) fn n2987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2982);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10448);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 21189);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 30692);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									match s {
										[] => true,
										['a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 41190);
											s.is_empty()
										}
										['n', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 41191);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'a', 'k', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37208);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37209);
									match s {
										[] => true,
										['a', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 41192);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37210);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 37211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 21190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 10449);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21191);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30693);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30694);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 30695);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21192);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30696);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10450);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10451);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10452);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21194);
					match s {
						[] => true,
						['m', '-', 'r', 'e', 't', 'n', 'u', 'o', 'c', '-', 'e', 'h', 't', '-', 'r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 30698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10453);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -let.
pub(crate) fn n2988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2983);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21196);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 21197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10455);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 21198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10456);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21199);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21200);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 30699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10457);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 21201);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 30700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10458);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21203);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21204);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10459);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21205);
					match s {
						[] => true,
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 30702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10460);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21206);
					match s {
						[] => true,
						['p', '-', s @ ..] => {
							super::push(out, 30703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21207);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37213);
									s.is_empty()
								}
								['n', 'e', 'g', 'r', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37214);
									s.is_empty()
								}
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
			super::push(out, 10461);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21208);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10462);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21210);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 30704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 21211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10463);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21212);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 30705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -met.
pub(crate) fn n2989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2984);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10465);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10466);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 21213);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							super::push(out, 30706);
							match s {
								[] => true,
								['y', 'g', '\u{1}', s @ ..] => {
									super::push(out, 37215);
									s.is_empty()
								}
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
			super::push(out, 10467);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10468);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21214);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 30707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21215);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 30708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 21216);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 30710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 21217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10469);
			match s {
				[] => true,
				['e', 'k', 'r', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 21218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10470);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21219);
					match s {
						[] => true,
						['p', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10471);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21221);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21222);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10472);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10473);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 21224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -net.
pub(crate) fn n2990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2985);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10474);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10475);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 21225);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21226);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10476);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10477);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 10478);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21227);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 30713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21228);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30714);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10479);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21229);
					match s {
						[] => true,
						['a', 't', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 30716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 21230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10480);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10481);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21231);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 30717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10482);
			match s {
				[] => true,
				['c', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 21232);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 21233);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10483);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21235);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 30718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -poet.
pub(crate) fn n10607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10484);
	s.is_empty()
}

/// Words ending -pet.
pub(crate) fn n2992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2987);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10485);
			match s {
				[] => true,
				['r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 21236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10486);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ret.
pub(crate) fn n2993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2988);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10488);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21237);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21238);
					match s {
						[] => true,
						['e', 'n', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 30719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10489);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21239);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 21240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10490);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10491);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10492);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21242);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 30721);
							match s {
								[] => true,
								['n', 'e', 's', 'ü', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 37218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 21243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10493);
			match s {
				[] => true,
				['h', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10494);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 21245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 10495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -set.
pub(crate) fn n2994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2989);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10496);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 10497);
			s.is_empty()
		}
		['m', 'l', s @ ..] => {
			super::push(out, 10498);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10499);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tet.
pub(crate) fn n2995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2990);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uet.
pub(crate) fn n2996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2991);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 10502);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 21246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vet.
pub(crate) fn n2997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2992);
	s.is_empty()
}

/// Words ending -wet.
pub(crate) fn n2998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2993);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zet.
pub(crate) fn n2999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2994);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10504);
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 21247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -aft.
pub(crate) fn n3000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2995);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 10506);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21248);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 21249);
					s.is_empty()
				}
				['c', s @ ..] => super::t14::n21862(s, out),
				['d', s @ ..] => {
					super::push(out, 21251);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21252);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 21253);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21254);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21255);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21256);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30724);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 37229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21257);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 30725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 21258);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 21259);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t14::n10630(s, out),
		['s', s @ ..] => {
			super::push(out, 10508);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21263);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'l', s @ ..] => {
							super::push(out, 30739);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41219);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'ü', s @ ..] => {
							super::push(out, 30740);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 30741);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 30742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 21264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10509);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --schrift.
pub(crate) fn n39377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37247);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 41221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ischrift.
pub(crate) fn n39378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37248);
	s.is_empty()
}

/// Words ending -salschrift.
pub(crate) fn n39379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37249);
	s.is_empty()
}

/// Words ending -amschrift.
pub(crate) fn n39380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37250);
	s.is_empty()
}

/// Words ending -nschrift.
pub(crate) fn n39381(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 41222);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 43397);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							super::push(out, 44404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 43398);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 44405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rschrift.
pub(crate) fn n39382(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 41223);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 41224);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 41225);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 43399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngsschrift.
pub(crate) fn n39383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37251);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 41226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tschrift.
pub(crate) fn n39384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37252);
	s.is_empty()
}

/// Words ending -auschrift.
pub(crate) fn n39385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37253);
	s.is_empty()
}

/// Words ending -zschrift.
pub(crate) fn n39386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37254);
	match s {
		[] => true,
		['r', 'u', 'k', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 43400);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 43401);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßschrift.
pub(crate) fn n39387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37255);
	s.is_empty()
}

/// Words ending -tift.
pub(crate) fn n10639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10516);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 30748);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 30749);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 37256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 30750);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 30751);
					match s {
						[] => true,
						['l', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 37257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 30752);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 30753);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 41227);
									match s {
										[] => true,
										['p', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 43402);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'a', 'r', 'f', s @ ..] => {
									super::push(out, 41228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 30754);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 30755);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 37258);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 37259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lft.
pub(crate) fn n3004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2999);
	match s {
		[] => true,
		['ö', 'w', 'z', 'r', 'h', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 10517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nft.
pub(crate) fn n3005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3000);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10518);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 21283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10519);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 30756);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 30757);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30758);
							s.is_empty()
						}
						['u', 'z', 'r', s @ ..] => {
							super::push(out, 30759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21284);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 10520);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oft.
pub(crate) fn n3006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3001);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 10521);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rft.
pub(crate) fn n3007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3002);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10523);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uft.
pub(crate) fn n3008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3003);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10525);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					super::push(out, 21287);
					s.is_empty()
				}
				['n', 'r', s @ ..] => {
					super::push(out, 21288);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21289);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
							super::push(out, 30761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10526);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10527);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21291);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 21292);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 21293);
					s.is_empty()
				}
				['h', 'c', 's', 'i', s @ ..] => {
					super::push(out, 21294);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					super::push(out, 21295);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 21296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30762);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 30763);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 30764);
							s.is_empty()
						}
						['i', 'b', s @ ..] => {
							super::push(out, 30765);
							s.is_empty()
						}
						['n', 'e', 's', s @ ..] => {
							super::push(out, 30766);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30767);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 37260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 21297);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21298);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 30768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21299);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 37261);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 37262);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 37263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21300);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 30769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21301);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 30770);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 37264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 21302);
					match s {
						[] => true,
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 30771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10528);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 21303);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äft.
pub(crate) fn n3009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3004);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 10529);
			match s {
				[] => true,
				['c', 's', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30772);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37265);
									s.is_empty()
								}
								['c', 'u', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 37266);
									s.is_empty()
								}
								['i', 's', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37267);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 30773);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 37268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30774);
							s.is_empty()
						}
						['r', 'i', s @ ..] => {
							super::push(out, 30775);
							s.is_empty()
						}
						['s', 'n', 'o', 'i', 's', s @ ..] => {
							super::push(out, 30776);
							s.is_empty()
						}
						['t', 'k', s @ ..] => {
							super::push(out, 30777);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 30778);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 37269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öft.
pub(crate) fn n3010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3005);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 10530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cht.
pub(crate) fn n3014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3009);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10536);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21308);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 21309);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 21310);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 21311);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30786);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 37273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30787);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 30788);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 41229);
											s.is_empty()
										}
										['t', 'p', s @ ..] => {
											super::push(out, 41230);
											s.is_empty()
										}
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
					super::push(out, 21312);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 30789);
							match s {
								[] => true,
								['s', 't', 'p', s @ ..] => {
									super::push(out, 37275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 21313);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 21314);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21315);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 30790);
							match s {
								[] => true,
								['c', 's', 's', s @ ..] => {
									super::push(out, 37276);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 41231);
											s.is_empty()
										}
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
					super::push(out, 21316);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30791);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 30792);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 37277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 30793);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 30794);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37278);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 37279);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 41232);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 41233);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30795);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37280);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 37281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 30796);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30797);
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 41234);
											match s {
												[] => true,
												['d', 'n', 'a', 'h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 43403);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'e', s @ ..] => {
											super::push(out, 41235);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', s @ ..] => {
									super::push(out, 37282);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 41236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 30798);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 37283);
									match s {
										[] => true,
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 41237);
											s.is_empty()
										}
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
					super::push(out, 21317);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 30799);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 30800);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 37284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 30801);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 37285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 30802);
							match s {
								[] => true,
								['o', 'r', 'g', 'o', 'p', 's', s @ ..] => {
									super::push(out, 37286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30803);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30804);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37287);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 37288);
									match s {
										[] => true,
										['ä', s @ ..] => {
											super::push(out, 41238);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							super::push(out, 30805);
							match s {
								[] => true,
								['a', 'f', 'r', 'e', 'b', 'i', 'e', 'w', 't', s @ ..] => {
									super::push(out, 37289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 30806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21318);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21319);
					s.is_empty()
				}
				['r', s @ ..] => super::t14::n21937(s, out),
				['s', s @ ..] => {
					super::push(out, 21321);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 30810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21322);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 21323);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 21324);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 21325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t14::n10660(s, out),
		['i', s @ ..] => super::t15::n10661(s, out),
		['o', s @ ..] => {
			super::push(out, 10539);
			match s {
				[] => true,
				['d', s @ ..] => super::t15::n21960(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t15::n10663(s, out),
		['s', s @ ..] => super::t15::n10664(s, out),
		['u', s @ ..] => super::t15::n10665(s, out),
		['ä', s @ ..] => super::t15::n10666(s, out),
		['ü', s @ ..] => super::t15::n10667(s, out),
		_ => false,
	}
}

/// Words ending -eit.
pub(crate) fn n3020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3015);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10574);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['d', 'r', s @ ..] => {
							super::push(out, 30901);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 37409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 30902);
							s.is_empty()
						}
						['h', 'i', s @ ..] => {
							super::push(out, 30903);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'i', 'r', s @ ..] => {
							super::push(out, 30904);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 30905);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 37411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'a', s @ ..] => {
							super::push(out, 30906);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
									super::push(out, 37412);
									s.is_empty()
								}
								['i', 'e', 'l', s @ ..] => {
									super::push(out, 37413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'h', s @ ..] => {
							super::push(out, 30907);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									match s {
										[] => true,
										['i', 'l', 't', s @ ..] => {
											super::push(out, 41353);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 41354);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 37414);
									s.is_empty()
								}
								['t', 'f', s @ ..] => {
									super::push(out, 37415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', s @ ..] => {
							super::push(out, 30908);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 30909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10575);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10576);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 21392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10577);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t15::n10701(s, out),
		['i', s @ ..] => {
			super::push(out, 10579);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 21405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t15::n10703(s, out),
		['l', s @ ..] => {
			super::push(out, 10581);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21408);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10582);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21410);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30972);
							match s {
								[] => true,
								['n', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 37525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 30973);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30974);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 30975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21411);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 37526);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 37527);
									match s {
										[] => true,
										['e', 'i', 'l', 'a', s @ ..] => {
											super::push(out, 41508);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 37528);
									match s {
										[] => true,
										['e', 'u', s @ ..] => {
											super::push(out, 41509);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 41510);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41511);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 10583);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21412);
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 30976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t15::n10707(s, out),
		_ => false,
	}
}

/// Words ending -fit.
pub(crate) fn n3021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3016);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10585);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 21425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10586);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 21426);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10587);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10588);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 21428);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10589);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 21430);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['m', 'u', 'i', s @ ..] => {
									match s {
										[] => true,
										['m', 'd', 'a', 'k', s @ ..] => {
											super::push(out, 41521);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41522);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'y', 'p', 'm', s @ ..] => {
									super::push(out, 37555);
									s.is_empty()
								}
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
			super::push(out, 10590);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21431);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10591);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 21433);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -git.
pub(crate) fn n3022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3017);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10592);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 21434);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10593);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10594);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21436);
					match s {
						[] => true,
						['l', 'a', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 31012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21437);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 37556);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 37557);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41523);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10595);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hit.
pub(crate) fn n3023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3018);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10597);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 10598);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 10599);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31013);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 31014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', s @ ..] => {
					super::push(out, 21438);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21439);
					match s {
						[] => true,
						['d', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 31015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 21440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10600);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10601);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 10602);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10603);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10604);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21442);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 31016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21443);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 31017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21444);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10605);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10606);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10607);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21446);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21447);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iit.
pub(crate) fn n3024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3019);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10608);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kit.
pub(crate) fn n3025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3020);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10610);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 10611);
			match s {
				[] => true,
				['e', 'b', 's', s @ ..] => {
					super::push(out, 21448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10612);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 21449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10613);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10614);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10615);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21450);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21451);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 31018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lit.
pub(crate) fn n3026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3021);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10616);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 10617);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 21454);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 31020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21455);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31021);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 31022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21456);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 21457);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10618);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21458);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21459);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 21460);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21461);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 21462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10619);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10621);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21464);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31025);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31026);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37558);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 21465);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31028);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 37560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 31029);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 37561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31030);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 31031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21466);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 21467);
					match s {
						[] => true,
						['h', 'p', 'o', 'k', s @ ..] => {
							super::push(out, 31032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10622);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31033);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 21468);
					match s {
						[] => true,
						['o', 'e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 31035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21469);
					s.is_empty()
				}
				['m', 'e', s @ ..] => {
					super::push(out, 21470);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10623);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21472);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 31036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21473);
					match s {
						[] => true,
						['n', 'e', 'n', s @ ..] => {
							super::push(out, 31037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10624);
			match s {
				[] => true,
				['e', 'h', 'c', 's', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 21474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10625);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 21475);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 21476);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mit.
pub(crate) fn n3027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3022);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10626);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10627);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21477);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 21478);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21479);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21480);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21481);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21482);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31038);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 31039);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 37562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10628);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21484);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10629);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10630);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 21485);
					match s {
						[] => true,
						['t', 'i', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 31041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10631);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10632);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21487);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21488);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21489);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21490);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 31043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10633);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10634);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21492);
					s.is_empty()
				}
				['z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nit.
pub(crate) fn n3028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3023);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10635);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					super::push(out, 21494);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21495);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31044);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21496);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 31046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21497);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 21498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10636);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21499);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21500);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31048);
							s.is_empty()
						}
						['e', 's', 'm', s @ ..] => {
							super::push(out, 31049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 21501);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31050);
							s.is_empty()
						}
						['l', 'i', '\u{1}', s @ ..] => {
							super::push(out, 31051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21502);
					match s {
						[] => true,
						['r', 'a', 'm', s @ ..] => {
							super::push(out, 31052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21503);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 21504);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 31053);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 37563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 21505);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10637);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10638);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 31055);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 31056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 21507);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21508);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37564);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21509);
					match s {
						[] => true,
						['e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10639);
			match s {
				[] => true,
				['e', 'l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 21510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10640);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21511);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 31058);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 21512);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 31060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10641);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 21513);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21514);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 31061);
							match s {
								[] => true,
								['t', 'i', 'l', 'o', 'y', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21515);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21516);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31062);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 31063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21517);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21518);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 31065);
							match s {
								[] => true,
								['e', 'v', 'f', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 31066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21519);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31067);
							match s {
								[] => true,
								['n', 'i', 'm', 'm', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 37569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 31068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10642);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10643);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21520);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 21521);
					match s {
						[] => true,
						['o', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 31069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 10644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oit.
pub(crate) fn n3029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3024);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10645);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10646);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10647);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10648);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10649);
			match s {
				[] => true,
				['o', 'h', 'p', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 21524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pit.
pub(crate) fn n3030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3025);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 10650);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10651);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10652);
			match s {
				[] => true,
				['g', 'o', 'l', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 21525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rit.
pub(crate) fn n3031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3026);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10653);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31070);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 31071);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 37570);
									s.is_empty()
								}
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
			super::push(out, 10654);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21526);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37571);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 37572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 31072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 21527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10655);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21528);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 21529);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31073);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21530);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21531);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21532);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 21533);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 31077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21534);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 21535);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10656);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10657);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10658);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21538);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 21539);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10659);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10660);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21542);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21543);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21544);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21545);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21546);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 31079);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 41524);
											s.is_empty()
										}
										['o', 'p', 'y', 'h', 'm', s @ ..] => {
											super::push(out, 41525);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 31080);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41526);
											s.is_empty()
										}
										['l', 'a', 'k', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 41527);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41528);
											s.is_empty()
										}
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
					super::push(out, 21547);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21548);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21549);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 21550);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10661);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 31082);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 37573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 31083);
							s.is_empty()
						}
						['o', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31084);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10662);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 31086);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 31087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10663);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21551);
					match s {
						[] => true,
						['n', 'm', s @ ..] => {
							super::push(out, 31088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10664);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21552);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 31089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 10665);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 21553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10666);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 21554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10667);
			match s {
				[] => true,
				['p', 'o', 'n', s @ ..] => {
					super::push(out, 21555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sit.
pub(crate) fn n3032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3027);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10668);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 21556);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31090);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 31091);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 31092);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37575);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37576);
									s.is_empty()
								}
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
			super::push(out, 10669);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10670);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 21558);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21559);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 21560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10671);
			match s {
				[] => true,
				['a', 'r', 't', 's', s @ ..] => {
					super::push(out, 21561);
					match s {
						[] => true,
						['u', 'n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 31093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10672);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21562);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21563);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 31094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10673);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21564);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21565);
					match s {
						[] => true,
						['p', 'o', 'r', 'y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 31095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 21566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10674);
			s.is_empty()
		}
		['y', 'h', s @ ..] => {
			super::push(out, 10675);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 10676);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tit.
pub(crate) fn n3033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3028);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10677);
			match s {
				[] => true,
				['p', 'a', 'l', s @ ..] => {
					super::push(out, 21568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10678);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 21569);
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37577);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 37578);
									s.is_empty()
								}
								['o', 'k', 'i', 's', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 37579);
									s.is_empty()
								}
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
			super::push(out, 10679);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21570);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10680);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 21572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10681);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10682);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10683);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uit.
pub(crate) fn n3034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3029);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10684);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 10685);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21576);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10686);
			match s {
				[] => true,
				['s', 'i', 'b', 'l', s @ ..] => {
					super::push(out, 21578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 10687);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10688);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21580);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 21581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10689);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vit.
pub(crate) fn n3035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3030);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10690);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21583);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31096);
							s.is_empty()
						}
						['l', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 31097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21584);
					match s {
						[] => true,
						['o', 'v', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 31098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 21585);
					match s {
						[] => true,
						['q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10691);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10692);
			match s {
				[] => true,
				['n', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 21587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10693);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10694);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wit.
pub(crate) fn n3036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3031);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10695);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10696);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21590);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10697);
			s.is_empty()
		}
		['r', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 10698);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xit.
pub(crate) fn n3037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3032);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10699);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21592);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21593);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10700);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yit.
pub(crate) fn n3038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3033);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10701);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10702);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zit.
pub(crate) fn n3039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3034);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10704);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10705);
			match s {
				[] => true,
				['f', 'e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31100);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31101);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 's', 'r', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 41529);
											s.is_empty()
										}
										['l', 'h', 'a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 41530);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 37580);
									match s {
										[] => true,
										['e', 'd', 'n', 'a', 'h', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43593);
													s.is_empty()
												}
												['n', 'e', 'ß', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 43594);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'i', 'e', 'k', 'm', 'a', 's', 'k', 'r', 'e', 'm', 'f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'n', 'a', 'l', 'i', 'b', 's', 'g', 'n', 'u', 'l', 'h', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 31102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10706);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 21596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10707);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10708);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 10709);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ekt.
pub(crate) fn n3041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3036);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 10720);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21612);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['m', 'u', 't', 'p', 'e', 's', 'l', 'e', 'k', 'i', 'r', 't', 'n', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 37586);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 41533);
											s.is_empty()
										}
										['n', 'i', 'h', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 41534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41535);
											s.is_empty()
										}
										['o', 't', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 41536);
											s.is_empty()
										}
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
					super::push(out, 21613);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31119);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37587);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 37588);
									match s {
										[] => true,
										['d', 'n', 'a', s @ ..] => {
											super::push(out, 41537);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 41538);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 41539);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 41540);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 41541);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 43595);
													s.is_empty()
												}
												['o', 'i', s @ ..] => {
													super::push(out, 43596);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 41542);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 41543);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 41544);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 37589);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 37590);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 37591);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 37592);
									match s {
										[] => true,
										['e', 's', s @ ..] => {
											super::push(out, 41545);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 41546);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37593);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41547);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 43597);
													match s {
														[] => true,
														['a', 'k', 's', '\u{1}', s @ ..] => super::t15::n47575(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 41548);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 37594);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37595);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 41549);
											match s {
												[] => true,
												['t', 'a', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 43598);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 37596);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 41550);
											match s {
												[] => true,
												['n', 'u', s @ ..] => {
													match s {
														[] => true,
														['l', s @ ..] => super::t15::n47576(s, out),
														['r', 'e', 'i', 'd', s @ ..] => super::t15::n47577(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', 'o', 'i', 't', 'a', 'd', s @ ..] => {
											super::push(out, 41551);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 41552);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 41553);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 43599);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21614);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31120);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 31121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21615);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31122);
							match s {
								[] => true,
								['p', s @ ..] => super::t15::n39800(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 21616);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 31123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => super::t15::n10845(s, out),
		['l', s @ ..] => super::t15::n10846(s, out),
		['p', s @ ..] => super::t15::n10847(s, out),
		['s', s @ ..] => super::t15::n10848(s, out),
		['t', s @ ..] => super::t15::n10849(s, out),
		_ => false,
	}
}

/// Words ending -lskonflikt.
pub(crate) fn n44238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41568);
	s.is_empty()
}

/// Words ending -tkonflikt.
pub(crate) fn n39823(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'e', 'd', s @ ..] => {
			super::push(out, 41569);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 41570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rikt.
pub(crate) fn n10852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10727);
	match s {
		[] => true,
		['t', 's', 'i', 'd', 't', s @ ..] => {
			super::push(out, 21630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vikt.
pub(crate) fn n10853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10728);
	s.is_empty()
}

/// Words ending -nkt.
pub(crate) fn n3043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3038);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10729);
			match s {
				[] => true,
				['t', 's', 'n', 'i', 'r', 'e', 't', s @ ..] => {
					super::push(out, 21631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10730);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 21632);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 31135);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 37615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 21633);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 31136);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 37616);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 37617);
									match s {
										[] => true,
										['p', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 41571);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['a', 'r', s @ ..] => {
							super::push(out, 31137);
							match s {
								[] => true,
								['t', 'n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'i', s @ ..] => {
							super::push(out, 31138);
							s.is_empty()
						}
						['f', 'f', 'e', 'r', 't', 's', s @ ..] => {
							super::push(out, 31139);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 31140);
							match s {
								[] => true,
								['c', 'i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 37619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'c', 'e', s @ ..] => {
							super::push(out, 31141);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 37621);
									s.is_empty()
								}
								['t', 'i', 'm', 's', s @ ..] => {
									super::push(out, 37622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 31142);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 41572);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
													super::push(out, 43601);
													s.is_empty()
												}
												['o', 'n', 'k', 's', 'r', 'h', 'e', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 43602);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'e', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 37623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 31143);
							match s {
								[] => true,
								['i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37624);
									s.is_empty()
								}
								['p', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'w', s @ ..] => {
							super::push(out, 31144);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31145);
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									super::push(out, 37626);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 41573);
											match s {
												[] => true,
												['u', 'ä', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43603);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', s @ ..] => {
											super::push(out, 41574);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'i', 't', 'a', 'm', s @ ..] => {
									super::push(out, 37627);
									match s {
										[] => true,
										['i', 'l', 'b', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 41575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31146);
							match s {
								[] => true,
								['i', 'e', 'z', 's', s @ ..] => {
									super::push(out, 37628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 31147);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 37629);
									s.is_empty()
								}
								['t', 'ü', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 41576);
											match s {
												[] => true,
												['a', 'i', 'c', '\u{1}', s @ ..] => {
													super::push(out, 43604);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 41577);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 43605);
													s.is_empty()
												}
												['n', 'i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 43606);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 41578);
											match s {
												[] => true,
												['e', 'd', 'n', 'u', 'b', '\u{1}', s @ ..] => {
													super::push(out, 43607);
													s.is_empty()
												}
												['l', 'e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43608);
													s.is_empty()
												}
												['n', 'o', 'i', 't', 'a', 'k', 'i', 'n', 'u', 'm', 'm', 'o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 43609);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -okt.
pub(crate) fn n3044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3039);
	s.is_empty()
}

/// Words ending -rkt.
pub(crate) fn n3045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3040);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21634);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 31148);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 37630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 21635);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31149);
							match s {
								[] => true,
								['s', 's', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', s @ ..] => {
							super::push(out, 31150);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 37632);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 37633);
									match s {
										[] => true,
										['a', 'v', 'i', 'r', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 41579);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 31151);
							match s {
								[] => true,
								['f', 'o', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['h', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 41580);
											s.is_empty()
										}
										['r', 'e', 's', 's', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 41581);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'm', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 37634);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37635);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 37636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 31152);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 31153);
							match s {
								[] => true,
								['a', 't', 'i', 'p', 'a', 'k', 's', s @ ..] => {
									super::push(out, 37637);
									match s {
										[] => true,
										['i', 'n', 'g', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 41582);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 37638);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 41583);
											s.is_empty()
										}
										_ => false,
									}
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
										['d', s @ ..] => {
											super::push(out, 41584);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 43610);
													s.is_empty()
												}
												['r', 'a', 'i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 43611);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 41585);
											match s {
												[] => true,
												['a', 'w', 't', 'h', 'c', 'u', 'a', 'r', 'b', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 43612);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 41586);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 41587);
											match s {
												[] => true,
												['l', 'u', 'b', '\u{1}', s @ ..] => {
													super::push(out, 43613);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 41588);
											match s {
												[] => true,
												['ä', 'b', '\u{1}', s @ ..] => {
													super::push(out, 43614);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 41589);
											match s {
												[] => true,
												['i', 'v', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 43615);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'k', s @ ..] => {
											super::push(out, 41590);
											match s {
												[] => true,
												['e', 'f', 'f', 'e', '\u{1}', s @ ..] => {
													super::push(out, 43616);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'm', s @ ..] => {
									super::push(out, 37639);
									match s {
										[] => true,
										['r', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 41591);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 31154);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 41592);
											s.is_empty()
										}
										['i', 'p', 'a', 'p', 't', 'r', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 41593);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 41594);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 37640);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 37641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'r', s @ ..] => {
									super::push(out, 37642);
									match s {
										[] => true,
										['h', 'ä', 'w', '\u{1}', s @ ..] => {
											super::push(out, 41596);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37643);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 41597);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 37644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31155);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 37645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 31156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ukt.
pub(crate) fn n3046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3041);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21636);
					match s {
						[] => true,
						['i', 'v', 'n', s @ ..] => {
							super::push(out, 31157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21637);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 31158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21638);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21639);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['h', 'o', s @ ..] => {
									super::push(out, 37646);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 37647);
									match s {
										[] => true,
										['a', 'b', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 41598);
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['m', 'a', 'r', 'e', 'k', '\u{1}', s @ ..] => {
													super::push(out, 43617);
													s.is_empty()
												}
												['t', 'e', 'm', 's', 'o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 43618);
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
										['e', 's', s @ ..] => {
											super::push(out, 41599);
											s.is_empty()
										}
										['l', 'a', 'f', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41600);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41601);
											s.is_empty()
										}
										['i', 'z', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 41602);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', 'l', s @ ..] => {
									super::push(out, 37648);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 41603);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 41604);
											s.is_empty()
										}
										['l', 'l', 'a', 'f', 'r', s @ ..] => {
											super::push(out, 41605);
											s.is_empty()
										}
										['t', 'ä', s @ ..] => {
											super::push(out, 41606);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 37649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 21640);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 31160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gehalt.
pub(crate) fn n32637(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37650);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 37651);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 41607);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 37652);
			match s {
				[] => true,
				['f', 'o', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['k', 'c', 'i', 't', 's', 't', s @ ..] => {
							super::push(out, 43619);
							s.is_empty()
						}
						['n', 'e', 'l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 43620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 37653);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 43621);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 43622);
							match s {
								[] => true,
								['i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 44567);
									s.is_empty()
								}
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
			super::push(out, 37654);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41609);
					match s {
						[] => true,
						['m', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 43623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', s @ ..] => {
					super::push(out, 41610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 37655);
			match s {
				[] => true,
				['u', 'i', 'd', s @ ..] => {
					super::push(out, 41611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 37656);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41612);
					match s {
						[] => true,
						['b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 43624);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 43625);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43626);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 43627);
							match s {
								[] => true,
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 44568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 43628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 41613);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 43629);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 43630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 37657);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 41614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 37658);
			match s {
				[] => true,
				['e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 41615);
					s.is_empty()
				}
				['o', 'u', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 41616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 37659);
			match s {
				[] => true,
				['e', 'r', 'h', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 41617);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 41618);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 43631);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 43632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 37660);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 41619);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41620);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 43633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nhalt.
pub(crate) fn n22289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21645);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 31162);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 37661);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 37662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37663);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 37664);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 43634);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 43635);
									s.is_empty()
								}
								['s', 's', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 43636);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rhalt.
pub(crate) fn n22291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21647);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 37665);
					match s {
						[] => true,
						['e', 's', 's', 'a', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 41621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 37666);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43637);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 43638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 37667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -agshaushalt.
pub(crate) fn n39885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37668);
	s.is_empty()
}

/// Words ending -bushalt.
pub(crate) fn n39886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37669);
	s.is_empty()
}

/// Words ending -thalt.
pub(crate) fn n22293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21649);
	match s {
		[] => true,
		['n', 'e', 'f', 'u', 'a', 'd', s @ ..] => {
			super::push(out, 31164);
			match s {
				[] => true,
				['n', 'a', 'l', 'h', 'c', 's', 't', 'u', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 37670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -malt.
pub(crate) fn n10861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10734);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 21650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nalt.
pub(crate) fn n10862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10735);
	s.is_empty()
}

/// Words ending -palt.
pub(crate) fn n10863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10736);
	match s {
		[] => true,
		['s', 'e', s @ ..] => {
			super::push(out, 21651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -salt.
pub(crate) fn n10864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10737);
	s.is_empty()
}

/// Words ending -talt.
pub(crate) fn n10865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10738);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 21652);
			match s {
				[] => true,
				['e', 'g', 'l', s @ ..] => {
					super::push(out, 31165);
					s.is_empty()
				}
				['n', 'a', 'd', s @ ..] => {
					super::push(out, 31166);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 37671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -walt.
pub(crate) fn n10866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10739);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21653);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 37672);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 37673);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 37674);
							match s {
								[] => true,
								['e', 'l', s @ ..] => {
									super::push(out, 41622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'g', s @ ..] => {
									super::push(out, 41623);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 41624);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 41625);
									s.is_empty()
								}
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
			super::push(out, 21654);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31167);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 37675);
							match s {
								[] => true,
								['i', 'l', 'i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 41626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -elt.
pub(crate) fn n3048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3043);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10741);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21656);
					match s {
						[] => true,
						['e', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 31168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21657);
					match s {
						[] => true,
						['n', 'e', 's', 'g', 'n', 'u', 'l', s @ ..] => {
							super::push(out, 31169);
							match s {
								[] => true,
								['t', 't', 'i', 'm', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 37676);
									s.is_empty()
								}
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
			super::push(out, 10742);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 21658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10743);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21659);
					s.is_empty()
				}
				['h', 'c', 'a', 'n', s @ ..] => {
					super::push(out, 21660);
					s.is_empty()
				}
				['l', 'l', s @ ..] => {
					super::push(out, 21661);
					s.is_empty()
				}
				['m', 'l', s @ ..] => {
					super::push(out, 21662);
					s.is_empty()
				}
				['n', 'e', 'g', 'o', s @ ..] => {
					super::push(out, 21663);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37677);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 37678);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 31170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', s @ ..] => {
					super::push(out, 21664);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 21665);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 21666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10744);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 21667);
					match s {
						[] => true,
						['n', 'i', 'l', 't', 'h', 'c', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 31171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21668);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21669);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 31172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21670);
					s.is_empty()
				}
				['s', 'l', s @ ..] => {
					super::push(out, 21671);
					match s {
						[] => true,
						['e', 'm', 'm', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 31173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21672);
					match s {
						[] => true,
						['s', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 31174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilt.
pub(crate) fn n3049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3044);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 10745);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olt.
pub(crate) fn n3050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3045);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 10747);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10748);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 21673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 10749);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21674);
					s.is_empty()
				}
				['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 21675);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21676);
					match s {
						[] => true,
						['e', 'n', 'o', 'r', 't', 'k', 'e', 'l', 'e', 'o', s @ ..] => {
							super::push(out, 31175);
							match s {
								[] => true,
								['l', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21677);
					match s {
						[] => true,
						['l', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 31176);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ult.
pub(crate) fn n3051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3046);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10750);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 21678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10751);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 10752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10753);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 21680);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 31178);
							match s {
								[] => true,
								['a', 'r', 'h', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 21681);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21682);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10754);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21684);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ült.
pub(crate) fn n3052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3047);
	s.is_empty()
}

/// Words ending -olant.
pub(crate) fn n22363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21717);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 31200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mant.
pub(crate) fn n10902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10775);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21719);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 21720);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 31201);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 37688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 21721);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 21722);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 31202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nant.
pub(crate) fn n10903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10776);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 21723);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 21724);
			match s {
				[] => true,
				['u', 'e', 'l', 'r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 31203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pant.
pub(crate) fn n10904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10777);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 21725);
			s.is_empty()
		}
		['u', 'k', 'k', 'o', '\u{1}', s @ ..] => {
			super::push(out, 21726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rant.
pub(crate) fn n10905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10778);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21727);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 21728);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 31204);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 31205);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 37689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 21729);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 21730);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 21731);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 21732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31206);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 21733);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 37690);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37691);
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
				['b', s @ ..] => {
					super::push(out, 31208);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 31209);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 31210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 21734);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 21735);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 21736);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31212);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 31213);
					match s {
						[] => true,
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 37692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 31214);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 31215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37693);
							s.is_empty()
						}
						['s', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 37694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31216);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 37695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sant.
pub(crate) fn n10906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10779);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 21738);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 21739);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31218);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tant.
pub(crate) fn n10907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10780);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 21740);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21741);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 31221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21742);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 31222);
					match s {
						[] => true,
						['o', 'c', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 37696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 21743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uant.
pub(crate) fn n10908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10781);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 21744);
			s.is_empty()
		}
		['t', 'l', 'u', 'm', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 21745);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vant.
pub(crate) fn n10909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10782);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21746);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 21747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -want.
pub(crate) fn n10910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10783);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yant.
pub(crate) fn n10911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10784);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 21749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ent.
pub(crate) fn n3059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3054);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10785);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 10786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21750);
					s.is_empty()
				}
				['o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 21751);
					s.is_empty()
				}
				['u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 21752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10787);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21753);
					match s {
						[] => true,
						['c', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 31223);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31224);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 37697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 31225);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 31226);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 37698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21754);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 21755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10788);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21756);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21757);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21758);
					match s {
						[] => true,
						['r', 'i', 'd', 'l', s @ ..] => {
							super::push(out, 31228);
							match s {
								[] => true,
								['a', 'i', 'r', 'e', 't', 's', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21759);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31229);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 37700);
									match s {
										[] => true,
										['t', 's', 'd', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41628);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 37701);
									match s {
										[] => true,
										['n', 'o', 'k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43641);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 43642);
													s.is_empty()
												}
												['n', 'e', 'p', 'p', 'u', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 43643);
													s.is_empty()
												}
												['s', 'g', 'n', 'i', 'l', 't', 'h', 'c', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 43644);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10789);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21760);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 21761);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 31230);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 41629);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 41630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 31231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10790);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21762);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 21763);
					match s {
						[] => true,
						['a', 'r', 'g', 'e', s @ ..] => {
							super::push(out, 31233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21764);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 31234);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 31235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21765);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31236);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 31237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21766);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 21767);
					match s {
						[] => true,
						['i', 'f', 'f', 'e', 'o', 'k', 's', 'n', 'o', 'i', 't', 'a', s @ ..] => {
							super::push(out, 31238);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 37702);
									match s {
										[] => true,
										['r', 'a', 't', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 41631);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10791);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21768);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 37703);
									s.is_empty()
								}
								['s', 'g', s @ ..] => {
									super::push(out, 37704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21769);
					match s {
						[] => true,
						['e', 'p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 31239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t15::n10919(s, out),
		['n', s @ ..] => {
			super::push(out, 10793);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21782);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 37741);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37742);
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41678);
											s.is_empty()
										}
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
					super::push(out, 21783);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 21784);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10794);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21786);
					match s {
						[] => true,
						['p', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 31272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 21787);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21788);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31273);
							match s {
								[] => true,
								['t', 't', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 37743);
									s.is_empty()
								}
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
			super::push(out, 10795);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21789);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 31274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21790);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21791);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 21792);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 31275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10796);
			match s {
				[] => true,
				['a', s @ ..] => super::t15::n22440(s, out),
				['e', s @ ..] => {
					super::push(out, 21794);
					s.is_empty()
				}
				['n', s @ ..] => super::t15::n22442(s, out),
				['s', s @ ..] => super::t15::n22443(s, out),
				['t', s @ ..] => super::t15::n22444(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n10924(s, out),
		['v', s @ ..] => super::t15::n10925(s, out),
		['z', s @ ..] => super::t15::n10926(s, out),
		_ => false,
	}
}

/// Words ending -hnt.
pub(crate) fn n3060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3055);
	match s {
		[] => true,
		['e', 'z', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 21808);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31289);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 31290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -int.
pub(crate) fn n3061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3056);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10800);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10801);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10802);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 21810);
					match s {
						[] => true,
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 31291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10803);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 21811);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 31292);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31293);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31294);
							match s {
								[] => true,
								['i', 'd', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 37754);
									s.is_empty()
								}
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
			super::push(out, 10804);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21812);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10805);
			match s {
				[] => true,
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 21814);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31296);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 31297);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 31298);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10806);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 21815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10807);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 21816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ont.
pub(crate) fn n3062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3057);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 10808);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10809);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10810);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21817);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 31300);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37755);
									s.is_empty()
								}
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
			super::push(out, 10811);
			match s {
				[] => true,
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 21818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10812);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 21819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10813);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21820);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 21821);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 31302);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 't', s @ ..] => {
							super::push(out, 31303);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 37757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'u', s @ ..] => {
							super::push(out, 31304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10814);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21822);
					match s {
						[] => true,
						['r', 'o', 'h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37758);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 37759);
									match s {
										[] => true,
										['g', 'a', 'l', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41680);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unt.
pub(crate) fn n3063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3058);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 10815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21823);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10816);
			match s {
				[] => true,
				['c', 'c', s @ ..] => {
					super::push(out, 21825);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word kot.
pub(crate) fn n10967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10839);
	s.is_empty()
}

/// Words ending -ekot.
pub(crate) fn n10968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10840);
	match s {
		[] => true,
		['d', 'r', 'e', 'f', 'p', '\u{1}', s @ ..] => {
			super::push(out, 21842);
			s.is_empty()
		}
		['s', 'u', 'ä', 'm', '\u{1}', s @ ..] => {
			super::push(out, 21843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ikot.
pub(crate) fn n10969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10841);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 21844);
			match s {
				[] => true,
				['t', 'g', s @ ..] => {
					super::push(out, 31313);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kkot.
pub(crate) fn n10970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10842);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 21845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lkot.
pub(crate) fn n10971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10843);
	match s {
		[] => true,
		['e', 'g', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 21846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nkot.
pub(crate) fn n10972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10844);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'u', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 31314);
					s.is_empty()
				}
				['h', 'c', 'n', 'i', 'n', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 31315);
					s.is_empty()
				}
				['t', 't', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 31316);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 31317);
					s.is_empty()
				}
				['z', 't', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 31318);
					s.is_empty()
				}
				['ß', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 31319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rkot.
pub(crate) fn n10973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10845);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 31320);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 37768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 31321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -skot.
pub(crate) fn n10974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10846);
	match s {
		[] => true,
		['h', 'c', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 21847);
			s.is_empty()
		}
		['s', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 21848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lot.
pub(crate) fn n3073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3068);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10847);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10848);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21849);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21850);
					s.is_empty()
				}
				['z', 'o', s @ ..] => {
					super::push(out, 21851);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10849);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21852);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31323);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37769);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 37770);
									s.is_empty()
								}
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
			super::push(out, 10850);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10851);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 21853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10852);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10853);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21854);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31325);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 31326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mot.
pub(crate) fn n3074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3069);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 10855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -not.
pub(crate) fn n3075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3070);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10856);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 21855);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10857);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21857);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10858);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10859);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oot.
pub(crate) fn n3076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3071);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10860);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21860);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 31327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 21861);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							super::push(out, 31328);
							s.is_empty()
						}
						['l', 'e', 'n', 'h', 'c', 's', 'n', s @ ..] => {
							super::push(out, 31329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'ä', 'r', 'n', 'e', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 21862);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21863);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31330);
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21864);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21865);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31331);
							s.is_empty()
						}
						['u', 'x', s @ ..] => {
							super::push(out, 31332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', 'r', s @ ..] => {
					super::push(out, 21866);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 31333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 21867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10861);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 21868);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21869);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pot.
pub(crate) fn n3077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3072);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10862);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 21870);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21871);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 31335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10863);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10864);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21872);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 31336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rot.
pub(crate) fn n3078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3073);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10865);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 10866);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10867);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 21873);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 21874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10868);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21875);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 21876);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21877);
					match s {
						[] => true,
						['d', 'a', 'l', 'e', 'm', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 31337);
							s.is_empty()
						}
						['k', 'c', 'ä', 'n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 31338);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 37773);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 37774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31339);
							match s {
								[] => true,
								['h', 'c', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 37775);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 37776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'c', 's', s @ ..] => {
					super::push(out, 21878);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41689);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 41690);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 37777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21879);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 31340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'd', 'n', 'r', 'ö', 'k', '\u{1}', s @ ..] => {
					super::push(out, 21880);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 21881);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 37778);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 43676);
													match s {
														[] => true,
														['e', 'm', 'r', 'a', 'm', '\u{1}', s @ ..] => super::t15::n47606(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 43677);
													match s {
														[] => true,
														['g', '\u{1}', s @ ..] => super::t15::n47607(s, out),
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
									super::push(out, 37779);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 41691);
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											super::push(out, 41692);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 37780);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 41693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37781);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41694);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 37782);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 41695);
											match s {
												[] => true,
												['u', 'r', '\u{1}', s @ ..] => {
													super::push(out, 43678);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'a', 'j', '\u{1}', s @ ..] => {
											super::push(out, 41696);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 37783);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 41697);
											s.is_empty()
										}
										['t', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 41698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'o', 'k', 'l', 'l', 'o', 'v', 'n', s @ ..] => {
							super::push(out, 31341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21882);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21883);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 31342);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 31343);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21884);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 21885);
					match s {
						[] => true,
						['n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 31344);
							s.is_empty()
						}
						['r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 31345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10869);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10870);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21887);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 37785);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 41699);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 37786);
									match s {
										[] => true,
										['r', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 41700);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 37787);
									match s {
										[] => true,
										['e', 'g', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 41701);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 37788);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 37789);
									s.is_empty()
								}
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
			super::push(out, 10871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10872);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10873);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10874);
			match s {
				[] => true,
				['e', s @ ..] => super::t15::n22542(s, out),
				['i', 'e', s @ ..] => super::t15::n22543(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t15::n11003(s, out),
		['t', s @ ..] => super::t15::n11004(s, out),
		_ => false,
	}
}

/// Words ending -sot.
pub(crate) fn n3079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3074);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10877);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tot.
pub(crate) fn n3080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3075);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uot.
pub(crate) fn n3081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3076);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 10880);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21895);
					s.is_empty()
				}
				['i', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vot.
pub(crate) fn n3082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3077);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xot.
pub(crate) fn n3083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3078);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yot.
pub(crate) fn n3084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3079);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10884);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10885);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10886);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 21897);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tart.
pub(crate) fn n11038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10908);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 21929);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 21930);
			s.is_empty()
		}
		['h', 'c', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 21931);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 21932);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 31371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 21933);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31372);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 31373);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 37799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 31374);
					match s {
						[] => true,
						['e', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 37800);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 37801);
							s.is_empty()
						}
						['u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 37802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 31375);
					match s {
						[] => true,
						['c', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 37803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 31376);
					match s {
						[] => true,
						['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 37804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 31377);
					match s {
						[] => true,
						['r', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 37805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 31378);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 41703);
									s.is_empty()
								}
								['t', 'e', 'k', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 41704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 37806);
							match s {
								[] => true,
								['s', 'i', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 41705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31379);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 37807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 31380);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 37808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31381);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 37809);
							match s {
								[] => true,
								['e', 'j', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 41706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'a', 'r', 'k', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 21934);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wart.
pub(crate) fn n11040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10910);
	match s {
		[] => true,
		['b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 21938);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 21939);
			match s {
				[] => true,
				['c', 'o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 31383);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 31384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 21940);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'f', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 37810);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 37811);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 37812);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 37813);
							match s {
								[] => true,
								['s', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 41707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 37814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 21941);
			s.is_empty()
		}
		['z', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 21942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ert.
pub(crate) fn n3090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3085);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10912);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21943);
					match s {
						[] => true,
						['e', 'm', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 31385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10913);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21944);
					match s {
						[] => true,
						['u', 'h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37815);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37816);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41708);
											match s {
												[] => true,
												['i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 43679);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 41709);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10914);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 10915);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 21945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10916);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10917);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 21947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10918);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21948);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31386);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10919);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10920);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21952);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10921);
			match s {
				[] => true,
				['n', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 21954);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10922);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 21956);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31389);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 31390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 10923);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21959);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10924);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 21961);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 31391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 21962);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31392);
							s.is_empty()
						}
						['r', 'a', 'd', 'n', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 31393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 21963);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21964);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 31394);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37817);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 41710);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 37818);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 41711);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 37819);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 41712);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 41713);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 43680);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 37820);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 41714);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 31395);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 37821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 31396);
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 37822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21965);
					match s {
						[] => true,
						['e', 'b', 's', 's', 'u', 'l', 'f', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 21966);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21967);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 31398);
							match s {
								[] => true,
								['t', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21968);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 37824);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 37825);
									match s {
										[] => true,
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 41715);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'n', s @ ..] => {
									super::push(out, 37826);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 37827);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 31399);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21969);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 31400);
							s.is_empty()
						}
						['h', 'e', s @ ..] => super::t15::n32900(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t15::n22625(s, out),
				['t', s @ ..] => super::t15::n22626(s, out),
				['z', s @ ..] => {
					super::push(out, 21972);
					match s {
						[] => true,
						['n', 'ü', s @ ..] => super::t15::n32909(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t15::n11055(s, out),
		_ => false,
	}
}

/// Words ending -hrt.
pub(crate) fn n3091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3086);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10926);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 31410);
							match s {
								[] => true,
								['f', 'i', 'h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', s @ ..] => {
											super::push(out, 41718);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41719);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 41720);
											s.is_empty()
										}
										['t', 'h', s @ ..] => {
											super::push(out, 41721);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 37834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'o', s @ ..] => {
							super::push(out, 31411);
							s.is_empty()
						}
						['m', 'u', 'a', 'r', 't', s @ ..] => {
							super::push(out, 31412);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31413);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31414);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31415);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31416);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 37835);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 41722);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 10927);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irt.
pub(crate) fn n3092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3087);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 10928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21975);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21976);
					match s {
						[] => true,
						['n', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 31417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 21977);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21978);
					match s {
						[] => true,
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 31418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 21979);
					match s {
						[] => true,
						['e', 'g', 'e', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 31419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 21980);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 21981);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 31420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10929);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 21982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10930);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21983);
					match s {
						[] => true,
						['d', 'r', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 31421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 't', 'h', 'c', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 21984);
					s.is_empty()
				}
				['s', 'k', s @ ..] => {
					super::push(out, 21985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ort.
pub(crate) fn n3093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3088);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10931);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 21986);
					s.is_empty()
				}
				['i', 'e', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 21987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10932);
			match s {
				[] => true,
				['n', 'a', 't', 's', 's', s @ ..] => {
					super::push(out, 21988);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31422);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10933);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 10934);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 21989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10935);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10936);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 21990);
					match s {
						[] => true,
						['r', 'e', 'x', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10937);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10938);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 21991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10939);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10940);
			match s {
				[] => true,
				['e', 't', 's', 'ü', s @ ..] => {
					super::push(out, 21992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10941);
			match s {
				[] => true,
				['e', 'r', '-', s @ ..] => {
					super::push(out, 21993);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21994);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 21995);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21996);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37836);
									s.is_empty()
								}
								['-', 'l', 'e', 'v', 'e', 'l', '-', 't', 's', 'r', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 37837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21997);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31426);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 31427);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 37838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', s @ ..] => {
							super::push(out, 31428);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 31429);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 31430);
							match s {
								[] => true,
								['a', 'r', 'r', s @ ..] => {
									super::push(out, 37839);
									s.is_empty()
								}
								['n', 'e', 'b', s @ ..] => {
									super::push(out, 37840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 31431);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 31432);
							match s {
								[] => true,
								['p', 'm', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 31433);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 31434);
							s.is_empty()
						}
						['k', 'c', s @ ..] => {
							super::push(out, 31435);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 31436);
							match s {
								[] => true,
								['a', 'n', 'o', 'i', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 37842);
									s.is_empty()
								}
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41723);
											s.is_empty()
										}
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41724);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'b', 'o', 'm', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37843);
									s.is_empty()
								}
								['l', 'a', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41725);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 41726);
											s.is_empty()
										}
										['ß', s @ ..] => {
											super::push(out, 41727);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 37844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 31437);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37845);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 37846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 31438);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 37847);
									match s {
										[] => true,
										['r', 't', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 43681);
													match s {
														[] => true,
														['a', '\u{1}', s @ ..] => super::t15::n47608(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													super::push(out, 43682);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 37848);
									match s {
										[] => true,
										['b', 'u', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 41728);
											s.is_empty()
										}
										['i', 'l', 'i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 41729);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 41730);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41731);
											s.is_empty()
										}
										['z', 't', 'i', 'p', 's', '\u{1}', s @ ..] => super::t15::n44426(s, out),
										_ => false,
									}
								}
								['n', s @ ..] => super::t15::n40081(s, out),
								['u', s @ ..] => super::t15::n40082(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t15::n32940(s, out),
						['r', s @ ..] => super::t15::n32941(s, out),
						['s', 'k', s @ ..] => super::t15::n32942(s, out),
						['t', s @ ..] => super::t15::n32943(s, out),
						['x', s @ ..] => super::t15::n32944(s, out),
						['ß', s @ ..] => super::t15::n32945(s, out),
						_ => false,
					}
				}
				['x', s @ ..] => super::t15::n22655(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t15::n11072(s, out),
		['s', s @ ..] => super::t15::n11073(s, out),
		['t', s @ ..] => super::t15::n11074(s, out),
		['u', s @ ..] => super::t15::n11075(s, out),
		['w', s @ ..] => super::t15::n11076(s, out),
		['z', s @ ..] => super::t15::n11077(s, out),
		['ß', s @ ..] => super::t15::n11078(s, out),
		_ => false,
	}
}

/// Words ending -urt.
pub(crate) fn n3094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3089);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10949);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 10950);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 10951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22024);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22025);
					s.is_empty()
				}
				['g', 'n', 'a', 'f', 'f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 22026);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22027);
					match s {
						[] => true,
						['l', 'a', 'n', 'h', 'c', 's', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22028);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22029);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 31480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22030);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 37894);
									s.is_empty()
								}
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 37895);
									s.is_empty()
								}
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
			super::push(out, 10952);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22031);
					match s {
						[] => true,
						['o', 'j', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37896);
									s.is_empty()
								}
								['e', 'l', s @ ..] => {
									super::push(out, 37897);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 37898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 22032);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 31481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10953);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10954);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 22034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31482);
							s.is_empty()
						}
						['d', 'n', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37899);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 37900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -alast.
pub(crate) fn n22706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22047);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 31492);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 37903);
					match s {
						[] => true,
						['e', 'é', 's', 'y', 'l', 'é', '\u{1}', s @ ..] => {
							super::push(out, 41747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 41748);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41749);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 43692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 37904);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -blast.
pub(crate) fn n22707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22048);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 31493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pelast.
pub(crate) fn n33007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31495);
	s.is_empty()
}

/// Words ending -glast.
pub(crate) fn n22710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31496);
			s.is_empty()
		}
		['n', 'e', 'n', 'n', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 31497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -klast.
pub(crate) fn n22711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22052);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 31498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -llast.
pub(crate) fn n22712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22053);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31499);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 37906);
					s.is_empty()
				}
				['i', 'z', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 37907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nlast.
pub(crate) fn n22713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22054);
	s.is_empty()
}

/// Words ending -plast.
pub(crate) fn n22714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22055);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 31500);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 37908);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 37909);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 37910);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 41750);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 41751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rlast.
pub(crate) fn n22715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22056);
	s.is_empty()
}

/// Words ending -slast.
pub(crate) fn n22716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22057);
	s.is_empty()
}

/// Words ending -tlast.
pub(crate) fn n22717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22058);
	s.is_empty()
}

/// Words ending -zlast.
pub(crate) fn n22718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22059);
	s.is_empty()
}

/// Words ending -mast.
pub(crate) fn n11096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10966);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22060);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 22061);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 31501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 22062);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31502);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 31503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22063);
			match s {
				[] => true,
				['n', 'u', 'f', 'l', 'i', 'b', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 31504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22064);
			match s {
				[] => true,
				['a', 'n', 'g', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 31505);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 31506);
					s.is_empty()
				}
				['h', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 31507);
					s.is_empty()
				}
				['l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 31508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 22065);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22066);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31509);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 31510);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 37911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 37912);
							s.is_empty()
						}
						['t', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 37913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22067);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 37914);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 37915);
							match s {
								[] => true,
								['t', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 41752);
									s.is_empty()
								}
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
			super::push(out, 22068);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 31511);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 31512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 22069);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 31513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'l', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 22070);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 22071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nast.
pub(crate) fn n11097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10967);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 22072);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31514);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22073);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 22074);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 31516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 22075);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 31517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 22076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oast.
pub(crate) fn n11098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10968);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 22077);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 31518);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 37916);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 37917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -past.
pub(crate) fn n11099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10969);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22078);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 22079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rast.
pub(crate) fn n11100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10970);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22080);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 22081);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 22082);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31519);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 31520);
					match s {
						[] => true,
						['n', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 37918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22083);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 31521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 22084);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 22085);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 22086);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sast.
pub(crate) fn n11101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10971);
	s.is_empty()
}

/// Words ending -tast.
pub(crate) fn n11102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10972);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 22087);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uast.
pub(crate) fn n11103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10973);
	s.is_empty()
}

/// Words ending -zast.
pub(crate) fn n11104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10974);
	s.is_empty()
}

/// Words ending -bst.
pub(crate) fn n3096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3091);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 10975);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10976);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10977);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22089);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37919);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 37920);
									s.is_empty()
								}
								['h', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 37921);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 37922);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 37923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -est.
pub(crate) fn n3097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3092);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10978);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 22090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10979);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22091);
					match s {
						[] => true,
						['p', 'z', s @ ..] => {
							super::push(out, 31522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10980);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10981);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 22093);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'b', 'a', 'g', 'i', 'l', 'i', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 37924);
									s.is_empty()
								}
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22094);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							super::push(out, 31523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', s @ ..] => {
					super::push(out, 22095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22096);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10983);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 22098);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 22099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10984);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22100);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10986);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 22102);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'e', 'w', s @ ..] => {
							super::push(out, 31524);
							s.is_empty()
						}
						['s', 'ü', s @ ..] => {
							super::push(out, 31525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 22103);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22104);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 22105);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 31527);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 31528);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 31529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22106);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', s @ ..] => {
									match s {
										[] => true,
										['o', 'n', 'k', '\u{1}', s @ ..] => {
											super::push(out, 41753);
											s.is_empty()
										}
										['s', 'a', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 41754);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'p', 'u', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 37926);
									s.is_empty()
								}
								['t', 'k', 'e', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 37927);
									s.is_empty()
								}
								['z', 'n', 'a', 'l', 'f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 37928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22107);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 37929);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 37930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', s @ ..] => {
					super::push(out, 22108);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22109);
					match s {
						[] => true,
						['t', 'e', 'l', 'e', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 31530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10987);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 22110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t15::n11118(s, out),
		['u', s @ ..] => {
			super::push(out, 10989);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 22129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 10990);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 22130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10991);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 22131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gst.
pub(crate) fn n3098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3093);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 10992);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22132);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 31571);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31572);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 41777);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 41778);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 31573);
							match s {
								[] => true,
								['g', 'n', 'u', 't', s @ ..] => {
									super::push(out, 37962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'l', s @ ..] => {
							super::push(out, 31574);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 31575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 22133);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 31576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hst.
pub(crate) fn n3099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3094);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10993);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ist.
pub(crate) fn n3100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3095);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10994);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10995);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 10996);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 10997);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10998);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 22134);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 22135);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22136);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 31577);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 31578);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 31579);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 31580);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 31581);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31582);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 31583);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31584);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37963);
									match s {
										[] => true,
										['h', 'e', s @ ..] => {
											super::push(out, 41779);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 41780);
											match s {
												[] => true,
												['l', 'i', s @ ..] => {
													super::push(out, 43699);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'e', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 41781);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 41782);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43700);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 37964);
									s.is_empty()
								}
								['ö', s @ ..] => {
									super::push(out, 37965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 31585);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 41783);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 41784);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41785);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41786);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 41787);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['f', s @ ..] => super::t15::n47617(s, out),
														['t', s @ ..] => super::t15::n47618(s, out),
														_ => false,
													}
												}
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
							super::push(out, 31586);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37966);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 37967);
									s.is_empty()
								}
								['t', 'r', s @ ..] => {
									super::push(out, 37968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31587);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 37969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'l', s @ ..] => {
							super::push(out, 31588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 22137);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 31589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 22138);
					match s {
						[] => true,
						['k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10999);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22139);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22140);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 31591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11000);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11001);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22141);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22142);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 22143);
					match s {
						[] => true,
						['o', 's', 'o', s @ ..] => {
							super::push(out, 31593);
							match s {
								[] => true,
								['n', 'm', 'y', 'g', '\u{1}', s @ ..] => {
									super::push(out, 37970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 22144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 11002);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22145);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 22146);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22147);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 31595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22148);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22149);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22150);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 22151);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31597);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22152);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 22153);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 31599);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => super::t15::n22816(s, out),
				['h', s @ ..] => {
					super::push(out, 22155);
					s.is_empty()
				}
				['i', s @ ..] => super::t15::n22818(s, out),
				['k', s @ ..] => super::t15::n22819(s, out),
				['l', s @ ..] => super::t15::n22820(s, out),
				['n', s @ ..] => super::t15::n22821(s, out),
				['o', s @ ..] => super::t15::n22822(s, out),
				['r', s @ ..] => super::t15::n22823(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t15::n11135(s, out),
		['o', s @ ..] => super::t15::n11136(s, out),
		['p', s @ ..] => super::t15::n11137(s, out),
		['r', s @ ..] => super::t15::n11138(s, out),
		['s', s @ ..] => super::t15::n11139(s, out),
		['t', s @ ..] => super::t15::n11140(s, out),
		['u', s @ ..] => super::t15::n11141(s, out),
		['v', s @ ..] => super::t15::n11142(s, out),
		['w', s @ ..] => super::t15::n11143(s, out),
		['y', s @ ..] => super::t15::n11144(s, out),
		['z', s @ ..] => super::t15::n11145(s, out),
		_ => false,
	}
}

/// Words ending -lst.
pub(crate) fn n3101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3096);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 11016);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 22187);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31626);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 31627);
							match s {
								[] => true,
								['c', 's', 'e', s @ ..] => {
									super::push(out, 37981);
									match s {
										[] => true,
										['g', 'n', s @ ..] => {
											super::push(out, 41788);
											match s {
												[] => true,
												['i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 43701);
													s.is_empty()
												}
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
							super::push(out, 31628);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 31629);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31630);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nst.
pub(crate) fn n3102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3097);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11017);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 22188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31632);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11018);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 22189);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 37982);
									match s {
										[] => true,
										['c', 'u', s @ ..] => {
											super::push(out, 41789);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 37983);
									match s {
										[] => true,
										['a', 'm', s @ ..] => {
											super::push(out, 41790);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 37984);
									match s {
										[] => true,
										['i', 'e', 'h', 'e', 'g', 'e', s @ ..] => {
											super::push(out, 41791);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37985);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', 'n', 'u', 'k', '\u{1}', s @ ..] => {
													super::push(out, 43702);
													s.is_empty()
												}
												['f', s @ ..] => {
													super::push(out, 43703);
													match s {
														[] => true,
														['i', s @ ..] => super::t15::n47619(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 43704);
													match s {
														[] => true,
														['n', 'i', s @ ..] => super::t15::n47620(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 43705);
													match s {
														[] => true,
														['f', s @ ..] => super::t15::n47621(s, out),
														['h', 'c', 'i', 'r', 'h', 'c', 'a', 'n', s @ ..] => super::t15::n47622(s, out),
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
									super::push(out, 37986);
									match s {
										[] => true,
										['e', 'v', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 43706);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 43707);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', 'ä', s @ ..] => {
											super::push(out, 41792);
											match s {
												[] => true,
												['f', '\u{1}', s @ ..] => {
													super::push(out, 43708);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 37987);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41793);
											s.is_empty()
										}
										['g', 'n', 'u', 'n', s @ ..] => {
											super::push(out, 41794);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 41795);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 41796);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41797);
											match s {
												[] => true,
												['i', 'e', 'b', s @ ..] => {
													super::push(out, 43709);
													match s {
														[] => true,
														['r', 'a', 's', s @ ..] => super::t15::n47623(s, out),
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
									match s {
										[] => true,
										['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 41798);
											s.is_empty()
										}
										['ä', s @ ..] => {
											super::push(out, 41799);
											match s {
												[] => true,
												['p', 's', '\u{1}', s @ ..] => {
													super::push(out, 43710);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22190);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 31634);
							match s {
								[] => true,
								['e', 'g', 'n', 'e', 'k', 'c', 'o', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 37988);
									s.is_empty()
								}
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
			super::push(out, 11019);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 22191);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 22192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11020);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11021);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t15::n11152(s, out),
		_ => false,
	}
}

/// Words ending -ost.
pub(crate) fn n3103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3098);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11023);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22198);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 22199);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 22200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 11025);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11026);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 22202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 11027);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11028);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11029);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11030);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 22203);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22204);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 31650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 22205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11031);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 22206);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11032);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22208);
					match s {
						[] => true,
						['r', 't', 'n', s @ ..] => {
							super::push(out, 31651);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 22209);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22210);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31652);
							s.is_empty()
						}
						['e', 'm', s @ ..] => {
							super::push(out, 31653);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31654);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 31655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 22211);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22212);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 38008);
									s.is_empty()
								}
								['h', 'c', 's', s @ ..] => {
									super::push(out, 38009);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 38010);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 38011);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 41808);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41809);
											s.is_empty()
										}
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
					super::push(out, 22213);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 22214);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 38012);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 38013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'o', 'r', 't', s @ ..] => {
							super::push(out, 31657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31658);
							s.is_empty()
						}
						['e', 'd', 'n', 'a', s @ ..] => {
							super::push(out, 31659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31660);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 31661);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 31662);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 31663);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11033);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22215);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 22216);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 22217);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 31665);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31666);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38014);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 22218);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22219);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 22220);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 22221);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22222);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 22223);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 22224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 11034);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pst.
pub(crate) fn n3104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3099);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11035);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 31668);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rst.
pub(crate) fn n3105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11037);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11038);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 22226);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 22227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11039);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 22228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11040);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 22229);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 22230);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 22231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11041);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 22232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 22233);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 22234);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 31672);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 11042);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 22235);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sst.
pub(crate) fn n3106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3101);
	s.is_empty()
}

/// Words ending -ust.
pub(crate) fn n3107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3102);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11043);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22236);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 22237);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 22238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11045);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31674);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 31675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11046);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22240);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 22241);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 22242);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22243);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 22244);
					match s {
						[] => true,
						['u', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 31676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 22245);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22246);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31677);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 31678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22247);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 31679);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 31680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22248);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38017);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 41810);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 38018);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41811);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 38019);
									match s {
										[] => true,
										['l', 'a', s @ ..] => {
											super::push(out, 41812);
											match s {
												[] => true,
												['t', 'o', 't', '\u{1}', s @ ..] => {
													super::push(out, 43711);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'h', s @ ..] => {
											super::push(out, 41813);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'u', 'n', s @ ..] => {
													super::push(out, 43712);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 43713);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'f', 'a', 'r', 'k', 'f', s @ ..] => {
											super::push(out, 41814);
											s.is_empty()
										}
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
					super::push(out, 22249);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31681);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 38020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 31682);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38021);
									s.is_empty()
								}
								['o', 'i', 't', s @ ..] => {
									super::push(out, 38022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 22250);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 22251);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 22252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11047);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 22253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11048);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 22254);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 31683);
							s.is_empty()
						}
						['n', 'e', 'h', s @ ..] => {
							super::push(out, 31684);
							match s {
								[] => true,
								['c', 'n', 'h', 'ü', s @ ..] => {
									super::push(out, 38023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 22255);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yst.
pub(crate) fn n3108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3103);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 11050);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11051);
			s.is_empty()
		}
		['m', 'o', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 11052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äst.
pub(crate) fn n3109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3104);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11053);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üst.
pub(crate) fn n3110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3105);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 11055);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'k', s @ ..] => {
							super::push(out, 31686);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['o', 'l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 41815);
											s.is_empty()
										}
										['ü', s @ ..] => {
											super::push(out, 41816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 31687);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38024);
									s.is_empty()
								}
								['t', 't', 'e', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nanenblatt.
pub(crate) fn n46642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43714);
	s.is_empty()
}

/// The word platanenblatt.
pub(crate) fn n46643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43715);
	s.is_empty()
}

/// The word guavenblatt.
pub(crate) fn n46644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43716);
	s.is_empty()
}

/// Words ending -ovenblatt.
pub(crate) fn n46645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43717);
	s.is_empty()
}

/// Words ending -ratt.
pub(crate) fn n11191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11061);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22267);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 22268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -satt.
pub(crate) fn n11192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11062);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 22269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tatt.
pub(crate) fn n11193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11063);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 22270);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 31699);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 31700);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 38038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -watt.
pub(crate) fn n11194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11064);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22271);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 22272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ett.
pub(crate) fn n3112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3107);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11065);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22273);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22274);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22275);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							super::push(out, 31701);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 38039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22276);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22277);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 31703);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 38040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22278);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 22279);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11066);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 11067);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22281);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 22282);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 22283);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 22284);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38041);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 38042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 22285);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 22286);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 38043);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38044);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 38045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 31707);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 31708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 31709);
							s.is_empty()
						}
						['m', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 31710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22287);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31711);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 38046);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38047);
									match s {
										[] => true,
										['e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 41826);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 38048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 31712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22288);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 31713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 22289);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 22290);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 11068);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11069);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 22291);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 22292);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 38049);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 38050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 31715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 22293);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 31716);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31717);
							match s {
								[] => true,
								['b', 'e', 'l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31718);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 41827);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 41828);
											s.is_empty()
										}
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
					super::push(out, 22294);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38052);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 41829);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 38053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22295);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 38054);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 41830);
											s.is_empty()
										}
										['r', 'o', 'h', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41831);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'g', 'h', 'c', 's', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 41832);
											s.is_empty()
										}
										['ä', 'r', 'g', 'h', 'c', 's', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 41833);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 22296);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 22297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t15::n11200(s, out),
		['m', s @ ..] => {
			super::push(out, 11071);
			s.is_empty()
		}
		['n', s @ ..] => super::t15::n11202(s, out),
		['r', s @ ..] => super::t15::n11203(s, out),
		['s', s @ ..] => {
			super::push(out, 11074);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 22315);
					match s {
						[] => true,
						['m', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 31746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 22316);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => super::t15::n33291(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t15::n22988(s, out),
				['r', 'o', 'k', '\u{1}', s @ ..] => super::t15::n22989(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t15::n11205(s, out),
		['u', s @ ..] => super::t15::n11206(s, out),
		['z', s @ ..] => super::t15::n11207(s, out),
		_ => false,
	}
}

/// Words ending -itt.
pub(crate) fn n3113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3108);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11078);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11079);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11080);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22324);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 22325);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 22326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11081);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 22327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11082);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 31748);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 41849);
											match s {
												[] => true,
												['e', 'p', s @ ..] => {
													super::push(out, 43721);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'n', s @ ..] => {
											super::push(out, 41850);
											match s {
												[] => true,
												['e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
													super::push(out, 43722);
													s.is_empty()
												}
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
							super::push(out, 31749);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 38077);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 38078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 31750);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'h', s @ ..] => {
									super::push(out, 38079);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 38080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 31751);
							match s {
								[] => true,
								['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 38081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 31752);
							match s {
								[] => true,
								['a', 'a', 'h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41851);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 41852);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['s', 's', s @ ..] => {
											super::push(out, 41853);
											s.is_empty()
										}
										['u', 'q', 'g', 'n', 'a', 'f', 'n', 'i', 'e', 'n', s @ ..] => {
											super::push(out, 41854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'i', s @ ..] => {
							super::push(out, 31753);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							super::push(out, 31754);
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 38083);
									s.is_empty()
								}
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
			super::push(out, 11083);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 22328);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 38084);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 38085);
									match s {
										[] => true,
										['c', 'i', s @ ..] => {
											super::push(out, 41855);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 38086);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 38087);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 41856);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 38088);
									match s {
										[] => true,
										['g', 'n', 'u', 't', s @ ..] => {
											super::push(out, 41857);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 41858);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 31755);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['a', 'z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 41859);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 41860);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 31756);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38089);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 38090);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 41861);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 41862);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 41863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 31757);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 31758);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38091);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 38092);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41864);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ott.
pub(crate) fn n3114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3109);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11084);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 11085);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22329);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 31760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11086);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22331);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 31761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 22332);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 22333);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22334);
					match s {
						[] => true,
						['l', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 38093);
									s.is_empty()
								}
								['ß', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 38094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'r', s @ ..] => {
					super::push(out, 22335);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22336);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'e', s @ ..] => {
									super::push(out, 38095);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 38096);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									super::push(out, 38097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 22337);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 31762);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 31763);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22338);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 22339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 11087);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22340);
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 31764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 11088);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 22341);
					match s {
						[] => true,
						['o', 'b', 's', 'l', 'e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 31765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11089);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 22342);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 22343);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 22344);
					match s {
						[] => true,
						['m', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38099);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 38100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 11090);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11091);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22345);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22346);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 22347);
					match s {
						[] => true,
						['o', 'k', 's', s @ ..] => {
							super::push(out, 31766);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 22348);
					match s {
						[] => true,
						['e', 'l', 'h', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 31767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11092);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22350);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 22351);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 38102);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 38103);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 38104);
									match s {
										[] => true,
										['r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
											super::push(out, 41866);
											s.is_empty()
										}
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
					super::push(out, 22352);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 22353);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 31768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utt.
pub(crate) fn n3115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3110);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11094);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 11096);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11097);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11098);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 22356);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 22357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11099);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ött.
pub(crate) fn n3116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3111);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 11101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ütt.
pub(crate) fn n3117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3112);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11102);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 11103);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kraut.
pub(crate) fn n23046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22373);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31783);
			s.is_empty()
		}
		['d', 'n', s @ ..] => {
			super::push(out, 31784);
			s.is_empty()
		}
		['g', 'i', s @ ..] => {
			super::push(out, 31785);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 31786);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 31787);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 41869);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 41870);
							match s {
								[] => true,
								['n', 'ü', s @ ..] => {
									super::push(out, 43723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 41871);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 43724);
									s.is_empty()
								}
								['r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 43725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 41872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 38111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 31788);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', 'r', '\u{1}', s @ ..] => {
							super::push(out, 41873);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 41874);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 41875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 38112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31789);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 41877);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 41878);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 41879);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41880);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 41881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 31790);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 38113);
					s.is_empty()
				}
				['f', 'o', s @ ..] => {
					super::push(out, 38114);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 38115);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 38116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 31791);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 31792);
			s.is_empty()
		}
		['z', 'u', s @ ..] => {
			super::push(out, 31793);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 31794);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nattribut.
pub(crate) fn n33346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31795);
	match s {
		[] => true,
		['e', 'r', 'r', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 38117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbut.
pub(crate) fn n11241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11111);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 22375);
			match s {
				[] => true,
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 31796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eut.
pub(crate) fn n3121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3116);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 11112);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fut.
pub(crate) fn n3122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3117);
	s.is_empty()
}

/// Words ending -gut.
pub(crate) fn n3123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3118);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 11115);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22378);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11116);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11117);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 22380);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11118);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11119);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22381);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 31798);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 31799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 22382);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 31800);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 38118);
									s.is_empty()
								}
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
			super::push(out, 11120);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 22383);
					s.is_empty()
				}
				['h', 'a', 'f', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 22384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11121);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 22385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11122);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22386);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 22387);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38119);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 38120);
									match s {
										[] => true,
										['n', 'u', 't', '\u{1}', s @ ..] => {
											super::push(out, 41882);
											s.is_empty()
										}
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
					super::push(out, 22388);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 38121);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 38122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 22389);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					super::push(out, 22390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11123);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 11124);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hut.
pub(crate) fn n3124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3119);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11125);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 22391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 11126);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 11127);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22393);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', s @ ..] => {
			super::push(out, 11128);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11129);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11130);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22395);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22396);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 31802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kut.
pub(crate) fn n3125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22397);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

