use flexion::Path;

/// Words ending -яйка.
pub(crate) fn n5652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5638);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21445);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 21446);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46763);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 46764);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 21447);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46765);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 75181);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 99637);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117330);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128482);
													match s {
														[] => true,
														['т', s @ ..] => super::t42::n136463(s, out),
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
									super::push(out, 99638);
									s.is_empty()
								}
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
			super::push(out, 21448);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46766);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75182);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 46767);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75183);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 99640);
									s.is_empty()
								}
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
			super::push(out, 21449);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46768);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 75184);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99641);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117331);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21450);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46769);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75185);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99642);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117332);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'н', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46770);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 46771);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алка.
pub(crate) fn n5659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5645);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21462);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 21463);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 46781);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 46782);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 75192);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46783);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 46784);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75193);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99646);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117333);
											s.is_empty()
										}
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
					super::push(out, 46785);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99647);
									s.is_empty()
								}
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
			super::push(out, 21464);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46786);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99648);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 75196);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 75197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'п', s @ ..] => {
					super::push(out, 46787);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75198);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 99650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117334);
											s.is_empty()
										}
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
						['б', s @ ..] => {
							super::push(out, 75199);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 75200);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99651);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117335);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128483);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46788);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 46789);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 46790);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75201);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 75202);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 99653);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117336);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128484);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46792);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 46793);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75203);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99654);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 117337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 75204);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 46794);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 75205);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99656);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117338);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128485);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46795);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99657);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 99658);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 117339);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 75206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 46796);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75207);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 99659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 75208);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 75209);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 46797);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75210);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99662);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 46798);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 75211);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 75212);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99663);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117340);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128486);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => super::t42::n47258(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t42::n21585(s, out),
		['ж', s @ ..] => super::t42::n21586(s, out),
		['з', s @ ..] => super::t42::n21587(s, out),
		['и', s @ ..] => super::t42::n21588(s, out),
		['к', s @ ..] => super::t42::n21589(s, out),
		['л', s @ ..] => super::t42::n21590(s, out),
		['м', s @ ..] => super::t42::n21591(s, out),
		['н', s @ ..] => super::t42::n21592(s, out),
		['п', s @ ..] => super::t42::n21593(s, out),
		['р', s @ ..] => super::t42::n21594(s, out),
		['с', s @ ..] => super::t42::n21595(s, out),
		['т', s @ ..] => super::t42::n21596(s, out),
		['у', s @ ..] => super::t42::n21597(s, out),
		['х', s @ ..] => super::t42::n21598(s, out),
		['ч', s @ ..] => super::t42::n21599(s, out),
		['ш', s @ ..] => super::t42::n21600(s, out),
		['щ', s @ ..] => super::t42::n21601(s, out),
		_ => false,
	}
}

/// The word богаделка.
pub(crate) fn n129858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128507);
	s.is_empty()
}

/// Words ending -овладелка.
pub(crate) fn n118638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117376);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 128508);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 135070);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иделка.
pub(crate) fn n47329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46868);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 75290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оделка.
pub(crate) fn n47330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46869);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 75291);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 99723);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117377);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128509);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135071);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сделка.
pub(crate) fn n47331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46870);
	s.is_empty()
}

/// Words ending -ыделка.
pub(crate) fn n47332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46871);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 75292);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99724);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -келка.
pub(crate) fn n21605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21486);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46872);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75293);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мелка.
pub(crate) fn n21606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21487);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 46873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пелка.
pub(crate) fn n21607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21488);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46874);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75295);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99727);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -релка.
pub(crate) fn n21608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46875);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 46876);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 75298);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99729);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117379);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 128510);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135072);
													match s {
														[] => true,
														['д', s @ ..] => super::t42::n140236(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75299);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117380);
									s.is_empty()
								}
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
			super::push(out, 46877);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75300);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 75301);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99731);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117381);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128511);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46878);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 46879);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75303);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -селка.
pub(crate) fn n21609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21490);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46880);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75304);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 75305);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 99732);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117382);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128512);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135073);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46881);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75306);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99733);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75307);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 99734);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117384);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128513);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135074);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телка.
pub(crate) fn n21610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21491);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46882);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 46883);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75308);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 75309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46884);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 75311);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117385);
									s.is_empty()
								}
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
			super::push(out, 46885);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75312);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99739);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 99740);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 117386);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 99741);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75313);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99742);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117388);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 128515);
											s.is_empty()
										}
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
					super::push(out, 75314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46886);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 46887);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 46888);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75317);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word целка.
pub(crate) fn n21611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21492);
	s.is_empty()
}

/// Words ending -челка.
pub(crate) fn n21612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21493);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 46889);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 46890);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 75318);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щелка.
pub(crate) fn n21613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21494);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46891);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46892);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75320);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99744);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117389);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128516);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46893);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илка.
pub(crate) fn n5661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5647);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21495);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46894);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99746);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75323);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99747);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 117390);
											s.is_empty()
										}
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
					super::push(out, 46895);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21496);
			match s {
				[] => true,
				['а', 'д', 'о', 'з', 'е', 'л', 'с', '\u{1}', s @ ..] => {
					super::push(out, 46896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21497);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46897);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 46898);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 75325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99748);
									s.is_empty()
								}
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
			super::push(out, 21498);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46899);
					match s {
						[] => true,
						['р', 'а', s @ ..] => {
							super::push(out, 75326);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99749);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117391);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128517);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46900);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 75327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46901);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 46902);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 75329);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									super::push(out, 99751);
									s.is_empty()
								}
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
			super::push(out, 21500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46903);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 46904);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 75330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 46905);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75331);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99753);
									s.is_empty()
								}
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
			super::push(out, 21501);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21502);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 21503);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46906);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 75332);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99754);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117392);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128518);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136480(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46907);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 46908);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99755);
									s.is_empty()
								}
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
			super::push(out, 21504);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46909);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 75334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 46910);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 75335);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 99756);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117393);
											s.is_empty()
										}
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
					super::push(out, 46911);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75336);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99757);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117394);
											s.is_empty()
										}
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
					super::push(out, 46912);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 46913);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 75338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21505);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 21506);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46914);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75339);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 75340);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99759);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117395);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128519);
													match s {
														[] => true,
														['к', s @ ..] => super::t42::n136481(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46915);
					s.is_empty()
				}
				['с', s @ ..] => super::t42::n47377(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t42::n21626(s, out),
		['ч', s @ ..] => {
			super::push(out, 21508);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46918);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 46919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t42::n76144(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t42::n21628(s, out),
		['щ', s @ ..] => super::t42::n21629(s, out),
		_ => false,
	}
}

/// Words ending -ллка.
pub(crate) fn n5662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5648);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21511);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46921);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75352);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олка.
pub(crate) fn n5663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5649);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21512);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46922);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99765);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117400);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128522);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46923);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75354);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 99766);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117401);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128523);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136482(s, out),
														_ => false,
													}
												}
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
							super::push(out, 75355);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99767);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117402);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 128524);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46924);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 75356);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 46925);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 75357);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 99769);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 117403);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128525);
													match s {
														[] => true,
														['ь', s @ ..] => super::t42::n136483(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46926);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75358);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99770);
									s.is_empty()
								}
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
			super::push(out, 21513);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46927);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75359);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99771);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117404);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128526);
													match s {
														[] => true,
														['н', s @ ..] => super::t42::n136484(s, out),
														_ => false,
													}
												}
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
							super::push(out, 75360);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75361);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99773);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117405);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 128527);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 99774);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117406);
											s.is_empty()
										}
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
					super::push(out, 46928);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99775);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 99776);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117407);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													super::push(out, 128528);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 99777);
									match s {
										[] => true,
										['е', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 117408);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21514);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46929);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 46930);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75362);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 75363);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 99778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t42::n47392(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21515);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46932);
					match s {
						[] => true,
						['к', s @ ..] => super::t42::n76160(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 21516);
			s.is_empty()
		}
		['к', s @ ..] => super::t42::n21636(s, out),
		['л', s @ ..] => super::t42::n21637(s, out),
		['м', s @ ..] => super::t42::n21638(s, out),
		['п', s @ ..] => super::t42::n21639(s, out),
		['с', s @ ..] => super::t42::n21640(s, out),
		['т', s @ ..] => super::t42::n21641(s, out),
		['х', s @ ..] => super::t42::n21642(s, out),
		_ => false,
	}
}

/// Words ending -улка.
pub(crate) fn n5664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5650);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21524);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 21525);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 46947);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75384);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99795);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117418);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128533);
													match s {
														[] => true,
														['с', s @ ..] => super::t42::n136486(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21526);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75386);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75387);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99797);
									s.is_empty()
								}
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
			super::push(out, 21527);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 46949);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99798);
									s.is_empty()
								}
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
			super::push(out, 21528);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 21529);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 46951);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 75389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылка.
pub(crate) fn n5665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5651);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21530);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46952);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75390);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 75391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21531);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 46953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21532);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 46954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21533);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 46955);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75393);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 75394);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99800);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117419);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128534);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21534);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46956);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 46957);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 46958);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99801);
									s.is_empty()
								}
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
			super::push(out, 21535);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 46959);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элка.
pub(crate) fn n5666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5652);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 21536);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 46960);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75398);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99802);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117420);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128535);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юлка.
pub(crate) fn n5667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5653);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21537);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ялка.
pub(crate) fn n5668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5654);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21538);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 46961);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75399);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99803);
									s.is_empty()
								}
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
			super::push(out, 21539);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46962);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 46963);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21540);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46964);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75401);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99804);
									match s {
										[] => true,
										['с', '-', s @ ..] => {
											super::push(out, 117421);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128536);
													match s {
														[] => true,
														['к', s @ ..] => super::t42::n136487(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21541);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 21542);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46965);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 75402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21543);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 46966);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 75403);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117422);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128537);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 117423);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128538);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21544);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21545);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46967);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 46968);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75405);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99805);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 117424);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 99806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхошижемка.
pub(crate) fn n129890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128539);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135083);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 138806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -земка.
pub(crate) fn n21681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21562);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 46986);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46987);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 75415);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99812);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 117429);
									s.is_empty()
								}
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
			super::push(out, 46988);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75416);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 99813);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 99814);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 99815);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117431);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 128540);
											s.is_empty()
										}
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
					super::push(out, 75417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 46989);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оемка.
pub(crate) fn n21685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21566);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 46992);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75420);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99817);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 117432);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128541);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135084);
													match s {
														[] => true,
														['в', s @ ..] => super::t42::n140242(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фемка.
pub(crate) fn n21687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21568);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46993);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 46994);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75421);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 99818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 46995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75422);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ешемка.
pub(crate) fn n47459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46997);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 75423);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 75424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъемка.
pub(crate) fn n21689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21570);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 46998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыемка.
pub(crate) fn n21690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21571);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 46999);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75425);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 75426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имка.
pub(crate) fn n5671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5657);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21572);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 47000);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 47001);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75427);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99821);
									s.is_empty()
								}
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
			super::push(out, 21573);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 47002);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21574);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47003);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75429);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99822);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 117433);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 128542);
													match s {
														[] => true,
														['а', s @ ..] => super::t42::n136490(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 47004);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 75430);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99823);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117434);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128543);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47005);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75431);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99824);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 117435);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21575);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 47006);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47007);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99825);
									s.is_empty()
								}
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
			super::push(out, 21576);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21577);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47008);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 75433);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99826);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117436);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128544);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21578);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 47009);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21579);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 47011);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 47012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 21580);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21581);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 47013);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75435);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99827);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117437);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128545);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21582);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 21583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47014);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75436);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99828);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117438);
											s.is_empty()
										}
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
					super::push(out, 47015);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75437);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99829);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117439);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21584);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 47016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 21585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ммка.
pub(crate) fn n5672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5658);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21586);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омка.
pub(crate) fn n5673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5659);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21587);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47018);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 75439);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99830);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117440);
											s.is_empty()
										}
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
					super::push(out, 47019);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 75440);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99831);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 117441);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21588);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47020);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75441);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 75442);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 99833);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117442);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 128546);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47021);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47022);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47023);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75443);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99834);
									s.is_empty()
								}
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
			super::push(out, 21590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47024);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75444);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 99835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 75445);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99836);
									s.is_empty()
								}
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
			super::push(out, 21591);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 47025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21592);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47026);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75446);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 75447);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47028);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 75448);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47029);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 21593);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47030);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмка.
pub(crate) fn n5674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5660);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21594);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 47031);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 75451);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99840);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117443);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умка.
pub(crate) fn n5675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5661);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21595);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 21596);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47032);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 75452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47033);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 75453);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 99841);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117444);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128547);
													match s {
														[] => true,
														['в', s @ ..] => super::t42::n136491(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 47034);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75454);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99842);
									s.is_empty()
								}
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
			super::push(out, 21597);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 47035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75455);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99843);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117445);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 128548);
													match s {
														[] => true,
														['л', s @ ..] => super::t42::n136492(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ымка.
pub(crate) fn n5676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5662);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21598);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юмка.
pub(crate) fn n5678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5664);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 21599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ямка.
pub(crate) fn n5679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5665);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21600);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анка.
pub(crate) fn n5680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5666);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21601);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47036);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47037);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 75456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 47038);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47039);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 75458);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47040);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 47041);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75460);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99845);
									s.is_empty()
								}
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
			super::push(out, 21602);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47042);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75461);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99846);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117446);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128549);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47043);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75462);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 75463);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 99847);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117447);
											s.is_empty()
										}
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
					super::push(out, 47044);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75464);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 47045);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 47046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47047);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75466);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 75467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47048);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75468);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 75469);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 75470);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47049);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47050);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 47051);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47052);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 75472);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 75473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21604);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47053);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75474);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 75475);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 75476);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 75477);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47054);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 75478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 47055);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75479);
							match s {
								[] => true,
								['р', 'г', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117448);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128550);
													match s {
														[] => true,
														['с', s @ ..] => super::t42::n136493(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 117449);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128551);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136494(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 128552);
													match s {
														[] => true,
														['в', s @ ..] => super::t42::n136495(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 47056);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75480);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 47057);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75481);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t42::n76276(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t42::n21724(s, out),
		['з', s @ ..] => super::t42::n21725(s, out),
		['и', s @ ..] => super::t42::n21726(s, out),
		['к', s @ ..] => super::t42::n21727(s, out),
		['л', s @ ..] => super::t42::n21728(s, out),
		['м', s @ ..] => super::t42::n21729(s, out),
		['н', s @ ..] => super::t42::n21730(s, out),
		['п', s @ ..] => super::t42::n21731(s, out),
		['р', s @ ..] => super::t42::n21732(s, out),
		['с', s @ ..] => super::t42::n21733(s, out),
		['т', s @ ..] => super::t42::n21734(s, out),
		['у', s @ ..] => {
			super::push(out, 21616);
			s.is_empty()
		}
		['х', s @ ..] => super::t42::n21736(s, out),
		['ц', s @ ..] => super::t42::n21737(s, out),
		['ч', s @ ..] => super::t42::n21738(s, out),
		['ш', s @ ..] => super::t42::n21739(s, out),
		['щ', s @ ..] => super::t42::n21740(s, out),
		['ю', s @ ..] => super::t42::n21741(s, out),
		_ => false,
	}
}

/// Words ending -енка.
pub(crate) fn n5681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5667);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21623);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 47158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21624);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47159);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47160);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 75607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 47161);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21625);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47163);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 75609);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99942);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117484);
											s.is_empty()
										}
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
					super::push(out, 47164);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 75610);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 99943);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117485);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 128569);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47165);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75611);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 99944);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 75612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75613);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99946);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117487);
											s.is_empty()
										}
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
					super::push(out, 47166);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75614);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99947);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 117488);
											s.is_empty()
										}
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
					super::push(out, 47167);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75615);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99948);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 117489);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128570);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 99949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75616);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99950);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 117490);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128571);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 99951);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117491);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 75617);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99952);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 117492);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128572);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21626);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47169);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75618);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99953);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117493);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128573);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21627);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 47170);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75619);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99954);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117494);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128574);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t42::n47636(s, out),
				['ж', s @ ..] => super::t42::n47637(s, out),
				['з', s @ ..] => {
					super::push(out, 47173);
					match s {
						[] => true,
						['и', s @ ..] => super::t42::n76420(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t42::n47639(s, out),
				['о', s @ ..] => super::t42::n47640(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t42::n21747(s, out),
		['ж', s @ ..] => super::t42::n21748(s, out),
		['з', s @ ..] => super::t42::n21749(s, out),
		['л', s @ ..] => super::t42::n21750(s, out),
		['м', s @ ..] => super::t42::n21751(s, out),
		['н', s @ ..] => super::t42::n21752(s, out),
		['о', s @ ..] => super::t42::n21753(s, out),
		['п', s @ ..] => super::t42::n21754(s, out),
		['р', s @ ..] => super::t42::n21755(s, out),
		['с', s @ ..] => super::t42::n21756(s, out),
		['т', s @ ..] => super::t42::n21757(s, out),
		['ц', s @ ..] => super::t42::n21758(s, out),
		['ч', s @ ..] => super::t42::n21759(s, out),
		['ш', s @ ..] => super::t42::n21760(s, out),
		['щ', s @ ..] => super::t42::n21761(s, out),
		_ => false,
	}
}

/// Words ending -инка.
pub(crate) fn n5682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5668);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21643);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21644);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47268);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75764);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 47269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21645);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75765);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 75766);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100083);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 100084);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117577);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128614);
													match s {
														[] => true,
														['п', s @ ..] => super::t42::n136524(s, out),
														_ => false,
													}
												}
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
							super::push(out, 75767);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100085);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117578);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 128615);
													match s {
														[] => true,
														['а', s @ ..] => super::t42::n136525(s, out),
														_ => false,
													}
												}
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
							super::push(out, 75768);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47271);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 47272);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75769);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 100087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 47273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75770);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 100088);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117579);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128616);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47274);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75771);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 75772);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 100089);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117580);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128617);
													s.is_empty()
												}
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
							super::push(out, 75773);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 100090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 75774);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117581);
											s.is_empty()
										}
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
					super::push(out, 47275);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75775);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 100092);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 117582);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128618);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136526(s, out),
														_ => false,
													}
												}
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
							super::push(out, 75776);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 100093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47276);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75777);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 75778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 75779);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 100095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100096);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 117583);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128619);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47277);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 47278);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 75782);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100098);
									match s {
										[] => true,
										['а', s @ ..] => super::t42::n118846(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t42::n47744(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t42::n21765(s, out),
		['г', s @ ..] => super::t42::n21766(s, out),
		['д', s @ ..] => super::t42::n21767(s, out),
		['е', s @ ..] => super::t42::n21768(s, out),
		['ж', s @ ..] => super::t42::n21769(s, out),
		['з', s @ ..] => super::t42::n21770(s, out),
		['и', s @ ..] => super::t42::n21771(s, out),
		['к', s @ ..] => super::t42::n21772(s, out),
		['л', s @ ..] => super::t42::n21773(s, out),
		['м', s @ ..] => super::t42::n21774(s, out),
		['н', s @ ..] => super::t42::n21775(s, out),
		['о', s @ ..] => super::t42::n21776(s, out),
		['п', s @ ..] => super::t42::n21777(s, out),
		['р', s @ ..] => super::t42::n21778(s, out),
		['с', s @ ..] => super::t42::n21779(s, out),
		['т', s @ ..] => super::t42::n21780(s, out),
		['у', s @ ..] => super::t42::n21781(s, out),
		['ф', s @ ..] => super::t42::n21782(s, out),
		['х', s @ ..] => super::t42::n21783(s, out),
		['ц', s @ ..] => super::t42::n21784(s, out),
		['ч', s @ ..] => super::t42::n21785(s, out),
		['ш', s @ ..] => super::t42::n21786(s, out),
		['щ', s @ ..] => super::t42::n21787(s, out),
		['ь', s @ ..] => super::t42::n21788(s, out),
		_ => false,
	}
}

/// Words ending -онка.
pub(crate) fn n5685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5671);
	match s {
		[] => true,
		['б', 'а', s @ ..] => {
			super::push(out, 21672);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 47439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'з', s @ ..] => {
			super::push(out, 21673);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47440);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76042);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100326);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 117718);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128679);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21674);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					super::push(out, 47441);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76043);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100327);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117719);
											s.is_empty()
										}
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
					super::push(out, 47442);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76044);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100328);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117720);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128680);
													s.is_empty()
												}
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
							super::push(out, 76045);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100329);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117721);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128681);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47443);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47444);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76046);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100330);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 100331);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117722);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 76047);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100332);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117723);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128682);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136543(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 47445);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76048);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100333);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117724);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128683);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47446);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 47447);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47448);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100334);
									s.is_empty()
								}
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
			super::push(out, 21675);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47449);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76050);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100335);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117725);
											s.is_empty()
										}
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
					super::push(out, 47450);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76051);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 100336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117726);
											s.is_empty()
										}
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
					super::push(out, 47451);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 21676);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47452);
					s.is_empty()
				}
				['е', 'л', s @ ..] => {
					super::push(out, 47453);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76053);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100337);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117727);
											s.is_empty()
										}
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
						['н', s @ ..] => {
							super::push(out, 76054);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 76055);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 21677);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47455);
					match s {
						[] => true,
						['м', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100339);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 100340);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117728);
											s.is_empty()
										}
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
					super::push(out, 47456);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t42::n21797(s, out),
		['к', s @ ..] => {
			super::push(out, 21679);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47460);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47461);
					s.is_empty()
				}
				['с', s @ ..] => super::t42::n47932(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t42::n21799(s, out),
		['м', s @ ..] => super::t42::n21800(s, out),
		['п', s @ ..] => super::t42::n21801(s, out),
		['р', s @ ..] => super::t42::n21802(s, out),
		['с', s @ ..] => super::t42::n21803(s, out),
		['т', s @ ..] => super::t42::n21804(s, out),
		['ф', s @ ..] => super::t42::n21805(s, out),
		['х', 'у', s @ ..] => super::t42::n21806(s, out),
		['ч', s @ ..] => super::t42::n21807(s, out),
		['ш', s @ ..] => super::t42::n21808(s, out),
		['ь', s @ ..] => super::t42::n21809(s, out),
		_ => false,
	}
}

/// Words ending -с-форсунка.
pub(crate) fn n130044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128692);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135142);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 138835);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янка.
pub(crate) fn n5691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21707);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 47529);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 76129);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100391);
									s.is_empty()
								}
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
			super::push(out, 21708);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47530);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 47531);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21709);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76131);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 100392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 76132);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 76133);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47532);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76134);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 47533);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76135);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47534);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76136);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100397);
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
						['б', s @ ..] => {
							super::push(out, 76137);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 76138);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 100399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 76139);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100400);
									s.is_empty()
								}
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
			super::push(out, 21710);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76140);
							s.is_empty()
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100401);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 100402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 47535);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 76141);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47536);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 47537);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76142);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100404);
									s.is_empty()
								}
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
			super::push(out, 21711);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 47538);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 47539);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 47540);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76143);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100405);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117754);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128693);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47541);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76144);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 47542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 47543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47544);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 76146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 47545);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 100407);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 100408);
									s.is_empty()
								}
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
			super::push(out, 21712);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 47546);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76147);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100409);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117755);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 76148);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100410);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117756);
											s.is_empty()
										}
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
					super::push(out, 47547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76149);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 76150);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 100411);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117757);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 128694);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136549(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t42::n48022(s, out),
				['с', s @ ..] => super::t42::n48023(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t43::n21833(s, out),
		['м', s @ ..] => super::t43::n21834(s, out),
		['н', s @ ..] => super::t43::n21835(s, out),
		['о', s @ ..] => super::t43::n21836(s, out),
		['п', s @ ..] => super::t43::n21837(s, out),
		['р', s @ ..] => super::t43::n21838(s, out),
		['с', s @ ..] => super::t43::n21839(s, out),
		['т', s @ ..] => super::t43::n21840(s, out),
		['ф', s @ ..] => super::t43::n21841(s, out),
		['ь', s @ ..] => super::t43::n21842(s, out),
		_ => false,
	}
}

/// Words ending -пол-яблока.
pub(crate) fn n130062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128710);
	s.is_empty()
}

/// Words ending -рока.
pub(crate) fn n5701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5687);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21737);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 47620);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 47621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 47622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21738);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 47623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76241);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 76242);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100480);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117791);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21739);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 47624);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76243);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100481);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117792);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сока.
pub(crate) fn n5702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5688);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21740);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 47625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21741);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тока.
pub(crate) fn n5703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5689);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21743);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21744);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 47627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21745);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47628);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76245);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100482);
									s.is_empty()
								}
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
			super::push(out, 21746);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47629);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word растрепка.
pub(crate) fn n130063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128711);
	s.is_empty()
}

/// Words ending -трясожопка.
pub(crate) fn n130064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128712);
	s.is_empty()
}

/// Words ending -рпка.
pub(crate) fn n5709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5695);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21769);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 47647);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 76260);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100494);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упка.
pub(crate) fn n5710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5696);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 21770);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 47648);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76261);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100495);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117800);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128713);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21771);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 21772);
			match s {
				[] => true,
				['с', 'ы', s @ ..] => {
					super::push(out, 47649);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76262);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпка.
pub(crate) fn n5711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5697);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 21773);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47650);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 47651);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47652);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47653);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76264);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100497);
									s.is_empty()
								}
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
			super::push(out, 21774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япка.
pub(crate) fn n5712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5698);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21775);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 47656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21776);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 47657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76266);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 76267);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100498);
									s.is_empty()
								}
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
			super::push(out, 21777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -занзибарка.
pub(crate) fn n130066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128714);
	s.is_empty()
}

/// Words ending -варка.
pub(crate) fn n21899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21779);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47659);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76271);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 76272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 76273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47660);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 76274);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100502);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128715);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47661);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47662);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76275);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100503);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117803);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128716);
											s.is_empty()
										}
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
					super::push(out, 76276);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 76277);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 76278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100504);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 100505);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117804);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128717);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47663);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76279);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117805);
									s.is_empty()
								}
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
			super::push(out, 47664);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 47665);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гарка.
pub(crate) fn n21900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21780);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47666);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47667);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 76283);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47668);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47669);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76285);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'а', '\u{1}', s @ ..] => {
			super::push(out, 47670);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 47671);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 76286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 47672);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одарка.
pub(crate) fn n48151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47674);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 76289);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 100510);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117806);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сударка.
pub(crate) fn n48152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47675);
	s.is_empty()
}

/// Words ending -жарка.
pub(crate) fn n21902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21782);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 47676);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76290);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 76291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зарка.
pub(crate) fn n21903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21783);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47678);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -карка.
pub(crate) fn n21904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21784);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 47679);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -марка.
pub(crate) fn n21905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21785);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47680);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47681);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47682);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76295);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 76296);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 100512);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117807);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128719);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 135151);
													match s {
														[] => true,
														['а', s @ ..] => super::t43::n140274(s, out),
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
									super::push(out, 117808);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128720);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135152);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47683);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 47684);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76297);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100513);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128721);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47685);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нарка.
pub(crate) fn n21906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21786);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47686);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76300);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 76301);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47687);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76302);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 76303);
					s.is_empty()
				}
				['р', 'е', 'т', 'е', 'в', '\u{1}', s @ ..] => {
					super::push(out, 76304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76305);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 76306);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 100517);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 76307);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100518);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117811);
									s.is_empty()
								}
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
			super::push(out, 47688);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -парка.
pub(crate) fn n21907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21787);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47689);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47690);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47691);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47692);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 47693);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рарка.
pub(crate) fn n21908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21788);
	s.is_empty()
}

/// Words ending -сарка.
pub(crate) fn n21909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21789);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47694);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76311);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100521);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117812);
									s.is_empty()
								}
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
				['л', s @ ..] => {
					super::push(out, 76312);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 76313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47695);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76314);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тарка.
pub(crate) fn n21910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21790);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76315);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 76316);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 76317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47696);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76318);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100525);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117813);
									s.is_empty()
								}
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
				['г', s @ ..] => {
					super::push(out, 76319);
					s.is_empty()
				}
				['н', 'а', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100526);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 100527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'е', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76320);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 76321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'у', s @ ..] => {
			super::push(out, 47697);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47698);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 76323);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100530);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117815);
									s.is_empty()
								}
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
			super::push(out, 47699);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 47700);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76325);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уарка.
pub(crate) fn n21911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21791);
	s.is_empty()
}

/// Words ending -харка.
pub(crate) fn n21912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21792);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47701);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76326);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47702);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76327);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 76328);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -царка.
pub(crate) fn n21913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21793);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 47703);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76329);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100534);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 117816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чарка.
pub(crate) fn n21914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21794);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47704);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 47705);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 47706);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76331);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шарка.
pub(crate) fn n21915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21795);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47707);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76332);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100536);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117817);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128722);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерка.
pub(crate) fn n5714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5700);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21796);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47708);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76333);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100537);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 117818);
											s.is_empty()
										}
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
					super::push(out, 47709);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76334);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100538);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117819);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128723);
													match s {
														[] => true,
														['к', s @ ..] => super::t43::n136559(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 47710);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 47711);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76336);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 100539);
									s.is_empty()
								}
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
			super::push(out, 21797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47712);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47713);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 47714);
					match s {
						[] => true,
						['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 76337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47715);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76338);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100540);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117820);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 76339);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47717);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76341);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100543);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117821);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 76342);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100544);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117822);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 76343);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 76344);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100545);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117823);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128724);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 100546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 47718);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 47719);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76345);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 100547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47720);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 76346);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 47721);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100549);
									s.is_empty()
								}
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
			super::push(out, 21798);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 47722);
					match s {
						[] => true,
						['е', 'в', '\u{1}', s @ ..] => {
							super::push(out, 76348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47723);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76349);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 100550);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117824);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21799);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47724);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47725);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76351);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100551);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 117825);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 76352);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100552);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117826);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128725);
													s.is_empty()
												}
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
							super::push(out, 76353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100553);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117827);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t43::n48209(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t43::n21920(s, out),
		['з', s @ ..] => super::t43::n21921(s, out),
		['к', s @ ..] => super::t43::n21922(s, out),
		['л', s @ ..] => super::t43::n21923(s, out),
		['м', s @ ..] => super::t43::n21924(s, out),
		['н', s @ ..] => super::t43::n21925(s, out),
		['п', s @ ..] => super::t43::n21926(s, out),
		['с', s @ ..] => super::t43::n21927(s, out),
		['т', s @ ..] => super::t43::n21928(s, out),
		['ф', s @ ..] => super::t43::n21929(s, out),
		['ц', s @ ..] => super::t43::n21930(s, out),
		['ч', s @ ..] => super::t43::n21931(s, out),
		['ш', s @ ..] => super::t43::n21932(s, out),
		['щ', s @ ..] => super::t43::n21933(s, out),
		['ь', s @ ..] => super::t43::n21934(s, out),
		_ => false,
	}
}

/// Words ending -ирка.
pub(crate) fn n5715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5701);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21815);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 21816);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47776);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21817);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47777);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76422);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 100608);
									s.is_empty()
								}
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
			super::push(out, 21818);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 47778);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76423);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100609);
									s.is_empty()
								}
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
			super::push(out, 21819);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47779);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76424);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100610);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117855);
											s.is_empty()
										}
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
					super::push(out, 47780);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 47781);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76426);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100611);
									s.is_empty()
								}
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
			super::push(out, 21820);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21821);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47783);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 47784);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76427);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 100612);
									s.is_empty()
								}
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
			super::push(out, 21822);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47785);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76428);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100613);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117856);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128738);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47786);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76429);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100614);
									s.is_empty()
								}
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
			super::push(out, 21823);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 47787);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76430);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100615);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117857);
											s.is_empty()
										}
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
					super::push(out, 47788);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21824);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 21825);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47789);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76432);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 100616);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117858);
											s.is_empty()
										}
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
					super::push(out, 47790);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 47791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 21826);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47792);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76433);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100617);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орка.
pub(crate) fn n5716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5702);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21827);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47793);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 47794);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47795);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76435);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100618);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117860);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128739);
													s.is_empty()
												}
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
							super::push(out, 76436);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 47796);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47797);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76438);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100620);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 76439);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 100621);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117862);
											s.is_empty()
										}
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
					super::push(out, 47798);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 76440);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100622);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 117863);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128740);
													s.is_empty()
												}
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
							super::push(out, 76441);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100623);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117864);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128741);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 100624);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117865);
											s.is_empty()
										}
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
					super::push(out, 47799);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47800);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 47801);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76444);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100625);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 100626);
									s.is_empty()
								}
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
			super::push(out, 21828);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					super::push(out, 47802);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76445);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 100627);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117866);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128742);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47803);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76446);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47804);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 76447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', s @ ..] => {
					super::push(out, 47805);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 76448);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100629);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117867);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128743);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21829);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47807);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47808);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76450);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 100630);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117868);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 76451);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100631);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117869);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 128744);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21830);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47809);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76452);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100632);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 117870);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 100633);
									match s {
										[] => true,
										['л', s @ ..] => super::t43::n119133(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t43::n48294(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t43::n21951(s, out),
		['з', s @ ..] => super::t43::n21952(s, out),
		['и', s @ ..] => super::t43::n21953(s, out),
		['й', s @ ..] => super::t43::n21954(s, out),
		['к', s @ ..] => super::t43::n21955(s, out),
		['л', s @ ..] => super::t43::n21956(s, out),
		['м', s @ ..] => super::t43::n21957(s, out),
		['н', s @ ..] => super::t43::n21958(s, out),
		['п', s @ ..] => super::t43::n21959(s, out),
		['с', s @ ..] => super::t43::n21960(s, out),
		['т', s @ ..] => super::t43::n21961(s, out),
		['ф', s @ ..] => super::t43::n21962(s, out),
		['х', s @ ..] => super::t43::n21963(s, out),
		['ц', s @ ..] => super::t43::n21964(s, out),
		['ш', s @ ..] => super::t43::n21965(s, out),
		_ => false,
	}
}

/// Words ending -ррка.
pub(crate) fn n5717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5703);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21846);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 47852);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76491);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урка.
pub(crate) fn n5718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5704);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21847);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21848);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 47853);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 47854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47855);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21850);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47857);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76492);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47858);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47859);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 76494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 47860);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47861);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 47862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21851);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47863);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76497);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100668);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117888);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47864);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47865);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 47866);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47867);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47868);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 47869);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 76501);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 100669);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117889);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128755);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21853);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 47870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47871);
					s.is_empty()
				}
				['о', 'д', s @ ..] => {
					super::push(out, 47872);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76502);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 100670);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117890);
											s.is_empty()
										}
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
					super::push(out, 47873);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47874);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100671);
									s.is_empty()
								}
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
			super::push(out, 21855);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47876);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47877);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 47878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21857);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47879);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'а', s @ ..] => {
			super::push(out, 21858);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 47880);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76505);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100672);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117891);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 21860);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47882);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 47883);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76507);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 100673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 47884);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 47885);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76509);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 76510);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100674);
									s.is_empty()
								}
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
			super::push(out, 21861);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47886);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 21862);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47887);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 47888);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76512);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 47889);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырка.
pub(crate) fn n5719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5705);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21863);
			s.is_empty()
		}
		['д', 'о', s @ ..] => {
			super::push(out, 21864);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 47890);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76514);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100676);
									s.is_empty()
								}
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
			super::push(out, 21865);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 21866);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21867);
			match s {
				[] => true,
				['а', 'г', 'о', 'б', '\u{1}', s @ ..] => {
					super::push(out, 47892);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 47893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76515);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 100677);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117892);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128756);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрка.
pub(crate) fn n5720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5706);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21868);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярка.
pub(crate) fn n5721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5707);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21870);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21871);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76516);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 100678);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117893);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 76517);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 76518);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 47894);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76519);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 100680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'п', s @ ..] => {
					super::push(out, 47895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21872);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 47896);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 47897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21873);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 47898);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 21874);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 47899);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гондураска.
pub(crate) fn n130109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128757);
	s.is_empty()
}

/// Words ending -ракобеска.
pub(crate) fn n119161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117899);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 128758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -веска.
pub(crate) fn n22007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21887);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47918);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 47919);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кеска.
pub(crate) fn n22008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21888);
	s.is_empty()
}

/// Words ending -леска.
pub(crate) fn n22009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21889);
	s.is_empty()
}

/// Words ending -меска.
pub(crate) fn n22010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21890);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47920);
			match s {
				[] => true,
				['т', 'с', '-', s @ ..] => {
					super::push(out, 76536);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100691);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117900);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128759);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135163);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реска.
pub(crate) fn n22011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47921);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47922);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 47923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76538);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 76539);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100692);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117901);
									s.is_empty()
								}
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
			super::push(out, 47924);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теска.
pub(crate) fn n22012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21892);
	s.is_empty()
}

/// Words ending -ческа.
pub(crate) fn n22013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21893);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 47925);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47926);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76540);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100693);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128760);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47927);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 47928);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иска.
pub(crate) fn n5724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21894);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21895);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47932);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47933);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76543);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 47934);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 47935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21898);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47936);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76544);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100696);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 117903);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21899);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 47937);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76545);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47938);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100698);
									s.is_empty()
								}
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
			super::push(out, 21900);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 47939);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47940);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47941);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76547);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100699);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 117904);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47942);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 47943);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47944);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76548);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 47945);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76549);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100701);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117905);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21902);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47946);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21903);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 47947);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76551);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100702);
									s.is_empty()
								}
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
			super::push(out, 21904);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 47948);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76552);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йска.
pub(crate) fn n5725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5711);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21905);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 47949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мска.
pub(crate) fn n5726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5712);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 21906);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47950);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оска.
pub(crate) fn n5727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5713);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21907);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 47951);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76553);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117906);
											s.is_empty()
										}
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
					super::push(out, 47952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47953);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 47954);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76554);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100705);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117907);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128761);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47955);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21909);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21910);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21911);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 47957);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76556);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100706);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 117908);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21912);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 47958);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 47959);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 47960);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 47961);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 47962);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 47963);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76558);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 100708);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117909);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128762);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 47964);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76559);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 76560);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 76561);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100710);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 76562);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 100711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100712);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117910);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 100713);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117911);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 76563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 47965);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 47966);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 47967);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100714);
									s.is_empty()
								}
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
			super::push(out, 21913);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 47968);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76565);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 76566);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100716);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117912);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128763);
													match s {
														[] => true,
														['с', s @ ..] => super::t43::n136570(s, out),
														_ => false,
													}
												}
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
							super::push(out, 76567);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100717);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117913);
											s.is_empty()
										}
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
					super::push(out, 47969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21914);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 47970);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 47971);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76568);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 100718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 47972);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76569);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t43::n48459(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t43::n22035(s, out),
		['ф', s @ ..] => {
			super::push(out, 21916);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47976);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пска.
pub(crate) fn n5728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5714);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 47977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сска.
pub(crate) fn n5729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5715);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21918);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 47978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47979);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 47980);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76577);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100724);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117918);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21919);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 47981);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уска.
pub(crate) fn n5730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5716);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21920);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 47982);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76579);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 100725);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 76580);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 100726);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117920);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128765);
													match s {
														[] => true,
														['к', s @ ..] => super::t43::n136571(s, out),
														_ => false,
													}
												}
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
							super::push(out, 76581);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100727);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117921);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128766);
													match s {
														[] => true,
														['т', s @ ..] => super::t43::n136572(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21921);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 47983);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76582);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100728);
									s.is_empty()
								}
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
			super::push(out, 21922);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 47984);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 76583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21923);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 76584);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 76585);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100729);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117922);
											s.is_empty()
										}
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
					super::push(out, 47985);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 47986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21924);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47987);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 47988);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 76587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21925);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 47989);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76588);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 100730);
									s.is_empty()
								}
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
			super::push(out, 21926);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 47990);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76589);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 76590);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100731);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117923);
											s.is_empty()
										}
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
					super::push(out, 47991);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 76591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100732);
									s.is_empty()
								}
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
			super::push(out, 21927);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыска.
pub(crate) fn n5731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5717);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 21928);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 47992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эска.
pub(crate) fn n5732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5718);
	s.is_empty()
}

/// Words ending -яска.
pub(crate) fn n5733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5719);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21929);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 47993);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 47994);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76593);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 100733);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117924);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128767);
													match s {
														[] => true,
														['м', s @ ..] => super::t43::n136573(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 117925);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128768);
													match s {
														[] => true,
														['с', s @ ..] => super::t43::n136574(s, out),
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
									super::push(out, 100734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 47995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21930);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21931);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 47996);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76594);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 100735);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117926);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128769);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t43::n136575(s, out),
														_ => false,
													}
												}
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
							super::push(out, 76595);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 76596);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100736);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117927);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кробатка.
pub(crate) fn n101819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 117928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рбатка.
pub(crate) fn n48486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47999);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 76599);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 100738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убатка.
pub(crate) fn n48487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48000);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 76600);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 100739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ватка.
pub(crate) fn n22054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21933);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 48001);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76601);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100740);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 100741);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117929);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128770);
											s.is_empty()
										}
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
					super::push(out, 76602);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100742);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117930);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128771);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76603);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гатка.
pub(crate) fn n22055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21934);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48004);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76604);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 76605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', '\u{1}', s @ ..] => {
			super::push(out, 48005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -датка.
pub(crate) fn n22056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21935);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 48006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48007);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76607);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100744);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117931);
									s.is_empty()
								}
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
			super::push(out, 48008);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76608);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еатка.
pub(crate) fn n22057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21936);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 48009);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76609);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100746);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ежатка.
pub(crate) fn n22058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21937);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 48010);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76610);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100747);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иатка.
pub(crate) fn n22059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21938);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 48011);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 48012);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -катка.
pub(crate) fn n22060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21939);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 48014);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 48015);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -латка.
pub(crate) fn n22061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21940);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48016);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 48017);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 48018);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76614);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 76615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100749);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117934);
									s.is_empty()
								}
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
			super::push(out, 48019);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48020);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', 'о', 'х', '\u{1}', s @ ..] => {
			super::push(out, 48021);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 48022);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матка.
pub(crate) fn n22062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21941);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 48023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76618);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100751);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117935);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128772);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													super::push(out, 135170);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', 'и', 'в', 'а', '\u{1}', s @ ..] => {
			super::push(out, 48024);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48025);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 48026);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48027);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 48028);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 48029);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76620);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 100752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натка.
pub(crate) fn n22063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48030);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76621);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76622);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 76623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 48031);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48032);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76625);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 76626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 48033);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76627);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48034);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 76628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48035);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48036);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76630);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оатка.
pub(crate) fn n22064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21943);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 48037);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -патка.
pub(crate) fn n22065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21944);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48038);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48039);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76632);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48040);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76633);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 76634);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100757);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 76635);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100758);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 76636);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 76637);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100759);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратка.
pub(crate) fn n22066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21945);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 48041);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 48042);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76639);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100760);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117940);
									s.is_empty()
								}
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
			super::push(out, 48043);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76640);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 76641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48044);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76642);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 76643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76644);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 76645);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100761);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 100762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48046);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76646);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сатка.
pub(crate) fn n22067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21946);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48047);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76647);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 100764);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117941);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128773);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 135171);
													match s {
														[] => true,
														['о', s @ ..] => super::t43::n140284(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48048);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48049);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76648);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -татка.
pub(crate) fn n22068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21947);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48050);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 48051);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 76649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76650);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100765);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 117942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 100766);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 76651);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48052);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76652);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 48053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хатка.
pub(crate) fn n22069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21948);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 48054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76653);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цатка.
pub(crate) fn n22070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21949);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 48055);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чатка.
pub(crate) fn n22071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21950);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 48056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76654);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100770);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117945);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128774);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135172);
													match s {
														[] => true,
														['б', s @ ..] => super::t43::n140285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48057);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76655);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 100771);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 117946);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128775);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135173);
													match s {
														[] => true,
														['т', s @ ..] => super::t43::n140286(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48058);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76656);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 100772);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117947);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 128776);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135174);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48059);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 48060);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 48061);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76657);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 48062);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76658);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100774);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117948);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128777);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135175);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -втка.
pub(crate) fn n5735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5721);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21951);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 48063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етка.
pub(crate) fn n5736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5722);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21952);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48064);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100775);
									s.is_empty()
								}
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
			super::push(out, 21953);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48065);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76660);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100776);
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
						['\u{1}', s @ ..] => {
							super::push(out, 76661);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 76662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 48066);
					match s {
						[] => true,
						['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 76663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48067);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 48068);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', 'а', s @ ..] => {
			super::push(out, 21955);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 48069);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 76665);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100777);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117949);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48070);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76666);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 100778);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117950);
											s.is_empty()
										}
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
						['м', s @ ..] => {
							super::push(out, 76667);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100779);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117951);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128778);
													s.is_empty()
												}
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
							super::push(out, 76668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21956);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48071);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 76669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 48072);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48073);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100780);
									s.is_empty()
								}
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
			super::push(out, 21957);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48074);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76672);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 76673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 48075);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 48076);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76674);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100782);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 117952);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 76675);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100783);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117953);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 128779);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t43::n136582(s, out),
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
									match s {
										[] => true,
										['в', 'е', 'д', '\u{1}', s @ ..] => {
											super::push(out, 117954);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 117955);
											s.is_empty()
										}
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
					super::push(out, 48077);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48078);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 76676);
							match s {
								[] => true,
								['у', s @ ..] => super::t43::n101867(s, out),
								_ => false,
							}
						}
						['г', s @ ..] => super::t43::n77486(s, out),
						['и', s @ ..] => {
							super::push(out, 76678);
							s.is_empty()
						}
						['л', s @ ..] => super::t43::n77488(s, out),
						['н', s @ ..] => super::t43::n77489(s, out),
						['р', s @ ..] => super::t43::n77490(s, out),
						['т', s @ ..] => super::t43::n77491(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t43::n48570(s, out),
				['р', s @ ..] => super::t43::n48571(s, out),
				['т', s @ ..] => super::t43::n48572(s, out),
				['у', s @ ..] => super::t43::n48573(s, out),
				['х', s @ ..] => super::t43::n48574(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t43::n22080(s, out),
		['н', s @ ..] => super::t43::n22081(s, out),
		['п', s @ ..] => super::t43::n22082(s, out),
		['р', s @ ..] => super::t43::n22083(s, out),
		['с', s @ ..] => super::t43::n22084(s, out),
		['т', s @ ..] => super::t43::n22085(s, out),
		['ф', s @ ..] => super::t43::n22086(s, out),
		['ч', s @ ..] => super::t43::n22087(s, out),
		['ш', s @ ..] => super::t43::n22088(s, out),
		['щ', s @ ..] => super::t43::n22089(s, out),
		_ => false,
	}
}

/// Words ending -итка.
pub(crate) fn n5737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5723);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21968);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 48105);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 76709);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100808);
									s.is_empty()
								}
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
			super::push(out, 21969);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48106);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48107);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76710);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 100809);
									s.is_empty()
								}
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
			super::push(out, 21970);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 21971);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48109);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 48110);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76712);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 100810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48111);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76713);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 100811);
									s.is_empty()
								}
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
			super::push(out, 21972);
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48112);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76714);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100812);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117976);
											s.is_empty()
										}
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
					super::push(out, 48113);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21973);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 48115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21974);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48116);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 48117);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 76717);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 76718);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 48118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76719);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100815);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117977);
											s.is_empty()
										}
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
					super::push(out, 48119);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76720);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100816);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117978);
											s.is_empty()
										}
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
					super::push(out, 48120);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 48121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 48122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21975);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48123);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76723);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 76724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76725);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 76726);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100817);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 117979);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128787);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 48124);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 48125);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48126);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76729);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100818);
									s.is_empty()
								}
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
				['г', s @ ..] => {
					super::push(out, 48127);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 76730);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 76731);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 100819);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117980);
											s.is_empty()
										}
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
					super::push(out, 48128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76732);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 48129);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76733);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48130);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 48131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 48132);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76735);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48133);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 48134);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76736);
							match s {
								[] => true,
								['в', s @ ..] => super::t43::n101906(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t43::n22103(s, out),
		['с', s @ ..] => super::t43::n22104(s, out),
		['т', s @ ..] => super::t43::n22105(s, out),
		['у', s @ ..] => super::t43::n22106(s, out),
		['ф', s @ ..] => super::t43::n22107(s, out),
		['ц', s @ ..] => super::t43::n22108(s, out),
		['ч', s @ ..] => super::t43::n22109(s, out),
		['щ', s @ ..] => super::t43::n22110(s, out),
		_ => false,
	}
}

/// Words ending -йтка.
pub(crate) fn n5738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5724);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лтка.
pub(crate) fn n5739(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21985);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21986);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48146);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтка.
pub(crate) fn n5740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5725);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21987);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21988);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 48147);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76747);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100833);
									match s {
										[] => true,
										['е', 'д', '\u{1}', s @ ..] => {
											super::push(out, 117985);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21989);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 48148);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 76748);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отка.
pub(crate) fn n5741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5726);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21990);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48149);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100835);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 117987);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128789);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 100836);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 100837);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 100838);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 100839);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117988);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128790);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 128791);
													match s {
														[] => true,
														['д', s @ ..] => super::t43::n136588(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21991);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48150);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100840);
									s.is_empty()
								}
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
			super::push(out, 21992);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 48151);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 76750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48152);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76751);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100841);
									s.is_empty()
								}
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
			super::push(out, 21993);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48153);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 76752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48154);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21994);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 48155);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76754);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100842);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117989);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 128792);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t43::n136589(s, out),
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
									super::push(out, 100843);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117990);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 100844);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117991);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128793);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t43::n136590(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48156);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48157);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76756);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100845);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 117992);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 100846);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117993);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128794);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 100847);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 76757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 48158);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 48159);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76758);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 100848);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117995);
											s.is_empty()
										}
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
					super::push(out, 48160);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76759);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 100849);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117996);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128795);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48161);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76760);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 100850);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t43::n119259(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => super::t43::n48658(s, out),
				['р', s @ ..] => super::t43::n48659(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t43::n22123(s, out),
		['п', s @ ..] => super::t43::n22124(s, out),
		['р', s @ ..] => super::t43::n22125(s, out),
		['с', s @ ..] => super::t43::n22126(s, out),
		['т', s @ ..] => super::t43::n22127(s, out),
		['х', s @ ..] => super::t43::n22128(s, out),
		['щ', s @ ..] => super::t43::n22129(s, out),
		['ь', s @ ..] => super::t43::n22130(s, out),
		_ => false,
	}
}

/// Words ending -ртка.
pub(crate) fn n5742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5727);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22004);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 22005);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 76781);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100868);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118010);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128801);
													match s {
														[] => true,
														['л', s @ ..] => super::t43::n136595(s, out),
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
									super::push(out, 100869);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 118011);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128802);
													s.is_empty()
												}
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
							super::push(out, 76782);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 76783);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100870);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118012);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22006);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48184);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 48185);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76784);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стка.
pub(crate) fn n5743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5728);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22007);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48186);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 76785);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100872);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118013);
											s.is_empty()
										}
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
					super::push(out, 48187);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 76786);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 100873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 48188);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76787);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 48189);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76788);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100875);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118014);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 76789);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 100876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48190);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76790);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100877);
									s.is_empty()
								}
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
			super::push(out, 22008);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48191);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76791);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 76792);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 100878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118015);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 76793);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 76794);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 48192);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 76795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 48193);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 48194);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76797);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 100880);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 118016);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128803);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22009);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48195);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 48196);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 48197);
					match s {
						[] => true,
						['н', 'и', s @ ..] => {
							match s {
								[] => true,
								['л', 'р', 'е', 'к', '\u{1}', s @ ..] => {
									super::push(out, 100881);
									s.is_empty()
								}
								['ф', 'р', 'е', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118017);
											s.is_empty()
										}
										['д', 'н', 'и', 'в', '\u{1}', s @ ..] => {
											super::push(out, 118018);
											s.is_empty()
										}
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
					super::push(out, 48198);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 48199);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 48200);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 48201);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 76798);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100882);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118019);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t43::n48699(s, out),
				['л', s @ ..] => super::t43::n48700(s, out),
				['м', s @ ..] => super::t43::n48701(s, out),
				['н', s @ ..] => super::t43::n48702(s, out),
				['о', 'г', s @ ..] => super::t43::n48703(s, out),
				['п', s @ ..] => super::t43::n48704(s, out),
				['р', s @ ..] => super::t43::n48705(s, out),
				['с', s @ ..] => super::t43::n48706(s, out),
				['т', s @ ..] => super::t43::n48707(s, out),
				['ф', s @ ..] => super::t43::n48708(s, out),
				['ц', s @ ..] => super::t43::n48709(s, out),
				['ч', s @ ..] => super::t43::n48710(s, out),
				['ш', s @ ..] => super::t43::n48711(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t43::n22137(s, out),
		['р', s @ ..] => super::t43::n22138(s, out),
		['у', s @ ..] => super::t43::n22139(s, out),
		['я', s @ ..] => super::t43::n22140(s, out),
		_ => false,
	}
}

/// Words ending -ттка.
pub(crate) fn n5744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5729);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22014);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утка.
pub(crate) fn n5745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5730);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22015);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22016);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22017);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 22018);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 48226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76823);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 100903);
									s.is_empty()
								}
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
			super::push(out, 22019);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 48227);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76824);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 76825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'я', s @ ..] => {
			super::push(out, 22020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22021);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48229);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 48230);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 48231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22022);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48232);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 76827);
							match s {
								[] => true,
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 100904);
									s.is_empty()
								}
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
			super::push(out, 22023);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48233);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 76828);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100905);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118038);
											s.is_empty()
										}
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
					super::push(out, 48234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22024);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48235);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76829);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 76830);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 100906);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118039);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 76831);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100907);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 100908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 76832);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 76833);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100909);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118040);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 76834);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 100910);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118041);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22025);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48236);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 48237);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76836);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100911);
									s.is_empty()
								}
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
			super::push(out, 22026);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 48238);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 22027);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 48239);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76838);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 100912);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118042);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128819);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 48240);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штка.
pub(crate) fn n5746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5731);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытка.
pub(crate) fn n5747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5732);
	match s {
		[] => true,
		['п', 'о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 48242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22029);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтка.
pub(crate) fn n5748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5733);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 22030);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 48244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этка.
pub(crate) fn n5749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5734);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 22031);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22032);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 48245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22033);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 48246);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютка.
pub(crate) fn n5750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5735);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 22034);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48247);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 76841);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 76842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 48248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76843);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100914);
									s.is_empty()
								}
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
			super::push(out, 22035);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 48249);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 76844);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 100915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятка.
pub(crate) fn n5751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5736);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48250);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 48251);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22036);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 48252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22037);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48253);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 76846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100916);
									s.is_empty()
								}
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
			super::push(out, 22038);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 48254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22039);
			s.is_empty()
		}
		['р', 'у', s @ ..] => {
			super::push(out, 22040);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 48255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22042);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 48257);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенографка.
pub(crate) fn n130172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128820);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 135204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жачка.
pub(crate) fn n22214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22084);
	match s {
		[] => true,
		['р', 'е', s @ ..] => {
			super::push(out, 48298);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48299);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 76881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зачка.
pub(crate) fn n22215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22085);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48300);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -качка.
pub(crate) fn n22216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22086);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48301);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 48302);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76883);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118045);
									s.is_empty()
								}
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
			super::push(out, 48303);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48304);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 48305);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76884);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 100929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118046);
									s.is_empty()
								}
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
			super::push(out, 48306);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48307);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76885);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118047);
									s.is_empty()
								}
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
			super::push(out, 48308);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лачка.
pub(crate) fn n22217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22087);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48309);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48310);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48311);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76889);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48312);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76890);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'к', 'с', s @ ..] => {
			super::push(out, 48313);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76891);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48314);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76892);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мачка.
pub(crate) fn n22218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22088);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48315);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48316);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 48317);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48318);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -начка.
pub(crate) fn n22219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22089);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48319);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76898);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пачка.
pub(crate) fn n22220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22090);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48321);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 48322);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76899);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 100937);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118048);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128821);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135205);
													match s {
														[] => true,
														['п', s @ ..] => super::t43::n140301(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48323);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48324);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76900);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48325);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76901);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48326);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76902);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рачка.
pub(crate) fn n22221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22091);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 48327);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 48328);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 48329);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 48330);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 48331);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 48332);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48333);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76904);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сачка.
pub(crate) fn n22222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22092);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 48334);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48335);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48336);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76907);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100942);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118049);
									s.is_empty()
								}
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
			super::push(out, 48337);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76908);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76909);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тачка.
pub(crate) fn n22223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22093);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 48338);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76910);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 76911);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100944);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118051);
									s.is_empty()
								}
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
			super::push(out, 48339);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76912);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100945);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хачка.
pub(crate) fn n22224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22094);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 48340);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76913);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ечка.
pub(crate) fn n5775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5760);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22095);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 48341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22096);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 48342);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22097);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 48344);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 48345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22098);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 48346);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 48347);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 76917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 48348);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22099);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 48349);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 48350);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 48351);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 48352);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76919);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 100946);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118053);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128822);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 48353);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 76920);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 100947);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118054);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128823);
													match s {
														[] => true,
														['с', s @ ..] => super::t43::n136612(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 48354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 76921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 48355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 22100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48356);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76922);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 100948);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 118055);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128824);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 48357);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 48358);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 48359);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 48360);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 76923);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 76924);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 100949);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118056);
											s.is_empty()
										}
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
					super::push(out, 48361);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 76925);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 100950);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118057);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128825);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t43::n136613(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22101);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 48362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 48363);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 76926);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 76927);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 100952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 48364);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 76928);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 100953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 48365);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 76929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 48366);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 76930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 100954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 76931);
							s.is_empty()
						}
						['р', s @ ..] => super::t43::n77747(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t43::n48867(s, out),
				['у', s @ ..] => super::t43::n48868(s, out),
				['ю', s @ ..] => super::t43::n48869(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t43::n22233(s, out),
		['о', s @ ..] => super::t43::n22234(s, out),
		['п', s @ ..] => super::t43::n22235(s, out),
		['р', s @ ..] => super::t43::n22236(s, out),
		['с', s @ ..] => super::t43::n22237(s, out),
		['т', s @ ..] => super::t43::n22238(s, out),
		['у', s @ ..] => {
			super::push(out, 22109);
			s.is_empty()
		}
		['ч', s @ ..] => super::t43::n22240(s, out),
		['ш', s @ ..] => super::t43::n22241(s, out),
		['щ', s @ ..] => super::t43::n22242(s, out),
		_ => false,
	}
}

