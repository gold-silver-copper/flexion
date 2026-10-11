use flexion::Path;

/// Words ending -стер.
pub(crate) fn n14983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37235);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65334);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91659);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111648);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124620);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 91660);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', '\u{1}', s @ ..] => {
					super::push(out, 65335);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 65336);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 65337);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65338);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91663);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111650);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124621);
											s.is_empty()
										}
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
					super::push(out, 65339);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 65340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91665);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 91666);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111651);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124622);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132459);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 111652);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 124623);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132460);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138512(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91667);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111653);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124624);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 91668);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111654);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124625);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132461);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 111655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 91669);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111656);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124627);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132462);
													s.is_empty()
												}
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
							super::push(out, 91670);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111657);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 124628);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132463);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138513(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ф', s @ ..] => {
											super::push(out, 124629);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 111658);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124630);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132464);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => super::t48::n112842(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t48::n92644(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t48::n66015(s, out),
				['р', s @ ..] => super::t48::n66016(s, out),
				['ф', s @ ..] => super::t48::n66017(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t48::n37587(s, out),
		['г', s @ ..] => super::t48::n37588(s, out),
		['д', s @ ..] => super::t48::n37589(s, out),
		['е', s @ ..] => super::t48::n37590(s, out),
		['и', s @ ..] => super::t48::n37591(s, out),
		['й', s @ ..] => super::t48::n37592(s, out),
		['к', s @ ..] => super::t48::n37593(s, out),
		['м', s @ ..] => super::t48::n37594(s, out),
		['н', s @ ..] => super::t48::n37595(s, out),
		['о', s @ ..] => super::t48::n37596(s, out),
		['п', s @ ..] => super::t48::n37597(s, out),
		['р', s @ ..] => super::t48::n37598(s, out),
		['у', s @ ..] => super::t48::n37599(s, out),
		['ь', s @ ..] => super::t48::n37600(s, out),
		['э', s @ ..] => super::t48::n37601(s, out),
		['ю', s @ ..] => super::t48::n37602(s, out),
		_ => false,
	}
}

/// Words ending -ттер.
pub(crate) fn n14984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14906);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37252);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65380);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91709);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 91710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 65381);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 65382);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 65383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37253);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65384);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 65385);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 65386);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91712);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 111682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65387);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91713);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 65388);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91714);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 91715);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111685);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124644);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91716);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111686);
									s.is_empty()
								}
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
			super::push(out, 37254);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65389);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 65390);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 65391);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 65392);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 65393);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91717);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 65394);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91718);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 91719);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 65395);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91720);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111690);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124645);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132473);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37255);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65396);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91721);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 65397);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91723);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111692);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124646);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 111693);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124647);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132474);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37256);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 37257);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утер.
pub(crate) fn n14985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14907);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37258);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65400);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91724);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111694);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124648);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132475);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138516(s, out),
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
									super::push(out, 111695);
									s.is_empty()
								}
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
			super::push(out, 37259);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 37260);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 65401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37261);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37262);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65403);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91725);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111696);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124649);
											s.is_empty()
										}
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
					super::push(out, 65404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 37263);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65405);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 37264);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65406);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91728);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111697);
									s.is_empty()
								}
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
			super::push(out, 37265);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 37266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65407);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 65408);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтер.
pub(crate) fn n14986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14908);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37267);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65409);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65410);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37268);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65411);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91732);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 111699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65412);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 65413);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтер.
pub(crate) fn n14987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14909);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37269);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65414);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 65415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37270);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытер.
pub(crate) fn n14988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14910);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 37271);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65417);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтер.
pub(crate) fn n14989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14911);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65418);
					match s {
						[] => true,
						['г', 'т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111700);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124650);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 132476);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 111701);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 124651);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132477);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65419);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91736);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этер.
pub(crate) fn n14990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14912);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 37273);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65420);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91737);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютер.
pub(crate) fn n14991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14913);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 37274);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65421);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91738);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111704);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124652);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132478);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138517(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65422);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятер.
pub(crate) fn n14992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14914);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37275);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 65423);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91740);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -брандмауер.
pub(crate) fn n133841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132479);
	s.is_empty()
}

/// Words ending -ерфер.
pub(crate) fn n37653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37302);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 65447);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 91754);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 111715);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124658);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 132480);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137102);
													s.is_empty()
												}
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
							super::push(out, 111716);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124659);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132481);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137103);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n141322(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 91755);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 111717);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124660);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 132482);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137104);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сфер.
pub(crate) fn n15005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14927);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37303);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 37304);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65449);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91757);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уфер.
pub(crate) fn n15006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14928);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65450);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 65451);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 91758);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 111719);
									s.is_empty()
								}
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
			super::push(out, 37306);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65452);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91759);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111720);
									s.is_empty()
								}
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
			super::push(out, 37307);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 37308);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 37309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65455);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ффер.
pub(crate) fn n15007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14929);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37310);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 65457);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37311);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 65458);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льфер.
pub(crate) fn n37663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37312);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 65459);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 91762);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зенкрейцер.
pub(crate) fn n133845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132483);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -спериенцер.
pub(crate) fn n133846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132484);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137106);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эрофинишер.
pub(crate) fn n133847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132485);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кстерьер.
pub(crate) fn n92782(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 111744);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 111745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 124670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льтерьер.
pub(crate) fn n112929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111746);
	s.is_empty()
}

/// Words ending -урьер.
pub(crate) fn n37730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37377);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 65531);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -асьер.
pub(crate) fn n15051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14973);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 37378);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65532);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тьер.
pub(crate) fn n15052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14974);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37379);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 37380);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 65534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фьер.
pub(crate) fn n15053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14975);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 37381);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65535);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91809);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111747);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -интервьюер.
pub(crate) fn n133848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132486);
	s.is_empty()
}

/// Words ending -ом-сувенир.
pub(crate) fn n133850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132488);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 137109);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 139883);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ес-сувенир.
pub(crate) fn n133851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132489);
	s.is_empty()
}

/// Words ending -ет-сувенир.
pub(crate) fn n133852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132490);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137110);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 139884);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-ампир.
pub(crate) fn n133853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132491);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137111);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 139885);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лер-кассир.
pub(crate) fn n133854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132492);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137112);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139886);
					match s {
						[] => true,
						['т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 141520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ньюгемпшир.
pub(crate) fn n133855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132493);
	s.is_empty()
}

/// Words ending -ью-гэмпшир.
pub(crate) fn n133856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132494);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омерсетшир.
pub(crate) fn n133857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132495);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онкистадор.
pub(crate) fn n133860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132498);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137115);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word исидор.
pub(crate) fn n92899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91923);
	s.is_empty()
}

/// Words ending -ссидор.
pub(crate) fn n66382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65706);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 91924);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 111796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тидор.
pub(crate) fn n37934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37578);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 65707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уидор.
pub(crate) fn n37935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37579);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 65708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хидор.
pub(crate) fn n37936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37580);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 65709);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91925);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111797);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124690);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кдор.
pub(crate) fn n15217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15134);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 37581);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65710);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдор.
pub(crate) fn n15218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15135);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37582);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 65711);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндор.
pub(crate) fn n15219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15136);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37583);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65712);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91927);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111798);
									s.is_empty()
								}
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
			super::push(out, 37584);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 65714);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91929);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111799);
									s.is_empty()
								}
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
			super::push(out, 37585);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 65715);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 91930);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111800);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 124691);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132500);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138532(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37586);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65716);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91931);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111801);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124692);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132501);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одор.
pub(crate) fn n15220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15137);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37587);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 65718);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 65719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37588);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65720);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91934);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111802);
									s.is_empty()
								}
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
			super::push(out, 37589);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91935);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111803);
									s.is_empty()
								}
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
			super::push(out, 37590);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65722);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 65723);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91936);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111804);
									s.is_empty()
								}
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
			super::push(out, 37591);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65724);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдор.
pub(crate) fn n15221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15138);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 37592);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдор.
pub(crate) fn n15222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15139);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 37593);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65726);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тдор.
pub(crate) fn n15223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15140);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 37594);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 65727);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -амбурмажор.
pub(crate) fn n133864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132502);
	s.is_empty()
}

/// Words ending -ейр-эз-зор.
pub(crate) fn n133865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132503);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 137117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рмаконадзор.
pub(crate) fn n133866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132504);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137118);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 139889);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ер-ревизор.
pub(crate) fn n133867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132505);
	match s {
		[] => true,
		['л', 'о', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
			super::push(out, 137119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озор.
pub(crate) fn n15239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15156);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 37615);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 37616);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 37617);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 37618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -узор.
pub(crate) fn n15240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15157);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 37619);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 65749);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91959);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туальденор.
pub(crate) fn n133868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132506);
	s.is_empty()
}

/// Words ending -мисдиминор.
pub(crate) fn n133869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132507);
	s.is_empty()
}

/// Words ending -ство-донор.
pub(crate) fn n133870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132508);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137120);
			match s {
				[] => true,
				['а', 'д', 'у', 'с', 'о', 'г', '\u{1}', s @ ..] => {
					super::push(out, 139890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -а-агрессор.
pub(crate) fn n133871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132509);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137121);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139891);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141521);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 142471);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142991);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -о-агрессор.
pub(crate) fn n133872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132510);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 137122);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 139892);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141522);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142472);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142992);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прогрессор.
pub(crate) fn n133873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132511);
	s.is_empty()
}

/// Words ending -усор.
pub(crate) fn n15314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15231);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 37735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65844);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 65845);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 92015);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111849);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124714);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 92016);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 111850);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124715);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 132512);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атор.
pub(crate) fn n15315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15232);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37736);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65846);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 92017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 65847);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92018);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111852);
									s.is_empty()
								}
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
			super::push(out, 37737);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65848);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92019);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111853);
									match s {
										[] => true,
										['к', 'э', '-', s @ ..] => {
											super::push(out, 124716);
											match s {
												[] => true,
												['н', 'а', s @ ..] => {
													super::push(out, 132513);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138539(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65849);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 92020);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 111854);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 124717);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132514);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138540(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65850);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111855);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124718);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 111856);
									match s {
										[] => true,
										['л', 'у', 'к', 'е', s @ ..] => {
											super::push(out, 124719);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132515);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138541(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65851);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 92021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 65852);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65853);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92023);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											super::push(out, 124720);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 124721);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132516);
													match s {
														[] => true,
														['к', s @ ..] => super::t48::n138542(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37738);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65854);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 111857);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 111858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 65855);
					match s {
						[] => true,
						['в', 'а', 'н', 'о', s @ ..] => {
							super::push(out, 92024);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111859);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124722);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 111860);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 124723);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => super::t48::n93003(s, out),
						['м', s @ ..] => super::t48::n93004(s, out),
						['т', s @ ..] => super::t48::n93005(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t48::n66534(s, out),
				['о', s @ ..] => super::t48::n66535(s, out),
				['ь', s @ ..] => super::t48::n66536(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t48::n38096(s, out),
		['е', s @ ..] => super::t48::n38097(s, out),
		['з', s @ ..] => super::t48::n38098(s, out),
		['и', s @ ..] => super::t48::n38099(s, out),
		['к', s @ ..] => super::t48::n38100(s, out),
		['л', s @ ..] => super::t48::n38101(s, out),
		['м', s @ ..] => super::t48::n38102(s, out),
		['н', s @ ..] => super::t48::n38103(s, out),
		['п', s @ ..] => super::t48::n38104(s, out),
		['р', s @ ..] => super::t48::n38105(s, out),
		['с', s @ ..] => super::t48::n38106(s, out),
		['т', s @ ..] => super::t48::n38107(s, out),
		['у', s @ ..] => super::t48::n38108(s, out),
		['ф', s @ ..] => super::t48::n38109(s, out),
		['ш', s @ ..] => super::t48::n38110(s, out),
		['ю', s @ ..] => {
			super::push(out, 37753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -экспедитор.
pub(crate) fn n134007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132639);
	s.is_empty()
}

/// Words ending -к-кредитор.
pub(crate) fn n134008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132640);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137214);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139948);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сокредитор.
pub(crate) fn n134009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132641);
	s.is_empty()
}

/// Words ending -йтор.
pub(crate) fn n15319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15236);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37766);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65938);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 92174);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112018);
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
						['е', s @ ..] => {
							super::push(out, 92175);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 112019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 92176);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 112020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 92177);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112021);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124895);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктор.
pub(crate) fn n15320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15237);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65939);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 65940);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92178);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 124896);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132642);
													s.is_empty()
												}
												['ф', s @ ..] => {
													super::push(out, 132643);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 124897);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132644);
													match s {
														[] => true,
														['д', s @ ..] => super::t48::n138634(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 132645);
													match s {
														[] => true,
														['д', s @ ..] => super::t48::n138635(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 132646);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 124898);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132647);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65941);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 92179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 65942);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92180);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65943);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112023);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 112024);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 112025);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124899);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132648);
													match s {
														[] => true,
														['з', s @ ..] => super::t48::n138636(s, out),
														_ => false,
													}
												}
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
							super::push(out, 92181);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 65944);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92182);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 65945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92183);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 92184);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 112028);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 112029);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 124900);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 132649);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138637(s, out),
														_ => false,
													}
												}
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
							super::push(out, 92185);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112030);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124901);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132650);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t48::n38126(s, out),
		['и', s @ ..] => {
			super::push(out, 37769);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 65958);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 65959);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112058);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124920);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 112059);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124921);
											s.is_empty()
										}
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
					super::push(out, 65960);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65961);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 92221);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112060);
									match s {
										[] => true,
										['н', s @ ..] => super::t48::n126218(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t48::n38128(s, out),
		['о', s @ ..] => super::t48::n38129(s, out),
		['у', s @ ..] => super::t48::n38130(s, out),
		['э', s @ ..] => super::t48::n38131(s, out),
		_ => false,
	}
}

/// Words ending -лтор.
pub(crate) fn n15321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15238);
	match s {
		[] => true,
		['е', 'и', 'р', '\u{1}', s @ ..] => {
			super::push(out, 37774);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 37775);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65968);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 92229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтор.
pub(crate) fn n15322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15239);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37776);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65971);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92231);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112066);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124928);
											s.is_empty()
										}
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
					super::push(out, 65972);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 65973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 37777);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отор.
pub(crate) fn n15323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15240);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 37778);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65975);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92233);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112067);
									s.is_empty()
								}
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
			super::push(out, 37779);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 37780);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65977);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							super::push(out, 92234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112068);
									s.is_empty()
								}
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
			super::push(out, 37781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птор.
pub(crate) fn n15324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15241);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37782);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65979);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 92235);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 112069);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124929);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132675);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37783);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 65980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'к', 'с', 'н', s @ ..] => {
			super::push(out, 37784);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65981);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 92236);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112070);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124930);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37785);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65982);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 92237);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112071);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124931);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртор.
pub(crate) fn n15325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15242);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37786);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65983);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стор.
pub(crate) fn n15326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37787);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65984);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92238);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 112072);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124932);
											s.is_empty()
										}
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
					super::push(out, 65985);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 65986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37788);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65987);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92240);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 92241);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 65988);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 92242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 65989);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 65990);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92243);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112074);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 124933);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37789);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 65991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37790);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65992);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92244);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112075);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124934);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37791);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65993);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 92245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттор.
pub(crate) fn n15327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15244);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37792);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утор.
pub(crate) fn n15328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15245);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37793);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 37794);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65995);
					match s {
						[] => true,
						['р', 'т', 'с', s @ ..] => {
							super::push(out, 92246);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112076);
									s.is_empty()
								}
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
			super::push(out, 37795);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65996);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 92247);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112077);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 124935);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37796);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 37797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтор.
pub(crate) fn n15329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15246);
	s.is_empty()
}

/// Words ending -ютор.
pub(crate) fn n15330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15247);
	match s {
		[] => true,
		['ъ', s @ ..] => {
			super::push(out, 37798);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 65998);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 37799);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65999);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 92249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятор.
pub(crate) fn n15331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15248);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37800);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 66000);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 92250);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112078);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124936);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132676);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138655(s, out),
														_ => false,
													}
												}
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
							super::push(out, 92251);
							match s {
								[] => true,
								['м', 'о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 112079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 92252);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112080);
									match s {
										[] => true,
										['е', 'в', '-', s @ ..] => {
											super::push(out, 124937);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132677);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138656(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 66001);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92253);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112081);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124938);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132678);
													s.is_empty()
												}
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
							super::push(out, 92254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 66002);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 66003);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92255);
							match s {
								[] => true,
								['а', 'р', 'т', 'е', s @ ..] => {
									super::push(out, 112082);
									match s {
										[] => true,
										['р', '-', s @ ..] => {
											super::push(out, 124939);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132679);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138657(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t48::n66695(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word христофор.
pub(crate) fn n134056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132687);
	s.is_empty()
}

/// Words ending -метр.
pub(crate) fn n15369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15286);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 37844);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 66038);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 92281);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112103);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124952);
											s.is_empty()
										}
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
					super::push(out, 66039);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92282);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112104);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 124953);
											s.is_empty()
										}
										_ => false,
									}
								}
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
					super::push(out, 66040);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 92283);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112105);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124954);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 132690);
													s.is_empty()
												}
												_ => false,
											}
										}
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
							super::push(out, 92284);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 92285);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112106);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 124955);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132691);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 66041);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92286);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112107);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124956);
											s.is_empty()
										}
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
					super::push(out, 66042);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 92287);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112108);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124957);
											s.is_empty()
										}
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
					super::push(out, 66043);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 66044);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 66045);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 92289);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112109);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124958);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37845);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 66046);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92290);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112110);
									s.is_empty()
								}
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
			super::push(out, 37846);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 66047);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92291);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112111);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 124959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 92292);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 112112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', '\u{1}', s @ ..] => {
					super::push(out, 66048);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 66049);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92293);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112113);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 112114);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124960);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132692);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n138668(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 66050);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92294);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 92295);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112115);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 124961);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t48::n66743(s, out),
				['р', s @ ..] => super::t48::n66744(s, out),
				['с', s @ ..] => super::t48::n66745(s, out),
				['т', s @ ..] => super::t48::n66746(s, out),
				['ц', s @ ..] => {
					super::push(out, 66054);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92310);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t48::n38208(s, out),
		['м', s @ ..] => super::t48::n38209(s, out),
		['о', s @ ..] => super::t48::n38210(s, out),
		['р', s @ ..] => super::t48::n38211(s, out),
		['с', s @ ..] => super::t48::n38212(s, out),
		['т', s @ ..] => super::t48::n38213(s, out),
		['у', s @ ..] => super::t48::n38214(s, out),
		_ => false,
	}
}

/// The word зе-министр.
pub(crate) fn n134079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132710);
	s.is_empty()
}

/// Words ending -дом-монстр.
pub(crate) fn n134080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132711);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137258);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омонстр.
pub(crate) fn n93356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92360);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 112171);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 124995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137259);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ростр.
pub(crate) fn n38240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37879);
	s.is_empty()
}

/// Words ending -пстр.
pub(crate) fn n15388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15305);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37880);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эстр.
pub(crate) fn n15389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15306);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юстр.
pub(crate) fn n15390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15307);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ястр.
pub(crate) fn n15391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15308);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атух-пондур.
pub(crate) fn n138679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137260);
	s.is_empty()
}

/// Words ending -уть-теллур.
pub(crate) fn n134083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132714);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137261);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word винтертур.
pub(crate) fn n134084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132715);
	s.is_empty()
}

/// Words ending -ур-мультур.
pub(crate) fn n134085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132716);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бальнинкас.
pub(crate) fn n134088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132719);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тамаулипас.
pub(crate) fn n134089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132720);
	s.is_empty()
}

/// Words ending -одиноцерас.
pub(crate) fn n134090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132721);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -альхесирас.
pub(crate) fn n134091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132722);
	s.is_empty()
}

/// Words ending -альдеритас.
pub(crate) fn n134092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132723);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -машино-час.
pub(crate) fn n134093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132724);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137267);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word мегаватт-час.
pub(crate) fn n134094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132725);
	s.is_empty()
}

/// Words ending -тильбоидес.
pub(crate) fn n134095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132726);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -с-анджелес.
pub(crate) fn n134096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132727);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137269);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -к-геркулес.
pub(crate) fn n134097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132728);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 137270);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 139978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -айндфулнес.
pub(crate) fn n134098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132729);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -меланерпес.
pub(crate) fn n134099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132730);
	s.is_empty()
}

/// Words ending -амилитарес.
pub(crate) fn n134100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132731);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бертадорес.
pub(crate) fn n134101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132732);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -коккомицес.
pub(crate) fn n134102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132733);
	s.is_empty()
}

/// Words ending -элафомицес.
pub(crate) fn n134103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132734);
	s.is_empty()
}

/// Words ending -метагенезис.
pub(crate) fn n134106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132737);
	s.is_empty()
}

/// Words ending -оциогенезис.
pub(crate) fn n134107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132738);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -илогенезис.
pub(crate) fn n134108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132739);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атогенезис.
pub(crate) fn n134109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132740);
	s.is_empty()
}

/// Words ending -ниверсалис.
pub(crate) fn n134110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132741);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 137278);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иннеаполис.
pub(crate) fn n134111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132742);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ополис.
pub(crate) fn n67182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66487);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 92560);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 112273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 92561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 112274);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 125051);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132743);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 137280);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 92562);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 112275);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 125052);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132744);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137281);
											s.is_empty()
										}
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
					super::push(out, 112276);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 125053);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 112277);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 125054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 112278);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 125055);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 132746);
									s.is_empty()
								}
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
			super::push(out, 92563);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 112279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улис.
pub(crate) fn n15780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15695);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 38371);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 66488);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 92564);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112280);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125057);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 38372);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 66489);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флис.
pub(crate) fn n15781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15696);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 38373);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 66490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элис.
pub(crate) fn n15782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15697);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38374);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эпидидимис.
pub(crate) fn n134116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132747);
	s.is_empty()
}

/// Words ending -есперорнис.
pub(crate) fn n134117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132748);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 137282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стеролепис.
pub(crate) fn n134118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132749);
	s.is_empty()
}

/// Words ending -рхеоптерис.
pub(crate) fn n134119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132750);
	s.is_empty()
}

/// Words ending -оссоптерис.
pub(crate) fn n134120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132751);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137283);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 139982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танакласис.
pub(crate) fn n134121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132752);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -надиплосис.
pub(crate) fn n134122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132753);
	s.is_empty()
}

/// Words ending -импотевтис.
pub(crate) fn n134123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132754);
	s.is_empty()
}

/// The word мегаплекс.
pub(crate) fn n134126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132757);
	s.is_empty()
}

/// Words ending -ультиплекс.
pub(crate) fn n134127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132758);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гуголплекс.
pub(crate) fn n134128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132759);
	s.is_empty()
}

/// Words ending --симплекс.
pub(crate) fn n126382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125084);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 132760);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137286);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 139983);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 141581);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омплекс.
pub(crate) fn n93633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92636);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 125085);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 132761);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137287);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139984);
											s.is_empty()
										}
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
					super::push(out, 125086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руплекс.
pub(crate) fn n67295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66600);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 92637);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 112326);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 125087);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 132762);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флекс.
pub(crate) fn n38855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38485);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 66601);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 92638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 66602);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 66603);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92639);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 112327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 66604);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 92640);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word мекс.
pub(crate) fn n38856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38486);
	s.is_empty()
}

/// Words ending -емекс.
pub(crate) fn n38857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38487);
	s.is_empty()
}

/// Words ending -пекс.
pub(crate) fn n15859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15773);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38488);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рекс.
pub(crate) fn n15860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15774);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38489);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 38490);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 38491);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 38492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -секс.
pub(crate) fn n15861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15775);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 38493);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 66606);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 38494);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 66607);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 92643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38495);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 66608);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 92644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 38496);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 66609);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38497);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 66610);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 92646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 38498);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 66611);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 92647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 66612);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -текс.
pub(crate) fn n15862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15776);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38499);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 66613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38500);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 66614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 38501);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 66615);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 92648);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112329);
									s.is_empty()
								}
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
			super::push(out, 38502);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 92649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 92650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 38503);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 66616);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 66617);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 92652);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 112331);
									s.is_empty()
								}
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
			super::push(out, 38504);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 66618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фекс.
pub(crate) fn n15863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15777);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66619);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цекс.
pub(crate) fn n15864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15778);
	s.is_empty()
}

/// Words ending -оминатрикс.
pub(crate) fn n134132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132763);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 137288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фикс.
pub(crate) fn n15874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15788);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38522);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 66638);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 92666);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 112341);
									s.is_empty()
								}
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
			super::push(out, 38523);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 66639);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 92667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 66640);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 66641);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92668);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112343);
									s.is_empty()
								}
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
			super::push(out, 38524);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66642);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 66643);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 66644);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 92669);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 66645);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 66646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 38525);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 66647);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 92670);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112345);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125094);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 38526);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 66648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 66649);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 66650);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 92672);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 66651);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 92673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 38528);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 66652);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 38529);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 66653);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92675);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112347);
									s.is_empty()
								}
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
			super::push(out, 38530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 66654);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 66655);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 92676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кс-ту-бокс.
pub(crate) fn n134133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132764);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137289);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 139985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экспириенс.
pub(crate) fn n134137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132768);
	s.is_empty()
}

/// Words ending -экстрасенс.
pub(crate) fn n134138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132769);
	s.is_empty()
}

/// Words ending -галактикос.
pub(crate) fn n134140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132771);
	s.is_empty()
}

/// Words ending -омпрачикос.
pub(crate) fn n134141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132772);
	s.is_empty()
}

/// Words ending -скотоволос.
pub(crate) fn n134142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132773);
	s.is_empty()
}

/// Words ending -жка-термос.
pub(crate) fn n134143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132774);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 137291);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139986);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 141582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аромеринос.
pub(crate) fn n134144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132775);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 137292);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139987);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гонос.
pub(crate) fn n39074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38702);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 66787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -донос.
pub(crate) fn n39075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38703);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 66788);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 66789);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92766);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 112392);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 125118);
									s.is_empty()
								}
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
			super::push(out, 66790);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 92767);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'в', '\u{1}', s @ ..] => {
			super::push(out, 66791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -конос.
pub(crate) fn n39076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38704);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 66792);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 66793);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 92769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 66794);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 66795);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 92771);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 112394);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонос.
pub(crate) fn n39077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38705);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 66796);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 92772);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 112395);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125120);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 132776);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 137293);
													match s {
														[] => true,
														['к', s @ ..] => super::t48::n141429(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 66797);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92773);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 112396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монос.
pub(crate) fn n39078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38706);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 66798);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мпонос.
pub(crate) fn n39079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38707);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 66799);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронос.
pub(crate) fn n39080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38708);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 66800);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 66801);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 66802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 92776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 66803);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 66804);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 92777);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонос.
pub(crate) fn n39081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38709);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 66805);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 92778);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112398);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125121);
									s.is_empty()
								}
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
			super::push(out, 66806);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 92779);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 112399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 66807);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92780);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 112400);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125122);
									s.is_empty()
								}
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
			super::push(out, 66808);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 66809);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92782);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хонос.
pub(crate) fn n39082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38710);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 66810);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 92783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пнос.
pub(crate) fn n16013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15926);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 38711);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 66811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнос.
pub(crate) fn n16014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15927);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 38712);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 66812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снос.
pub(crate) fn n16015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15928);
	s.is_empty()
}

/// Words ending -тнос.
pub(crate) fn n16016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15929);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 38713);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 38714);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66813);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 66814);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унос.
pub(crate) fn n16017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15930);
	s.is_empty()
}

/// Words ending -хнос.
pub(crate) fn n16018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15931);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 38715);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 66815);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 92786);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынос.
pub(crate) fn n16019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15932);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38716);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ведливорос.
pub(crate) fn n134146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132777);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -прос.
pub(crate) fn n16034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15947);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38744);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 38745);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 38746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трос.
pub(crate) fn n16035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15948);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38747);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66845);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 92805);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112409);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125126);
											s.is_empty()
										}
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
					super::push(out, 66846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 38748);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 66847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 38749);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 66848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38750);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 66849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38751);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 66850);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урос.
pub(crate) fn n16036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15949);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 38752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 38753);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66852);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрос.
pub(crate) fn n16037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15950);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 38754);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 66853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрос.
pub(crate) fn n16038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15951);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -от-пылесос.
pub(crate) fn n134147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132778);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 137295);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ра-медосос.
pub(crate) fn n134148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132779);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 137296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139990);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 141584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полиморсос.
pub(crate) fn n134149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132780);
	s.is_empty()
}

/// Words ending -ьяс-а-дьос.
pub(crate) fn n134150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132781);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137297);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ик-конкурс.
pub(crate) fn n134153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132784);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137299);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 139992);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 141585);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 142518);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 143029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тр-конкурс.
pub(crate) fn n134154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132785);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137300);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 139993);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерс-тауэрс.
pub(crate) fn n134155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132786);
	s.is_empty()
}

/// The word поход-кросс.
pub(crate) fn n141435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139994);
	s.is_empty()
}

/// Words ending -ейбус-автобус.
pub(crate) fn n141436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139995);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдин-ангус.
pub(crate) fn n134159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132790);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -традивариус.
pub(crate) fn n134160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132791);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гомункулус.
pub(crate) fn n134161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132792);
	s.is_empty()
}

/// Words ending -ралиссимус.
pub(crate) fn n134162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132793);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аиниссимус.
pub(crate) fn n134163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132794);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137306);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139996);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 141588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аптенармус.
pub(crate) fn n134164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132795);
	s.is_empty()
}

/// Words ending -плюс-минус.
pub(crate) fn n134165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132796);
	s.is_empty()
}

/// Words ending -гиппокампус.
pub(crate) fn n134166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132797);
	s.is_empty()
}

/// Words ending -ротогиппус.
pub(crate) fn n134167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132798);
	s.is_empty()
}

/// Words ending -блок-корпус.
pub(crate) fn n134168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132799);
	s.is_empty()
}

/// Words ending -нотоптерус.
pub(crate) fn n134169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132800);
	s.is_empty()
}

/// The word ротавирус.
pub(crate) fn n134170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132801);
	s.is_empty()
}

/// Words ending -аурус.
pub(crate) fn n39411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39037);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 67082);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92944);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 112493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мурус.
pub(crate) fn n39412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39038);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 67083);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 92945);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 112494);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рурус.
pub(crate) fn n39413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39039);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 67084);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92946);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 112495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъярус.
pub(crate) fn n39414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39040);
	s.is_empty()
}

/// Words ending -олеостефус.
pub(crate) fn n134171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132802);
	s.is_empty()
}

/// Words ending -грязнохват.
pub(crate) fn n134174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132805);
	s.is_empty()
}

/// Words ending -ыба-солдат.
pub(crate) fn n134175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132806);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уперсолдат.
pub(crate) fn n134176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132807);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аудофовеат.
pub(crate) fn n134177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132808);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вка-плакат.
pub(crate) fn n134178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132809);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137312);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 139999);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -магнификат.
pub(crate) fn n134179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132810);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --ректификат.
pub(crate) fn n134180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132811);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137314);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140000);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141591);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 142520);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 143030);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -выкат.
pub(crate) fn n39591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39215);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 67247);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атье-халат.
pub(crate) fn n134181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132812);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137315);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ат-перемат.
pub(crate) fn n134182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132813);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оклимат.
pub(crate) fn n94072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93075);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 112559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олимат.
pub(crate) fn n67987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67289);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 93076);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 112560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слимат.
pub(crate) fn n67988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67290);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 93077);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 112561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -римат.
pub(crate) fn n39626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39250);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 67291);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 93078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тимат.
pub(crate) fn n39627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39251);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 67292);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 93079);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 93080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 67293);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93081);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -химат.
pub(crate) fn n39628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39252);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 67294);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93082);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нмат.
pub(crate) fn n16395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16301);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39253);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67295);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 93083);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112562);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125189);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омат.
pub(crate) fn n16396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16302);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 39254);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 39255);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 39256);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 67296);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93084);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 112563);
									s.is_empty()
								}
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
			super::push(out, 39257);
			match s {
				[] => true,
				['х', 'и', s @ ..] => {
					super::push(out, 67297);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 39258);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 67298);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 112564);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125190);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132814);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n138736(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 125191);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132815);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138737(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 125192);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132816);
													match s {
														[] => true,
														['н', s @ ..] => super::t48::n138738(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 125193);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 112565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмат.
pub(crate) fn n16397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16303);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39259);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 67299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39260);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 67300);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хмат.
pub(crate) fn n16399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16305);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39261);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67301);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 67302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 67303);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93087);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 112566);
									s.is_empty()
								}
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
			super::push(out, 39262);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шмат.
pub(crate) fn n16400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16306);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рит-гранат.
pub(crate) fn n134186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132817);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137320);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140005);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 141595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word антифанат.
pub(crate) fn n134187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132818);
	s.is_empty()
}

/// Words ending -агнат.
pub(crate) fn n39651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39275);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 67321);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93105);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 112576);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125198);
									s.is_empty()
								}
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
			super::push(out, 67322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -игнат.
pub(crate) fn n39652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39276);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 67323);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 67324);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 93106);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 93107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енат.
pub(crate) fn n16404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16310);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			super::push(out, 39278);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 39279);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 39280);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67325);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инат.
pub(crate) fn n16405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16311);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39281);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67326);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93109);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 125199);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 132819);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138740(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 125200);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39282);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 67327);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112577);
									s.is_empty()
								}
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
			super::push(out, 39283);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 39284);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67328);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112578);
									s.is_empty()
								}
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
			super::push(out, 39285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67329);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 67330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 39286);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67331);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93113);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112579);
									s.is_empty()
								}
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
			super::push(out, 39287);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93114);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 93115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 67333);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 67334);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93117);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 67335);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93118);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112581);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 112582);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125201);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132820);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 39288);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 67336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 39289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 39290);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67338);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112583);
									s.is_empty()
								}
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
			super::push(out, 39291);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67339);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 39292);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 67340);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93121);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112584);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125202);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39293);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67341);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ннат.
pub(crate) fn n16406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16312);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39294);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 67342);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112586);
									s.is_empty()
								}
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
			super::push(out, 39295);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 67343);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93124);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112587);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125203);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39296);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67344);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онат.
pub(crate) fn n16407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16313);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39297);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 39298);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 39299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнат.
pub(crate) fn n16408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16314);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39300);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 67345);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93125);
							match s {
								[] => true,
								['и', '-', s @ ..] => {
									super::push(out, 112588);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125204);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132821);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138741(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 125205);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132822);
													match s {
														[] => true,
														['д', s @ ..] => super::t48::n138742(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унат.
pub(crate) fn n16409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16315);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39301);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 39302);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67346);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word джавгарат.
pub(crate) fn n134192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132823);
	s.is_empty()
}

/// Words ending -дерат.
pub(crate) fn n39712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39336);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 67374);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 93141);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112601);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125210);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132824);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 67375);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 93142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лерат.
pub(crate) fn n39713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39337);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 67376);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 93143);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 112602);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мерат.
pub(crate) fn n39714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39338);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 67377);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93144);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 112603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерат.
pub(crate) fn n39715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39339);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 67378);
			match s {
				[] => true,
				['г', 'е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 112604);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 112605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ферат.
pub(crate) fn n39716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39340);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 67379);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церат.
pub(crate) fn n39717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39341);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 67380);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 93146);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 112606);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132825);
											s.is_empty()
										}
										['о', 'р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 132826);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зрат.
pub(crate) fn n16429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16334);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39342);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 67381);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 67382);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ират.
pub(crate) fn n16430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16335);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39343);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67383);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93147);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 93148);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 39344);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67384);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 39345);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67385);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93150);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112607);
									s.is_empty()
								}
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
			super::push(out, 39346);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 39347);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					super::push(out, 67387);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 67388);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 39348);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67389);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 67390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 39349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67391);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 67392);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 67393);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 39350);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67394);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 93154);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 93155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 39351);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 67395);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 93156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 39352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67396);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 67397);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112608);
									s.is_empty()
								}
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
			super::push(out, 39353);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67398);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йрат.
pub(crate) fn n16431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16336);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39354);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 67399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крат.
pub(crate) fn n16432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16337);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39356);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 39357);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 67400);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 67401);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93160);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112610);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125213);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39358);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 67402);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39359);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67403);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 67404);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 67405);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мрат.
pub(crate) fn n16433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16338);
	s.is_empty()
}

/// Words ending -орат.
pub(crate) fn n16434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16339);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 39360);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 39361);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 67406);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ррат.
pub(crate) fn n16435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16340);
	s.is_empty()
}

/// Words ending -срат.
pub(crate) fn n16436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16341);
	s.is_empty()
}

/// Words ending -трат.
pub(crate) fn n16437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16342);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39362);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 67407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 39363);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 67408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 39364);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 39365);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67410);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93162);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 112611);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125214);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132827);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n138743(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 67411);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93163);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 67412);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 67413);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93164);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112613);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 93165);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112614);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'р', 'е', 'г', '\u{1}', s @ ..] => {
					super::push(out, 67414);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 67415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 39366);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 67416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урат.
pub(crate) fn n16438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16343);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39367);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 39368);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 39369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67417);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 67418);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93166);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 112615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 67419);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93167);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112616);
									s.is_empty()
								}
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
			super::push(out, 39370);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 67420);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 39371);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 67421);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 67422);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 93170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 67423);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 39372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67424);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 67425);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93172);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 112617);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125217);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 125218);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132828);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 39373);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67426);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 67427);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрат.
pub(crate) fn n16439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16344);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 39374);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полубархат.
pub(crate) fn n134198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132829);
	s.is_empty()
}

/// Words ending -атье-жакет.
pub(crate) fn n134203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132834);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137328);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140012);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умка-пакет.
pub(crate) fn n134204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132835);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137329);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 140013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранд-кокет.
pub(crate) fn n134205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132836);
	s.is_empty()
}

/// Words ending -орэкет.
pub(crate) fn n68248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67550);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 93261);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 112665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 125238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильм-балет.
pub(crate) fn n134206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132837);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атье-жилет.
pub(crate) fn n134207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132838);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137331);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -букмарклет.
pub(crate) fn n134208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132839);
	s.is_empty()
}

/// Words ending -бом-буклет.
pub(crate) fn n134209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132840);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 137332);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140015);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллет.
pub(crate) fn n16548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16453);
	match s {
		[] => true,
		['о', 'к', s @ ..] => {
			super::push(out, 39519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млет.
pub(crate) fn n16549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16454);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39520);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 67566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нлет.
pub(crate) fn n16550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16455);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39522);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 67567);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олет.
pub(crate) fn n16551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16456);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39523);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67568);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 67569);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 67570);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 93275);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112673);
									s.is_empty()
								}
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
			super::push(out, 39524);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 67571);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93276);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112674);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125245);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', '\u{1}', s @ ..] => {
			super::push(out, 39525);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 39526);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 39527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67572);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 112675);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125246);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132841);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n138752(s, out),
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
									super::push(out, 112676);
									s.is_empty()
								}
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
			super::push(out, 39528);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 39529);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 67573);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93277);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112677);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125247);
											s.is_empty()
										}
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
					super::push(out, 67574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 39530);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 67575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 39531);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плет.
pub(crate) fn n16552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16457);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39532);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 67576);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93278);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 67577);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 93279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 67578);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93280);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112679);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125248);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39533);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 67579);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39534);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67580);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 93282);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112680);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125249);
											s.is_empty()
										}
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
					super::push(out, 67581);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 67582);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93283);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112681);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 125250);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39535);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 39536);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 67584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлет.
pub(crate) fn n16554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16459);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39537);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 67585);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 67586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 67587);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39538);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 39539);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улет.
pub(crate) fn n16555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16460);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39540);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 67589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылет.
pub(crate) fn n16557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16462);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 39541);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 67590);
					match s {
						[] => true,
						['о', 'т', 'е', 'л', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
							super::push(out, 93287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 67591);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ет-пулемет.
pub(crate) fn n134211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132842);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137334);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140017);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141602);
							match s {
								[] => true,
								['с', 'и', 'п', '\u{1}', s @ ..] => {
									super::push(out, 142524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-гранатомет.
pub(crate) fn n126553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132843);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 137335);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140018);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 141603);
									match s {
										[] => true,
										['в', 'а', '\u{1}', s @ ..] => {
											super::push(out, 142525);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -афомет.
pub(crate) fn n68302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67604);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 93298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -исмет.
pub(crate) fn n16567(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39559);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 39560);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хмет.
pub(crate) fn n16569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16473);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39561);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67605);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 67606);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93299);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 112691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 67607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 39562);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 67608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шмет.
pub(crate) fn n16570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16474);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -изнес-счет.
pub(crate) fn n134213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132844);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 137336);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word карт-счет.
pub(crate) fn n134214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132845);
	s.is_empty()
}

/// Words ending -скроу-счет.
pub(crate) fn n134215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132846);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 137337);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нкет-фуршет.
pub(crate) fn n134216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132847);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137338);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 140019);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word аффидевит.
pub(crate) fn n134217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132848);
	s.is_empty()
}

/// Words ending -ненадкевит.
pub(crate) fn n134218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132849);
	s.is_empty()
}

/// Words ending -стохолангит.
pub(crate) fn n134219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132850);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137339);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 140020);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141604);
							match s {
								[] => true,
								['л', 'о', 'х', '\u{1}', s @ ..] => {
									super::push(out, 142526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -третлингит.
pub(crate) fn n134220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132851);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 137340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пентландит.
pub(crate) fn n134221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132852);
	s.is_empty()
}

/// Words ending -ермафродит.
pub(crate) fn n134222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132853);
	s.is_empty()
}

/// Words ending -эрудит.
pub(crate) fn n68445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67747);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 93395);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьдит.
pub(crate) fn n16664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16567);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 39722);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67748);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 67749);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93396);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 112741);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125280);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132854);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эдит.
pub(crate) fn n16665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16568);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инк-циппеит.
pub(crate) fn n138760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137341);
	s.is_empty()
}

/// Words ending -во-паразит.
pub(crate) fn n134225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132856);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137342);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140021);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндопаразит.
pub(crate) fn n134226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132857);
	s.is_empty()
}

/// Words ending -ллопаразит.
pub(crate) fn n134227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132858);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -емопаразит.
pub(crate) fn n134228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132859);
	s.is_empty()
}

/// Words ending -втопаразит.
pub(crate) fn n134229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132860);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ктопаразит.
pub(crate) fn n134230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132861);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 137345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -космополит.
pub(crate) fn n134231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132862);
	s.is_empty()
}

/// Words ending -тернебомит.
pub(crate) fn n134232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132863);
	s.is_empty()
}

/// Words ending -домит.
pub(crate) fn n40182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39798);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 67810);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 93443);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 67811);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 93444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оломит.
pub(crate) fn n68512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67812);
	s.is_empty()
}

/// Words ending -ромит.
pub(crate) fn n40184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39800);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 67813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сомит.
pub(crate) fn n40185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39801);
	s.is_empty()
}

/// Words ending -рмит.
pub(crate) fn n16711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16614);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 67814);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93445);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 67815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93446);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 93447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сумит.
pub(crate) fn n16712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16615);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 39802);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кристианит.
pub(crate) fn n134233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132864);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -айдерхенит.
pub(crate) fn n134234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132865);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137347);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 140022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онит.
pub(crate) fn n16719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16622);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39818);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 67833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 39819);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 39820);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 39821);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67835);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93462);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'е', s @ ..] => {
					super::push(out, 67836);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112780);
									s.is_empty()
								}
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
			super::push(out, 39822);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67837);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112781);
									s.is_empty()
								}
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
			super::push(out, 39823);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 39824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орнит.
pub(crate) fn n40212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39826);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 67838);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 93465);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 112782);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125298);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 132866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унит.
pub(crate) fn n16721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16624);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 39827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнит.
pub(crate) fn n16722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16625);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рхипиздрит.
pub(crate) fn n134236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132867);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рхимандрит.
pub(crate) fn n134237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132868);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ретерит.
pub(crate) fn n94483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93482);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 112794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 125304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херит.
pub(crate) fn n40237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39849);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 67864);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 93483);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112795);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125305);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132869);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 137350);
													match s {
														[] => true,
														['ш', s @ ..] => super::t48::n141464(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зрит.
pub(crate) fn n16737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16640);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39850);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 67865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирит.
pub(crate) fn n16738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16641);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 39851);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 67866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'с', s @ ..] => {
			super::push(out, 39852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йрит.
pub(crate) fn n16739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39853);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 67868);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93486);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112796);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125306);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крит.
pub(crate) fn n16740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16643);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39855);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 39856);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 67869);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93487);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 112797);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125307);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132870);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 67870);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93488);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 67871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 39857);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 67872);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орит.
pub(crate) fn n16741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16644);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67873);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93490);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112799);
									s.is_empty()
								}
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
			super::push(out, 39859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67874);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 93491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 39860);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 39861);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 67875);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 93492);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112800);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 125308);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132871);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138770(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'а', s @ ..] => {
			super::push(out, 39862);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 39863);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 67877);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93493);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112801);
									s.is_empty()
								}
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
			super::push(out, 39864);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 39865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -прит.
pub(crate) fn n16742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16645);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 39866);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 39867);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 67878);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93494);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 112802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125309);
											s.is_empty()
										}
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
					super::push(out, 67879);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 93495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ррит.
pub(crate) fn n16743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16646);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39868);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 67880);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93496);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112803);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125310);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132872);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 67881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 39869);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 67882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трит.
pub(crate) fn n16744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16647);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39870);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 39871);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 39872);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 67883);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93498);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112804);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125311);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 132873);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138771(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрит.
pub(crate) fn n16746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16649);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39873);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 67884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 39874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67885);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярит.
pub(crate) fn n16747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16650);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39875);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 67886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 39876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рансвестит.
pub(crate) fn n134243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132874);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лсендорфит.
pub(crate) fn n134244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132875);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137354);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140025);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нитопсихит.
pub(crate) fn n134245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132876);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -надкевичит.
pub(crate) fn n134246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132877);
	s.is_empty()
}

/// Words ending -электрощит.
pub(crate) fn n134247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132878);
	s.is_empty()
}

/// Words ending -ка-клевант.
pub(crate) fn n134253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132884);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 137357);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140027);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141606);
							match s {
								[] => true,
								['с', 'а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 142527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --гигант.
pub(crate) fn n94577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93576);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 112849);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 125332);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 132885);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 137358);
									s.is_empty()
								}
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
			super::push(out, 112850);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 125333);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 132886);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137359);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 140028);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 141607);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огигант.
pub(crate) fn n94579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93578);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 112852);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 125335);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хгигант.
pub(crate) fn n94580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93579);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 112853);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 125336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мигант.
pub(crate) fn n68686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67984);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 93580);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 112854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгант.
pub(crate) fn n40381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39986);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 67985);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 93581);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дант.
pub(crate) fn n16838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16741);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39987);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 39988);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 67986);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93582);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112855);
									s.is_empty()
								}
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
			super::push(out, 39989);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67987);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жант.
pub(crate) fn n16839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16742);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 39990);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зант.
pub(crate) fn n16840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16743);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39991);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 67989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 39992);
			match s {
				[] => true,
				['л', 'а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93583);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 93584);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112856);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125337);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132887);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 67990);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 39993);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 67991);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 39994);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 67992);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иант.
pub(crate) fn n16841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16744);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 39995);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67993);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93588);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 67994);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93589);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112858);
									s.is_empty()
								}
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
			super::push(out, 39996);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67995);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 67996);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93591);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 67997);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93592);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 112860);
									s.is_empty()
								}
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
			super::push(out, 39997);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 67998);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 67999);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 93593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 39998);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 39999);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68000);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68001);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93595);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112861);
									s.is_empty()
								}
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
			super::push(out, 40000);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68002);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 93596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 68003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кант.
pub(crate) fn n16842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16745);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40001);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68004);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 68005);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93597);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112862);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125338);
											s.is_empty()
										}
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
					super::push(out, 68006);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93598);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 112863);
									s.is_empty()
								}
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
			super::push(out, 40002);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93599);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112864);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 112865);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125339);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 112866);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125340);
											s.is_empty()
										}
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
					super::push(out, 68007);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93600);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 112867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 68008);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 93601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93602);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 112868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 93603);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 93604);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 68009);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93605);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 93606);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112872);
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
						['к', s @ ..] => {
							super::push(out, 93607);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112873);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125341);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 132888);
													s.is_empty()
												}
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
							super::push(out, 93608);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 68010);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68011);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 68012);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40003);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68013);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112875);
									s.is_empty()
								}
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
			super::push(out, 40004);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 68014);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 40005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 40006);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 68016);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лант.
pub(crate) fn n16843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16746);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40007);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 68017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 40008);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 40009);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68018);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 68019);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 68020);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93615);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 112876);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125342);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132889);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 40010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 40011);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68023);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40012);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68024);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 40013);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68025);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 68026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'а', '\u{1}', s @ ..] => {
			super::push(out, 40014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мант.
pub(crate) fn n16844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16747);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68027);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 68028);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40016);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68030);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40017);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 68031);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 93620);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112877);
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
						['д', s @ ..] => {
							super::push(out, 93621);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 112878);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 93622);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 112879);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 93623);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112880);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125345);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['и', s @ ..] => {
					super::push(out, 68032);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 93624);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112881);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'ф', 'н', s @ ..] => {
					super::push(out, 68033);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нант.
pub(crate) fn n16845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16748);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40018);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 40019);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68034);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40020);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68035);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93627);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 68036);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93628);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68037);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 93629);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112884);
									match s {
										[] => true,
										['л', '-', 'д', s @ ..] => {
											super::push(out, 125346);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132890);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138779(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40021);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68038);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93630);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93631);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112886);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 93632);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112887);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125348);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 93633);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 93634);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112889);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125349);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132891);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68039);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93635);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112890);
									s.is_empty()
								}
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
			super::push(out, 40022);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68040);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40023);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68041);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пант.
pub(crate) fn n16846(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40024);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 40025);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 68042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			super::push(out, 40026);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 40027);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68043);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93638);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112891);
									s.is_empty()
								}
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
			super::push(out, 40028);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68044);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93639);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рант.
pub(crate) fn n16847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40029);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 40030);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68045);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 93640);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112893);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125350);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132892);
													match s {
														[] => true,
														['б', s @ ..] => super::t48::n138780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 68046);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93641);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'а', '\u{1}', s @ ..] => {
					super::push(out, 68047);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 68048);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93642);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 68049);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93643);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 93644);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112896);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125351);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40031);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68050);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 40032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68051);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 68052);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 40033);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68053);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93647);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 68054);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40034);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 68055);
					s.is_empty()
				}
				['п', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93649);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 93650);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 93651);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112898);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125352);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132893);
													s.is_empty()
												}
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
							super::push(out, 93652);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112899);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125353);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132894);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68056);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93653);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112900);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125354);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93654);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 68059);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93655);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 112902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 68060);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93656);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112903);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 125355);
											s.is_empty()
										}
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
					super::push(out, 68061);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93657);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 68062);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 68063);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68064);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40036);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68065);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 40037);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68067);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93662);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 112905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 68068);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t48::n68778(s, out),
				['т', s @ ..] => super::t48::n68779(s, out),
				_ => false,
			}
		}
		['ф', '\u{1}', s @ ..] => {
			super::push(out, 40038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сант.
pub(crate) fn n16848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16750);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40039);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 68070);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							super::push(out, 93666);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112909);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125359);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132896);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68071);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93667);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112910);
									s.is_empty()
								}
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
			super::push(out, 40040);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 68072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 40041);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68073);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93668);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112911);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125360);
											s.is_empty()
										}
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
					super::push(out, 68074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40042);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68075);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 68076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'е', 'р', 'п', s @ ..] => {
			super::push(out, 40043);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93669);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112912);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125361);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 93670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тант.
pub(crate) fn n16849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16751);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40044);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68077);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93671);
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
				['д', s @ ..] => {
					super::push(out, 68078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 68079);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93673);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112913);
									s.is_empty()
								}
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
			super::push(out, 40045);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68080);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93674);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112914);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125362);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132897);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68081);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93675);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 68082);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93676);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112916);
									s.is_empty()
								}
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
			super::push(out, 40046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93677);
							s.is_empty()
						}
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112917);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125363);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132898);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 112918);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125364);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 93678);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112919);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125365);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 93679);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112920);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 125366);
											s.is_empty()
										}
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
					super::push(out, 68084);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 68085);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 68086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 68087);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93683);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 112921);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125367);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40047);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68088);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 93684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 40048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40049);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68091);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 68092);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93685);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112922);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125368);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132899);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138782(s, out),
														_ => false,
													}
												}
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
							super::push(out, 93686);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 93687);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112924);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125369);
											s.is_empty()
										}
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
					super::push(out, 68093);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 68094);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93689);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 68095);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93690);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112926);
									s.is_empty()
								}
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
			super::push(out, 40051);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68096);
					s.is_empty()
				}
				['м', 'м', s @ ..] => {
					super::push(out, 68097);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93691);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112927);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125370);
											s.is_empty()
										}
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
					super::push(out, 68098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => super::t48::n40454(s, out),
		['ю', s @ ..] => super::t48::n40455(s, out),
		_ => false,
	}
}

/// Words ending -уант.
pub(crate) fn n16850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16752);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 40054);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68103);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 40055);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 40056);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 68104);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фант.
pub(crate) fn n16851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16753);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40057);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 40058);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68106);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93697);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'т', s @ ..] => {
					super::push(out, 68107);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хант.
pub(crate) fn n16852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16754);
	s.is_empty()
}

/// Words ending -чант.
pub(crate) fn n16853(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40059);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 40060);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68108);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -делинквент.
pub(crate) fn n134272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132902);
	s.is_empty()
}

/// Words ending -банк-агент.
pub(crate) fn n134273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132903);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137370);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -контрагент.
pub(crate) fn n134274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132904);
	s.is_empty()
}

/// Words ending -экстрагент.
pub(crate) fn n134275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132905);
	s.is_empty()
}

/// Words ending -егент.
pub(crate) fn n40472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40069);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 68123);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 93710);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112943);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125382);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132906);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 137371);
													match s {
														[] => true,
														['п', s @ ..] => super::t48::n141480(s, out),
														_ => false,
													}
												}
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
							super::push(out, 112944);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 125383);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 132907);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137372);
													match s {
														[] => true,
														['п', s @ ..] => super::t48::n141481(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 93711);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112945);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игент.
pub(crate) fn n40473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40070);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 68124);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93712);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 112946);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгент.
pub(crate) fn n40474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40071);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 68125);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 93713);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112947);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргент.
pub(crate) fn n40475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40072);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 68126);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 93714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 68127);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 93715);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112948);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дент.
pub(crate) fn n16857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16758);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40073);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68128);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40074);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 68129);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93717);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112949);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125388);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 112950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 93718);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112951);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125389);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40075);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 68130);
					match s {
						[] => true,
						['е', 'р', 'п', '-', s @ ..] => {
							super::push(out, 93719);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112952);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125390);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 112953);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125391);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132908);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138792(s, out),
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
									super::push(out, 112954);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125392);
											match s {
												[] => true,
												['э', '\u{1}', s @ ..] => {
													super::push(out, 132909);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68131);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93720);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112955);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125393);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 132910);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138793(s, out),
														_ => false,
													}
												}
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
							super::push(out, 93721);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 68132);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93722);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112957);
									s.is_empty()
								}
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
			super::push(out, 40076);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68133);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93723);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 112958);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125394);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40077);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68134);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93724);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 93725);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 112960);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125395);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 132911);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68135);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 93726);
							match s {
								[] => true,
								['с', 'е', 'р', 'р', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 125396);
											match s {
												[] => true,
												['к', s @ ..] => super::t48::n134282(s, out),
												['н', s @ ..] => super::t48::n134283(s, out),
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 125397);
											match s {
												[] => true,
												['т', s @ ..] => super::t48::n134284(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t48::n40481(s, out),
		['у', s @ ..] => super::t48::n40482(s, out),
		_ => false,
	}
}

/// Words ending -жент.
pub(crate) fn n16858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16759);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 40080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зент.
pub(crate) fn n16859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16760);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40081);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 40082);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68139);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 93729);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112962);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125398);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иент.
pub(crate) fn n16860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16761);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40083);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 112963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 93731);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 112964);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125399);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132915);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138796(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40084);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68141);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93732);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 112965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93733);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 93734);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112967);
									s.is_empty()
								}
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
			super::push(out, 40085);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68142);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 93735);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112968);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125401);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 112969);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125402);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132916);
													match s {
														[] => true,
														['ю', s @ ..] => super::t48::n138797(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40086);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68143);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 93736);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 112970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 68144);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 93737);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112971);
									s.is_empty()
								}
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
			super::push(out, 40087);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 68145);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93738);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 68146);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93739);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 112973);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125403);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132917);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68147);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 93740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 68148);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 93741);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 112974);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 125404);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кент.
pub(crate) fn n16861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16762);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40089);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 40090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лент.
pub(crate) fn n16862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16763);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40091);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 40092);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 40093);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мент.
pub(crate) fn n16863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16764);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40094);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 40095);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93743);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 93744);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112976);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125405);
											s.is_empty()
										}
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
					super::push(out, 68151);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93745);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 112977);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 125406);
											s.is_empty()
										}
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
					super::push(out, 68152);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93746);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 112978);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125407);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132918);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68153);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 93747);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 112979);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 93748);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 112980);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125409);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 93749);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112981);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125410);
											s.is_empty()
										}
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
					super::push(out, 68154);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93750);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112982);
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
						['е', s @ ..] => {
							super::push(out, 93751);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 112983);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 125411);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132919);
													s.is_empty()
												}
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
							super::push(out, 93752);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112984);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 125412);
											s.is_empty()
										}
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
					super::push(out, 68155);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93753);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 112985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 93754);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68156);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93755);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112987);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125413);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132920);
													s.is_empty()
												}
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
							super::push(out, 93756);
							match s {
								[] => true,
								['а', 'п', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125414);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132921);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 132922);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138798(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 125415);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 132923);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138799(s, out),
														['с', s @ ..] => super::t48::n138800(s, out),
														_ => false,
													}
												}
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
							super::push(out, 93757);
							match s {
								[] => true,
								['е', s @ ..] => super::t48::n114195(s, out),
								['о', s @ ..] => super::t48::n114196(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t48::n40499(s, out),
		['д', s @ ..] => super::t48::n40500(s, out),
		['е', s @ ..] => super::t48::n40501(s, out),
		['ж', s @ ..] => super::t48::n40502(s, out),
		['з', s @ ..] => super::t48::n40503(s, out),
		['и', s @ ..] => super::t48::n40504(s, out),
		['й', s @ ..] => super::t48::n40505(s, out),
		['л', s @ ..] => super::t48::n40506(s, out),
		['м', s @ ..] => super::t48::n40507(s, out),
		['н', s @ ..] => super::t48::n40508(s, out),
		['о', s @ ..] => super::t48::n40509(s, out),
		['п', s @ ..] => super::t48::n40510(s, out),
		['р', s @ ..] => super::t48::n40511(s, out),
		['с', s @ ..] => super::t48::n40512(s, out),
		['т', s @ ..] => super::t48::n40513(s, out),
		['у', s @ ..] => super::t48::n40514(s, out),
		['ч', s @ ..] => super::t48::n40515(s, out),
		['ш', s @ ..] => super::t48::n40516(s, out),
		['ь', s @ ..] => super::t48::n40517(s, out),
		_ => false,
	}
}

/// Words ending -нент.
pub(crate) fn n16864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16765);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40115);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40116);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93790);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113017);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125434);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 93791);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113018);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125435);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40117);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93792);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 93793);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113020);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125436);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 93794);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 93795);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 93796);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113023);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 125437);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пент.
pub(crate) fn n16865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16766);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 40118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рент.
pub(crate) fn n16866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16767);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40119);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68199);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 68200);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93798);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 68201);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93799);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113025);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 113026);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 125438);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 132935);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138804(s, out),
														_ => false,
													}
												}
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
							super::push(out, 93800);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 93801);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113027);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 113028);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125439);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40120);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68202);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93802);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113029);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125440);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132936);
													s.is_empty()
												}
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
							super::push(out, 93803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113030);
									s.is_empty()
								}
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
			super::push(out, 40121);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68203);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93804);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113031);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сент.
pub(crate) fn n16867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16768);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40122);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 40123);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68205);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40124);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68206);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тент.
pub(crate) fn n16868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16769);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40125);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 68207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40126);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68208);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93807);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 93808);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', '-', s @ ..] => {
							super::push(out, 93809);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113035);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125442);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132937);
													match s {
														[] => true,
														['б', s @ ..] => super::t48::n138805(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40127);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68209);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40128);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 68210);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93811);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113036);
									s.is_empty()
								}
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
			super::push(out, 40129);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68211);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93812);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113037);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125443);
											s.is_empty()
										}
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
					super::push(out, 68212);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 93813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уент.
pub(crate) fn n16869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16770);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 40130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цент.
pub(crate) fn n16870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16771);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 40131);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68213);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 93814);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113038);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125444);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132938);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40132);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 68214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93815);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 93816);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113039);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125445);
											s.is_empty()
										}
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
					super::push(out, 68215);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 93817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40133);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68216);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 93818);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113040);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125446);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132939);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68217);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 68218);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93819);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юент.
pub(crate) fn n16871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16772);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 40134);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 68219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 40135);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68220);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93820);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113042);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125447);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132940);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катаробионт.
pub(crate) fn n138806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137387);
	s.is_empty()
}

/// The word дендробионт.
pub(crate) fn n138807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137388);
	s.is_empty()
}

/// Words ending -остерилянт.
pub(crate) fn n134314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132944);
	s.is_empty()
}

/// Words ending -щ-тренчкот.
pub(crate) fn n134315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132945);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137389);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140051);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word лук-шалот.
pub(crate) fn n134316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132946);
	s.is_empty()
}

/// Words ending -олна-пилот.
pub(crate) fn n134317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132947);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 137390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 140052);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оект-пилот.
pub(crate) fn n134318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132948);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137391);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олот.
pub(crate) fn n16957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16858);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 40259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 40260);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68332);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 68333);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93892);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113084);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 125470);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132949);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 113085);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 125471);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132950);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138811(s, out),
														_ => false,
													}
												}
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
							super::push(out, 93893);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113086);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125472);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 132951);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138812(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 68334);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 68335);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 68336);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93896);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 68337);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113088);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125473);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 132952);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 113089);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 125474);
											s.is_empty()
										}
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
					super::push(out, 68338);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 68339);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93897);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113090);
									s.is_empty()
								}
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
			super::push(out, 40261);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 68340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -м-слот.
pub(crate) fn n69060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68341);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 93898);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тлот.
pub(crate) fn n16961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16862);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -флот.
pub(crate) fn n16962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16863);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 40270);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68342);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113091);
									s.is_empty()
								}
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
			super::push(out, 40271);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68343);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 93900);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40272);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 40273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68346);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлот.
pub(crate) fn n16963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16864);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40274);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 40275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юлот.
pub(crate) fn n16964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16865);
	match s {
		[] => true,
		['к', 'н', s @ ..] => {
			super::push(out, 40276);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68347);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экстраверт.
pub(crate) fn n134324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132954);
	s.is_empty()
}

/// The word переподвыподверт.
pub(crate) fn n134325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132955);
	s.is_empty()
}

/// Words ending -трубковерт.
pub(crate) fn n134326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132956);
	s.is_empty()
}

/// Words ending -уповерт.
pub(crate) fn n94963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93955);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 113128);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 125491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роверт.
pub(crate) fn n69140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68420);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 93956);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 113129);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 125492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шверт.
pub(crate) fn n40763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40354);
	s.is_empty()
}

/// Words ending -ыверт.
pub(crate) fn n40764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40355);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 68421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 93957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьверт.
pub(crate) fn n40765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40356);
	s.is_empty()
}

/// Words ending -керт.
pub(crate) fn n17030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16931);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40357);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 40358);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68422);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 93958);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113130);
									s.is_empty()
								}
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
			super::push(out, 40359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лерт.
pub(crate) fn n17031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16932);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -перт.
pub(crate) fn n17032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16933);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 40361);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68423);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 93959);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 113131);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125493);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132957);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138813(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -серт.
pub(crate) fn n17033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16934);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40362);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 68424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терт.
pub(crate) fn n17034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16935);
	s.is_empty()
}

/// Words ending -ферт.
pub(crate) fn n17035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16936);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 40363);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 68425);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херт.
pub(crate) fn n17036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16937);
	s.is_empty()
}

/// Words ending -церт.
pub(crate) fn n17037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16938);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 40364);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68426);
					match s {
						[] => true,
						['к', '-', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 113132);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125494);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132958);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138814(s, out),
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
									super::push(out, 113133);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 125495);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132959);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138815(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черт.
pub(crate) fn n17038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16939);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ранко-борт.
pub(crate) fn n134330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132960);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юберипорт.
pub(crate) fn n126797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125498);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 132961);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 137398);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -типорт.
pub(crate) fn n69161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68441);
	s.is_empty()
}

/// Words ending -мпорт.
pub(crate) fn n40791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40382);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 68442);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 93972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 68443);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93973);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113139);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нпорт.
pub(crate) fn n40792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40383);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68444);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 93974);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 68445);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 93975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опорт.
pub(crate) fn n40793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40384);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 68446);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 93976);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ппорт.
pub(crate) fn n40794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68447);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93977);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 93978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 68448);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 93979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпорт.
pub(crate) fn n40795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40386);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 68449);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 93980);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спорт.
pub(crate) fn n40796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40387);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 68450);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93981);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113142);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125500);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 132962);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 68451);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 93982);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113143);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125501);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132963);
											s.is_empty()
										}
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
					super::push(out, 93983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 68452);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 93984);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 113144);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125502);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 132964);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 68453);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 93985);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 68454);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93986);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 68455);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93987);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113147);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132965);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 137399);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 132966);
											s.is_empty()
										}
										['ь', s @ ..] => {
											super::push(out, 132967);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 137400);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n141498(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 93988);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125503);
									s.is_empty()
								}
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
			super::push(out, 68456);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93989);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113149);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 93990);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113150);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 113151);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 125506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тпорт.
pub(crate) fn n40797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40388);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 68457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фпорт.
pub(crate) fn n40798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40389);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 68458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93992);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чпорт.
pub(crate) fn n40799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40390);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68459);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 93993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 68460);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рорт.
pub(crate) fn n17047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16948);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 40391);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

