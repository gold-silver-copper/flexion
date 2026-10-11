use flexion::Path;

/// Words ending -del.
pub(crate) fn n1929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1927);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16418);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 16419);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 16420);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 16421);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16422);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16423);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26479);
							match s {
								[] => true,
								['l', 'e', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 34500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16424);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16425);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16426);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7164);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16427);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 26481);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16428);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16429);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 26483);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26484);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7165);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16430);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 26486);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 16431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7166);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16432);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26488);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34501);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 34502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 16433);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7167);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16435);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26490);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 26491);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26492);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34503);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 34504);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39797);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'u', s @ ..] => {
									super::push(out, 34505);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 34506);
									match s {
										[] => true,
										['c', 's', 'u', s @ ..] => {
											super::push(out, 39798);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 't', s @ ..] => {
									super::push(out, 34507);
									match s {
										[] => true,
										['t', 'i', 'm', 's', 'n', s @ ..] => {
											super::push(out, 39799);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 39800);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 39801);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 42698);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 39802);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 42699);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'i', s @ ..] => {
									super::push(out, 34508);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 34509);
									s.is_empty()
								}
								['t', 'l', s @ ..] => {
									super::push(out, 34510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 26493);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 26494);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26495);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 34512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39803);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 26496);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34513);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 39804);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 39805);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t13::n16839(s, out),
				['i', s @ ..] => super::t13::n16840(s, out),
				['o', s @ ..] => super::t13::n16841(s, out),
				['u', s @ ..] => super::t13::n16842(s, out),
				['ä', s @ ..] => super::t13::n16843(s, out),
				['ü', s @ ..] => super::t13::n16844(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t13::n7249(s, out),
		['r', s @ ..] => super::t13::n7250(s, out),
		['u', s @ ..] => super::t13::n7251(s, out),
		['ä', s @ ..] => super::t13::n7252(s, out),
		['ö', s @ ..] => super::t13::n7253(s, out),
		['ü', s @ ..] => super::t13::n7254(s, out),
		_ => false,
	}
}

/// Words ending -eel.
pub(crate) fn n1930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1928);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7174);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7175);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26526);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 26527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fel.
pub(crate) fn n1931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7176);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16457);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16458);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 16459);
					match s {
						[] => true,
						['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 26528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7177);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16460);
					match s {
						[] => true,
						['t', 's', 'n', s @ ..] => {
							super::push(out, 26529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 16461);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7178);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16462);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26531);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 34527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26533);
							match s {
								[] => true,
								['s', 'r', 'ü', s @ ..] => {
									super::push(out, 34529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 26534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16463);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26535);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 26536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16464);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34530);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 34531);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39807);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 39808);
											match s {
												[] => true,
												['m', 'u', 'a', 'l', 'f', 'p', '\u{1}', s @ ..] => {
													super::push(out, 42702);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 34532);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 34533);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16465);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34534);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 34535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 16466);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 26537);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26538);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 16467);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26539);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26541);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39810);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 39811);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7179);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16468);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 26542);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 26543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7180);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16470);
					match s {
						[] => true,
						['n', 'r', s @ ..] => {
							super::push(out, 26544);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 26545);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 34539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16471);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26546);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 16472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7182);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7183);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16473);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16474);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 16475);
					s.is_empty()
				}
				['ä', 'l', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 16476);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gel.
pub(crate) fn n1932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7184);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7185);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16477);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16479);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 26549);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 26550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16480);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 16481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7186);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16482);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16483);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 26552);
							match s {
								[] => true,
								['s', 'n', 'i', s @ ..] => {
									super::push(out, 34541);
									match s {
										[] => true,
										['n', 'a', s @ ..] => {
											super::push(out, 39812);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26553);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26554);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 26555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 16484);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16485);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16486);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 16487);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16488);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26556);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 34542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 26557);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 34543);
									match s {
										[] => true,
										['e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 39813);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26558);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 34544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16489);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 16490);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 26560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7187);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 16491);
					match s {
						[] => true,
						['m', 'h', s @ ..] => {
							super::push(out, 26561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7188);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16492);
					match s {
						[] => true,
						['s', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 26562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7189);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7190);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26564);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 26565);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34545);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											super::push(out, 39814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t13::n27648(s, out),
						['n', s @ ..] => {
							super::push(out, 26567);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26568);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34555);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 34556);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39824);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 34557);
									match s {
										[] => true,
										['e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 39825);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ü', s @ ..] => {
									super::push(out, 34558);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											super::push(out, 39826);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t13::n27651(s, out),
						['t', s @ ..] => super::t13::n27652(s, out),
						['ß', s @ ..] => super::t13::n27653(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16496);
					s.is_empty()
				}
				['i', s @ ..] => super::t13::n16902(s, out),
				['u', s @ ..] => super::t13::n16903(s, out),
				['ä', s @ ..] => super::t13::n16904(s, out),
				['ü', s @ ..] => super::t13::n16905(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t13::n7272(s, out),
		['r', s @ ..] => super::t13::n7273(s, out),
		['t', s @ ..] => super::t13::n7274(s, out),
		['u', s @ ..] => super::t13::n7275(s, out),
		['ö', s @ ..] => super::t13::n7276(s, out),
		['ü', s @ ..] => super::t13::n7277(s, out),
		_ => false,
	}
}

/// Words ending -hel.
pub(crate) fn n1933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1931);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7197);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7198);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26600);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 26601);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26602);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16521);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16522);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26604);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34571);
									match s {
										[] => true,
										['r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 39828);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 34572);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 39829);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 34573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 26605);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26606);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26607);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 34575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16523);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16524);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34576);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 34577);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 39830);
											s.is_empty()
										}
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
					super::push(out, 16525);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26609);
							match s {
								[] => true,
								['w', s @ ..] => {
									match s {
										[] => true,
										['e', 'd', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39831);
											s.is_empty()
										}
										['r', 'h', 'o', '\u{1}', s @ ..] => {
											super::push(out, 39832);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 26610);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34578);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 39833);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 34579);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 39834);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 34580);
									match s {
										[] => true,
										['s', 's', s @ ..] => {
											super::push(out, 39835);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 42708);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 34581);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 39836);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 34582);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 39837);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 26611);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 26612);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 34583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 16526);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 16527);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 16528);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7199);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7200);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 16530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7202);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16532);
					match s {
						[] => true,
						['p', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34584);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 34585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 7203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iel.
pub(crate) fn n1934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1932);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7204);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7205);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7206);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26615);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 26616);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 39838);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42709);
													s.is_empty()
												}
												['n', 'e', 'm', 'a', 'd', '\u{1}', s @ ..] => {
													super::push(out, 42710);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 39839);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 42711);
													s.is_empty()
												}
												['w', '\u{1}', s @ ..] => {
													super::push(out, 42712);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'm', s @ ..] => {
							super::push(out, 26617);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 34586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 26618);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 26619);
							match s {
								[] => true,
								['a', 'l', 'h', 'c', 's', 'k', 'c', 'ü', 'r', '\u{1}', s @ ..] => {
									super::push(out, 34588);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 34589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'r', s @ ..] => {
							super::push(out, 26620);
							match s {
								[] => true,
								['i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 34590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 26621);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => super::t13::n42300(s, out),
										['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 39841);
											s.is_empty()
										}
										['r', 'ö', 'h', '\u{1}', s @ ..] => super::t13::n42302(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'n', 'i', s @ ..] => super::t13::n27706(s, out),
						['l', s @ ..] => super::t13::n27707(s, out),
						['m', s @ ..] => super::t13::n27708(s, out),
						['n', 'e', s @ ..] => super::t13::n27709(s, out),
						['r', s @ ..] => super::t13::n27710(s, out),
						['s', s @ ..] => super::t13::n27711(s, out),
						['t', s @ ..] => super::t13::n27712(s, out),
						['y', s @ ..] => super::t13::n27713(s, out),
						['ß', s @ ..] => super::t13::n27714(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t13::n7288(s, out),
		['s', s @ ..] => super::t13::n7289(s, out),
		['t', s @ ..] => super::t13::n7290(s, out),
		['v', s @ ..] => super::t13::n7291(s, out),
		['w', s @ ..] => super::t13::n7292(s, out),
		['z', s @ ..] => super::t13::n7293(s, out),
		_ => false,
	}
}

/// Words ending -kel.
pub(crate) fn n1935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7213);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16543);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16544);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16545);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26637);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26638);
							match s {
								[] => true,
								['e', 'b', 'a', s @ ..] => {
									super::push(out, 34620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26639);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26640);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 26641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16547);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26642);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7214);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16548);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26644);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 26645);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26646);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 34622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', 'r', 'u', 'p', 'r', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 26647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16549);
					match s {
						[] => true,
						['d', 'm', s @ ..] => {
							super::push(out, 26648);
							match s {
								[] => true,
								['u', 'a', 'r', 'r', 'e', 'f', 'f', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 34623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16550);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26649);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39861);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 42726);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 42727);
													s.is_empty()
												}
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
							super::push(out, 26650);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 26651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16551);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16552);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 16553);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 16554);
					match s {
						[] => true,
						['t', 's', 'b', s @ ..] => {
							super::push(out, 26653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7215);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16555);
					s.is_empty()
				}
				['l', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 16556);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16557);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16558);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26654);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7216);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16559);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16560);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 16561);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 26658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16562);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16563);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16564);
					match s {
						[] => true,
						['r', s @ ..] => super::t13::n27748(s, out),
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 16565);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t13::n7298(s, out),
		['o', s @ ..] => {
			super::push(out, 7218);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t13::n7300(s, out),
		['s', s @ ..] => super::t13::n7301(s, out),
		['u', s @ ..] => {
			super::push(out, 7221);
			match s {
				[] => true,
				['a', s @ ..] => super::t13::n16982(s, out),
				_ => false,
			}
		}
		['ä', s @ ..] => super::t13::n7303(s, out),
		_ => false,
	}
}

/// Words ending -lel.
pub(crate) fn n1936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1934);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7223);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 16578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mel.
pub(crate) fn n1937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1935);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7224);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 16579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7225);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7226);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7227);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16580);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26681);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26682);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 34636);
									match s {
										[] => true,
										['i', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 39874);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26683);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 34637);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 34638);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16581);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 26685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16582);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26686);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34639);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 26687);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26688);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 39876);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 39877);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 39878);
											s.is_empty()
										}
										['r', 'e', 't', 's', 'd', s @ ..] => {
											super::push(out, 39879);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', 'm', s @ ..] => {
											super::push(out, 39880);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 39881);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 34641);
									match s {
										[] => true,
										['g', 'a', 't', 't', 'i', 'm', 'h', s @ ..] => {
											super::push(out, 39882);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'h', s @ ..] => {
									super::push(out, 34642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 26689);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 26690);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34643);
									match s {
										[] => true,
										['g', 'n', s @ ..] => {
											super::push(out, 39883);
											s.is_empty()
										}
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
					super::push(out, 16583);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26691);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16584);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26693);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26694);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26695);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26696);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 16585);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26698);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26699);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26700);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34645);
									s.is_empty()
								}
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
			super::push(out, 7228);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 16586);
					match s {
						[] => true,
						['f', '-', s @ ..] => {
							super::push(out, 26701);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16587);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34647);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34648);
									s.is_empty()
								}
								['s', 'a', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 34649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 16588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7229);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 7230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nel.
pub(crate) fn n1938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1936);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7231);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 16589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7232);
			match s {
				[] => true,
				['u', 't', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26702);
							s.is_empty()
						}
						['l', 'a', 'n', s @ ..] => {
							super::push(out, 26703);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 26704);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 34650);
									s.is_empty()
								}
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
			super::push(out, 7233);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16590);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16591);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7234);
			match s {
				[] => true,
				['e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 16593);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oel.
pub(crate) fn n1939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1937);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7235);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 16594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pel.
pub(crate) fn n1940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1938);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7236);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16595);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 26705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7237);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16597);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16598);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16599);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16600);
					match s {
						[] => true,
						['s', 'r', 'a', 'l', 'o', 'i', 't', 'e', 'p', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 26706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7238);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7239);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16601);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26707);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 26708);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26709);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34651);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 34652);
									match s {
										[] => true,
										['n', 'r', 'e', 'u', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39884);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26710);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 34653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 26711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16602);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26712);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34654);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 34655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39885);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26713);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 34656);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 39886);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 34657);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'n', s @ ..] => {
									super::push(out, 34658);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 39888);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 34659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 26714);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16603);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 26715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 16604);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16605);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26716);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 26717);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26718);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 34661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 16606);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26719);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7241);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26721);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26722);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 34663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 26723);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26724);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 34664);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 26725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16609);
					match s {
						[] => true,
						['n', 'h', s @ ..] => {
							super::push(out, 26726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16610);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26727);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26728);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26729);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 26730);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16611);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 16612);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 16613);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26733);
							s.is_empty()
						}
						['r', 'k', 's', s @ ..] => {
							super::push(out, 26734);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34666);
									s.is_empty()
								}
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
			super::push(out, 7242);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7243);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16614);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26735);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26736);
							match s {
								[] => true,
								['s', 'o', s @ ..] => {
									super::push(out, 34667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16615);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 26737);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 26738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16616);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 26739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7244);
			match s {
				[] => true,
				['a', s @ ..] => super::t13::n17023(s, out),
				['r', s @ ..] => super::t13::n17024(s, out),
				['t', s @ ..] => super::t13::n17025(s, out),
				_ => false,
			}
		}
		['ä', s @ ..] => super::t13::n7326(s, out),
		['ö', s @ ..] => super::t13::n7327(s, out),
		_ => false,
	}
}

/// Words ending -rel.
pub(crate) fn n1941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1939);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7247);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7248);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sel.
pub(crate) fn n1942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1940);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7250);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 16620);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26742);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 26743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 16623);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 7251);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16625);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34669);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 34670);
									s.is_empty()
								}
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
			super::push(out, 7252);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16626);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26744);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 26745);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26746);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 34672);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26747);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 34673);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 34674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 26748);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 34675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 26749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7253);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7254);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26750);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26751);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34676);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39889);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 34677);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 39890);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 42736);
													s.is_empty()
												}
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
							super::push(out, 26752);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7255);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16630);
					match s {
						[] => true,
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 26753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26754);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 26755);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26756);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 26757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34679);
									s.is_empty()
								}
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
			super::push(out, 7256);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7257);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7258);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7259);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16633);
					match s {
						[] => true,
						['h', 's', 's', 'e', 'z', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 26758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16634);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26759);
							match s {
								[] => true,
								['l', 'a', 'h', 'n', s @ ..] => {
									super::push(out, 34680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 26760);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 26761);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26762);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16635);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 16636);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 26765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7260);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 26766);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16637);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 16638);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 16639);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 26769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7262);
			match s {
				[] => true,
				['a', s @ ..] => super::t13::n17049(s, out),
				['e', s @ ..] => super::t13::n17050(s, out),
				['i', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 16642);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 16643);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 26777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t13::n17053(s, out),
				['ö', s @ ..] => super::t13::n17054(s, out),
				['ü', s @ ..] => super::t13::n17055(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t13::n7344(s, out),
		['u', s @ ..] => super::t13::n7345(s, out),
		['ö', s @ ..] => {
			super::push(out, 7265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tel.
pub(crate) fn n1943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1941);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7266);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16655);
					match s {
						[] => true,
						['s', 's', s @ ..] => {
							super::push(out, 26790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16656);
					match s {
						[] => true,
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 26791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 7267);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7268);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16657);
					s.is_empty()
				}
				['r', 'g', 'n', 'e', 't', 't', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 16658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7269);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 16659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7270);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34690);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 34691);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 34692);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 39894);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 26793);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 34693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 26794);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 34694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 26795);
							match s {
								[] => true,
								['f', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 34695);
									s.is_empty()
								}
								['w', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 34696);
									s.is_empty()
								}
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
			super::push(out, 7271);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16660);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16661);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 16662);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7272);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7273);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26797);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 26798);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26799);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 26800);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34698);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 34699);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 39895);
											match s {
												[] => true,
												['r', 'ö', 'b', '\u{1}', s @ ..] => {
													super::push(out, 42739);
													s.is_empty()
												}
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
							super::push(out, 26801);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16666);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26802);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 26803);
							match s {
								[] => true,
								['l', 'k', 't', s @ ..] => {
									super::push(out, 34701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16667);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16668);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16669);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7274);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7275);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7276);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26807);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 26808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34702);
									s.is_empty()
								}
								['u', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 34703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16672);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26809);
							match s {
								[] => true,
								['v', 's', 'e', s @ ..] => {
									super::push(out, 34704);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 39896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 26810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 16673);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 26811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 16674);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 16675);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 26812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 16676);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 16677);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26813);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 34705);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 39897);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', s @ ..] => {
									super::push(out, 34706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t13::n7358(s, out),
		['t', s @ ..] => super::t13::n7359(s, out),
		['u', s @ ..] => {
			super::push(out, 7279);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16696);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 26834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16697);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 16698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 7280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uel.
pub(crate) fn n1944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1942);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7281);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7282);
			match s {
				[] => true,
				['-', 'e', '\u{1}', s @ ..] => {
					super::push(out, 16699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7283);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7284);
			match s {
				[] => true,
				['r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 16700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 7285);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 7286);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16702);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 16703);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vel.
pub(crate) fn n1945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1943);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7287);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wel.
pub(crate) fn n1946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1944);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 7288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xel.
pub(crate) fn n1947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1945);
	s.is_empty()
}

/// Words ending -zel.
pub(crate) fn n1948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1946);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7289);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16706);
					match s {
						[] => true,
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 26836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7290);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16708);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 16709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7291);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16710);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26837);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 34721);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16711);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 26839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34722);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 26840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 16712);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 26841);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26842);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 26843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7292);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16713);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 26844);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16714);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 26846);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 26847);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 34723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 26848);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34724);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 34725);
									match s {
										[] => true,
										['c', 's', s @ ..] => {
											match s {
												[] => true,
												['r', 'e', 'i', s @ ..] => {
													super::push(out, 42747);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 42748);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['k', 's', '\u{1}', s @ ..] => {
											super::push(out, 39909);
											s.is_empty()
										}
										['t', 's', '\u{1}', s @ ..] => {
											super::push(out, 39910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 26849);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 34726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26850);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 34727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16715);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 26851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 16716);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 34728);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 34729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 16717);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 16718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7293);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 7294);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 16719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßel.
pub(crate) fn n1949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1947);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7295);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26852);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odenstrahl.
pub(crate) fn n45538(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 't', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 44099);
			s.is_empty()
		}
		['t', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 44100);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word röntgenstrahl.
pub(crate) fn n45539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42749);
	s.is_empty()
}

/// Words ending -afdiebstahl.
pub(crate) fn n45540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42750);
	s.is_empty()
}

/// Words ending -ierstahl.
pub(crate) fn n27963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26868);
	s.is_empty()
}

/// Words ending -wahl.
pub(crate) fn n7383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7302);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 26869);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 34738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'r', s @ ..] => {
					super::push(out, 26870);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 34739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'r', s @ ..] => {
			super::push(out, 16730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zahl.
pub(crate) fn n7384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7303);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 16731);
			s.is_empty()
		}
		['i', 'e', s @ ..] => {
			super::push(out, 16732);
			s.is_empty()
		}
		['l', 'e', s @ ..] => {
			super::push(out, 16733);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 26871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 26872);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 34740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 26873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16734);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 34741);
							s.is_empty()
						}
						['d', 'n', s @ ..] => {
							super::push(out, 34742);
							match s {
								[] => true,
								['i', 'm', s @ ..] => {
									super::push(out, 39918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', s @ ..] => {
							super::push(out, 34743);
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
				['i', s @ ..] => {
					super::push(out, 26874);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 34744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'i', 't', 'k', 'a', s @ ..] => {
					super::push(out, 26875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 16735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ehl.
pub(crate) fn n1954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1952);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7304);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16736);
					match s {
						[] => true,
						['b', 's', 'g', 'n', 'u', 'r', s @ ..] => {
							super::push(out, 26876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7305);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 7306);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7307);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16737);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 26877);
							s.is_empty()
						}
						['z', 'z', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 26878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16738);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26879);
							match s {
								[] => true,
								['ä', 's', '\u{1}', s @ ..] => {
									super::push(out, 34745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16739);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 16740);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16741);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 34746);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 34747);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 34748);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 26880);
							match s {
								[] => true,
								['k', 't', s @ ..] => {
									super::push(out, 34749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'i', s @ ..] => {
					super::push(out, 16742);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16743);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16744);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 16745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16746);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ihl.
pub(crate) fn n1955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1953);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7309);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ohl.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1954);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7310);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 16747);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 26881);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7312);
			match s {
				[] => true,
				['e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 16750);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16751);
					match s {
						[] => true,
						['i', 'e', 'm', 'e', 'g', 'l', s @ ..] => {
							super::push(out, 26883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -thl.
pub(crate) fn n1957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1955);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uhl.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1956);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7314);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 16752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7315);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26884);
							match s {
								[] => true,
								['g', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 34750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', 'e', s @ ..] => {
									super::push(out, 34751);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42751);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 42752);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', 'a', s @ ..] => {
									super::push(out, 34752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26885);
							match s {
								[] => true,
								['g', 'n', 'i', s @ ..] => {
									super::push(out, 34753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26886);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 34754);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 34755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ühl.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1957);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7316);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 16753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7317);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26887);
							s.is_empty()
						}
						['d', 'l', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 26888);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26889);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 34756);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 26890);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39920);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 39921);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 34757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 26891);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 26892);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26893);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 34758);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 34759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 26894);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26895);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 34760);
									match s {
										[] => true,
										['n', 'u', 't', s @ ..] => {
											super::push(out, 39922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 34761);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 39923);
											match s {
												[] => true,
												['a', 'h', 'c', 's', 'n', 'i', 'e', 'm', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 42753);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 39924);
											s.is_empty()
										}
										['i', 'e', 'k', 'g', 'i', s @ ..] => {
											match s {
												[] => true,
												['r', 'ö', 'h', 'e', 'g', 'n', s @ ..] => {
													super::push(out, 42754);
													s.is_empty()
												}
												['t', s @ ..] => {
													match s {
														[] => true,
														['r', 'e', 'w', 'r', 'e', 'd', 'n', 'i', 'm', '\u{1}', s @ ..] => super::t13::n47063(s, out),
														['s', s @ ..] => super::t13::n47064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 34762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 26896);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 34763);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 39925);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 39926);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 34764);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39927);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 34765);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 39928);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 39929);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 34766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 26897);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 26898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7318);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7319);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 7320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eil.
pub(crate) fn n1964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1962);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7330);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16767);
					match s {
						[] => true,
						['h', 'e', 'w', 'r', 'e', 'u', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 26905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'g', 'e', 'i', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 16768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7331);
			match s {
				[] => true,
				['p', 'd', 'g', s @ ..] => {
					super::push(out, 16769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7332);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7333);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16771);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16772);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26906);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 26907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16773);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 26908);
							match s {
								[] => true,
								['e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 34770);
									s.is_empty()
								}
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
			super::push(out, 7334);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7335);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7336);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7337);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 16775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16776);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 16777);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26909);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 16778);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26911);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26912);
							match s {
								[] => true,
								['a', 't', 's', 'e', 'b', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 39930);
											s.is_empty()
										}
										['s', 'g', 'n', 'u', 'r', 'e', s @ ..] => {
											super::push(out, 39931);
											s.is_empty()
										}
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
					super::push(out, 16779);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 26913);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 16780);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 26915);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 26916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 16781);
					s.is_empty()
				}
				['h', s @ ..] => super::t13::n17199(s, out),
				['k', s @ ..] => {
					super::push(out, 16783);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 26918);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 26919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t13::n17201(s, out),
				['m', s @ ..] => super::t13::n17202(s, out),
				['n', s @ ..] => super::t13::n17203(s, out),
				['r', s @ ..] => super::t13::n17204(s, out),
				['s', s @ ..] => super::t13::n17205(s, out),
				['t', s @ ..] => super::t13::n17206(s, out),
				['u', s @ ..] => super::t13::n17207(s, out),
				['z', s @ ..] => super::t13::n17208(s, out),
				['ß', s @ ..] => super::t13::n17209(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t13::n7420(s, out),
		_ => false,
	}
}

/// Words ending -entil.
pub(crate) fn n17222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16803);
	s.is_empty()
}

/// Words ending -intil.
pub(crate) fn n17223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16804);
	s.is_empty()
}

/// Words ending -ptil.
pub(crate) fn n7438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7357);
	match s {
		[] => true,
		['e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 16805);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stil.
pub(crate) fn n7439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16806);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 16807);
			match s {
				[] => true,
				['d', 'o', 'c', '-', 'e', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 26952);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 26953);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 26954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', 'i', s @ ..] => {
			super::push(out, 16808);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 16809);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 16810);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 16811);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 16812);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16813);
			s.is_empty()
		}
		['k', 'c', s @ ..] => {
			super::push(out, 16814);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 16815);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 26955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 16816);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 26956);
					match s {
						[] => true,
						['g', 'o', 'b', 'd', 'n', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 34810);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 34811);
							s.is_empty()
						}
						['t', 'i', 'u', 's', 'e', 'j', '\u{1}', s @ ..] => {
							super::push(out, 34812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 26957);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 26958);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 39964);
									s.is_empty()
								}
								['u', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 39965);
									s.is_empty()
								}
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
			super::push(out, 16817);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 26959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16818);
			match s {
				[] => true,
				['g', 'n', 'u', 't', s @ ..] => {
					super::push(out, 26960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 16819);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 26961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 16820);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 16821);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -util.
pub(crate) fn n7440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7359);
	s.is_empty()
}

/// Words ending -xtil.
pub(crate) fn n7441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7360);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 26962);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 26963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uil.
pub(crate) fn n1974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1972);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vil.
pub(crate) fn n1975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1973);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7362);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xil.
pub(crate) fn n1976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1974);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zil.
pub(crate) fn n1977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1975);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7364);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7365);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 16822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -all.
pub(crate) fn n1979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1977);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7367);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 7368);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16823);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 26964);
							match s {
								[] => true,
								['n', 'i', 'h', 'c', 'n', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 34813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 16824);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 26965);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 26966);
							s.is_empty()
						}
						['n', 'a', 'h', 'd', s @ ..] => {
							super::push(out, 26967);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 26968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 16825);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26969);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 26970);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 26971);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34814);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 34815);
									match s {
										[] => true,
										['e', 'r', 'p', 's', 'e', 'd', 'n', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 39966);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 26972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 16826);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16827);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 26973);
							match s {
								[] => true,
								['h', 'c', 'n', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 34816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 16828);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 26974);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 26975);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 34817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 26976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 16829);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 26977);
							s.is_empty()
						}
						['m', 'm', 'u', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34818);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 34819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 16830);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16831);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 16832);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16833);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 34820);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 34821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16834);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26978);
							match s {
								[] => true,
								['d', 'l', s @ ..] => {
									super::push(out, 34822);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 34823);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 34824);
									match s {
										[] => true,
										['l', 'ö', 'v', '\u{1}', s @ ..] => {
											super::push(out, 39967);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 34825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 26979);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 26980);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 34827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16835);
					match s {
						[] => true,
						['i', 'n', 'n', 'e', 't', 'h', 'c', 's', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 26981);
							s.is_empty()
						}
						['u', 'e', s @ ..] => {
							super::push(out, 26982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16836);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 26983);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 39968);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 39969);
											match s {
												[] => true,
												['a', 'b', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n47071(s, out),
														['d', s @ ..] => super::t13::n47072(s, out),
														['h', 'c', 'a', 'e', 'b', '\u{1}', s @ ..] => super::t13::n47073(s, out),
														['l', s @ ..] => super::t13::n47074(s, out),
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
									super::push(out, 34828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 26984);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 26985);
							match s {
								[] => true,
								['i', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 34829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => super::t13::n28090(s, out),
						['s', s @ ..] => super::t13::n28091(s, out),
						_ => false,
					}
				}
				['y', s @ ..] => super::t13::n17257(s, out),
				['z', s @ ..] => {
					super::push(out, 16838);
					s.is_empty()
				}
				['ß', s @ ..] => super::t13::n17259(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t13::n7450(s, out),
		['d', s @ ..] => super::t13::n7451(s, out),
		['f', s @ ..] => super::t13::n7452(s, out),
		['g', s @ ..] => super::t13::n7453(s, out),
		['h', s @ ..] => super::t13::n7454(s, out),
		['m', s @ ..] => super::t13::n7455(s, out),
		['n', s @ ..] => super::t13::n7456(s, out),
		['r', s @ ..] => super::t13::n7457(s, out),
		['s', s @ ..] => super::t13::n7458(s, out),
		['t', s @ ..] => super::t13::n7459(s, out),
		['v', s @ ..] => super::t13::n7460(s, out),
		['w', s @ ..] => super::t13::n7461(s, out),
		_ => false,
	}
}

/// The word kameramodell.
pub(crate) fn n45597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42801);
	s.is_empty()
}

/// Words ending -kriegsmodell.
pub(crate) fn n45598(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 44124);
			s.is_empty()
		}
		['r', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 44125);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rpell.
pub(crate) fn n17320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16899);
	match s {
		[] => true,
		['a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 27031);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rell.
pub(crate) fn n7471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7390);
	s.is_empty()
}

/// Words ending -sell.
pub(crate) fn n7472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7391);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 16900);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 27032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 16901);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34889);
							s.is_empty()
						}
						['t', 'l', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16902);
			match s {
				[] => true,
				['u', 'r', 'a', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34891);
							s.is_empty()
						}
						['n', 'e', 't', 't', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tell.
pub(crate) fn n7473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7392);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16903);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 16904);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 16905);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34893);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 34894);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 34895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 16906);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27033);
					s.is_empty()
				}
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 't', s @ ..] => {
							super::push(out, 34896);
							s.is_empty()
						}
						['n', 'e', 'l', 'l', 'i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 39999);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42802);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', s @ ..] => {
							super::push(out, 34897);
							match s {
								[] => true,
								['t', 'e', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 34898);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 40002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uell.
pub(crate) fn n7474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7393);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 16907);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 16908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27034);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 27035);
					s.is_empty()
				}
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 27036);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -well.
pub(crate) fn n7475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7394);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 16909);
			match s {
				[] => true,
				['i', 'e', 'b', '-', s @ ..] => {
					super::push(out, 27037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ill.
pub(crate) fn n1981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1979);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7395);
			match s {
				[] => true,
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 16910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7397);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16912);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27038);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 16913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7398);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7399);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27040);
							s.is_empty()
						}
						['g', 'n', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 27041);
							s.is_empty()
						}
						['r', 'e', 'k', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 27042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7400);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27043);
							s.is_empty()
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 27044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 16917);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27045);
							match s {
								[] => true,
								['e', 'x', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 34899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 16918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 16919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 7401);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7402);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 7403);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 16920);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 27047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oll.
pub(crate) fn n1982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1980);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7405);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 16921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7406);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 16922);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27048);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 34900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 27049);
							match s {
								[] => true,
								['o', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['s', 'i', 'n', 't', 'h', 'c', 'ä', 'd', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 40003);
											s.is_empty()
										}
										['t', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 40004);
											s.is_empty()
										}
										['v', 'i', 't', 'a', 't', 'l', 'u', 'k', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 40005);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7407);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7408);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 16924);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 27050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 16925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27052);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 27053);
							s.is_empty()
						}
						['t', 'n', 'e', 't', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 27054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7409);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16927);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 16928);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16929);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 16930);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 16931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 7411);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 16932);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 34901);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 34902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 7412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16933);
					s.is_empty()
				}
				['e', 'g', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 16934);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 16935);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 16936);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16937);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16938);
					s.is_empty()
				}
				['r', 'h', 'u', 'f', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 16939);
					s.is_empty()
				}
				['s', 'g', 'n', 'i', 'h', 'c', 's', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 16940);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16941);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27056);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27057);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 27058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 't', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 16942);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ull.
pub(crate) fn n1983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7413);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 7414);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7415);
			match s {
				[] => true,
				['s', 'd', 'n', 'a', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 16943);
					s.is_empty()
				}
				['t', 'k', 'c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 16944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16945);
					s.is_empty()
				}
				['-', 'l', 'l', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 16946);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yll.
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1982);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7417);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7418);
			match s {
				[] => true,
				['p', 'o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27059);
							match s {
								[] => true,
								['i', 'r', 'e', 't', 'p', '\u{1}', s @ ..] => {
									super::push(out, 34903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7419);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äll.
pub(crate) fn n1985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1983);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 7420);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öll.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1984);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 7421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üll.
pub(crate) fn n1987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1985);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7422);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16949);
					s.is_empty()
				}
				['f', 'f', 'o', 't', 's', 't', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 16950);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					super::push(out, 16951);
					match s {
						[] => true,
						['a', 'r', 't', 'l', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 27061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'r', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 16952);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'i', 't', 'a', 'm', 'r', 'o', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 27062);
							s.is_empty()
						}
						['u', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 27063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 16953);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 27064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16954);
					match s {
						[] => true,
						['e', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 27065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7423);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 16955);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 27066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 16956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gerl.
pub(crate) fn n7558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7476);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 16998);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 27080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 27081);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 27082);
					match s {
						[] => true,
						['r', 'i', 'm', 'm', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 34919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 27083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -herl.
pub(crate) fn n7559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7477);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 34920);
							s.is_empty()
						}
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 40015);
									s.is_empty()
								}
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 40016);
									s.is_empty()
								}
								['t', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 40017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 34922);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42803);
											s.is_empty()
										}
										['h', 'c', 's', 'o', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 42804);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 40018);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'e', 'f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40019);
									s.is_empty()
								}
								['n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 16999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kerl.
pub(crate) fn n7561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7479);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17001);
			s.is_empty()
		}
		['a', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27084);
					s.is_empty()
				}
				['-', 'u', '\u{1}', s @ ..] => {
					super::push(out, 27085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 17002);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 34924);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 34925);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 34926);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 34927);
							s.is_empty()
						}
						['n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 34928);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40021);
									s.is_empty()
								}
								['n', 'e', 't', 't', 'e', 'r', 'a', 'g', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 40022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40023);
									s.is_empty()
								}
								['k', 'i', 't', 's', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40024);
									s.is_empty()
								}
								['l', 'r', 'e', 'k', 'c', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 40025);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['h', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 42805);
											s.is_empty()
										}
										['r', 'u', 't', '\u{1}', s @ ..] => {
											super::push(out, 42806);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'i', 'p', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40026);
									s.is_empty()
								}
								['t', 's', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 40027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 27086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34929);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 34930);
							s.is_empty()
						}
						['l', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40028);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 40029);
									match s {
										[] => true,
										['e', 'k', 'n', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 42807);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', 'a', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 40030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 34931);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 34932);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 34933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34934);
							s.is_empty()
						}
						['-', 'z', 'f', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34935);
							s.is_empty()
						}
						['k', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 34937);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 34938);
							s.is_empty()
						}
						['n', 'r', 'e', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 34939);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 34940);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 34941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 34942);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40031);
									s.is_empty()
								}
								['e', 'd', 'a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 40032);
									s.is_empty()
								}
								['h', 'c', 's', 'i', 'b', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 40033);
									s.is_empty()
								}
								['n', 'e', 'd', 'a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 40034);
									s.is_empty()
								}
								['o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 40035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 27087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 17003);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 17004);
			s.is_empty()
		}
		['h', 'c', 's', 't', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 17005);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17006);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 17007);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 17008);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27088);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 27089);
					match s {
						[] => true,
						['t', 'u', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 34943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 17009);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 17010);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 17011);
			s.is_empty()
		}
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 27090);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ß', s @ ..] => {
			super::push(out, 17012);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -merl.
pub(crate) fn n7562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7480);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', 'h', s @ ..] => {
					super::push(out, 27092);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40036);
									s.is_empty()
								}
								['r', 'e', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 40037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 27093);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 27094);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 27095);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 34944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -perl.
pub(crate) fn n7563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7481);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17013);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 17014);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 27096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 17015);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 34945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 34946);
							s.is_empty()
						}
						['l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 34947);
							s.is_empty()
						}
						['n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 34948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 27097);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 34949);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 34950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 17016);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -terl.
pub(crate) fn n7565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7483);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'k', 'n', 'e', 'm', 'u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 27099);
					s.is_empty()
				}
				['n', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 27100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zerl.
pub(crate) fn n7567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7485);
	match s {
		[] => true,
		['n', 'u', s @ ..] => {
			super::push(out, 17018);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 27101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 'k', 'h', 'c', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 17019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irl.
pub(crate) fn n2011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2009);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7486);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17020);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 27102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7487);
			match s {
				[] => true,
				['q', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27103);
							s.is_empty()
						}
						['l', 'l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 27104);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27105);
							s.is_empty()
						}
						['z', 'l', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 27106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7488);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27107);
							s.is_empty()
						}
						['d', 'l', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 27108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrl.
pub(crate) fn n2012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2010);
	match s {
		[] => true,
		['e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 7489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -url.
pub(crate) fn n2013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2011);
	match s {
		[] => true,
		['w', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 7490);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -örl.
pub(crate) fn n2014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2012);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7491);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 17021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word retardierungsproblem.
pub(crate) fn n45605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42808);
	s.is_empty()
}

/// Words ending -tem.
pub(crate) fn n2097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2093);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7624);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 17122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7625);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7626);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 17124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7627);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 17125);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 27136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34964);
									s.is_empty()
								}
								['-', s @ ..] => {
									match s {
										[] => true,
										['e', 'd', s @ ..] => {
											super::push(out, 40044);
											s.is_empty()
										}
										['t', 'h', s @ ..] => {
											super::push(out, 40045);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'l', 'e', 'f', s @ ..] => {
									super::push(out, 34965);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 34966);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 34967);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40046);
											match s {
												[] => true,
												['t', 'a', 'd', '\u{1}', s @ ..] => {
													super::push(out, 42809);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 34968);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 40047);
											match s {
												[] => true,
												['t', 'e', 'n', 'o', 'h', 'p', '\u{1}', s @ ..] => {
													super::push(out, 42810);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 34969);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40048);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 42811);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 42812);
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 42813);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 40049);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 40050);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 40051);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 40052);
											match s {
												[] => true,
												['h', 'c', 's', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t13::n47088(s, out),
														['h', s @ ..] => super::t13::n47089(s, out),
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
									super::push(out, 34970);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 42814);
													match s {
														[] => true,
														['o', s @ ..] => super::t13::n47090(s, out),
														_ => false,
													}
												}
												['h', s @ ..] => {
													super::push(out, 42815);
													match s {
														[] => true,
														['c', 'o', s @ ..] => super::t13::n47091(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', 'a', s @ ..] => {
											super::push(out, 40053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 34971);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 34972);
									match s {
										[] => true,
										['e', s @ ..] => super::t13::n42537(s, out),
										['h', s @ ..] => {
											super::push(out, 40054);
											s.is_empty()
										}
										['ä', s @ ..] => {
											super::push(out, 40055);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => super::t13::n36903(s, out),
								['t', s @ ..] => super::t13::n36904(s, out),
								['v', s @ ..] => super::t13::n36905(s, out),
								['z', s @ ..] => super::t13::n36906(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -schlamm.
pub(crate) fn n28305(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 34981);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 40062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 34982);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 40063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 34983);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 40064);
					match s {
						[] => true,
						['l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 42823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ramm.
pub(crate) fn n7763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7672);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 27170);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 34984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 27171);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 34985);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40065);
									s.is_empty()
								}
								['-', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42824);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 42825);
											s.is_empty()
										}
										_ => false,
									}
								}
								['a', s @ ..] => {
									super::push(out, 40066);
									s.is_empty()
								}
								['l', 'i', 'e', 'f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 40067);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42826);
											s.is_empty()
										}
										['h', 'c', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42827);
											s.is_empty()
										}
										['i', 'n', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 42828);
											s.is_empty()
										}
										['k', 'l', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 42829);
											s.is_empty()
										}
										['l', 'u', 'ä', 's', '\u{1}', s @ ..] => {
											super::push(out, 42830);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 42831);
											s.is_empty()
										}
										['t', 'r', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 42832);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'a', 't', 's', 'u', 'z', '\u{1}', s @ ..] => {
											super::push(out, 42833);
											s.is_empty()
										}
										['i', 'e', 'r', 'k', '\u{1}', s @ ..] => {
											super::push(out, 42834);
											s.is_empty()
										}
										['s', 'u', 'l', 'f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44131);
													s.is_empty()
												}
												['n', 'e', 't', 'a', 'd', '\u{1}', s @ ..] => {
													super::push(out, 44132);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'ä', 't', 'i', 'v', 'i', 't', 'k', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42835);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 40068);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 34986);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 34987);
							s.is_empty()
						}
						['r', 't', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 34988);
							s.is_empty()
						}
						['t', 'n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 27172);
					match s {
						[] => true,
						['l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 34990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 27173);
					match s {
						[] => true,
						['b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 34991);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 34992);
							s.is_empty()
						}
						['n', 'a', 'g', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 34993);
							s.is_empty()
						}
						['p', 'e', '\u{1}', s @ ..] => super::t13::n36924(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t13::n28311(s, out),
				['o', s @ ..] => super::t13::n28312(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t13::n17611(s, out),
		_ => false,
	}
}

/// Words ending -tamm.
pub(crate) fn n7764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7673);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 17167);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 27177);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 35005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'n', s @ ..] => {
					super::push(out, 27178);
					match s {
						[] => true,
						['i', 'u', 'd', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 35006);
							s.is_empty()
						}
						['u', 't', 'h', 'c', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 35007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wamm.
pub(crate) fn n7765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7674);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 17168);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 35008);
							s.is_empty()
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 35009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -emm.
pub(crate) fn n2119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2115);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7675);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7676);
			match s {
				[] => true,
				['s', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 17169);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imm.
pub(crate) fn n2120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2116);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7677);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7678);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7679);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 17170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27179);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 27180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17171);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -umm.
pub(crate) fn n2121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2117);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7680);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7681);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17172);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7682);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -drom.
pub(crate) fn n7802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7711);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 17194);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27184);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40078);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 42846);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 40079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 27185);
					s.is_empty()
				}
				['y', 's', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 35012);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 40080);
									s.is_empty()
								}
								['e', 'i', s @ ..] => {
									super::push(out, 40081);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 40082);
									match s {
										[] => true,
										['l', 'o', 'h', 'k', 'c', 'o', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 42847);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42848);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 42849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 35013);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 40083);
									s.is_empty()
								}
								['n', 'i', 'l', 'r', 'e', 'd', 'r', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 35014);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35015);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 35016);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 42850);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42851);
											match s {
												[] => true,
												['u', 'r', s @ ..] => {
													super::push(out, 44157);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 42852);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 35017);
							match s {
								[] => true,
								['i', 'z', 'i', 'f', 'e', 'd', 's', 't', 'i', 'e', 'k', 'm', 'a', 's', 'k', 'r', 'e', 'm', 'f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 40085);
									s.is_empty()
								}
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
			super::push(out, 17195);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 27186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -esrom.
pub(crate) fn n17645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17199);
	s.is_empty()
}

/// Words ending -trom.
pub(crate) fn n7808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7717);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 17200);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 27190);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 27191);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 27192);
					s.is_empty()
				}
				['f', 'l', s @ ..] => {
					super::push(out, 27193);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 27194);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 35018);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 35019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 27195);
					match s {
						[] => true,
						['e', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 35020);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 35021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 27196);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35022);
							match s {
								[] => true,
								['m', 'u', 'l', 'o', 'v', 's', s @ ..] => {
									super::push(out, 40086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 35023);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 35024);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 42853);
											s.is_empty()
										}
										['w', '\u{1}', s @ ..] => {
											super::push(out, 42854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'i', s @ ..] => {
							super::push(out, 35025);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 40087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 27197);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 27198);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35026);
							match s {
								[] => true,
								['e', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 40088);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 40089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 35027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 27199);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27200);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35028);
							s.is_empty()
						}
						['s', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 35029);
							s.is_empty()
						}
						['t', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 35030);
							s.is_empty()
						}
						['u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 35031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -som.
pub(crate) fn n2132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7718);
			match s {
				[] => true,
				['m', 'o', 'r', 'h', 'c', s @ ..] => {
					match s {
						[] => true,
						['-', 'y', '\u{1}', s @ ..] => {
							super::push(out, 27201);
							s.is_empty()
						}
						['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 27202);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 27203);
							match s {
								[] => true,
								['r', 'e', 't', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 35032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tom.
pub(crate) fn n2133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2129);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7719);
			match s {
				[] => true,
				['d', 'o', 'i', s @ ..] => {
					super::push(out, 17201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 17202);
					match s {
						[] => true,
						['f', 'o', 't', 's', 'r', 'e', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 35033);
									s.is_empty()
								}
								['u', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 35034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 17203);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17204);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17205);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'o', 'b', s @ ..] => {
					super::push(out, 17206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7720);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7721);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yom.
pub(crate) fn n2134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2130);
	s.is_empty()
}

/// Words ending -zom.
pub(crate) fn n2135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2131);
	s.is_empty()
}

/// Words ending -urm.
pub(crate) fn n2140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2136);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 7743);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17231);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 27222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17232);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 27223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'z', s @ ..] => {
					super::push(out, 17233);
					match s {
						[] => true,
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 27224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27225);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 27226);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 27227);
							s.is_empty()
						}
						['n', 'e', 'g', s @ ..] => {
							super::push(out, 27228);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27229);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35044);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', s @ ..] => {
							super::push(out, 27230);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27231);
							match s {
								[] => true,
								['r', 'ä', 'm', '\u{1}', s @ ..] => {
									super::push(out, 35046);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 7744);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17235);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ärm.
pub(crate) fn n2141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2137);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7745);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7746);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 17238);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['r', 'o', 't', 'o', 'r', s @ ..] => {
							super::push(out, 27232);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 27233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17239);
					s.is_empty()
				}
				['s', 't', 'i', s @ ..] => {
					super::push(out, 17240);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 17241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word guajavenbaum.
pub(crate) fn n45662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42855);
	s.is_empty()
}

/// Words ending -uavenbaum.
pub(crate) fn n45663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42856);
	s.is_empty()
}

/// Words ending -bschaum.
pub(crate) fn n28386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27238);
	s.is_empty()
}

/// Words ending -eschaum.
pub(crate) fn n28387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27239);
	s.is_empty()
}

/// Words ending -hschaum.
pub(crate) fn n28388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27240);
	s.is_empty()
}

/// Words ending -bierschaum.
pub(crate) fn n28389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27241);
	s.is_empty()
}

/// Words ending -sschaum.
pub(crate) fn n28390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27242);
	s.is_empty()
}

/// Words ending -laum.
pub(crate) fn n7843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7752);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 17248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -raum.
pub(crate) fn n7844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7753);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 17249);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 27244);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 27245);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 27246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 17250);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 27247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', s @ ..] => {
					super::push(out, 27248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 17251);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 27249);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 27250);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 35063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 27251);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 35064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'a', 'r', s @ ..] => {
			super::push(out, 17252);
			match s {
				[] => true,
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 27252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 17253);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17254);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 17255);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 35065);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 35066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 27253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 17256);
			match s {
				[] => true,
				['e', 'i', 'p', 's', 's', s @ ..] => {
					super::push(out, 27254);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 35067);
							match s {
								[] => true,
								['n', 'u', 'l', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 40095);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27255);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 35068);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 35069);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'i', 'w', 'z', 'n', s @ ..] => {
							super::push(out, 35070);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40097);
									s.is_empty()
								}
								['h', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 40098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 27256);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35071);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 40099);
									s.is_empty()
								}
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
			super::push(out, 17257);
			match s {
				[] => true,
				['r', 'u', s @ ..] => {
					super::push(out, 27257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 27258);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35072);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 40100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', s @ ..] => {
							super::push(out, 35073);
							match s {
								[] => true,
								['m', 'n', 'h', 'e', 'z', 'h', 'c', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 40101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 17258);
			match s {
				[] => true,
				['g', 'n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 35074);
							s.is_empty()
						}
						['u', 'l', 'd', s @ ..] => {
							super::push(out, 35075);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 40102);
									match s {
										[] => true,
										['i', 's', '\u{1}', s @ ..] => {
											super::push(out, 42857);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', 'i', 't', 'a', 't', s @ ..] => {
					super::push(out, 27260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 17259);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 27261);
					match s {
						[] => true,
						['e', 'z', 'e', s @ ..] => {
							super::push(out, 35076);
							match s {
								[] => true,
								['g', 'a', 'l', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 40103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 27262);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27263);
					match s {
						[] => true,
						['e', 'b', 'e', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 35078);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 35079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'u', s @ ..] => {
					super::push(out, 27264);
					s.is_empty()
				}
				['t', 'e', s @ ..] => {
					super::push(out, 27265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 17260);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 27266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'n', 'e', 'i', s @ ..] => {
			super::push(out, 17261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lsaum.
pub(crate) fn n7845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7754);
	match s {
		[] => true,
		['ü', s @ ..] => {
			super::push(out, 17262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zaum.
pub(crate) fn n7846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7755);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17263);
			s.is_empty()
		}
		['p', 'p', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 17264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bum.
pub(crate) fn n2146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7756);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17265);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7757);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7758);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27267);
							s.is_empty()
						}
						['l', 'a', 'd', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 27268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cum.
pub(crate) fn n2147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2143);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7759);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7760);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7761);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17268);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17269);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dum.
pub(crate) fn n2148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2144);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7762);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7763);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 17271);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 27269);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eum.
pub(crate) fn n2149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2145);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7764);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27271);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27272);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27273);
							s.is_empty()
						}
						['s', 'u', 'a', 'm', '-', s @ ..] => {
							super::push(out, 27274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7765);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 17272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7766);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 17273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7767);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 17274);
					s.is_empty()
				}
				['u', 'm', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27275);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 27276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fum.
pub(crate) fn n2150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2146);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 7768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gum.
pub(crate) fn n2151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2147);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7769);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17275);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hum.
pub(crate) fn n2152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2148);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7771);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7772);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17277);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 17278);
					match s {
						[] => true,
						['i', 'n', 'ö', 'k', '\u{1}', s @ ..] => {
							super::push(out, 27277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 27278);
							s.is_empty()
						}
						['n', 'ö', 'm', '\u{1}', s @ ..] => {
							super::push(out, 27279);
							s.is_empty()
						}
						['s', 't', 'u', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 27280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['g', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 27281);
							s.is_empty()
						}
						['h', 'c', 's', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 27282);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 35080);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 40104);
											s.is_empty()
										}
										['r', 'ü', 'f', s @ ..] => {
											match s {
												[] => true,
												['-', 'r', 'u', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 42858);
													s.is_empty()
												}
												['r', 'u', s @ ..] => {
													match s {
														[] => true,
														['h', 'c', '\u{1}', s @ ..] => super::t13::n47127(s, out),
														['k', '\u{1}', s @ ..] => super::t13::n47128(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'f', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 35081);
									s.is_empty()
								}
								['s', 'i', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 35082);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40105);
											s.is_empty()
										}
										['t', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 40106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 27283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 35083);
									s.is_empty()
								}
								['n', 'ö', 'm', '\u{1}', s @ ..] => {
									super::push(out, 35084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 27284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17279);
					match s {
						[] => true,
						['f', 'i', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 27285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ium.
pub(crate) fn n2153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2149);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7774);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 7775);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17281);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17282);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17283);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 17284);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17285);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17286);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 35085);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 35086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 27287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7776);
			match s {
				[] => true,
				['i', 'l', 'a', s @ ..] => {
					super::push(out, 17287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7777);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17288);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27288);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27289);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27290);
							match s {
								[] => true,
								['s', 't', s @ ..] => {
									super::push(out, 35087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 27291);
							match s {
								[] => true,
								['q', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 40107);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 40108);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40109);
											s.is_empty()
										}
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
					super::push(out, 17289);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27292);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27293);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 27294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 17290);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27295);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 35089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 27296);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 27297);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 27298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27299);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27300);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17291);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27302);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 27303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17292);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 27304);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['-', 'a', s @ ..] => {
									super::push(out, 35090);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 35091);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 40110);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 40111);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'i', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40112);
											s.is_empty()
										}
										['m', 'e', s @ ..] => {
											super::push(out, 40113);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 35092);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 42859);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 42860);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'i', s @ ..] => {
									super::push(out, 35093);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35094);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35095);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									super::push(out, 35096);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 40114);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'a', 'b', 'g', s @ ..] => {
									super::push(out, 35097);
									s.is_empty()
								}
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
			super::push(out, 7778);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7779);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							super::push(out, 27305);
							match s {
								[] => true,
								['i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 35098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 27306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17293);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 27307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 17294);
					match s {
						[] => true,
						['r', 'e', 't', 'p', s @ ..] => {
							match s {
								[] => true,
								['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35099);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['r', 'i', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 40115);
											s.is_empty()
										}
										['y', 'h', 't', 'h', 'c', 'i', '\u{1}', s @ ..] => super::t13::n42611(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t13::n7871(s, out),
		['l', s @ ..] => super::t13::n7872(s, out),
		['m', s @ ..] => super::t13::n7873(s, out),
		['n', s @ ..] => super::t13::n7874(s, out),
		['p', s @ ..] => super::t13::n7875(s, out),
		['r', s @ ..] => super::t13::n7876(s, out),
		['s', s @ ..] => super::t13::n7877(s, out),
		['t', s @ ..] => super::t13::n7878(s, out),
		['u', 'q', 'o', 'l', 'l', s @ ..] => super::t13::n7879(s, out),
		['v', s @ ..] => super::t13::n7880(s, out),
		['w', s @ ..] => super::t13::n7881(s, out),
		['x', s @ ..] => super::t13::n7882(s, out),
		['z', s @ ..] => super::t13::n7883(s, out),
		_ => false,
	}
}

/// Words ending -kum.
pub(crate) fn n2154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2150);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7792);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7793);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17350);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 17351);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 17352);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 17353);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 27373);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35133);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 35134);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35135);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 35136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17354);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27374);
							match s {
								[] => true,
								['u', 'p', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 40124);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 40125);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40126);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 40127);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 27375);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 35137);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 40128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 27376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17355);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17356);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17357);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27377);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17358);
					s.is_empty()
				}
				['s', 's', s @ ..] => {
					super::push(out, 17359);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17360);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 27379);
							match s {
								[] => true,
								['l', 'k', s @ ..] => {
									super::push(out, 35138);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 40129);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27380);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									super::push(out, 35139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 27381);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27382);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35140);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35141);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40130);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 27383);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 17361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7794);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lum.
pub(crate) fn n2155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2151);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7795);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 17362);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 17363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7796);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					super::push(out, 17364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7797);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7798);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 17365);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 35143);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 35144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7799);
			match s {
				[] => true,
				['h', 'p', 'b', s @ ..] => {
					super::push(out, 17366);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 27384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mum.
pub(crate) fn n2156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2152);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7800);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7801);
			match s {
				[] => true,
				['n', 'i', 'm', 'r', s @ ..] => {
					super::push(out, 17368);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 17369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -num.
pub(crate) fn n2157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2153);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7802);
			match s {
				[] => true,
				['k', 'r', s @ ..] => {
					super::push(out, 17370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7803);
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 17371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7804);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17372);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 27385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17373);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 27386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7805);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17374);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pum.
pub(crate) fn n2158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2154);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7807);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rum.
pub(crate) fn n2159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2155);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7808);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7809);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17376);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17377);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 17378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 7810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17379);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 27388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 17380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7811);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 17381);
					s.is_empty()
				}
				['s', 'n', 'u', 'm', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 17382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7812);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7813);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 27389);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							super::push(out, 27390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 17384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7814);
			match s {
				[] => true,
				['u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 17385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7815);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 17386);
					match s {
						[] => true,
						['e', 'p', 's', 's', 'u', s @ ..] => {
							super::push(out, 27391);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 35145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 17387);
					match s {
						[] => true,
						['e', 'z', 's', 'g', 'n', 'u', s @ ..] => {
							super::push(out, 27392);
							match s {
								[] => true,
								['r', 'e', 'i', s @ ..] => {
									super::push(out, 35146);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 40131);
											s.is_empty()
										}
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
					super::push(out, 17388);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 27393);
							s.is_empty()
						}
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 27394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7816);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17390);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					super::push(out, 17391);
					s.is_empty()
				}
				['t', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 17392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sum.
pub(crate) fn n2160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2156);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7818);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 17393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7819);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17394);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 27395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17395);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27396);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27397);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 35147);
									s.is_empty()
								}
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
			super::push(out, 7820);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7821);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17396);
					match s {
						[] => true,
						['v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 27398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17397);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 27399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7822);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17398);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17399);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tum.
pub(crate) fn n2161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7823);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17401);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 17402);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27400);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 27401);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 35148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'l', s @ ..] => {
							super::push(out, 27402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 17403);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17404);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17405);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17406);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17407);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 17408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7824);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17409);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17410);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27404);
							s.is_empty()
						}
						['o', 'b', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 27405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7825);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7826);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17412);
					match s {
						[] => true,
						['l', 'i', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 27406);
							s.is_empty()
						}
						['n', 'ö', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35149);
									s.is_empty()
								}
								['e', 's', 'i', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 35150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17413);
					match s {
						[] => true,
						['z', 'r', 'e', 'h', 's', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 27407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7827);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27408);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 27409);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40132);
											match s {
												[] => true,
												['i', 'n', s @ ..] => {
													super::push(out, 42865);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 40133);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27410);
							match s {
								[] => true,
								['ö', 'm', '\u{1}', s @ ..] => {
									super::push(out, 35151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 27411);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 35152);
									match s {
										[] => true,
										['u', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 40135);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 27412);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35153);
									match s {
										[] => true,
										['r', 'b', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42866);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 42867);
													s.is_empty()
												}
												['s', 't', 'n', s @ ..] => {
													super::push(out, 42868);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7828);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17414);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 17415);
					s.is_empty()
				}
				['r', 'e', 'm', s @ ..] => {
					super::push(out, 17416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17417);
					match s {
						[] => true,
						['o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 27414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7829);
			match s {
				[] => true,
				['a', 'f', s @ ..] => {
					super::push(out, 17418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7830);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7831);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17419);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 27416);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 27417);
							match s {
								[] => true,
								['q', 's', s @ ..] => {
									super::push(out, 35154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t13::n17894(s, out),
				['r', s @ ..] => {
					super::push(out, 17421);
					match s {
						[] => true,
						['e', 'u', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 27428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t13::n7925(s, out),
		['p', s @ ..] => {
			super::push(out, 7833);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 17425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t13::n7927(s, out),
		['s', s @ ..] => super::t13::n7928(s, out),
		['t', s @ ..] => super::t13::n7929(s, out),
		['u', s @ ..] => super::t13::n7930(s, out),
		['y', s @ ..] => {
			super::push(out, 7838);
			s.is_empty()
		}
		['z', s @ ..] => super::t13::n7932(s, out),
		_ => false,
	}
}

/// Words ending -uum.
pub(crate) fn n2162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2158);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7840);
			s.is_empty()
		}
		['k', 'a', 'v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17439);
					s.is_empty()
				}
				['h', 'c', 'o', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27446);
							s.is_empty()
						}
						['a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
							super::push(out, 27447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 17440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 't', 'n', 'o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17441);
					s.is_empty()
				}
				['-', 't', 'i', 'e', 'z', '-', 'm', 'u', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 17442);
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 17443);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 't', 's', 'n', 'e', 'm', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 7841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vum.
pub(crate) fn n2163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2159);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7842);
			match s {
				[] => true,
				['s', 's', s @ ..] => {
					super::push(out, 17445);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xum.
pub(crate) fn n2164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2160);
	s.is_empty()
}

/// Words ending -äum.
pub(crate) fn n2165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2161);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7843);
			match s {
				[] => true,
				['ö', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 17446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7844);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word seidenkostüm.
pub(crate) fn n45697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42887);
	s.is_empty()
}

/// The word halloweenkostüm.
pub(crate) fn n45698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42888);
	s.is_empty()
}

/// The word affenkostüm.
pub(crate) fn n45699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42889);
	s.is_empty()
}

/// The word hofnarrenkostüm.
pub(crate) fn n45700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42890);
	s.is_empty()
}

/// The word frauenkostüm.
pub(crate) fn n45701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42891);
	s.is_empty()
}

/// The word nervenkostüm.
pub(crate) fn n45702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42892);
	s.is_empty()
}

/// The word katzenkostüm.
pub(crate) fn n45703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42893);
	s.is_empty()
}

/// The word sensenmannkostüm.
pub(crate) fn n45704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42894);
	s.is_empty()
}

/// The word weihnachtsmannkostüm.
pub(crate) fn n45705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42895);
	s.is_empty()
}

/// The word lederkostüm.
pub(crate) fn n45706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42896);
	s.is_empty()
}

/// The word sommerkostüm.
pub(crate) fn n45707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42897);
	s.is_empty()
}

/// The word männerkostüm.
pub(crate) fn n45708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42898);
	s.is_empty()
}

/// The word theaterkostüm.
pub(crate) fn n45709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42899);
	s.is_empty()
}

/// Words ending -ian.
pub(crate) fn n2185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2181);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7886);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17481);
					match s {
						[] => true,
						['m', 'o', s @ ..] => {
							super::push(out, 27473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7887);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 7888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 7889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7890);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 17485);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27474);
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 35202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 27475);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17486);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 27477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7891);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 7892);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 7893);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17487);
					s.is_empty()
				}
				['t', 'u', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 17488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kan.
pub(crate) fn n2187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7896);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7897);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7898);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17489);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17490);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7899);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17492);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 27478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17493);
					match s {
						[] => true,
						['v', 'r', 'e', 'p', s @ ..] => {
							super::push(out, 27479);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 35203);
									s.is_empty()
								}
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
			super::push(out, 7900);
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 17494);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27480);
							s.is_empty()
						}
						['s', 't', 'h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 27481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lan.
pub(crate) fn n2188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7902);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 17495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17496);
					s.is_empty()
				}
				['n', 'e', 'i', 'l', 'i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 17497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7904);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17498);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7905);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7906);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 17501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7907);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17502);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17503);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17504);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 17505);
					match s {
						[] => true,
						['x', 'u', 'l', 'k', '-', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 27483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7909);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 27484);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27485);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7910);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17506);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17507);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 17508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7911);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17509);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'h', 'c', 'ä', s @ ..] => {
					super::push(out, 17510);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17511);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27488);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27489);
							match s {
								[] => true,
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17512);
					match s {
						[] => true,
						['a', 'p', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35205);
									s.is_empty()
								}
								['s', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 27490);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35207);
									match s {
										[] => true,
										['f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42900);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 42901);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 42902);
													match s {
														[] => true,
														['t', 'a', 's', 'r', 'e', '\u{1}', s @ ..] => super::t13::n47142(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', 'l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40174);
											s.is_empty()
										}
										['n', 'e', 'm', 'h', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 40175);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'f', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35208);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 't', 'k', 'u', 'r', 't', 's', 't', 'k', 'e', 'j', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 27491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17513);
					match s {
						[] => true,
						['e', 'r', 'h', 'a', 'j', 'f', s @ ..] => {
							super::push(out, 27492);
							match s {
								[] => true,
								['n', 'ü', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', 'u', s @ ..] => {
							match s {
								[] => true,
								['t', 't', s @ ..] => {
									super::push(out, 35210);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 35211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'f', 'a', s @ ..] => {
							super::push(out, 27493);
							match s {
								[] => true,
								['h', 'c', 's', 't', 'r', 'i', 'w', 'l', 'l', 'a', 'f', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17514);
					match s {
						[] => true,
						['l', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 27494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 17515);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 17516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7912);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17517);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 27495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7913);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17518);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17519);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -man.
pub(crate) fn n2189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2185);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7915);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7916);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17521);
					match s {
						[] => true,
						['d', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 27496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17522);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 17523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7917);
			match s {
				[] => true,
				['d', 'a', 'm', 'f', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 17524);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 17525);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7918);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7919);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17527);
					match s {
						[] => true,
						['k', 'n', 'r', 'i', 't', 's', 't', 't', 'a', 'l', 'g', '-', 'n', 'e', 'u', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 27497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7920);
			match s {
				[] => true,
				['l', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 17529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7921);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17530);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17531);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 27498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7922);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7923);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17533);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 27499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27500);
							s.is_empty()
						}
						['-', 'h', s @ ..] => {
							super::push(out, 27501);
							match s {
								[] => true,
								['c', 'i', '\u{1}', s @ ..] => {
									super::push(out, 35213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 27502);
							match s {
								[] => true,
								['e', 'i', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 35214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35215);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40176);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', 'f', 'k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 35216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'i', 't', 'o', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 27503);
							s.is_empty()
						}
						['l', 'e', 's', 's', 'ü', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 27504);
							s.is_empty()
						}
						['n', 'e', 'm', 'l', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 27505);
							s.is_empty()
						}
						['r', 'e', 'u', 'e', 't', 'n', 'e', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 27506);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 35217);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 35218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', s @ ..] => {
							super::push(out, 27507);
							match s {
								[] => true,
								['e', 'z', '\u{1}', s @ ..] => {
									super::push(out, 35219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 17535);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27508);
							s.is_empty()
						}
						['r', 'o', 'h', 'c', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 27509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7924);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17536);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 27510);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 27511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7925);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 17538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7926);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17539);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 27512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 17540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7927);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17541);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -anan.
pub(crate) fn n8025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7928);
	s.is_empty()
}

/// Words ending -knan.
pub(crate) fn n8026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7929);
	s.is_empty()
}

/// Words ending -innan.
pub(crate) fn n8027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7930);
	s.is_empty()
}

/// Words ending -onan.
pub(crate) fn n8028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7931);
	s.is_empty()
}

/// Words ending -oan.
pub(crate) fn n2191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2187);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pan.
pub(crate) fn n2192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2188);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7933);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 17542);
					match s {
						[] => true,
						['e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 27513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27514);
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 27515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 17544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7934);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17545);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7935);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 17547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7936);
			match s {
				[] => true,
				['i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 17548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7937);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17549);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35220);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35221);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 35222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17550);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17551);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27516);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35223);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 27517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7938);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ran.
pub(crate) fn n2193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2189);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7939);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17552);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 17553);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 27518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17554);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 17555);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27519);
							match s {
								[] => true,
								['e', 'p', 'p', 'e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 35225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'd', 'o', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 27520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 7940);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7941);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 17557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7942);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17558);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7944);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7945);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7946);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17561);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 27521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 17562);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 17563);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7947);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 17565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7948);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17566);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7949);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17568);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 17569);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 27522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17570);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7950);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -san.
pub(crate) fn n2194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2190);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7951);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7952);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 17573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27524);
							s.is_empty()
						}
						['a', 'm', 'r', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 27525);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 27526);
							s.is_empty()
						}
						['z', 'n', 'a', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 27527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7953);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17574);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17575);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 17576);
					match s {
						[] => true,
						['ö', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 27528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7954);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17577);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17578);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 17579);
					match s {
						[] => true,
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 27529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7955);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7956);
			match s {
				[] => true,
				['a', 'v', s @ ..] => {
					super::push(out, 17581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7957);
			match s {
				[] => true,
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 17582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tan.
pub(crate) fn n2195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2191);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7958);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7959);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 7960);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7961);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7962);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7963);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7964);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17584);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17585);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17586);
					match s {
						[] => true,
						['o', 'p', 'o', 'm', 's', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 27530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27531);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 27532);
							match s {
								[] => true,
								['f', 'f', 'a', 'r', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 35226);
									s.is_empty()
								}
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
			super::push(out, 7965);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 7966);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17589);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17590);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 27534);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17591);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17592);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							super::push(out, 27536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7968);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17594);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 17595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7969);
			match s {
				[] => true,
				['e', 'h', 'o', s @ ..] => {
					super::push(out, 17596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7970);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17597);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7971);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17599);
					match s {
						[] => true,
						['g', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 27537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17600);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7972);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27539);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7973);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17602);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 17603);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 27541);
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 35227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 27542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uan.
pub(crate) fn n2196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2192);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7974);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7975);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7976);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 7977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17604);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 17605);
					match s {
						[] => true,
						['v', 'i', 't', 'u', 'k', 'e', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 27543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 17606);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -van.
pub(crate) fn n2197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2193);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7978);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 17607);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17608);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7979);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wan.
pub(crate) fn n2198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2194);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7981);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 7982);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27545);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 27546);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 27547);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17611);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 17612);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xan.
pub(crate) fn n2199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2195);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7985);
			match s {
				[] => true,
				['h', 'o', s @ ..] => {
					super::push(out, 17614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yan.
pub(crate) fn n2200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2196);
	match s {
		[] => true,
		['c', 'o', s @ ..] => {
			super::push(out, 7986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zan.
pub(crate) fn n2201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2197);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 7987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ōan.
pub(crate) fn n2202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2198);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eben.
pub(crate) fn n8087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7990);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 17619);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 17620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 17621);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 17622);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 17623);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 27557);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 27558);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27559);
					s.is_empty()
				}
				['t', 's', 'r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 27560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 17624);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 27561);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 27562);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35239);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 35240);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 35241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', s @ ..] => {
					super::push(out, 27563);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 27564);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 40180);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 40181);
									s.is_empty()
								}
								['l', 'h', s @ ..] => {
									super::push(out, 40182);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 40183);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 42903);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 42904);
											s.is_empty()
										}
										['s', 'i', s @ ..] => {
											super::push(out, 42905);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 35242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27565);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 40184);
									s.is_empty()
								}
								['g', 'a', s @ ..] => {
									super::push(out, 40185);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 40186);
									s.is_empty()
								}
								['n', 'u', 'e', s @ ..] => {
									super::push(out, 40187);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 40188);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42906);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 42907);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 40189);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 40190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 35243);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 40191);
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
						['e', 'b', s @ ..] => {
							super::push(out, 35244);
							s.is_empty()
						}
						['g', 'a', s @ ..] => {
							super::push(out, 35245);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 35246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'h', s @ ..] => {
					super::push(out, 27566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 17625);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 17626);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 27567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 17627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iben.
pub(crate) fn n8088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7991);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27568);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 35247);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 42908);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 42909);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 42910);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 42911);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 42912);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 44172);
													s.is_empty()
												}
												['i', 'e', 'l', s @ ..] => {
													super::push(out, 44173);
													s.is_empty()
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
														['b', s @ ..] => super::t13::n47768(s, out),
														['t', s @ ..] => super::t13::n47769(s, out),
														['v', s @ ..] => super::t13::n47770(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 44174);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 42913);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 44175);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 44176);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 44177);
													s.is_empty()
												}
												['s', 'a', s @ ..] => {
													super::push(out, 44178);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 44179);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 42914);
											s.is_empty()
										}
										['ß', s @ ..] => {
											super::push(out, 42915);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 35248);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 35249);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 40192);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 40193);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 40194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lben.
pub(crate) fn n8089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7992);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27569);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 17629);
			match s {
				[] => true,
				['k', 's', s @ ..] => {
					super::push(out, 27571);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35250);
							match s {
								[] => true,
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 40195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 17630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nben.
pub(crate) fn n8090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7993);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 17631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oben.
pub(crate) fn n8091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7994);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 17632);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 27572);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 27573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 17633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rben.
pub(crate) fn n8092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7995);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17634);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 27574);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 27575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 17635);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 27576);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 35251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 27577);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40196);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 40197);
									match s {
										[] => true,
										['u', 'a', 'n', s @ ..] => {
											super::push(out, 42916);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uben.
pub(crate) fn n8093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7996);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 17637);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27578);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 35252);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 35253);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 27579);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 27580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 17638);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 27581);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cen.
pub(crate) fn n2207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2203);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -den.
pub(crate) fn n2208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2204);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7999);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17639);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 17640);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 27582);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 35255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 27583);
							match s {
								[] => true,
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 35256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 17641);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27584);
							match s {
								[] => true,
								['e', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 35257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 17642);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17643);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27585);
							s.is_empty()
						}
						['e', 'b', s @ ..] => {
							super::push(out, 27586);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 27587);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35258);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35259);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 35260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 27588);
							match s {
								[] => true,
								['c', 'a', s @ ..] => {
									super::push(out, 35261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 27589);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 35262);
									match s {
										[] => true,
										['ü', 'b', '\u{1}', s @ ..] => {
											super::push(out, 40198);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 35263);
									match s {
										[] => true,
										['o', 'v', s @ ..] => {
											super::push(out, 40199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27590);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35264);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 35265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27591);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 40200);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 40201);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 35266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 27592);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 35267);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', s @ ..] => {
							super::push(out, 27593);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 27594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17644);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 17645);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 17646);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 17647);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8000);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 17648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8001);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 17649);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27597);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 35268);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 42917);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 42918);
													s.is_empty()
												}
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
							super::push(out, 27598);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 40202);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40203);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 42919);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 42920);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 40204);
											s.is_empty()
										}
										['t', 'l', s @ ..] => {
											super::push(out, 40205);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 27599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8002);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8003);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17651);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27600);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27601);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 35269);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35270);
									s.is_empty()
								}
								['r', 'e', 'v', s @ ..] => {
									super::push(out, 35271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27602);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27603);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 35272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 17652);
					s.is_empty()
				}
				['l', s @ ..] => super::t13::n18135(s, out),
				['n', s @ ..] => super::t13::n18136(s, out),
				['r', s @ ..] => super::t13::n18137(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t13::n8101(s, out),
		['m', s @ ..] => super::t13::n8102(s, out),
		['n', s @ ..] => super::t13::n8103(s, out),
		['o', s @ ..] => super::t13::n8104(s, out),
		['r', s @ ..] => super::t13::n8105(s, out),
		['u', s @ ..] => super::t13::n8106(s, out),
		['ö', s @ ..] => super::t13::n8107(s, out),
		['ü', s @ ..] => super::t13::n8108(s, out),
		_ => false,
	}
}

/// Words ending -een.
pub(crate) fn n2209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2205);
	match s {
		[] => true,
		['b', 'e', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 8012);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 8013);
			match s {
				[] => true,
				['a', 'r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 17675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8014);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8015);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17676);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 17677);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8016);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 17678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 8017);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 17679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

