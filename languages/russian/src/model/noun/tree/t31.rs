use flexion::Path;

/// Words ending -ичка.
pub(crate) fn n5776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5761);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22113);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 48416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22114);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48417);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77003);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 101013);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118094);
											s.is_empty()
										}
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
					super::push(out, 48418);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77004);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101014);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118095);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 101015);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 77005);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 77006);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 77007);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101016);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 118097);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 77008);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 101017);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118098);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128852);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 118099);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128853);
													match s {
														[] => true,
														['м', s @ ..] => super::t43::n136625(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48419);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77009);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48420);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 77010);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 101019);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118100);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128854);
													s.is_empty()
												}
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
							super::push(out, 77011);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 77012);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 101020);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118101);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128855);
													s.is_empty()
												}
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
							super::push(out, 77013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48421);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77014);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101021);
									s.is_empty()
								}
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
			super::push(out, 22115);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48422);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48423);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101022);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 101023);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118102);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128856);
													match s {
														[] => true,
														['н', s @ ..] => super::t43::n136626(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22116);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48425);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 77015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 48426);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48427);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77016);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 77017);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101025);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118103);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128857);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22117);
			match s {
				[] => true,
				['д', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77018);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 77019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48428);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 48429);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 77020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22118);
			match s {
				[] => true,
				['и', s @ ..] => super::t43::n48931(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22119);
			s.is_empty()
		}
		['л', s @ ..] => super::t43::n22250(s, out),
		['м', s @ ..] => super::t43::n22251(s, out),
		['н', s @ ..] => super::t43::n22252(s, out),
		['о', s @ ..] => super::t43::n22253(s, out),
		['п', s @ ..] => super::t43::n22254(s, out),
		['р', s @ ..] => super::t43::n22255(s, out),
		['с', s @ ..] => super::t43::n22256(s, out),
		['т', s @ ..] => super::t43::n22257(s, out),
		['ф', s @ ..] => super::t43::n22258(s, out),
		['х', 'и', 'с', 'п', '\u{1}', s @ ..] => super::t43::n22259(s, out),
		['ч', s @ ..] => super::t43::n22260(s, out),
		['ш', s @ ..] => super::t43::n22261(s, out),
		['щ', s @ ..] => super::t43::n22262(s, out),
		_ => false,
	}
}

/// Words ending -лчка.
pub(crate) fn n5777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5762);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22131);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 48484);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77091);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101077);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118138);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очка.
pub(crate) fn n5778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5763);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 22132);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 48485);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77092);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101078);
									s.is_empty()
								}
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
			super::push(out, 22133);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 48486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48487);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101079);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 101080);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118139);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128873);
													match s {
														[] => true,
														['а', s @ ..] => super::t43::n136631(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 128874);
													match s {
														[] => true,
														['н', s @ ..] => super::t43::n136632(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 118140);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 128875);
													match s {
														[] => true,
														['т', s @ ..] => super::t43::n136633(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48488);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 48489);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 48490);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 48491);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 77096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48492);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77097);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 77098);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101082);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118141);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128876);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 128877);
													match s {
														[] => true,
														['н', s @ ..] => super::t43::n136634(s, out),
														_ => false,
													}
												}
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
							super::push(out, 77099);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 77100);
							match s {
								[] => true,
								['о', s @ ..] => super::t43::n102169(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t43::n77917(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t43::n48995(s, out),
				['у', s @ ..] => super::t43::n48996(s, out),
				['ы', s @ ..] => super::t43::n48997(s, out),
				['ю', s @ ..] => super::t43::n48998(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t43::n22267(s, out),
		['д', s @ ..] => super::t43::n22268(s, out),
		['з', s @ ..] => super::t43::n22269(s, out),
		['к', s @ ..] => super::t43::n22270(s, out),
		['л', s @ ..] => super::t43::n22271(s, out),
		['м', s @ ..] => super::t43::n22272(s, out),
		['н', s @ ..] => super::t43::n22273(s, out),
		['п', s @ ..] => super::t43::n22274(s, out),
		['р', s @ ..] => super::t43::n22275(s, out),
		['с', s @ ..] => super::t43::n22276(s, out),
		['т', s @ ..] => super::t43::n22277(s, out),
		['ф', s @ ..] => super::t43::n22278(s, out),
		_ => false,
	}
}

/// Words ending -учка.
pub(crate) fn n5779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5764);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22147);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 48587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22148);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48588);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77342);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101313);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118298);
											s.is_empty()
										}
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
					super::push(out, 48589);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 48590);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 77344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 48591);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22149);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 48593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22150);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48594);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 77346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 48595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77347);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101315);
									s.is_empty()
								}
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
			super::push(out, 22151);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 48596);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77348);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101316);
									s.is_empty()
								}
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
			super::push(out, 22152);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 48597);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 77349);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101317);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118299);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22154);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 48599);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 48600);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48601);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 48602);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77351);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 77352);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101319);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118300);
											s.is_empty()
										}
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
					super::push(out, 48603);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 77353);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101320);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118301);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22156);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48604);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77354);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101321);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118302);
											s.is_empty()
										}
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
					super::push(out, 48605);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 77355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22157);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 48606);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77356);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 101322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 48607);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77357);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101323);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 118303);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128969);
													match s {
														[] => true,
														['я', s @ ..] => super::t43::n136677(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22158);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22159);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48609);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 77358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 48610);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 77359);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 101324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48611);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77360);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 77361);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101326);
									match s {
										[] => true,
										['а', s @ ..] => super::t43::n119566(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => super::t43::n78187(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t43::n49116(s, out),
				['ы', s @ ..] => super::t43::n49117(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t43::n22292(s, out),
		['т', s @ ..] => super::t43::n22293(s, out),
		['х', s @ ..] => super::t43::n22294(s, out),
		['ч', s @ ..] => super::t43::n22295(s, out),
		['щ', s @ ..] => super::t43::n22296(s, out),
		['ы', s @ ..] => super::t43::n22297(s, out),
		_ => false,
	}
}

/// Words ending -ычка.
pub(crate) fn n5780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5765);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22166);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48623);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 48624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22167);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 48625);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 77368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101332);
									s.is_empty()
								}
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
			super::push(out, 22168);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48626);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 48627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22169);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48628);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 48629);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77370);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 101333);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118306);
											s.is_empty()
										}
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
					super::push(out, 48630);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48631);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101334);
									s.is_empty()
								}
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
			super::push(out, 22170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эчка.
pub(crate) fn n5781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5766);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 22171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ючка.
pub(crate) fn n5782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5767);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48632);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 77373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22173);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 48633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 48634);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 77375);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 77376);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118307);
											s.is_empty()
										}
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
					super::push(out, 48635);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22174);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 48636);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77378);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 101337);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118308);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128970);
													match s {
														[] => true,
														['м', s @ ..] => super::t43::n136678(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22175);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 48637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77379);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 77380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 22176);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячка.
pub(crate) fn n5783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5768);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22177);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 48640);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101338);
									s.is_empty()
								}
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
			super::push(out, 22178);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 48641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22179);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 48642);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 48643);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48644);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 77384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22180);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 48646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22181);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 48647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22182);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 48648);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101340);
									s.is_empty()
								}
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
			super::push(out, 22183);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 48649);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 48650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22184);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48651);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 48652);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 48653);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48654);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 77386);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 77387);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101341);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118309);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128971);
													s.is_empty()
												}
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
							super::push(out, 77388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 48655);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 48656);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 77389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'и', s @ ..] => {
			super::push(out, 22185);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22186);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 48658);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', 'л', s @ ..] => {
			super::push(out, 22187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48659);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ашка.
pub(crate) fn n5784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5769);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22188);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 22189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48660);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 48661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77392);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 77393);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 77394);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101344);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118310);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 77395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48662);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77396);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101345);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118311);
											s.is_empty()
										}
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
					super::push(out, 48663);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77397);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 101346);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118312);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 128972);
													match s {
														[] => true,
														['т', s @ ..] => super::t43::n136679(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22190);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48664);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 48665);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48666);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77398);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101347);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 118313);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128973);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 48667);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77399);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 101348);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118314);
											s.is_empty()
										}
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
					super::push(out, 48668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48669);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 77401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 48670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22191);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48671);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 77402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48672);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77403);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 101349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48673);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 77404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101350);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 77405);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 101351);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118315);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128974);
													match s {
														[] => true,
														['м', s @ ..] => super::t43::n136680(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48674);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77406);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101352);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118316);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48675);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 48676);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 48677);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 48678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22193);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48679);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22194);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48680);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77408);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 77409);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101353);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118317);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t43::n49185(s, out),
				['л', s @ ..] => super::t43::n49186(s, out),
				['о', s @ ..] => super::t43::n49187(s, out),
				['у', s @ ..] => {
					super::push(out, 48684);
					match s {
						[] => true,
						['б', s @ ..] => super::t43::n78240(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t43::n22327(s, out),
		['м', s @ ..] => super::t43::n22328(s, out),
		['н', s @ ..] => super::t43::n22329(s, out),
		['п', s @ ..] => super::t43::n22330(s, out),
		['р', s @ ..] => super::t43::n22331(s, out),
		['с', s @ ..] => super::t43::n22332(s, out),
		['т', s @ ..] => super::t43::n22333(s, out),
		['ч', s @ ..] => super::t43::n22334(s, out),
		['ш', s @ ..] => super::t43::n22335(s, out),
		_ => false,
	}
}

/// Words ending -англадешка.
pub(crate) fn n130337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128985);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 135278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -устосмешка.
pub(crate) fn n130338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128986);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 135279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дурешка.
pub(crate) fn n78304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77479);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 101400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрешка.
pub(crate) fn n49250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48745);
	s.is_empty()
}

/// Words ending -тешка.
pub(crate) fn n22347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22215);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 48746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фешка.
pub(crate) fn n22348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22216);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48747);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77480);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цешка.
pub(crate) fn n22349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22217);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чешка.
pub(crate) fn n22350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22218);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ишка.
pub(crate) fn n5786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5771);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22219);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77481);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 77482);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101401);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118346);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128987);
													match s {
														[] => true,
														['с', s @ ..] => super::t43::n136686(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48751);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 77483);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101402);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118347);
											s.is_empty()
										}
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
					super::push(out, 48752);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77484);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101403);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 118348);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 128988);
													match s {
														[] => true,
														['о', s @ ..] => super::t43::n136687(s, out),
														_ => false,
													}
												}
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
							super::push(out, 77485);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101404);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118349);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128989);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22220);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 48754);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 77487);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101405);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118350);
											s.is_empty()
										}
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
					super::push(out, 48755);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77488);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 101406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 77489);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101407);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118351);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22221);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 48756);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 77490);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 101408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 48757);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 77491);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 48758);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77492);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 77493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48759);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77494);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101410);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118352);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128990);
													match s {
														[] => true,
														['ч', s @ ..] => super::t43::n136688(s, out),
														_ => false,
													}
												}
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
							super::push(out, 77495);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101411);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118353);
											s.is_empty()
										}
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
					super::push(out, 48760);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77496);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 101412);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 118354);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128991);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22222);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77497);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101413);
									s.is_empty()
								}
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
			super::push(out, 22223);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 48762);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 77498);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t43::n22356(s, out),
		['к', s @ ..] => {
			super::push(out, 22225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t43::n22358(s, out),
		['м', s @ ..] => {
			super::push(out, 22227);
			s.is_empty()
		}
		['н', s @ ..] => super::t43::n22360(s, out),
		['п', s @ ..] => super::t43::n22361(s, out),
		['р', s @ ..] => super::t43::n22362(s, out),
		['с', s @ ..] => super::t43::n22363(s, out),
		['т', s @ ..] => super::t43::n22364(s, out),
		['у', s @ ..] => super::t43::n22365(s, out),
		['ф', s @ ..] => {
			super::push(out, 22234);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t43::n22367(s, out),
		['ш', s @ ..] => super::t43::n22368(s, out),
		['ь', s @ ..] => super::t43::n22369(s, out),
		_ => false,
	}
}

/// Words ending -ошка.
pub(crate) fn n5787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5772);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22238);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 22239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48823);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77572);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 77573);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101484);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 77574);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101485);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118401);
											s.is_empty()
										}
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
					super::push(out, 48824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48825);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 77575);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 101486);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118402);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129014);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22241);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48826);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 48827);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77577);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101487);
									s.is_empty()
								}
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
			super::push(out, 22242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48828);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 48829);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77578);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101488);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 118403);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129015);
													match s {
														[] => true,
														['а', s @ ..] => super::t43::n136701(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48830);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22243);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48831);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77580);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 101489);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118404);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 129016);
													match s {
														[] => true,
														['л', s @ ..] => super::t43::n136702(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48832);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 48833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48834);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 48835);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 77582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48836);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77583);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101491);
									s.is_empty()
								}
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
			super::push(out, 22245);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48837);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77584);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 48838);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22246);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 48839);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22247);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48840);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 48841);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 48842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77588);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 77589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101493);
									match s {
										[] => true,
										['п', s @ ..] => super::t43::n119667(s, out),
										_ => false,
									}
								}
								['к', s @ ..] => super::t43::n102581(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 77590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t43::n49348(s, out),
				['п', s @ ..] => {
					super::push(out, 48844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22248);
			s.is_empty()
		}
		['т', s @ ..] => super::t43::n22381(s, out),
		_ => false,
	}
}

/// Words ending -ушка.
pub(crate) fn n5788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5773);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22250);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48850);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77599);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101498);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 77600);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101499);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118409);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129019);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 101500);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118410);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129020);
													match s {
														[] => true,
														['р', s @ ..] => super::t43::n136705(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48851);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77601);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 101501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 77602);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 77603);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 77604);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 101504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 48852);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 77605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 48853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77606);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 101505);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118411);
											s.is_empty()
										}
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
					super::push(out, 48854);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77607);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48855);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 77608);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 101507);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118412);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129021);
													s.is_empty()
												}
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
							super::push(out, 77609);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101508);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118413);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 77610);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 101509);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118414);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 129022);
													s.is_empty()
												}
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
							super::push(out, 77611);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101510);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118415);
											s.is_empty()
										}
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
					super::push(out, 48856);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77612);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48857);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77613);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101512);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118416);
											s.is_empty()
										}
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
					super::push(out, 48858);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t43::n22383(s, out),
		['г', s @ ..] => {
			super::push(out, 22252);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48866);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77638);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101547);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118447);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 77639);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101548);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118448);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129040);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 48867);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 48868);
					match s {
						[] => true,
						['и', s @ ..] => super::t43::n78466(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t43::n49374(s, out),
				['ы', s @ ..] => super::t43::n49375(s, out),
				['я', s @ ..] => super::t43::n49376(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t43::n22385(s, out),
		['з', s @ ..] => super::t43::n22386(s, out),
		['к', s @ ..] => super::t43::n22387(s, out),
		['л', s @ ..] => super::t43::n22388(s, out),
		['м', s @ ..] => super::t43::n22389(s, out),
		['н', s @ ..] => super::t44::n22390(s, out),
		['о', s @ ..] => super::t44::n22391(s, out),
		['п', s @ ..] => super::t44::n22392(s, out),
		['р', s @ ..] => super::t44::n22393(s, out),
		['с', s @ ..] => super::t44::n22394(s, out),
		['т', s @ ..] => super::t44::n22395(s, out),
		['ф', s @ ..] => super::t44::n22396(s, out),
		['х', s @ ..] => super::t44::n22397(s, out),
		['ч', s @ ..] => super::t44::n22398(s, out),
		['ш', s @ ..] => super::t44::n22399(s, out),
		_ => false,
	}
}

/// Words ending -ышка.
pub(crate) fn n5789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5774);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22268);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 22269);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 48971);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 77828);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101713);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118541);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 129082);
													match s {
														[] => true,
														['о', s @ ..] => super::t44::n136731(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22270);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48972);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77829);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101714);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118542);
											s.is_empty()
										}
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
					super::push(out, 48973);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 48974);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77831);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48975);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 48976);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77832);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 101716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 48977);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48978);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77834);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 101717);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118543);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 77835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48979);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 77836);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 77837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48980);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 77838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 77839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 48981);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48982);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 48983);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48984);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48985);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 77843);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 101719);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118544);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129083);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48986);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 48987);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77844);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 101720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 48988);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77845);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 101721);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118545);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129084);
													match s {
														[] => true,
														['м', s @ ..] => super::t44::n136732(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48989);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77846);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101722);
									s.is_empty()
								}
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
			super::push(out, 22275);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48990);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 48991);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 77848);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 101723);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118546);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 129085);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t44::n22408(s, out),
		_ => false,
	}
}

/// Words ending -эшка.
pub(crate) fn n5790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5775);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22277);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 22278);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48997);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 48998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49001);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 49002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22282);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 22283);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юшка.
pub(crate) fn n5791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5776);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22285);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49004);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77855);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 77856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22286);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49005);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 49006);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77858);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 101728);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118548);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129086);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 49007);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77859);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 77860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 49008);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 49009);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 77862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 49010);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 77863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22287);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77864);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101729);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118549);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129087);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 49012);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77865);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22288);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 49014);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 77866);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 101731);
									s.is_empty()
								}
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
			super::push(out, 22289);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49015);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77867);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101732);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118550);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22290);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49016);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 49017);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 77869);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101733);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118551);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 77870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 49018);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 49019);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 49020);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 77872);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101734);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118552);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 77873);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101735);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118553);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 77874);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 77875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 49021);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 49022);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 77877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 49023);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 77878);
							match s {
								[] => true,
								['т', 'н', 'и', 'ф', '\u{1}', s @ ..] => {
									super::push(out, 101738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 49024);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77879);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 101739);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118554);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 129088);
													s.is_empty()
												}
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
							super::push(out, 77880);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101740);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118555);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129089);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t44::n22423(s, out),
		['р', s @ ..] => super::t44::n22424(s, out),
		['т', s @ ..] => super::t44::n22425(s, out),
		['ь', s @ ..] => super::t44::n22426(s, out),
		['я', s @ ..] => {
			super::push(out, 22295);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49044);
					match s {
						[] => true,
						['о', s @ ..] => super::t44::n78734(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яшка.
pub(crate) fn n5792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5777);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22296);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22297);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49045);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22298);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49046);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 49047);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 77911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22299);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49048);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 77912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22300);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49050);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 49051);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77913);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 49052);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 49053);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 49054);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 77914);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 101765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 49055);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 77915);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101766);
									s.is_empty()
								}
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
			super::push(out, 22302);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49056);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 77916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22303);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49057);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 77917);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101767);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118566);
											s.is_empty()
										}
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
					super::push(out, 49058);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 77918);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101768);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118567);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 129096);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 101769);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118568);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129097);
													match s {
														[] => true,
														['б', s @ ..] => super::t44::n136735(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 49059);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 77919);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101770);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118569);
											s.is_empty()
										}
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
					super::push(out, 49060);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 77920);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101771);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118570);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 101772);
									match s {
										[] => true,
										['т', 'с', s @ ..] => {
											super::push(out, 118571);
											s.is_empty()
										}
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
					super::push(out, 49061);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 77921);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 77922);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 101773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118572);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 77923);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 101774);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 118573);
											s.is_empty()
										}
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
					super::push(out, 49062);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 77924);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 101775);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118574);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 77925);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 101776);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 118575);
											s.is_empty()
										}
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
					super::push(out, 49063);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 77926);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101777);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118576);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118577);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 118578);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22304);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t44::n22437(s, out),
		['с', s @ ..] => {
			super::push(out, 22306);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22307);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 22308);
			match s {
				[] => true,
				['л', s @ ..] => super::t44::n49575(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алька.
pub(crate) fn n22455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22323);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 49087);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77937);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101788);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118585);
									s.is_empty()
								}
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
			super::push(out, 49088);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 49089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -елька.
pub(crate) fn n22456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22324);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 49090);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77938);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101789);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 101790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 77939);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 49091);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77940);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101792);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118586);
									s.is_empty()
								}
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
			super::push(out, 49092);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77941);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 77942);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77943);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 49093);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77944);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101794);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118587);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129100);
											s.is_empty()
										}
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
					super::push(out, 77945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 49094);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77946);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101795);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118588);
									s.is_empty()
								}
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
			super::push(out, 49095);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77947);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 77948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 49096);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77949);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101796);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 101797);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 101798);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118590);
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
							super::push(out, 101799);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118591);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 101800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49097);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 49098);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77950);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 101802);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 77951);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 101803);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129102);
											s.is_empty()
										}
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
					super::push(out, 77952);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 77953);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 77954);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 77955);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 49099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77956);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 77957);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101806);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118595);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129103);
											s.is_empty()
										}
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
					super::push(out, 77958);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101807);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118596);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129104);
											s.is_empty()
										}
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
					super::push(out, 77959);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101808);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 118597);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 129105);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135330);
													match s {
														[] => true,
														['р', s @ ..] => super::t44::n140359(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илька.
pub(crate) fn n22457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22325);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 49100);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 49101);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 49102);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 49103);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77960);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 49104);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 77961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 49105);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77962);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 49106);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 77963);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101811);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118598);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 129106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олька.
pub(crate) fn n22458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22326);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 49107);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 49108);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 49109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77964);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 77965);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118599);
									s.is_empty()
								}
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
			super::push(out, 49110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77966);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 77967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49111);
			match s {
				[] => true,
				['и', 'т', '\u{1}', s @ ..] => {
					super::push(out, 77968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улька.
pub(crate) fn n22459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22327);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 49112);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77969);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101813);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 101814);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118600);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129107);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 101815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118601);
									s.is_empty()
								}
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
				['е', s @ ..] => {
					super::push(out, 77970);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101816);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 77971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 49113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77972);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 77973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77974);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 77975);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101817);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118603);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 129108);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49114);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					super::push(out, 77976);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 77977);
					s.is_empty()
				}
				['с', 'и', 'п', '\u{1}', s @ ..] => {
					super::push(out, 77978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77979);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101818);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 49115);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77981);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 101819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 77982);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 49116);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77983);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 77984);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 77985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49117);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77986);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101823);
							s.is_empty()
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
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101824);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 101825);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118606);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129109);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135331);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77987);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 49118);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77988);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101827);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118607);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129110);
											match s {
												[] => true,
												['к', '\u{1}', s @ ..] => {
													super::push(out, 135332);
													s.is_empty()
												}
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
							super::push(out, 101828);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118608);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129111);
											match s {
												[] => true,
												['к', '\u{1}', s @ ..] => {
													super::push(out, 135333);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77989);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101829);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118609);
									s.is_empty()
								}
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
			super::push(out, 49119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юлька.
pub(crate) fn n22460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22328);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 49120);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77990);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 49121);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 77991);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101831);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118610);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129112);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135334);
													match s {
														[] => true,
														['р', s @ ..] => super::t44::n140360(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 49122);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77992);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49123);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77993);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 77994);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101833);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118611);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129113);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77995);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 77996);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 101834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лялька.
pub(crate) fn n49634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49125);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 77997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нька.
pub(crate) fn n5803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5788);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22330);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 77998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 49127);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 77999);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101835);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118612);
											s.is_empty()
										}
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
					super::push(out, 49128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78000);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 101836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 49129);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78001);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 101837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49130);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78002);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 49131);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 49132);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78003);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101839);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 118613);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129114);
													match s {
														[] => true,
														['к', s @ ..] => super::t44::n136741(s, out),
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
									super::push(out, 101840);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118614);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129115);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 49133);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 78004);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 101841);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118615);
											s.is_empty()
										}
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
					super::push(out, 49134);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t44::n22463(s, out),
		['и', s @ ..] => {
			super::push(out, 22332);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49152);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78041);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 78042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 49153);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 49154);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78043);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101894);
									s.is_empty()
								}
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
			super::push(out, 22333);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78044);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 78045);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101895);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118645);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 78046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 49156);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78047);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 101896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78048);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101897);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118646);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 129131);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 49157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78049);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 49158);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78051);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101900);
									s.is_empty()
								}
								['р', s @ ..] => super::t44::n102989(s, out),
								_ => false,
							}
						}
						['и', s @ ..] => super::t44::n78879(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t44::n49668(s, out),
				['м', s @ ..] => super::t44::n49669(s, out),
				['п', s @ ..] => super::t44::n49670(s, out),
				['с', s @ ..] => super::t44::n49671(s, out),
				['т', s @ ..] => super::t44::n49672(s, out),
				['х', s @ ..] => super::t44::n49673(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t44::n22466(s, out),
		['ы', s @ ..] => super::t44::n22467(s, out),
		['я', s @ ..] => super::t44::n22468(s, out),
		_ => false,
	}
}

/// Words ending -рька.
pub(crate) fn n5804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5789);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22337);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49174);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78072);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 101922);
									s.is_empty()
								}
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
			super::push(out, 22338);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 49175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22339);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 22340);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49177);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ська.
pub(crate) fn n5805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5790);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22342);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49179);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 49180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22343);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49181);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 49182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22344);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 22345);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49184);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тька.
pub(crate) fn n5806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22346);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49185);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 78077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 49186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22347);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22348);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49188);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 49189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 22349);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49190);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыба-гадюка.
pub(crate) fn n130489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129137);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яка-маляка.
pub(crate) fn n130490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129138);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 135344);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138928);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пол-байкала.
pub(crate) fn n136751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135345);
	s.is_empty()
}

/// Words ending -пол-отдела.
pub(crate) fn n130494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129142);
	s.is_empty()
}

/// Words ending -ран-стрела.
pub(crate) fn n130495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129143);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 135346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бромила.
pub(crate) fn n79002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78175);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 101975);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 118673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмила.
pub(crate) fn n22639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22507);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 49347);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 78177);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тмила.
pub(crate) fn n22640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22508);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 49348);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78178);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101977);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умила.
pub(crate) fn n22641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22509);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 49349);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78179);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нила.
pub(crate) fn n5885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5870);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22510);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49350);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 49351);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 49352);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78180);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101979);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118675);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22511);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 49353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22512);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49354);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22513);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49355);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 78182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 22514);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 49356);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пила.
pub(crate) fn n5886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5871);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22515);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 22516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49357);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78184);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 101980);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118676);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129144);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22517);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22518);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49359);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 78185);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 101981);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118677);
											s.is_empty()
										}
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
					super::push(out, 49360);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22519);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49361);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рила.
pub(crate) fn n5887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5872);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22520);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 49362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22521);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49363);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22522);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49364);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 49365);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22523);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22524);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22525);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49368);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78192);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 101982);
									s.is_empty()
								}
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
			super::push(out, 22526);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49369);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22527);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сила.
pub(crate) fn n5888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5873);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 22528);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49371);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22529);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 22530);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22531);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 22532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тила.
pub(crate) fn n5889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5874);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22533);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49372);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78195);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101983);
									s.is_empty()
								}
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
			super::push(out, 22534);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49373);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22535);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22536);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уила.
pub(crate) fn n5890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5875);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22537);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49376);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 22538);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фила.
pub(crate) fn n5891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5876);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22539);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 22540);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49377);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22541);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49378);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78199);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101984);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118678);
											s.is_empty()
										}
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
					super::push(out, 49379);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хила.
pub(crate) fn n5892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5877);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22542);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22543);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чила.
pub(crate) fn n5893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5878);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22544);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22545);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22546);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 49384);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78203);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 101985);
									s.is_empty()
								}
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
			super::push(out, 22547);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49385);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78204);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 78205);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101986);
									s.is_empty()
								}
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
			super::push(out, 22548);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 49386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шила.
pub(crate) fn n5894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5879);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22549);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22550);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49387);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78206);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 101987);
									s.is_empty()
								}
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
			super::push(out, 22551);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49388);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78207);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 101988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -секс-кукла.
pub(crate) fn n130497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129145);
	s.is_empty()
}

/// Words ending -сардинелла.
pub(crate) fn n130498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129146);
	s.is_empty()
}

/// Words ending -рихинелла.
pub(crate) fn n119948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118686);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 129147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онелла.
pub(crate) fn n49920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49410);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 78225);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 102000);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 118687);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пелла.
pub(crate) fn n22705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22572);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -релла.
pub(crate) fn n22706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22573);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49412);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 49413);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102001);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 102002);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118688);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129149);
											s.is_empty()
										}
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
					super::push(out, 78227);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 102003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телла.
pub(crate) fn n22707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49414);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78228);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118689);
									s.is_empty()
								}
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
			super::push(out, 49415);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78230);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -целла.
pub(crate) fn n22708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22575);
	s.is_empty()
}

/// Words ending -илла.
pub(crate) fn n5902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5887);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22576);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49417);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'и', s @ ..] => {
			super::push(out, 22577);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22578);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 22579);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78232);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 78233);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102005);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49420);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22580);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 22581);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49422);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78235);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 102006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'к', s @ ..] => {
					super::push(out, 49423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 49424);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78237);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 78238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22582);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 22583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49425);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 49426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78240);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 78241);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102008);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118690);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22584);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49427);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78242);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 102009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олла.
pub(crate) fn n5903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5888);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22585);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49428);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78243);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102010);
									s.is_empty()
								}
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
			super::push(out, 22586);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49429);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22587);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 49430);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улла.
pub(crate) fn n5904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5889);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22588);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 22589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49431);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102011);
									s.is_empty()
								}
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
			super::push(out, 22590);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элла.
pub(crate) fn n5905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5890);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22591);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 49433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22592);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49434);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коллембола.
pub(crate) fn n130502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129150);
	s.is_empty()
}

/// Words ending -никола.
pub(crate) fn n49964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49454);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 78262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ккола.
pub(crate) fn n22750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22616);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 49455);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укола.
pub(crate) fn n22751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22617);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 49456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78264);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -школа.
pub(crate) fn n22752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22618);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 49457);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкола.
pub(crate) fn n22753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22619);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 49458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкола.
pub(crate) fn n22754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22620);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 49459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лола.
pub(crate) fn n5915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5900);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мола.
pub(crate) fn n5916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22622);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49460);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22623);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22624);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 49462);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78267);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102021);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118694);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129151);
													match s {
														[] => true,
														['е', s @ ..] => super::t44::n136753(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 49463);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78268);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102022);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118695);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нола.
pub(crate) fn n5917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5902);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пола.
pub(crate) fn n5918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5903);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рола.
pub(crate) fn n5919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5904);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22627);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22628);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49464);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78269);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тола.
pub(crate) fn n5920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5905);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 22629);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49465);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78270);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102024);
									s.is_empty()
								}
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
			super::push(out, 22630);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22631);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49467);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78271);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 49468);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 49469);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22632);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49470);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78274);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 102026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьола.
pub(crate) fn n5921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5906);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 22633);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щеневмерла.
pub(crate) fn n130504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129152);
	s.is_empty()
}

/// Words ending -сомнамбула.
pub(crate) fn n130505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129153);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -астула.
pub(crate) fn n50007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49497);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 78292);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 118700);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 118701);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 129154);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135349);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 138930);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t44::n142415(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фистула.
pub(crate) fn n79120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78293);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 102037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фула.
pub(crate) fn n5946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5931);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хула.
pub(crate) fn n5947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5932);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 22675);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22676);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оман-панорама.
pub(crate) fn n136756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135350);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138931);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word гонионема.
pub(crate) fn n130509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129157);
	s.is_empty()
}

/// Words ending -нхима.
pub(crate) fn n22899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22765);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 49576);
			match s {
				[] => true,
				['р', 'а', s @ ..] => {
					super::push(out, 78339);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ецима.
pub(crate) fn n22900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22766);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 49577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -редзавкома.
pub(crate) fn n130510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129158);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 135351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -драбочкома.
pub(crate) fn n130511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129159);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135352);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -простипома.
pub(crate) fn n130512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129160);
	s.is_empty()
}

/// Words ending -аскострома.
pub(crate) fn n130513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129161);
	s.is_empty()
}

/// Words ending -сома.
pub(crate) fn n6020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6005);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22799);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49601);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 49602);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78357);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102075);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118725);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129162);
													match s {
														[] => true,
														['т', s @ ..] => super::t44::n136759(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'и', s @ ..] => {
					super::push(out, 49603);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78358);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					super::push(out, 49604);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78359);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 102077);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118726);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22800);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49605);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78360);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102078);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118727);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129163);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тома.
pub(crate) fn n6021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6006);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22801);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22802);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 22803);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49606);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 78361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 49607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22804);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 22805);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49608);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102080);
									s.is_empty()
								}
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
			super::push(out, 22806);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49609);
					s.is_empty()
				}
				['и', 'б', s @ ..] => {
					super::push(out, 49610);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78363);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118728);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'л', s @ ..] => {
					super::push(out, 49611);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78364);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102082);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118729);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129164);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t44::n136760(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фома.
pub(crate) fn n6022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6007);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22807);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цома.
pub(crate) fn n6023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6008);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22808);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49612);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78365);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102083);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118730);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одхидхарма.
pub(crate) fn n130517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129165);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 135355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аздраперма.
pub(crate) fn n130518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129166);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 135356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -з-агрофирма.
pub(crate) fn n130519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129167);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135357);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 138933);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 140971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атапетасма.
pub(crate) fn n130520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129168);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 135358);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word моисеевна.
pub(crate) fn n130521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129169);
	s.is_empty()
}

/// Words ending -георгиевна.
pub(crate) fn n130522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129170);
	s.is_empty()
}

/// Words ending -дмитриевна.
pub(crate) fn n130523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129171);
	s.is_empty()
}

/// Words ending -рфентьевна.
pub(crate) fn n130524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129172);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135359);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 138934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овна.
pub(crate) fn n6082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6067);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49740);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78454);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 78455);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 49741);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22946);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 22947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49742);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78457);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102133);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118751);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 129173);
													match s {
														[] => true,
														['а', s @ ..] => super::t44::n136766(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 49743);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 49744);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78458);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102134);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118752);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 129174);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22948);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49745);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 78459);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102135);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118753);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 102136);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118754);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22949);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 22950);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49746);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78460);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102137);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118755);
											s.is_empty()
										}
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
					super::push(out, 49747);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78461);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102138);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118756);
											s.is_empty()
										}
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
					super::push(out, 49748);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78462);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 78463);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 78464);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102139);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118757);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22951);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49749);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78465);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102140);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118758);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 129175);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t44::n136767(s, out),
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
									super::push(out, 102141);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118759);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129176);
													match s {
														[] => true,
														['т', s @ ..] => super::t44::n136768(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 49750);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 49751);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78466);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102142);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118760);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129177);
													s.is_empty()
												}
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
							super::push(out, 78467);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102143);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118761);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129178);
													match s {
														[] => true,
														['х', s @ ..] => super::t44::n136769(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 49752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22952);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49753);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78468);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t44::n23088(s, out),
		['с', s @ ..] => {
			super::push(out, 22954);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t44::n23090(s, out),
		['ф', s @ ..] => super::t44::n23091(s, out),
		['х', s @ ..] => super::t44::n23092(s, out),
		_ => false,
	}
}

/// Words ending -параселена.
pub(crate) fn n130536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129184);
	s.is_empty()
}

/// Words ending -мена.
pub(crate) fn n6096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6081);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22983);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49793);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 49794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22984);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49795);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102164);
									s.is_empty()
								}
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
			super::push(out, 22985);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22986);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22987);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49798);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102165);
									s.is_empty()
								}
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
			super::push(out, 22988);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 49799);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78504);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102166);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118776);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129185);
													match s {
														[] => true,
														['п', s @ ..] => super::t44::n136772(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 49800);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 78505);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102167);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118777);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129186);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22989);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22990);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 22991);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 22992);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49802);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78506);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рена.
pub(crate) fn n6098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6083);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22994);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 49805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22995);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 22996);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 22997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49806);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 49807);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 49808);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 49809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22998);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22999);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 23000);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49812);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102170);
									s.is_empty()
								}
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
			super::push(out, 23001);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сена.
pub(crate) fn n6099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6084);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23002);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тена.
pub(crate) fn n6100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6085);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23003);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49815);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23004);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49816);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 49817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49818);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 49819);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78515);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 78516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 49820);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78517);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102172);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118778);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129187);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 49821);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 78518);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102173);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118779);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 129188);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фена.
pub(crate) fn n6101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6086);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23006);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49822);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78519);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102174);
									s.is_empty()
								}
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
			super::push(out, 23007);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49823);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78520);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102175);
									s.is_empty()
								}
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
			super::push(out, 23008);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 49824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цена.
pub(crate) fn n6102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6087);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23009);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23010);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49825);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78521);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102176);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118780);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23011);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 49826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ороговизна.
pub(crate) fn n130541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129189);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 135367);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word хрюкраина.
pub(crate) fn n130542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129190);
	s.is_empty()
}

/// Words ending -пережабина.
pub(crate) fn n130543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129191);
	s.is_empty()
}

/// Words ending -доебина.
pub(crate) fn n79374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78547);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 102188);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 118788);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 102189);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 118789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 102190);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 118790);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хоебина.
pub(crate) fn n79375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78548);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102191);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 118791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыебина.
pub(crate) fn n50365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49854);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 78549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 102192);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жбина.
pub(crate) fn n23166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23031);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 49855);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 49856);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 78551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ибина.
pub(crate) fn n23167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23032);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 49857);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 78552);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбина.
pub(crate) fn n23169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23034);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 49858);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78553);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102194);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обина.
pub(crate) fn n23170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23035);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 49859);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 49860);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78554);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 78555);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 102196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49861);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78556);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 102197);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 78557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49862);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбина.
pub(crate) fn n23171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23036);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 49863);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 78559);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 102198);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118794);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49864);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 49865);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 78561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убина.
pub(crate) fn n23172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23037);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 49866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 49867);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 49868);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49869);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78565);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 102199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбина.
pub(crate) fn n23173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23038);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 49870);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьбина.
pub(crate) fn n23174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23039);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 49872);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 78568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118795);
									s.is_empty()
								}
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
			super::push(out, 49873);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябина.
pub(crate) fn n23175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23040);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 49874);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вина.
pub(crate) fn n6110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6095);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23041);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23042);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					super::push(out, 49875);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102202);
									s.is_empty()
								}
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
			super::push(out, 23043);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23045);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78571);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'ч', s @ ..] => {
							super::push(out, 78572);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102204);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118796);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129194);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 49878);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78573);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102205);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 78574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 49879);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 49880);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78576);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 49881);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 49882);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 78577);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102207);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118798);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23046);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 49883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23047);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49884);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23048);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 49885);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 49886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23049);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23050);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49888);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 49889);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78581);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 78582);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102208);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 102209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78583);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 78584);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102210);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 118799);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 129195);
													s.is_empty()
												}
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
							super::push(out, 78585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t44::n50402(s, out),
				['к', s @ ..] => super::t44::n50403(s, out),
				['л', s @ ..] => super::t44::n50404(s, out),
				['м', s @ ..] => super::t44::n50405(s, out),
				['н', s @ ..] => super::t44::n50406(s, out),
				['п', s @ ..] => super::t44::n50407(s, out),
				['р', s @ ..] => super::t44::n50408(s, out),
				['с', s @ ..] => super::t44::n50409(s, out),
				['т', s @ ..] => super::t44::n50410(s, out),
				['х', s @ ..] => super::t44::n50411(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t44::n23186(s, out),
		['ь', s @ ..] => super::t44::n23187(s, out),
		['я', s @ ..] => super::t44::n23188(s, out),
		_ => false,
	}
}

/// Words ending -гина.
pub(crate) fn n6111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6096);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23054);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23055);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49903);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23057);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дина.
pub(crate) fn n6112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6097);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23058);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 49907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 49908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 49909);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78623);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 49910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78624);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 78625);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78626);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102247);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118817);
											s.is_empty()
										}
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
					super::push(out, 49911);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49912);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 49913);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23059);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49914);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78630);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102248);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118818);
											s.is_empty()
										}
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
					super::push(out, 49915);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 49916);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78631);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 49917);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 49918);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78632);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 78633);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 78634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 49919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78635);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 78636);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102252);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118819);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 78637);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102253);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118820);
											s.is_empty()
										}
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
					super::push(out, 49920);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78638);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102254);
									s.is_empty()
								}
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
			super::push(out, 23060);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49921);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23061);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49922);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 49923);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23062);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49924);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78642);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 49925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23063);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78643);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 78644);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102256);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 118821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 102257);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118822);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129199);
													s.is_empty()
												}
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
							super::push(out, 78645);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 49927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78646);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 78647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 49928);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78648);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78649);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102260);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 102261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t44::n50442(s, out),
				['р', s @ ..] => super::t44::n50443(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t44::n23199(s, out),
		['у', s @ ..] => super::t44::n23200(s, out),
		['ь', s @ ..] => super::t44::n23201(s, out),
		['ю', s @ ..] => super::t44::n23202(s, out),
		['я', s @ ..] => super::t44::n23203(s, out),
		_ => false,
	}
}

/// Words ending -еина.
pub(crate) fn n6113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6098);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23069);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49942);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78665);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102269);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118826);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129203);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23070);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49943);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жина.
pub(crate) fn n6114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6099);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23071);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49944);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78667);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49945);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78668);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 49946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78669);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102272);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118827);
											s.is_empty()
										}
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
					super::push(out, 49947);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78670);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102273);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 102274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118828);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23072);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49948);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78671);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 102275);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118829);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23073);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49949);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78672);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 78673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 49950);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 49951);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49952);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78675);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102278);
									s.is_empty()
								}
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
			super::push(out, 23074);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 49953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23075);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49954);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102279);
									s.is_empty()
								}
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
			super::push(out, 23076);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49955);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 49956);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102280);
									s.is_empty()
								}
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
			super::push(out, 23077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49957);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 49958);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49959);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102281);
									s.is_empty()
								}
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
			super::push(out, 23078);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49960);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 49961);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78681);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102283);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118830);
											s.is_empty()
										}
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
					super::push(out, 49962);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78682);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49963);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78683);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102285);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118831);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 78684);
							s.is_empty()
						}
						['п', 'а', s @ ..] => {
							super::push(out, 78685);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 49964);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 49965);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78686);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102287);
									s.is_empty()
								}
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
			super::push(out, 23079);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49966);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 49967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49968);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78688);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 78689);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102288);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 118832);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23080);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49969);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 49970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 23081);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49971);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 49972);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78691);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зина.
pub(crate) fn n6115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6100);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23082);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23083);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49973);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78692);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102290);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118833);
											s.is_empty()
										}
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
					super::push(out, 49974);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78693);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102291);
									s.is_empty()
								}
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
			super::push(out, 23084);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49975);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102292);
									s.is_empty()
								}
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
			super::push(out, 23085);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49976);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78695);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 102293);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118834);
											s.is_empty()
										}
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
					super::push(out, 49977);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78696);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 78697);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118835);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23086);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49978);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78698);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102295);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118836);
											s.is_empty()
										}
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
					super::push(out, 49979);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 49980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23087);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 49981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23088);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49982);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78700);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 102296);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 118837);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23089);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23090);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49984);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23091);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49985);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 49986);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 49987);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78702);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102297);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118838);
											s.is_empty()
										}
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
					super::push(out, 49988);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 49989);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78703);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102298);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 118839);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129204);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иина.
pub(crate) fn n6116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6101);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 23093);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49990);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 78704);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102299);
									s.is_empty()
								}
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
			super::push(out, 23094);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49991);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кина.
pub(crate) fn n6117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6102);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23095);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49992);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78706);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 102300);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 118840);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 78707);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102301);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118841);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23096);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лина.
pub(crate) fn n6118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6103);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23097);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49994);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 49995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78708);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 102302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 78709);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102303);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78710);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 49996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78711);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 78712);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 102305);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118843);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 102306);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118844);
											s.is_empty()
										}
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
					super::push(out, 49997);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78713);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102307);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 78714);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102308);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118846);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 78715);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102309);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118847);
											s.is_empty()
										}
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
					super::push(out, 49998);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 49999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78716);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 78717);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 50000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78718);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 50001);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78719);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102312);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118848);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78720);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102313);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 78721);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102314);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 118850);
											s.is_empty()
										}
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
					super::push(out, 50002);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78722);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102315);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118851);
											s.is_empty()
										}
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
					super::push(out, 50003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78723);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 78724);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 78725);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102317);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118852);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 78726);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78727);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102319);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118853);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 78728);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 78729);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 50004);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 50005);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78731);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102321);
									s.is_empty()
								}
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
			super::push(out, 23099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50006);
					match s {
						[] => true,
						['б', s @ ..] => super::t44::n79560(s, out),
						['р', s @ ..] => super::t44::n79561(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t44::n50520(s, out),
				['о', s @ ..] => super::t44::n50521(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t44::n23235(s, out),
		['г', s @ ..] => super::t44::n23236(s, out),
		['д', s @ ..] => super::t44::n23237(s, out),
		['е', s @ ..] => super::t44::n23238(s, out),
		['з', s @ ..] => super::t44::n23239(s, out),
		['и', s @ ..] => super::t44::n23240(s, out),
		['й', s @ ..] => super::t44::n23241(s, out),
		['к', s @ ..] => super::t44::n23242(s, out),
		['о', s @ ..] => super::t44::n23243(s, out),
		['п', s @ ..] => super::t44::n23244(s, out),
		['р', s @ ..] => super::t44::n23245(s, out),
		['с', s @ ..] => super::t44::n23246(s, out),
		['т', s @ ..] => super::t44::n23247(s, out),
		['у', s @ ..] => super::t44::n23248(s, out),
		['х', s @ ..] => super::t44::n23249(s, out),
		['ш', s @ ..] => super::t44::n23250(s, out),
		['ы', s @ ..] => super::t44::n23251(s, out),
		['э', s @ ..] => super::t44::n23252(s, out),
		['ю', s @ ..] => super::t44::n23253(s, out),
		['я', s @ ..] => super::t44::n23254(s, out),
		_ => false,
	}
}

/// Words ending -мина.
pub(crate) fn n6119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6104);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23120);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50062);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 50063);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78796);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50064);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 78797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50065);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 78798);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 78799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23121);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50066);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78800);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102367);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118877);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129213);
													match s {
														[] => true,
														['п', s @ ..] => super::t44::n136780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23122);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 23123);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50067);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23124);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 50068);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 50069);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23125);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 50071);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78805);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50072);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78806);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50073);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78807);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 50074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 23126);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50075);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23127);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 23128);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50077);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 50078);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78809);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102371);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 118878);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 129214);
													match s {
														[] => true,
														['и', s @ ..] => super::t44::n136781(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50079);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 23129);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 50080);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 23130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50081);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 50082);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78812);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нина.
pub(crate) fn n6120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6105);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23131);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23132);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50083);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 78814);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 50084);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 50085);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 50086);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 50087);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78815);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 102375);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118879);
											s.is_empty()
										}
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
					super::push(out, 50088);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78816);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102376);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118880);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129215);
													match s {
														[] => true,
														['д', s @ ..] => super::t44::n136782(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50089);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78817);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102377);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118881);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 78818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 50090);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 50091);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 50092);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78819);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118882);
											s.is_empty()
										}
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
					super::push(out, 50093);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23133);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50094);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23134);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23135);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 50095);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78822);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78823);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 78824);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 78825);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102381);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118883);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129216);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 129217);
													s.is_empty()
												}
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
							super::push(out, 78826);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 78827);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 102382);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118884);
											s.is_empty()
										}
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
					super::push(out, 50097);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 50098);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23136);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 50099);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 50100);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78830);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102383);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118885);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129218);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78831);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 50102);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78832);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102385);
									s.is_empty()
								}
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
			super::push(out, 23137);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23138);
			match s {
				[] => true,
				['е', s @ ..] => super::t44::n50617(s, out),
				['и', s @ ..] => super::t44::n50618(s, out),
				['к', s @ ..] => super::t44::n50619(s, out),
				['л', s @ ..] => super::t44::n50620(s, out),
				['р', s @ ..] => super::t44::n50621(s, out),
				['с', s @ ..] => super::t44::n50622(s, out),
				['т', s @ ..] => super::t44::n50623(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t44::n23274(s, out),
		['р', s @ ..] => super::t44::n23275(s, out),
		['с', s @ ..] => super::t44::n23276(s, out),
		['ч', s @ ..] => super::t44::n23277(s, out),
		['ш', s @ ..] => super::t44::n23278(s, out),
		['я', s @ ..] => super::t44::n23279(s, out),
		_ => false,
	}
}

/// Words ending -оина.
pub(crate) fn n6121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6106);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23145);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50120);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 50121);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102395);
									s.is_empty()
								}
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
			super::push(out, 23146);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50122);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 50123);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пина.
pub(crate) fn n6122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6107);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50125);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78851);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 102396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 78852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23148);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50126);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 50127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23149);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50128);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78853);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 102397);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118892);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129222);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50129);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 23150);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 50130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78855);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102398);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118893);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23151);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23152);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50133);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рина.
pub(crate) fn n6123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6108);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50134);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 50135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 50136);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 50137);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78860);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102399);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118894);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 78861);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102400);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118895);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 78862);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102401);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118896);
											s.is_empty()
										}
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
					super::push(out, 50138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 50139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 50140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78865);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 78866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 50141);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78867);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 78868);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78869);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 102403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 50142);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78870);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 50143);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78871);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 78872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 50144);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78873);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102406);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118897);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129223);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50145);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78874);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102407);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118898);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129224);
													s.is_empty()
												}
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
							super::push(out, 78875);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102408);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118899);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50146);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78876);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102409);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 78877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 50147);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 50148);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 78879);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102410);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 118901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 102411);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 118902);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 78880);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102412);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118903);
											s.is_empty()
										}
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
					super::push(out, 50149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78881);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 78882);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102414);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118904);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23155);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50150);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78883);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t44::n23291(s, out),
		['е', s @ ..] => super::t44::n23292(s, out),
		['и', s @ ..] => super::t44::n23293(s, out),
		['о', s @ ..] => super::t44::n23294(s, out),
		['п', s @ ..] => {
			super::push(out, 23160);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 50173);
					match s {
						[] => true,
						['ч', s @ ..] => super::t44::n79742(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t44::n23296(s, out),
		['у', s @ ..] => super::t44::n23297(s, out),
		['ы', s @ ..] => super::t44::n23298(s, out),
		_ => false,
	}
}

/// Words ending -сина.
pub(crate) fn n6124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6109);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23164);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50181);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78924);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102444);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118915);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129229);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50182);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 50183);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78925);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 102445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50184);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78926);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102446);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 118916);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129230);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23165);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 50185);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78927);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102447);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118917);
											s.is_empty()
										}
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
					super::push(out, 50186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78928);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 78929);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78930);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102449);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118918);
											s.is_empty()
										}
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
					super::push(out, 50187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23166);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50188);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78931);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 50189);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50190);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78933);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102451);
									s.is_empty()
								}
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
			super::push(out, 23167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50191);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 50192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78934);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 50193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78935);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 78936);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 78937);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 78938);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102455);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 78939);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 102456);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118920);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129231);
													match s {
														[] => true,
														['р', s @ ..] => super::t44::n136784(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50194);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78940);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102457);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 102458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 50195);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 50196);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78941);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102459);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118921);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129232);
													s.is_empty()
												}
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
							super::push(out, 78942);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102460);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 118922);
											s.is_empty()
										}
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
					super::push(out, 50197);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78943);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 102461);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 118923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t44::n79772(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t44::n23303(s, out),
		['р', s @ ..] => super::t44::n23304(s, out),
		['с', s @ ..] => super::t44::n23305(s, out),
		['у', s @ ..] => super::t44::n23306(s, out),
		['ы', s @ ..] => super::t44::n23307(s, out),
		['ь', s @ ..] => super::t44::n23308(s, out),
		['я', s @ ..] => super::t44::n23309(s, out),
		_ => false,
	}
}

/// Words ending -тина.
pub(crate) fn n6125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23175);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 50211);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78956);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102470);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118927);
											s.is_empty()
										}
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
					super::push(out, 50212);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78957);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 50213);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78958);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78959);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 78960);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 102473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 78961);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 78962);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102474);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118928);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129235);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50215);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78963);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 50216);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 50217);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 50218);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78965);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 50219);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78967);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102477);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 78968);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 78969);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 78970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 50220);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78971);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 78972);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102479);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 118930);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 129236);
													s.is_empty()
												}
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
							super::push(out, 78973);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102480);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 118931);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23176);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50221);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78974);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102481);
									s.is_empty()
								}
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
			super::push(out, 23177);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50222);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78975);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102482);
									s.is_empty()
								}
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
			super::push(out, 23178);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 50224);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 50225);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78977);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 102483);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 78978);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102484);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118933);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 78979);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 50226);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 50227);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78981);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102486);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118934);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129237);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t44::n79810(s, out),
						['а', s @ ..] => super::t44::n79811(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t44::n50742(s, out),
				['щ', s @ ..] => super::t44::n50743(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t44::n23314(s, out),
		['н', s @ ..] => super::t44::n23315(s, out),
		['о', s @ ..] => super::t44::n23316(s, out),
		['р', s @ ..] => super::t44::n23317(s, out),
		['с', s @ ..] => super::t44::n23318(s, out),
		['у', s @ ..] => super::t44::n23319(s, out),
		['ф', s @ ..] => super::t44::n23320(s, out),
		['ш', s @ ..] => super::t44::n23321(s, out),
		['ю', s @ ..] => super::t44::n23322(s, out),
		['я', s @ ..] => super::t44::n23323(s, out),
		_ => false,
	}
}

/// Words ending -уина.
pub(crate) fn n6126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6111);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 23189);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23190);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50274);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79040);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102535);
									s.is_empty()
								}
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
			super::push(out, 23191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фина.
pub(crate) fn n6127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6112);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23192);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23193);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 50275);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79041);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 102536);
									s.is_empty()
								}
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
			super::push(out, 23194);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хина.
pub(crate) fn n6128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23195);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50277);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 50278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23196);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 50279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23197);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 50280);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23198);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 50281);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79045);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102537);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118957);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23199);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50282);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цина.
pub(crate) fn n6129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6114);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23200);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 50283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23201);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50284);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 50285);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79048);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102538);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118958);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129249);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23202);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50286);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 50287);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102539);
									s.is_empty()
								}
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
			super::push(out, 23203);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

